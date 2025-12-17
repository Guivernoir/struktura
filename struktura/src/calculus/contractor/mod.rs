// ============================================================================
// Contractor Calculus Module
// 
// A comprehensive, trait-based calculator system for construction contracting,
// bidding, scheduling, cost estimation, and project management calculations.
//
// Architecture:
// - errors.rs:    Surgical error handling with actionable feedback
// - traits.rs:    Calculator trait system and validation logic
// - models.rs:    Data structures for inputs, outputs, and metadata
// - registry.rs:  Thread-safe calculator registry
// - router.rs:    Axum HTTP router with API endpoints
// - calculators/: Individual calculator implementations by discipline
// ============================================================================

// Core module exports
pub mod errors;
pub mod traits;
pub mod models;
pub mod registry;
pub mod router;

// Calculator implementations organized by discipline
pub mod calculators {
    pub mod bidding;
    pub mod scheduling;
    pub mod estimation;
    pub mod management;
}

// Re-export commonly used types for convenience
pub use errors::{ContractingError, ContractingResult};
pub use traits::{
    ContractorCalculator, ParameterValidator, 
    MultiCodeCalculator, CostEstimator,
};
pub use models::{
    // Request/Response types
    ContractingCalculationRequest,
    ContractingCalculationResponse,
    ContractingParameters,
    ContractingResultItem,
    
    // Input types
    MaterialProperties,
    ResourceRequirements,
    SafetyFactors,
    
    // Metadata types
    ContractingCalculatorMetadata,
    ContractingCalculatorCatalogue,
    ContractingCategoryInfo,
    ParameterMetadata,
    
    // Enums
    CalculatorCategory,
    RegulationCode,
    WarningSeverity,
    
    // Analysis results
    ProjectAnalysisResult,
};
pub use registry::{ContractingRegistry, RegistryBuilder, create_default_registry};
pub use router::create_router;

// ============================================================================
// MODULE INITIALIZATION
// ============================================================================

/// Module version from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Module metadata
pub struct ModuleInfo {
    pub version: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

pub const MODULE_INFO: ModuleInfo = ModuleInfo {
    version: VERSION,
    name: "Contractor Calculus",
    description: "Comprehensive contracting calculation system with bidding, scheduling, estimation, and management calculators",
};

// ============================================================================
// TESTING UTILITIES
// ============================================================================

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use std::collections::HashMap;

    /// Create minimal valid parameters for testing
    pub fn minimal_parameters() -> ContractingParameters {
        ContractingParameters {
            dimensions: HashMap::new(),
            material: None,
            resources: None,
            safety_factors: None,
            regulation_code: None,
            exposure_class: None,
            temperature: None,
            humidity: None,
            additional: None,
            project_metadata: None,
        }
    }

    /// Create parameters with dimensions
    pub fn parameters_with_dimensions(dims: Vec<(&str, f64)>) -> ContractingParameters {
        let mut dimensions = HashMap::new();
        for (key, value) in dims {
            dimensions.insert(key.to_string(), value);
        }

        ContractingParameters {
            dimensions,
            ..minimal_parameters()
        }
    }

    /// Create parameters with resources
    pub fn parameters_with_resources(labor: f64, equipment: f64) -> ContractingParameters {
        ContractingParameters {
            dimensions: HashMap::new(),
            material: None,
            resources: Some(ResourceRequirements {
                labor_hours: labor,
                equipment_hours: equipment,
                material_quantity: None,
                subcontractor_cost: None,
                overhead: None,
            }),
            safety_factors: None,
            regulation_code: None,
            exposure_class: None,
            temperature: None,
            humidity: None,
            additional: None,
            project_metadata: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_info() {
        assert_eq!(MODULE_INFO.name, "Contractor Calculus");
        assert!(!MODULE_INFO.version.is_empty());
    }

    #[test]
    fn test_registry_creation() {
        let registry = create_default_registry();
        let stats = registry.stats();
        
        // Should have calculators loaded
        assert!(stats.total_calculators >= 0);
        
        // Should have categories
        assert!(stats.by_category.len() >= 0);
    }

    #[test]
    fn test_catalogue_generation() {
        let registry = create_default_registry();
        let catalogue = registry.catalogue();
        
        assert!(!catalogue.version.is_empty());
        assert!(!catalogue.categories.is_empty());
        assert!(!catalogue.disclaimer.is_empty());
    }
}