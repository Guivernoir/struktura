use axum::{
    extract::{FromRequestParts, State},
    http::{
        header::{AUTHORIZATION, HeaderMap, HeaderValue, HeaderName},
        request::Parts,
        StatusCode, Method,
    },
    middleware::Next,
    response::{IntoResponse, Response, Json},
};
use jsonwebtoken::{decode, encode, Header, EncodingKey, DecodingKey, Validation, Algorithm, TokenData};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString, Error as ArgonError};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use lazy_static::lazy_static;
use crate::state::AppState;
use validator::ValidationErrors;
use rand::Rng;

// =============================================================================
// ERROR HANDLING
// =============================================================================

#[derive(Debug)]
pub enum AppError {
    InvalidCredentials,
    MissingToken,
    InvalidToken,
    ExpiredToken,
    BlacklistedToken,
    UserNotFound,
    MissingCsrf,
    InvalidCsrf,
    ValidationError(ValidationErrors),
    DbError(sqlx::Error),
    PasswordError(ArgonError),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AppError::MissingToken | AppError::InvalidToken | AppError::ExpiredToken | AppError::BlacklistedToken => {
                (StatusCode::UNAUTHORIZED, "Authentication failed")
            }
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::MissingCsrf | AppError::InvalidCsrf => (StatusCode::FORBIDDEN, "CSRF validation failed"),
            AppError::ValidationError(ref e) => {
                return (StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({"error": e.to_string()}))).into_response();
            }
            AppError::DbError(ref e) => {
                eprintln!("[DB_ERROR] {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::PasswordError(ref e) => {
                eprintln!("[PASS_ERROR] {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Password error")
            }
            AppError::Internal(ref msg) => {
                eprintln!("[INTERNAL] {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
            }
        };
        (status, Json(serde_json::json!({"error": msg}))).into_response()
    }
}

impl From<sqlx::Error> for AppError { fn from(e: sqlx::Error) -> Self { AppError::DbError(e) } }
impl From<ArgonError> for AppError { fn from(e: ArgonError) -> Self { AppError::PasswordError(e) } }
impl From<ValidationErrors> for AppError { fn from(e: ValidationErrors) -> Self { AppError::ValidationError(e) } }

// =============================================================================
// JWT CLAIMS
// =============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
    pub session_fp: String,
}

pub fn generate_jwt(user_id: String, username: String, session_fp: String, jwt_secret: &str) -> Result<(String, Claims), AppError> {
    let now = OffsetDateTime::now_utc();
    let exp = (now + Duration::days(7)).unix_timestamp() as usize;
    let iat = now.unix_timestamp() as usize;
    let jti = Uuid::new_v4().to_string();

    let claims = Claims { sub: user_id, username, exp, iat, jti, session_fp };

    let token = encode(
        &Header::new(Algorithm::HS384),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).map_err(|e| AppError::Internal(format!("JWT encode: {}", e)))?;

    Ok((token, claims))
}

impl FromRequestParts<Arc<AppState>> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &Arc<AppState>) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .filter(|s| s.starts_with("Bearer "))
            .map(|s| s.trim_start_matches("Bearer "))
            .ok_or(AppError::MissingToken)?;

        let validation = Validation::new(Algorithm::HS384);
        let token_data: TokenData<Claims> = decode(
            auth_header,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &validation,
        ).map_err(|e| {
            log_security_event("JWT_FAIL", None, None, &format!("{}", e));
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::ExpiredToken,
                _ => AppError::InvalidToken,
            }
        })?;

        let claims = token_data.claims;

        if state.token_blacklist.is_revoked(&claims.jti) {
            log_security_event("TOKEN_REVOKED", Some(&claims.username), None, "Blacklisted token used");
            return Err(AppError::BlacklistedToken);
        }

        let (ip, ua_hash) = extract_ip_and_ua(&parts.headers)?;
        let current_fp = compute_session_fingerprint(ip.as_deref(), ua_hash.as_deref());

        if claims.session_fp != current_fp {
            log_security_event("SESSION_HIJACK", Some(&claims.username), ip.as_deref(), "Fingerprint mismatch");
            state.token_blacklist.revoke(&claims.jti);
            return Err(AppError::InvalidToken);
        }

        Ok(claims)
    }
}

// =============================================================================
// PASSWORD HASHING
// =============================================================================

pub struct PasswordManager;

impl PasswordManager {
    pub fn hash_password(password: &str) -> Result<String, ArgonError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
        Ok(hash.to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, ArgonError> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

// =============================================================================
// TOKEN BLACKLIST
// =============================================================================

lazy_static! {
    static ref BLACKLIST: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

#[derive(Clone)]
pub struct TokenBlacklist;

impl TokenBlacklist {
    pub fn new() -> Self { Self }
    pub fn revoke(&self, jti: &str) {
        if let Ok(mut list) = BLACKLIST.write() {
            list.insert(jti.to_string());
        }
    }
    pub fn is_revoked(&self, jti: &str) -> bool {
        BLACKLIST.read().map(|list| list.contains(jti)).unwrap_or(false)
    }
}

// =============================================================================
// CSRF STORE
// =============================================================================

lazy_static! {
    static ref CSRF_TOKENS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

#[derive(Clone)]
pub struct CsrfTokenStore;

impl CsrfTokenStore {
    pub fn new() -> Self { Self }
    
    pub fn generate_and_store(&self, user_id: &str) -> Result<String, AppError> {
        let token = Uuid::new_v4().to_string();
        let hash = hash_csrf_token(&token);
        CSRF_TOKENS.write()
            .map_err(|_| AppError::Internal("CSRF write lock".into()))?
            .insert(user_id.to_string(), hash);
        Ok(token)
    }

    // NEW: Function to generate a token without storing it, for unauthenticated use
    // NOTE: This token cannot be validated by the validate_token method.
    pub fn generate_unauthenticated_token(&self) -> String {
        Uuid::new_v4().to_string()
    }
    
    pub fn validate_token(&self, user_id: &str, token: &str) -> bool {
        let store = match CSRF_TOKENS.read() {
            Ok(s) => s,
            Err(_) => return false,
        };
        
        store.get(user_id)
            .map(|stored| *stored == hash_csrf_token(token))
            .unwrap_or(false)
    }
    
    pub fn invalidate_token(&self, user_id: &str) {
        if let Ok(mut store) = CSRF_TOKENS.write() {
            store.remove(user_id);
        }
    }
}

fn hash_csrf_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    general_purpose::STANDARD.encode(hasher.finalize())
}

// =============================================================================
// SECURITY CONFIG
// =============================================================================

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub allowed_origins: Vec<String>,
    pub hsts_max_age: u64,
}

// =============================================================================
// UTILITIES
// =============================================================================

pub fn extract_ip_and_ua(headers: &HeaderMap) -> Result<(Option<String>, Option<String>), AppError> {
    let ip = headers.get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|h| h.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string());

    let ua_hash = headers.get(axum::http::header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .map(|ua| {
            let mut hasher = Sha256::new();
            hasher.update(ua.as_bytes());
            general_purpose::STANDARD.encode(hasher.finalize())
        });
    
    Ok((ip, ua_hash))
}

pub fn compute_session_fingerprint(ip: Option<&str>, ua_hash: Option<&str>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(ip.unwrap_or("unknown").as_bytes());
    hasher.update(ua_hash.unwrap_or("unknown").as_bytes());
    general_purpose::STANDARD.encode(hasher.finalize())
}

pub fn log_security_event(event: &str, user: Option<&str>, ip: Option<&str>, details: &str) {
    eprintln!("[AUDIT] {} | {} | User: {} | IP: {} | {}", 
        OffsetDateTime::now_utc(), event, 
        user.unwrap_or("anon"), ip.unwrap_or("unknown"), details
    );
}

pub async fn traffic_jitter(delay_ms: u64) {
    let jitter = rand::rng().random_range(0..10);
    tokio::time::sleep(std::time::Duration::from_millis(delay_ms + jitter)).await;
}

// =============================================================================
// MIDDLEWARE
// =============================================================================

pub async fn csrf_protection_middleware(
    State(app_state): State<Arc<AppState>>,
    claims: Claims,
    headers: HeaderMap,
    request: axum::extract::Request,
    next: Next,
) -> Result<Response, AppError> {
    let method = request.method();
    
    if matches!(*method, Method::GET | Method::HEAD | Method::OPTIONS) {
        return Ok(next.run(request).await);
    }
    
    let csrf_token = headers.get("x-csrf-token")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::MissingCsrf)?;

    if !app_state.csrf_store.validate_token(&claims.sub, csrf_token) {
        return Err(AppError::InvalidCsrf);
    }

    Ok(next.run(request).await)
}

pub async fn rate_limit_middleware(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: axum::extract::Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = headers.get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));
    
    match app_state.rate_limiter.check_key(&ip) {
        Ok(_) => Ok(next.run(request).await),
        Err(_) => {
            log_security_event("RATE_LIMIT", None, Some(&ip.to_string()), "Rate limited");
            Err(StatusCode::TOO_MANY_REQUESTS)
        }
    }
}

pub async fn security_headers_middleware(
    State(app_state): State<Arc<AppState>>,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    let config = &app_state.security_config;
    
    // HSTS: Force HTTPS
    if let Ok(hsts) = HeaderValue::from_str(&format!("max-age={}", config.hsts_max_age)) {
        headers.insert(HeaderName::from_static("strict-transport-security"), hsts);
    }
    
    // Prevent MIME type sniffing
    headers.insert(
        HeaderName::from_static("x-content-type-options"), 
        HeaderValue::from_static("nosniff")
    );
    
    // CSP: Allow external JS/CSS files from /assets
    // No inline scripts/styles are used by Vite's production build
    let csp = "default-src 'self'; \
               script-src 'self'; \
               style-src 'self'; \
               img-src 'self' data: https:; \
               font-src 'self' data:; \
               connect-src 'self'; \
               frame-ancestors 'none'; \
               base-uri 'self'; \
               form-action 'self'";
    
    if let Ok(csp_value) = HeaderValue::from_str(csp) {
        headers.insert(
            HeaderName::from_static("content-security-policy"), 
            csp_value
        );
    }
    
    // Prevent clickjacking
    headers.insert(
        HeaderName::from_static("x-frame-options"), 
        HeaderValue::from_static("DENY")
    );
    
    // Referrer policy
    headers.insert(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin")
    );
    
    // Permissions policy (disable unnecessary features)
    headers.insert(
        HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("geolocation=(), microphone=(), camera=()")
    );

    response
}