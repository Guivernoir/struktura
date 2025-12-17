// ============================================================================
// Beginner Calculus Module
// 
// A comprehensive, trait-based calculator system for beginner-level
// construction, gardening, and home improvement calculations.
//
// Architecture:
// - errors.rs:    Surgical error handling with actionable feedback
// - traits.rs:    Calculator trait system and validation logic
// - models.rs:    Data structures for inputs, outputs, and metadata
// - registry.rs:  Thread-safe calculator registry
// - router.rs:    Axum HTTP router with API endpoints
// - calculators/: Individual calculator implementations by category
// ============================================================================

// Core module exports
pub mod errors;
pub mod traits;
pub mod models;
pub mod registry;
pub mod router;

// Calculator implementations organized by category
pub mod calculators {
    pub mod garden;
    pub mod interiors;
    pub mod outdoors;
    pub mod utilities;
}

// Re-export commonly used types for convenience
pub use errors::{BeginnerError, BeginnerResult};
pub use traits::{
    BeginnerCalculator, ParameterValidator
};
pub use models::{
    // Request/Response types
    BeginnerCalculationRequest,
    BeginnerCalculationResponse,
    BeginnerParameters,
    BeginnerResultItem,
    
    // Metadata types
    BeginnerCalculatorMetadata,
    BeginnerCalculatorCatalogue,
    BeginnerCategoryInfo,
    ParameterMetadata,
    
    // Enums
    CalculatorCategory,
    WarningSeverity,
};
pub use registry::{BeginnerRegistry, RegistryBuilder, create_default_registry};
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
    name: "Beginner Calculus",
    description: "Comprehensive beginner calculation system with garden, interiors, outdoors, and utilities calculators",
};

// ============================================================================
// TESTING UTILITIES
// ============================================================================

#[cfg(test)]
pub mod test_utils {
    use super::*;

    /// Create minimal valid parameters for testing
    pub fn minimal_parameters() -> BeginnerParameters {
        BeginnerParameters {
            width: 1.0,
            length: 1.0,
            height: 0.1,
            additional: None,
        }
    }

    /// Create parameters with dimensions
    pub fn parameters_with_dimensions(w: f64, l: f64, h: f64) -> BeginnerParameters {
        BeginnerParameters {
            width: w,
            length: l,
            height: h,
            additional: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_info() {
        assert_eq!(MODULE_INFO.name, "Beginner Calculus");
        assert!(!MODULE_INFO.version.is_empty());
    }

    #[test]
    fn test_registry_creation() {
        let registry = create_default_registry();
        let stats = registry.stats();
        
        assert!(stats.total_calculators > 0);
        
        assert!(stats.by_category.len() > 0);
    }

    #[test]
    fn test_catalogue_generation() {
        let registry = create_default_registry();
        let catalogue = registry.catalogue();
        
        assert!(!catalogue.version.is_empty());
        assert!(!catalogue.categories.is_empty());
    }
}