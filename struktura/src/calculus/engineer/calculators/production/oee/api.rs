//! REST API using Axum
//! 
//! Nested router, no state, all translation-ready.
//! Accepts JSON, returns JSON, handles errors gracefully.
//! 
//! Now includes:
//! - Core OEE calculation
//! - Economic analysis
//! - Sensitivity analysis
//! - Temporal scrap analysis
//! - Multi-machine system analysis
//! - Leverage analysis

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
        // Core OEE calculations
        .route("/calculate", post(calculate_handler))
        .route("/calculate-with-economics", post(calculate_with_economics_handler))
        .route("/calculate-full", post(calculate_full_handler))
        
        // Analysis endpoints
        .route("/sensitivity", post(sensitivity_handler))
        .route("/leverage", post(leverage_handler))
        .route("/temporal-scrap", post(temporal_scrap_handler))
        
        // Multi-machine endpoints
        .route("/system/aggregate", post(system_aggregate_handler))
        .route("/system/compare-methods", post(system_compare_methods_handler))
}

// ============================================================================
// Core OEE Calculation Endpoints
// ============================================================================

/// Request body for basic OEE calculation
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

/// Request body for full analysis (OEE + all optional analyses)
#[derive(Debug, Deserialize)]
pub struct CalculateFullRequest {
    pub input: crate::calculus::engineer::calculators::production::oee::OeeInput,
    pub economic_parameters: Option<crate::calculus::engineer::calculators::production::oee::domain::economics::EconomicParameters>,
    /// Include sensitivity analysis (default: true)
    pub include_sensitivity: Option<bool>,
    /// Sensitivity variation percentage (default: 10.0)
    pub sensitivity_variation: Option<f64>,
    /// Include temporal scrap analysis if data available (default: true)
    pub include_temporal_scrap: Option<bool>,
}

/// Response body for full analysis
#[derive(Debug, Serialize)]
pub struct CalculateFullResponse {
    pub result: crate::calculus::engineer::calculators::production::oee::OeeResult,
    pub sensitivity_analysis: Option<crate::calculus::engineer::calculators::production::oee::engine::sensitivity::SensitivityAnalysis>,
    pub temporal_scrap_analysis: Option<crate::calculus::engineer::calculators::production::oee::engine::temporal_scrap::TemporalScrapAnalysis>,
}

/// Calculate basic OEE
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

/// Calculate OEE with all optional analyses
async fn calculate_full_handler(
    Json(request): Json<CalculateFullRequest>,
) -> Result<Json<CalculateFullResponse>, ApiError> {
    // Calculate base OEE (with or without economics)
    let mut result = if let Some(economic_params) = request.economic_parameters {
        crate::calculus::engineer::calculators::production::oee::engine::calculate_oee_with_economics(
            request.input.clone(),
            economic_params,
        ).map_err(ApiError::from)?
    } else {
        crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(request.input.clone())
            .map_err(ApiError::from)?
    };
    
    // Sensitivity analysis
    let sensitivity_analysis = if request.include_sensitivity.unwrap_or(true) {
        let variation = request.sensitivity_variation.unwrap_or(10.0);
        Some(crate::calculus::engineer::calculators::production::oee::engine::sensitivity::analyze_sensitivity(
            &request.input,
            &result.core_metrics,
            variation,
        ))
    } else {
        None
    };
    
    // Temporal scrap analysis (if data available)
    let temporal_scrap_analysis = if request.include_temporal_scrap.unwrap_or(true) {
        // Check if temporal scrap data is available in the input
        // Note: This assumes temporal_scrap field exists in ProductionSummary
        // You'll need to add this field per the IMPLEMENTATION_GUIDE
        None // Placeholder - would check input.production.temporal_scrap
    } else {
        None
    };
    
    Ok(Json(CalculateFullResponse {
        result,
        sensitivity_analysis,
        temporal_scrap_analysis,
    }))
}

// ============================================================================
// Sensitivity Analysis Endpoint
// ============================================================================

/// Request body for sensitivity analysis
#[derive(Debug, Deserialize)]
pub struct SensitivityRequest {
    pub input: crate::calculus::engineer::calculators::production::oee::OeeInput,
    /// Variation percentage (default: 10.0 for ±10%)
    pub variation_percent: Option<f64>,
}

/// Response body for sensitivity analysis
#[derive(Debug, Serialize)]
pub struct SensitivityResponse {
    pub analysis: crate::calculus::engineer::calculators::production::oee::engine::sensitivity::SensitivityAnalysis,
}

/// Analyze parameter sensitivity
async fn sensitivity_handler(
    Json(request): Json<SensitivityRequest>,
) -> Result<Json<SensitivityResponse>, ApiError> {
    // Calculate baseline metrics first
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(request.input.clone())
        .map_err(ApiError::from)?;
    
    // Run sensitivity analysis
    let variation = request.variation_percent.unwrap_or(10.0);
    let analysis = crate::calculus::engineer::calculators::production::oee::engine::sensitivity::analyze_sensitivity(
        &request.input,
        &result.core_metrics,
        variation,
    );
    
    Ok(Json(SensitivityResponse { analysis }))
}

// ============================================================================
// Leverage Analysis Endpoint
// ============================================================================

/// Request body for leverage analysis
#[derive(Debug, Deserialize)]
pub struct LeverageRequest {
    pub input: crate::calculus::engineer::calculators::production::oee::OeeInput,
}

/// Response body for leverage analysis
#[derive(Debug, Serialize)]
pub struct LeverageResponse {
    pub leverage_impacts: Vec<crate::calculus::engineer::calculators::production::oee::engine::leverage::LeverageImpact>,
    pub baseline_oee: f64,
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
    
    Ok(Json(LeverageResponse {
        leverage_impacts,
        baseline_oee: result.core_metrics.oee.value * 100.0,
    }))
}

// ============================================================================
// Temporal Scrap Analysis Endpoint
// ============================================================================

/// Request body for temporal scrap analysis
#[derive(Debug, Deserialize)]
pub struct TemporalScrapRequest {
    pub scrap_data: crate::calculus::engineer::calculators::production::oee::engine::temporal_scrap::TemporalScrapData,
    pub ideal_cycle_time: std::time::Duration,
    pub startup_config: Option<crate::calculus::engineer::calculators::production::oee::engine::temporal_scrap::StartupWindowConfig>,
}

/// Response body for temporal scrap analysis
#[derive(Debug, Serialize)]
pub struct TemporalScrapResponse {
    pub analysis: crate::calculus::engineer::calculators::production::oee::engine::temporal_scrap::TemporalScrapAnalysis,
}

/// Analyze temporal scrap patterns
async fn temporal_scrap_handler(
    Json(request): Json<TemporalScrapRequest>,
) -> Result<Json<TemporalScrapResponse>, ApiError> {
    let config = request.startup_config
        .unwrap_or_else(|| crate::calculus::engineer::calculators::production::oee::engine::temporal_scrap::StartupWindowConfig::default());
    
    let analysis = crate::calculus::engineer::calculators::production::oee::engine::temporal_scrap::analyze_temporal_scrap(
        &request.scrap_data,
        request.ideal_cycle_time,
        &config,
    );
    
    Ok(Json(TemporalScrapResponse { analysis }))
}

// ============================================================================
// Multi-Machine System Analysis Endpoints
// ============================================================================

/// Request body for system aggregation
#[derive(Debug, Deserialize)]
pub struct SystemAggregateRequest {
    pub machines: Vec<crate::calculus::engineer::calculators::production::oee::engine::multi_machine::MachineOeeData>,
    pub aggregation_method: crate::calculus::engineer::calculators::production::oee::engine::multi_machine::AggregationMethod,
}

/// Response body for system aggregation
#[derive(Debug, Serialize)]
pub struct SystemAggregateResponse {
    pub analysis: crate::calculus::engineer::calculators::production::oee::engine::multi_machine::SystemOeeAnalysis,
}

/// Request body for comparing aggregation methods
#[derive(Debug, Deserialize)]
pub struct SystemCompareMethodsRequest {
    pub machines: Vec<crate::calculus::engineer::calculators::production::oee::engine::multi_machine::MachineOeeData>,
}

/// Response body for method comparison
#[derive(Debug, Serialize)]
pub struct SystemCompareMethodsResponse {
    pub comparisons: std::collections::HashMap<
        String,  // Method name
        SystemMethodComparison,
    >,
    pub recommended_method: String,
}

#[derive(Debug, Serialize)]
pub struct SystemMethodComparison {
    pub method: String,
    pub system_oee: f64,
    pub use_case: String,
}

/// Aggregate multiple machines into system-level OEE
async fn system_aggregate_handler(
    Json(request): Json<SystemAggregateRequest>,
) -> Result<Json<SystemAggregateResponse>, ApiError> {
    if request.machines.is_empty() {
        return Err(ApiError::invalid_input("At least one machine is required"));
    }
    
    let analysis = crate::calculus::engineer::calculators::production::oee::engine::multi_machine::aggregate_system_oee(
        request.machines,
        request.aggregation_method,
    );
    
    Ok(Json(SystemAggregateResponse { analysis }))
}

/// Compare different aggregation methods
async fn system_compare_methods_handler(
    Json(request): Json<SystemCompareMethodsRequest>,
) -> Result<Json<SystemCompareMethodsResponse>, ApiError> {
    if request.machines.is_empty() {
        return Err(ApiError::invalid_input("At least one machine is required"));
    }
    
    let results = crate::calculus::engineer::calculators::production::oee::engine::multi_machine::compare_aggregation_methods(
        request.machines.clone()
    );
    
    // Build comparison response
    let mut comparisons = std::collections::HashMap::new();
    
    for (method, oee) in results.iter() {
        let (method_name, use_case) = match method {
            crate::calculus::engineer::calculators::production::oee::engine::multi_machine::AggregationMethod::SimpleAverage => {
                ("SimpleAverage", "api.system.use_case.simple_average")
            }
            crate::calculus::engineer::calculators::production::oee::engine::multi_machine::AggregationMethod::ProductionWeighted => {
                ("ProductionWeighted", "api.system.use_case.production_weighted")
            }
            crate::calculus::engineer::calculators::production::oee::engine::multi_machine::AggregationMethod::TimeWeighted => {
                ("TimeWeighted", "api.system.use_case.time_weighted")
            }
            crate::calculus::engineer::calculators::production::oee::engine::multi_machine::AggregationMethod::Minimum => {
                ("Minimum", "api.system.use_case.minimum")
            }
            crate::calculus::engineer::calculators::production::oee::engine::multi_machine::AggregationMethod::Multiplicative => {
                ("Multiplicative", "api.system.use_case.multiplicative")
            }
        };
        
        comparisons.insert(method_name.to_string(), SystemMethodComparison {
            method: method_name.to_string(),
            system_oee: *oee * 100.0,
            use_case: use_case.to_string(),
        });
    }
    
    // Recommend TimeWeighted as default
    let recommended_method = "TimeWeighted".to_string();
    
    Ok(Json(SystemCompareMethodsResponse {
        comparisons,
        recommended_method,
    }))
}

// ============================================================================
// Error Handling
// ============================================================================

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
    
    pub fn invalid_input(message: &str) -> Self {
        Self {
            code: "INVALID_INPUT".to_string(),
            message_key: "api.error.invalid_input".to_string(),
            params: serde_json::json!({
                "message": message,
            }),
            status: StatusCode::BAD_REQUEST,
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
                ApiError::invalid_input(&msg)
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