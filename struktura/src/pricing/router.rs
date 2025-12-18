use crate::pricing::{
    errors::PricingError,
    models::*,
    registry::PriceProviderRegistry,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// API router for pricing endpoints
pub fn create_pricing_router(registry: Arc<PriceProviderRegistry>) -> Router {
    Router::new()
        .route("/prices", post(fetch_prices))
        .route("/prices/single", get(fetch_single_price))
        .route("/providers", get(list_providers))
        .route("/providers/:name", get(get_provider_info))
        .route("/stats", get(get_stats))
        .route("/health", get(health_check))
        .with_state(registry)
}

/// Fetch multiple prices
async fn fetch_prices(
    State(registry): State<Arc<PriceProviderRegistry>>,
    Json(request): Json<PriceRequest>,
) -> Result<Json<PriceResponse>, ApiError> {
    let response = registry.fetch_prices(&request).await?;
    Ok(Json(response))
}

/// Query parameters for single price fetch
#[derive(Debug, Deserialize)]
struct SinglePriceQuery {
    category: String,
    item_code: String,
    unit: String,
    country_code: String,
    region: Option<String>,
    city: Option<String>,
    currency: Option<String>,
}

/// Fetch single price (convenience endpoint)
async fn fetch_single_price(
    State(registry): State<Arc<PriceProviderRegistry>>,
    Query(params): Query<SinglePriceQuery>,
) -> Result<Json<Option<PriceInfo>>, ApiError> {
    // Parse category
    let category = match params.category.as_str() {
        "ductwork" => ItemCategory::Ductwork,
        "registers" => ItemCategory::Registers,
        "vents" => ItemCategory::Vents,
        "dampers" => ItemCategory::Dampers,
        "labor" => ItemCategory::Labor,
        "equipment" => ItemCategory::Equipment,
        _ => ItemCategory::Custom(params.category.clone()),
    };
    
    let item_id = PriceItemId::new(category, params.item_code, params.unit);
    
    let mut location = Location::new(params.country_code);
    if let Some(region) = params.region {
        location = location.with_region(region);
    }
    if let Some(city) = params.city {
        location = location.with_city(city);
    }
    
    let mut request = PriceRequest::new(location).add_item(item_id);
    
    if let Some(currency_code) = params.currency {
        let currency = match currency_code.as_str() {
            "USD" => Currency::USD,
            "BRL" => Currency::BRL,
            "EUR" => Currency::EUR,
            "GBP" => Currency::GBP,
            _ => return Err(ApiError::InvalidInput("Unsupported currency".to_string())),
        };
        request = request.with_currency(currency);
    }
    
    let response = registry.fetch_prices(&request).await?;
    Ok(Json(response.prices.into_iter().next()))
}

/// List all providers
async fn list_providers(
    State(registry): State<Arc<PriceProviderRegistry>>,
) -> Result<Json<Vec<String>>, ApiError> {
    let providers = registry.list_providers().await;
    Ok(Json(providers))
}

/// Response for provider info
#[derive(Debug, Serialize)]
struct ProviderInfoResponse {
    name: String,
    config: ProviderConfig,
}

/// Get provider information
async fn get_provider_info(
    State(registry): State<Arc<PriceProviderRegistry>>,
    Path(name): Path<String>,
) -> Result<Json<ProviderInfoResponse>, ApiError> {
    let provider = registry.get_provider(&name).await?;
    
    Ok(Json(ProviderInfoResponse {
        name: provider.name().to_string(),
        config: provider.config().clone(),
    }))
}

/// Get registry statistics
async fn get_stats(
    State(registry): State<Arc<PriceProviderRegistry>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let stats = registry.stats().await;
    
    let mut response = serde_json::json!({
        "provider_count": stats.provider_count,
        "enabled_providers": stats.enabled_providers,
    });
    
    if let Some(cache_stats) = stats.cache_stats {
        response["cache"] = serde_json::json!({
            "hits": cache_stats.hits,
            "misses": cache_stats.misses,
            "entries": cache_stats.entries,
            "hit_rate": cache_stats.hit_rate,
        });
    }
    
    Ok(Json(response))
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "pricing-engine"
    }))
}

/// API error wrapper
#[derive(Debug)]
pub enum ApiError {
    Pricing(PricingError),
    InvalidInput(String),
}

impl From<PricingError> for ApiError {
    fn from(err: PricingError) -> Self {
        ApiError::Pricing(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Pricing(e) => match e {
                PricingError::ProviderNotFound(_) => (StatusCode::NOT_FOUND, e.to_string()),
                PricingError::LocationNotSupported(_, _) => (StatusCode::BAD_REQUEST, e.to_string()),
                PricingError::ItemNotFound(_) => (StatusCode::NOT_FOUND, e.to_string()),
                PricingError::InvalidLocation(_) => (StatusCode::BAD_REQUEST, e.to_string()),
                PricingError::FetchTimeout(_) => (StatusCode::GATEWAY_TIMEOUT, e.to_string()),
                PricingError::NetworkError(_) => (StatusCode::BAD_GATEWAY, e.to_string()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            },
            ApiError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
        };
        
        let body = Json(serde_json::json!({
            "error": message
        }));
        
        (status, body).into_response()
    }
}