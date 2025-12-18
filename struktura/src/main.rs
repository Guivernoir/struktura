use axum::{
    http::{StatusCode, Method, HeaderValue, HeaderName},
    routing::{get, post, put},
    Router, middleware,
    response::{Html, IntoResponse},
    extract::Query,
};
use sqlx::postgres::PgPoolOptions; // Changed from just PgPool
use std::sync::Arc;
use anyhow::Context;
use tower_http::{
    trace::TraceLayer,
    compression::CompressionLayer,
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    timeout::TimeoutLayer,
    services::ServeDir,
};
use std::time::Duration;
use governor::Quota;
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod auth; 
pub mod stats;
pub mod sec;
pub mod state;
pub mod calculus;
//pub mod pricing;
pub mod seo;

use sec::{
    Claims, SecurityConfig, TokenBlacklist, CsrfTokenStore, 
    security_headers_middleware, rate_limit_middleware, csrf_protection_middleware,
};
use state::{AppState, IpRateLimiter};
use seo::{index_handler, sitemap_handler};

async fn health_check() -> axum::http::StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if it exists (requires dotenvy crate or similar)
    // dotenvy::dotenv().ok(); 

    // 1. Database Connection
    let database_url = std::env::var("DATABASE_URL")
        .context("DATABASE_URL environment variable must be set")?;

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .context("Failed to connect to Postgres")?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    // 2. Environment Configuration
    let jwt_secret = std::env::var("JWT_SECRET")
        .context("JWT_SECRET environment variable required")?;
    
    let allowed_origins_str = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    let allowed_origins: Vec<String> = allowed_origins_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    
    // 3. State Initialization
    let quota = Quota::per_minute(std::num::NonZeroU32::new(100).unwrap());
    let rate_limiter: IpRateLimiter = Arc::new(governor::RateLimiter::dashmap(quota));

    let security_config = SecurityConfig {
        allowed_origins: allowed_origins.clone(),
        hsts_max_age: 31536000,
    };
    
    // Initialize calculator registries
    let calculators_beginner = Arc::new(calculus::beginner::create_default_registry());
    let calculators_engineer = Arc::new(calculus::engineer::create_default_registry());
    let calculators_contractor = Arc::new(calculus::contractor::create_default_registry());
    
    let app_state = AppState {
        pool,
        jwt_secret,
        security_config: security_config.clone(),
        token_blacklist: TokenBlacklist::new(),
        csrf_store: CsrfTokenStore::new(),
        rate_limiter,
        calculators_beginner,
        calculators_engineer,
        calculators_contractor,
    };

    let shared_state = Arc::new(app_state);

    // 4. Middleware & Router Setup
    let cors_layer = tower_http::cors::CorsLayer::new()
        .allow_origin(
            allowed_origins
                .iter()
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse::<HeaderValue>().ok())
                .collect::<Vec<HeaderValue>>()
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            HeaderName::from_static("x-csrf-token"),
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    let middleware_stack = tower::ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(middleware::from_fn_with_state(shared_state.clone(), rate_limit_middleware))
        .layer(cors_layer)
        .layer(middleware::from_fn_with_state(shared_state.clone(), security_headers_middleware))
        .layer(TimeoutLayer::new(Duration::from_secs(30)));

    let public_routes = Router::new()
        .route("/signup", post(auth::signup_handler))
        .route("/login", post(auth::login_handler))
        .route("/csrf", get(auth::get_csrf_token_handler))
        .route("/sitemap.xml", get(sitemap_handler))
        .route("/health", get(|| async { StatusCode::OK }));

    let protected_routes = Router::new()
        .route("/profile/me", get(auth::get_my_profile_handler))
        .route("/profile/update", put(auth::update_profile_handler))
        .route("/stats/me", get(stats::get_my_usage_stats_handler))
        .route("/logout", post(auth::logout_handler))
        .route_layer(middleware::from_fn_with_state(shared_state.clone(), csrf_protection_middleware))
        .layer(middleware::from_extractor_with_state::<Claims, Arc<AppState>>(shared_state.clone()));

    // Create calculator routers
    let beginner_router = calculus::beginner::create_router();
    let engineer_router = calculus::engineer::create_router();
    let contractor_router = calculus::contractor::create_router();

    // Get registry stats for startup banner
    let engineer_stats = shared_state.calculators_engineer.stats();
    let beginner_stats = shared_state.calculators_beginner.stats();
    let contractor_stats = shared_state.calculators_contractor.stats();

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/health", get(health_check))
        .nest_service("/assets", ServeDir::new("static/dist/assets"))
        .nest_service("/fonts", ServeDir::new("static/dist/fonts"))
        .nest_service("/favicon", ServeDir::new("static/dist/favicon"))
        .fallback(index_handler)
        .nest("/api/v1/auth", public_routes)
        .nest("/api/v1/user", protected_routes)
        .nest("/api/v1/calculus/beginner", beginner_router)
        .nest("/api/v1/calculus/engineer", engineer_router)
        .nest("/api/v1/calculus/contractor", contractor_router)
        .with_state(shared_state.clone())
        .layer(middleware_stack);

    // 5. Server Startup
    let port_str = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let port = port_str.parse::<u16>().context("Invalid PORT environment variable")?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("\n╔═══════════════════════════════════════════════════╗");
    println!("║              SECURITY MANIFEST v2.3               ║");
    println!("╠═══════════════════════════════════════════════════╣");
    println!("║ ✓ System               : Standalone (Tokio)      ║");
    println!("║ ✓ Port                 : {:<24}║", port);
    println!("║ ✓ JWT Auth (HS384) + Session Binding             ║");
    println!("║ ✓ CSRF Protection (Token-in-Header)              ║");
    println!("║ ✓ Rate Limiting (100 req/min per IP)             ║");
    println!("║ ✓ Security Headers (HSTS, CSP, X-Frame, etc.)    ║");
    println!("║ ✓ Argon2id Password Hashing                      ║");
    println!("║ ✓ Timing Attack Prevention                       ║");
    println!("║ ✓ SPA Routing (Client-Side Fallback)             ║");
    println!("║ ✓ Static Asset Serving (/assets/*)               ║");
    println!("╠═══════════════════════════════════════════════════╣");
    println!("║              CALCULUS MODULES                     ║");
    println!("╠═══════════════════════════════════════════════════╣");
    println!("║ ✓ Beginner Module ({} calculators)                ║", beginner_stats.total_calculators);
    println!("║ ✓ Contractor Module({} calculators)              ║", contractor_stats.total_calculators);
    println!("║ ✓ Engineer Module ({} calculators)              ║", engineer_stats.total_calculators);
    println!("║   • Civil Engineering: {}                        ║", 
        engineer_stats.by_category.get("civil").unwrap_or(&0));
    println!("║   • Structural Engineering: {}                   ║",
        engineer_stats.by_category.get("structural").unwrap_or(&0));
    println!("║   • Mechanical Engineering: {}                   ║",
        engineer_stats.by_category.get("mechanical").unwrap_or(&0));
    println!("║   • Production Engineering: {}                   ║",
        engineer_stats.by_category.get("production").unwrap_or(&0));
    println!("╚═══════════════════════════════════════════════════╝\n");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}