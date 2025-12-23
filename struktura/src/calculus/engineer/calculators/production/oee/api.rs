//! REST API using Axum
//! 
//! Nested router, no state, all translation-ready.
//! Accepts JSON, returns JSON, handles errors gracefully.

use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::state::AppState;

/// Create the OEE calculator API router
/// 
/// This router can be nested under a parent router:
/// ```
/// let app = Router::new()
///     .nest("/api/oee", oee::api::router());
/// ```
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/calculate", post(calculate_handler))
        .route("/calculate-with-economics", post(calculate_with_economics_handler))
        .route("/sensitivity", post(sensitivity_handler))
        .route("/leverage", post(leverage_handler))
}

/// Request body for OEE calculation
#[derive(Debug, Deserialize)]
pub struct CalculateRequest {
    pub input: crate::calculus::engineer::calculators::production::oee::OeeInput,
}

/// Response body for OEE calculation
#[derive(Debug, Serialize)]
pub struct CalculateResponse {
    pub result: crate::calculus::engineer::calculators::production::oee::OeeResult,
}

/// Request body for OEE calculation with economics
#[derive(Debug, Deserialize)]
pub struct CalculateWithEconomicsRequest {
    pub input: crate::calculus::engineer::calculators::production::oee::OeeInput,
    pub economic_parameters: crate::calculus::engineer::calculators::production::oee::domain::economics::EconomicParameters,
}

/// Request body for leverage analysis
#[derive(Debug, Deserialize)]
pub struct LeverageRequest {
    pub input: crate::calculus::engineer::calculators::production::oee::OeeInput,
}

/// Response body for leverage analysis
#[derive(Debug, Serialize)]
pub struct LeverageResponse {
    pub leverage_impacts: Vec<crate::calculus::engineer::calculators::production::oee::engine::leverage::LeverageImpact>,
}

/// Request body for sensitivity analysis
#[derive(Debug, Deserialize)]
pub struct SensitivityRequest {
    pub input: crate::calculus::engineer::calculators::production::oee::OeeInput,
    pub variation_percent: Option<f64>,
}

/// Response body for sensitivity analysis
#[derive(Debug, Serialize)]
pub struct SensitivityResponse {
    pub sensitivity_results: Vec<crate::calculus::engineer::calculators::production::oee::engine::sensitivity::SensitivityResult>,
}

/// Calculate OEE
async fn calculate_handler(
    Json(request): Json<CalculateRequest>,
) -> Result<Json<CalculateResponse>, ApiError> {
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(request.input)
        .map_err(ApiError::from)?;
    
    Ok(Json(CalculateResponse { result }))
}

/// Calculate OEE with economic analysis
async fn calculate_with_economics_handler(
    Json(request): Json<CalculateWithEconomicsRequest>,
) -> Result<Json<CalculateResponse>, ApiError> {
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee_with_economics(
        request.input,
        request.economic_parameters,
    ).map_err(ApiError::from)?;
    
    Ok(Json(CalculateResponse { result }))
}

/// Analyze leverage opportunities
async fn leverage_handler(
    Json(request): Json<LeverageRequest>,
) -> Result<Json<LeverageResponse>, ApiError> {
    // Calculate baseline first
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(request.input.clone())
        .map_err(ApiError::from)?;
    
    let leverage_impacts = crate::calculus::engineer::calculators::production::oee::engine::leverage::calculate_leverage(
        &request.input,
        &result.core_metrics,
    );
    
    Ok(Json(LeverageResponse { leverage_impacts }))
}

/// Analyze sensitivity
async fn sensitivity_handler(
    Json(request): Json<SensitivityRequest>,
) -> Result<Json<SensitivityResponse>, ApiError> {
    let sensitivity_results = crate::calculus::engineer::calculators::production::oee::engine::sensitivity::analyze_sensitivity(&request.input);
    
    Ok(Json(SensitivityResponse { sensitivity_results }))
}

/// API error type (translation-ready)
#[derive(Debug, Serialize)]
pub struct ApiError {
    /// Error code for programmatic handling
    pub code: String,
    /// Error message key for translation
    pub message_key: String,
    /// Parameters for translation
    pub params: serde_json::Value,
    /// HTTP status code
    #[serde(skip)]
    pub status: StatusCode,
}

impl ApiError {
    pub fn validation_failed(validation: crate::calculus::engineer::calculators::production::oee::validation::ValidationResult) -> Self {
        Self {
            code: "VALIDATION_FAILED".to_string(),
            message_key: "api.error.validation_failed".to_string(),
            params: serde_json::json!({
                "issues": validation.issues,
            }),
            status: StatusCode::BAD_REQUEST,
        }
    }
    
    pub fn calculation_error(message: &str) -> Self {
        Self {
            code: "CALCULATION_ERROR".to_string(),
            message_key: "api.error.calculation_error".to_string(),
            params: serde_json::json!({
                "message": message,
            }),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<crate::calculus::engineer::calculators::production::oee::engine::EngineError> for ApiError {
    fn from(err: crate::calculus::engineer::calculators::production::oee::engine::EngineError) -> Self {
        match err {
            crate::calculus::engineer::calculators::production::oee::engine::EngineError::ValidationFailed(validation) => {
                ApiError::validation_failed(validation)
            }
            crate::calculus::engineer::calculators::production::oee::engine::EngineError::InvalidInput(msg) => {
                ApiError::calculation_error(&msg)
            }
            crate::calculus::engineer::calculators::production::oee::engine::EngineError::CalculationError(msg) => {
                ApiError::calculation_error(&msg)
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status;
        let body = Json(self);
        (status, body).into_response()
    }
}