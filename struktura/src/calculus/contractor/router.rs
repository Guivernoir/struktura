use crate::calculus::contractor::{
    errors::ContractingError,
    models::*,
    registry::ContractingRegistry,
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
use crate::state::AppState;

/// Application state containing the calculator registry
#[derive(Clone)]
pub struct ContractingState {
    pub registry: Arc<ContractingRegistry>,
}

impl ContractingState {
    pub fn new(registry: Arc<ContractingRegistry>) -> Self {
        Self { registry }
    }
}

/// Query parameters for catalogue endpoint
#[derive(Debug, Deserialize)]
pub struct CatalogueQuery {
    /// Filter by category
    category: Option<String>,
    /// Search query
    q: Option<String>,
    /// Only show calculators requiring certification review
    certification_required: Option<bool>,
}

/// Health check response
#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
    calculators_loaded: usize,
}

// ============================================================================
// HANDLERS
// ============================================================================

/// POST /api/v1/calculus/contractor/calculate
/// Execute a contracting calculation
async fn calculate_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ContractingCalculationRequest>,
) -> Result<Json<ContractingCalculationResponse>, ContractingError> {
    // Find calculator in registry
    let calculator = state.calculators_contractor.find(&payload.calculation_type)?;

    // Validate parameters
    calculator.validate(&payload.parameters)?;

    // Execute calculation
    let response = calculator.calculate(payload.parameters).await?;

    Ok(Json(response))
}

/// GET /api/v1/calculus/contractor/catalogue
/// Get complete calculator catalogue with optional filtering
async fn catalogue_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CatalogueQuery>,
) -> Json<ContractingCalculatorCatalogue> {
    let mut catalogue = state.calculators_contractor.catalogue();

    // Apply filters if specified
    if let Some(category_filter) = query.category {
        catalogue.calculators.retain(|calc| calc.category == category_filter);
    }

    if let Some(true) = query.certification_required {
        catalogue.calculators.retain(|calc| calc.requires_certification_review);
    }

    // Apply search if specified
    if let Some(search_query) = query.q {
        let search_results = state.calculators_contractor.search(&search_query);
        let result_ids: Vec<String> = search_results.iter().map(|c| c.id().to_string()).collect();
        
        catalogue.calculators.retain(|calc| result_ids.contains(&calc.id));
    }

    Json(catalogue)
}

/// GET /api/v1/calculus/contractor/catalogue/:id
/// Get metadata for specific calculator
async fn calculator_metadata_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ContractingCalculatorMetadata>, ContractingError> {
    let calculator = state.calculators_contractor.find(&id)?;
    Ok(Json(calculator.metadata()))
}

/// GET /api/v1/calculus/contractor/categories
/// Get list of all categories
async fn categories_handler(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<ContractingCategoryInfo>> {
    let catalogue = state.calculators_contractor.catalogue();
    Json(catalogue.categories)
}

/// GET /api/v1/calculus/contractor/categories/:category
/// Get calculators in specific category
async fn category_calculators_handler(
    State(state): State<Arc<AppState>>,
    Path(category_str): Path<String>,
) -> Result<Json<Vec<ContractingCalculatorMetadata>>, ContractingError> {
    // Parse category
    let category = match category_str.as_str() {
        "bidding" => CalculatorCategory::Bidding,
        "scheduling" => CalculatorCategory::Scheduling,
        "estimation" => CalculatorCategory::Estimation,
        "management" => CalculatorCategory::Management,
        _ => {
            return Err(ContractingError::InvalidParameter {
                parameter: "category".to_string(),
                value: category_str,
                reason: "Invalid category".to_string(),
            })
        }
    };

    let calculators = state.calculators_contractor.by_category(category);
    let metadata: Vec<ContractingCalculatorMetadata> = calculators
        .iter()
        .map(|calc| calc.metadata())
        .collect();

    Ok(Json(metadata))
}

/// GET /api/v1/calculus/contractor/search
/// Search calculators by keyword
#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

async fn search_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Json<Vec<ContractingCalculatorMetadata>> {
    let results = state.calculators_contractor.search(&query.q);
    let metadata: Vec<ContractingCalculatorMetadata> = results
        .iter()
        .map(|calc| calc.metadata())
        .collect();

    Json(metadata)
}

/// GET /api/v1/calculus/contractor/health
/// Health check endpoint
async fn health_handler(
    State(state): State<Arc<AppState>>,
) -> Json<HealthResponse> {
    let stats = state.calculators_contractor.stats();
    
    Json(HealthResponse {
        status: "operational".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        calculators_loaded: stats.total_calculators,
    })
}

/// GET /api/v1/calculus/contractor/stats
/// Registry statistics
#[derive(Serialize)]
struct StatsResponse {
    total_calculators: usize,
    by_category: std::collections::HashMap<String, usize>,
    requires_certification_count: usize,
    total_parameters: usize,
}

async fn stats_handler(
    State(state): State<Arc<AppState>>,
) -> Json<StatsResponse> {
    let stats = state.calculators_contractor.stats();
    
    Json(StatsResponse {
        total_calculators: stats.total_calculators,
        by_category: stats.by_category,
        requires_certification_count: stats.requires_certification_count,
        total_parameters: stats.total_parameters,
    })
}

// ============================================================================
// ROUTER CONSTRUCTION
// ============================================================================

/// Create the complete contracting calculator router
/// 
/// This router should be nested under `/api/v1/calculus/contractor`
pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        // Main calculation endpoint
        .route("/calculate", post(calculate_handler))
        
        // Catalogue and discovery endpoints
        .route("/catalogue", get(catalogue_handler))
        .route("/catalogue/meta", get(calculator_metadata_handler))
        
        // Category endpoints
        .route("/categories", get(categories_handler))
        .route("/categories/handler", get(category_calculators_handler))
        
        // Search endpoint
        .route("/search", get(search_handler))
        
        // System endpoints
        .route("/health", get(health_handler))
        .route("/stats", get(stats_handler))
}

/// Create router with default registry (convenience function)
pub fn create_default_router() -> Router<Arc<AppState>> {
    create_router()
}
