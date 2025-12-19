// ============================================================================
// Engineering Calculus Module
// 
// A comprehensive, trait-based calculator system for civil, structural,
// mechanical, and production engineering calculations.
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
    pub mod civil;
    pub mod structural;
    pub mod mechanical;
    pub mod production;
}

// Re-export commonly used types for convenience
pub use errors::{EngineeringError, EngineeringResult};
pub use traits::{
    EngineerCalculator, ParameterValidator, 
    StructuralCalculator, MultiCodeCalculator, CostEstimator,
};
pub use models::{
    // Request/Response types
    EngineeringCalculationRequest,
    EngineeringCalculationResponse,
    EngineeringParameters,
    EngineeringResultItem,
    
    // Input types
    MaterialProperties,
    LoadCase,
    SafetyFactors,
    
    // Metadata types
    EngineeringCalculatorMetadata,
    EngineeringCalculatorCatalogue,
    EngineeringCategoryInfo,
    ParameterMetadata,
    
    // Enums
    CalculatorCategory,
    DesignCode,
    WarningSeverity,
    
    // Analysis results
    StructuralAnalysisResult,
};
pub use registry::{EngineeringRegistry, RegistryBuilder, create_default_registry};
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
    name: "Engineering Calculus",
    description: "Comprehensive engineering calculation system with civil, structural, mechanical, and production calculators",
};

// ============================================================================
// TESTING UTILITIES
// ============================================================================

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use std::collections::HashMap;

    /// Create minimal valid parameters for testing
    pub fn minimal_parameters() -> EngineeringParameters {
        EngineeringParameters {
            dimensions: HashMap::new(),
            material: None,
            loads: None,
            safety_factors: None,
            design_code: None,
            exposure_class: None,
            temperature: None,
            humidity: None,
            additional: None,
            structured_data: None,
            project_metadata: None,
        }
    }

    /// Create parameters with dimensions
    pub fn parameters_with_dimensions(dims: Vec<(&str, f64)>) -> EngineeringParameters {
        let mut dimensions = HashMap::new();
        for (key, value) in dims {
            dimensions.insert(key.to_string(), value);
        }

        EngineeringParameters {
            dimensions,
            ..minimal_parameters()
        }
    }

    /// Create parameters with loads
    pub fn parameters_with_loads(dead: f64, live: f64) -> EngineeringParameters {
        EngineeringParameters {
            dimensions: HashMap::new(),
            material: None,
            loads: Some(LoadCase {
                dead_load: dead,
                live_load: live,
                wind_load: None,
                seismic_load: None,
                snow_load: None,
                impact_load: None,
                shear_load: None,
                tension_load: None,
                load_combination: "LRFD".to_string(),
            }),
            safety_factors: None,
            design_code: None,
            exposure_class: None,
            temperature: None,
            humidity: None,
            additional: None,
            structured_data: None,
            project_metadata: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_info() {
        assert_eq!(MODULE_INFO.name, "Engineering Calculus");
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