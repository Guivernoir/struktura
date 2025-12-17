use axum::{
    extract::State,
    http::{StatusCode, HeaderMap, HeaderValue},
    response::{Json, IntoResponse},
};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use std::sync::Arc;
use validator::Validate;
use uuid::Uuid;

use crate::sec::{self, AppError, Claims, PasswordManager};
use crate::state::AppState;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct AuthPayload {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 12))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct UpdateProfilePayload {
    #[validate(length(max = 100))]
    pub fav_experience_level: Option<String>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: Option<String>,
    pub csrf_token: Option<String>,
    pub user_profile: Option<UserProfile>,
}

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct UserProfile {
    pub username: String,
    pub is_pro: Option<bool>,
    pub fav_experience_level: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

// =============================================================================
// HANDLERS
// =============================================================================

pub async fn signup_handler(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthResponse>, AppError> {
    
    payload.validate()?;
    
    let (ip, ua_hash) = sec::extract_ip_and_ua(&headers)?;
    let ip_str = ip.as_deref();

    let user_exists: bool = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
        payload.username
    )
    .fetch_one(&app_state.pool)
    .await?
    .unwrap_or(false);

    if user_exists {
        sec::log_security_event("SIGNUP_DUPLICATE", Some(&payload.username), ip_str, "Username exists");
        sec::traffic_jitter(100).await;
        return Err(AppError::InvalidCredentials);
    }

    let hashed_password = PasswordManager::hash_password(&payload.password)?;

    let profile = sqlx::query!(
        r#"
        INSERT INTO users (username, hash)
        VALUES ($1, $2)
        RETURNING id, username, is_pro, fav_experience_level, created_at
        "#,
        payload.username,
        hashed_password
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|e| {
        sec::log_security_event("SIGNUP_DB_ERROR", Some(&payload.username), ip_str, &e.to_string());
        AppError::DbError(e)
    })?;

    let session_fp = sec::compute_session_fingerprint(ip.as_deref(), ua_hash.as_deref());
    let (token, claims) = sec::generate_jwt(
        profile.id.to_string(),
        profile.username.clone(),
        session_fp,
        &app_state.jwt_secret
    )?;
    let csrf_token = app_state.csrf_store.generate_and_store(&claims.sub)?;

    sec::log_security_event("USER_SIGNUP", Some(&payload.username), ip_str, "Success");

    Ok(Json(AuthResponse {
        message: "Registered successfully".to_string(),
        token: Some(token),
        csrf_token: Some(csrf_token),
        user_profile: Some(UserProfile {
            username: profile.username,
            is_pro: profile.is_pro,
            fav_experience_level: profile.fav_experience_level,
            created_at: profile.created_at,
        }),
    }))
}

pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthResponse>, AppError> {

    payload.validate()?;

    let (ip, ua_hash) = sec::extract_ip_and_ua(&headers)?;
    let ip_str = ip.as_deref();

    let user_record = sqlx::query!(
        r#"
        SELECT id, username, hash, is_pro, fav_experience_level, created_at 
        FROM users 
        WHERE username = $1
        "#,
        payload.username
    )
    .fetch_optional(&app_state.pool)
    .await?;

    let user_record = match user_record {
        Some(r) => r,
        None => {
            sec::log_security_event("LOGIN_FAIL", Some(&payload.username), ip_str, "User not found");
            sec::traffic_jitter(100).await;
            return Err(AppError::InvalidCredentials);
        }
    };

    let password_verified = PasswordManager::verify_password(&payload.password, &user_record.hash)?;
    
    if !password_verified {
        sec::log_security_event("LOGIN_FAIL", Some(&payload.username), ip_str, "Bad password");
        sec::traffic_jitter(200).await;
        return Err(AppError::InvalidCredentials);
    }

    let session_fp = sec::compute_session_fingerprint(ip.as_deref(), ua_hash.as_deref());
    let (token, claims) = sec::generate_jwt(
        user_record.id.to_string(),
        user_record.username.clone(),
        session_fp,
        &app_state.jwt_secret
    )?;
    let csrf_token = app_state.csrf_store.generate_and_store(&claims.sub)?;

    sec::log_security_event("USER_LOGIN", Some(&claims.username), ip_str, "Success");

    Ok(Json(AuthResponse {
        message: "Login successful".to_string(),
        token: Some(token),
        csrf_token: Some(csrf_token),
        user_profile: Some(UserProfile {
            username: user_record.username,
            is_pro: user_record.is_pro,
            fav_experience_level: user_record.fav_experience_level,
            created_at: user_record.created_at,
        }),
    }))
}

pub async fn logout_handler(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    claims: Claims,
) -> Result<StatusCode, AppError> {
    
    app_state.token_blacklist.revoke(&claims.jti);
    app_state.csrf_store.invalidate_token(&claims.sub);
    
    let (ip, _) = sec::extract_ip_and_ua(&headers)?;
    sec::log_security_event("USER_LOGOUT", Some(&claims.username), ip.as_deref(), "Success");

    Ok(StatusCode::OK)
}

pub async fn get_my_profile_handler(
    State(app_state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<Json<AuthResponse>, AppError> {

    let user_uuid = Uuid::parse_str(&claims.sub).map_err(|_| AppError::InvalidToken)?;

    let profile = sqlx::query_as!(
        UserProfile,
        r#"
        SELECT username, is_pro, fav_experience_level, created_at 
        FROM users 
        WHERE id = $1
        "#,
        user_uuid
    )
    .fetch_optional(&app_state.pool)
    .await?
    .ok_or(AppError::UserNotFound)?;
    
    sec::log_security_event("PROFILE_FETCH", Some(&claims.username), None, "Success");

    Ok(Json(AuthResponse {
        message: "Profile retrieved".to_string(),
        token: None,
        csrf_token: None,
        user_profile: Some(profile),
    }))
}

pub async fn update_profile_handler(
    State(app_state): State<Arc<AppState>>,
    claims: Claims,
    headers: HeaderMap,
    Json(payload): Json<UpdateProfilePayload>,
) -> Result<Json<AuthResponse>, AppError> {

    payload.validate()?;
    
    let (ip, _) = sec::extract_ip_and_ua(&headers)?;
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::InvalidToken)?;

    let updated_profile = sqlx::query_as!(
        UserProfile,
        r#"
        UPDATE users 
        SET fav_experience_level = $1
        WHERE id = $2
        RETURNING username, is_pro, fav_experience_level, created_at
        "#,
        payload.fav_experience_level,
        user_id
    )
    .fetch_optional(&app_state.pool)
    .await?
    .ok_or(AppError::UserNotFound)?;

    sec::log_security_event("PROFILE_UPDATE", Some(&claims.username), ip.as_deref(), "Success");

    Ok(Json(AuthResponse {
        message: "Profile updated".to_string(),
        token: None,
        csrf_token: None,
        user_profile: Some(updated_profile),
    }))
}

pub async fn get_csrf_token_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> { // Changed to AppError to allow ? operator on generate_and_store
    // The previous fix (requiring Claims) failed because this is called before login.
    // The best approach for unauthenticated endpoints (like login/signup) is to
    // generate a token without linking it to a user_id, which will be validated
    // by the CSRF middleware on the next authenticated request (which also regenerates the token).

    let token = state.csrf_store.generate_unauthenticated_token();
    
    let mut headers = HeaderMap::new();
    
    // We must handle the HeaderValue::from_str error to avoid crashing
    let header_value = HeaderValue::from_str(&token)
        .map_err(|e| AppError::Internal(format!("Failed to create header value: {}", e)))?;
        
    headers.insert("X-CSRF-Token", header_value);
    
    Ok((
        headers,
        Json(serde_json::json!({ "csrf_token": token }))
    ))
}