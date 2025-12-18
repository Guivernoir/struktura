use crate::calculus::beginner::{
    errors::BeginnerError,
    models::*,
    registry::BeginnerRegistry,
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

/// Application state
#[derive(Clone)]
pub struct BeginnerState {
    pub registry: Arc<BeginnerRegistry>,
}

impl BeginnerState {
    pub fn new(registry: Arc<BeginnerRegistry>) -> Self {
        Self { registry }
    }
}

/// Query parameters for catalogue
#[derive(Debug, Deserialize)]
pub struct CatalogueQuery {
    /// Filter by category
    category: Option<String>,
    /// Search query
    q: Option<String>,
}

/// Health check response
#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
    calculators_loaded: usize,
}

// Handlers

async fn calculate_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BeginnerCalculationRequest>,
) -> Result<Json<BeginnerCalculationResponse>, BeginnerError> {
    // Find calculator in registry
    let calculator = state.calculators_beginner.find(&payload.calculation_type)?;

    // Validate parameters
    calculator.validate(&payload.parameters)?;

    // Execute calculation
    let response = calculator.calculate(payload.parameters).await?;

    Ok(Json(response))
}

async fn catalogue_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CatalogueQuery>,
) -> Json<BeginnerCalculatorCatalogue> {
    let mut catalogue = state.calculators_beginner.catalogue();

    if let Some(category_filter) = query.category {
        catalogue.calculators.retain(|calc| calc.category == category_filter);
    }

    if let Some(search_query) = query.q {
        let search_results = state.calculators_beginner.search(&search_query);
        let result_ids: Vec<String> = search_results.iter().map(|c| c.id().to_string()).collect();
        
        catalogue.calculators.retain(|calc| result_ids.contains(&calc.id));
    }

    Json(catalogue)
}

async fn calculator_metadata_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<BeginnerCalculatorMetadata>, BeginnerError> {
    let calculator = state.calculators_beginner.find(&id)?;
    Ok(Json(calculator.metadata()))
}

async fn categories_handler(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<BeginnerCategoryInfo>> {
    let catalogue = state.calculators_beginner.catalogue();
    Json(catalogue.categories)
}

async fn category_calculators_handler(
    State(state): State<Arc<AppState>>,
    Path(category_str): Path<String>,
) -> Result<Json<Vec<BeginnerCalculatorMetadata>>, BeginnerError> {
    let category = match category_str.as_str() {
        "garden" => CalculatorCategory::Garden,
        "interiors" => CalculatorCategory::Interiors,
        "outdoors" => CalculatorCategory::Outdoors,
        "utilities" => CalculatorCategory::Utilities,
        _ => {
            return Err(BeginnerError::InvalidParameter {
                parameter: "category".to_string(),
                value: category_str,
                reason: "Invalid category".to_string(),
            })
        }
    };

    let calculators = state.calculators_beginner.by_category(category);
    let metadata: Vec<BeginnerCalculatorMetadata> = calculators
        .iter()
        .map(|calc| calc.metadata())
        .collect();

    Ok(Json(metadata))
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

async fn search_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Json<Vec<BeginnerCalculatorMetadata>> {
    let results = state.calculators_beginner.search(&query.q);
    let metadata: Vec<BeginnerCalculatorMetadata> = results
        .iter()
        .map(|calc| calc.metadata())
        .collect();

    Json(metadata)
}

async fn health_handler(
    State(state): State<Arc<AppState>>,
) -> Json<HealthResponse> {
    let stats = state.calculators_beginner.stats();
    
    Json(HealthResponse {
        status: "operational".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        calculators_loaded: stats.total_calculators,
    })
}

/// Registry statistics
#[derive(Serialize)]
struct StatsResponse {
    total_calculators: usize,
    by_category: std::collections::HashMap<String, usize>,
    total_parameters: usize,
}

async fn stats_handler(
    State(state): State<Arc<AppState>>,
) -> Json<StatsResponse> {
    let stats = state.calculators_beginner.stats();
    
    Json(StatsResponse {
        total_calculators: stats.total_calculators,
        by_category: stats.by_category,
        total_parameters: stats.total_parameters,
    })
}

/// Create the router
pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/calculate", post(calculate_handler))
        
        .route("/catalogue", get(catalogue_handler))
        .route("/catalogue/meta", get(calculator_metadata_handler))
        
        .route("/categories", get(categories_handler))
        .route("/categories/handler", get(category_calculators_handler))
        
        .route("/search", get(search_handler))
        
        .route("/health", get(health_handler))
        .route("/stats", get(stats_handler))
}
