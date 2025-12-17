use sqlx::postgres::PgPool;
use std::sync::Arc;
use std::net::IpAddr;
use governor::{RateLimiter, state::keyed::DashMapStateStore, clock::DefaultClock};

use crate::sec::{SecurityConfig, TokenBlacklist, CsrfTokenStore};
use crate::calculus::beginner::BeginnerRegistry;
use crate::calculus::engineer::EngineeringRegistry;
use crate::calculus::contractor::ContractingRegistry;

/// Type alias for IP-based rate limiter using DashMap state store
pub type IpRateLimiter = Arc<RateLimiter<IpAddr, DashMapStateStore<IpAddr>, DefaultClock>>;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
    pub security_config: SecurityConfig,
    pub token_blacklist: TokenBlacklist,
    pub csrf_store: CsrfTokenStore,
    pub rate_limiter: IpRateLimiter,
    
    /// Beginner calculator registry - old system (wrapped in Arc for cloning)
    pub calculators_beginner: Arc<BeginnerRegistry>,
    
    /// Engineering calculator registry - new trait-based system
    pub calculators_engineer: Arc<EngineeringRegistry>,

    /// Contractor calculator registry
    pub calculators_contractor: Arc<ContractingRegistry>,
}