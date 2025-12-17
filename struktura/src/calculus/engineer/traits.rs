use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
};
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

/// Core trait for all engineering calculators
/// 
/// Each calculator implements this trait to provide:
/// - Metadata for API discovery
/// - Parameter validation
/// - Calculation logic
/// - Result formatting
#[async_trait]
pub trait EngineerCalculator: Send + Sync {
    /// Unique identifier for the calculator
    fn id(&self) -> &str;
    
    /// Human-readable name
    fn name(&self) -> &str;
    
    /// Engineering discipline category
    fn category(&self) -> CalculatorCategory;
    
    /// Full metadata for API discovery
    fn metadata(&self) -> EngineeringCalculatorMetadata;
    
    /// Validate input parameters before calculation
    /// 
    /// This should check:
    /// - Required parameters exist
    /// - Values are within physical/engineering constraints
    /// - Units are consistent
    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()>;
    
    /// Perform the engineering calculation
    /// 
    /// This is where the actual engineering happens.
    /// Should return warnings, recommendations, and compliance notes.
    async fn calculate(&self, params: EngineeringParameters) 
        -> EngineeringResult<EngineeringCalculationResponse>;
    
    /// Optional: Pre-process parameters (unit conversion, normalization)
    fn preprocess(&self, params: &mut EngineeringParameters) -> EngineeringResult<()> {
        // Default: no preprocessing
        Ok(())
    }
    
    /// Optional: Post-process results (rounding, formatting)
    fn postprocess(&self, response: &mut EngineeringCalculationResponse) -> EngineeringResult<()> {
        // Default: no postprocessing
        Ok(())
    }
}

/// Parameter validator trait for reusable validation logic
pub trait ParameterValidator {
    /// Validate a single dimension parameter
    fn validate_dimension(
        &self,
        name: &str,
        value: Option<f64>,
        min: f64,
        max: f64,
    ) -> EngineeringResult<f64> {
        match value {
            Some(v) if v < min => Err(EngineeringError::InvalidParameter {
                parameter: name.to_string(),
                value: v.to_string(),
                reason: format!("Must be >= {}", min),
            }),
            Some(v) if v > max => Err(EngineeringError::InvalidParameter {
                parameter: name.to_string(),
                value: v.to_string(),
                reason: format!("Must be <= {}", max),
            }),
            Some(v) => Ok(v),
            None => Err(EngineeringError::MissingParameter {
                parameter: name.to_string(),
                calculator: self.calculator_id().to_string(),
            }),
        }
    }
    
    /// Validate load case exists and is reasonable
    fn validate_load_case(&self, loads: &Option<LoadCase>) -> EngineeringResult<LoadCase> {
        loads.clone().ok_or_else(move || EngineeringError::MissingParameter {
            parameter: "loads".to_string(),
            calculator: self.calculator_id().to_string(),
        })
    }
    
    /// Validate material properties
    fn validate_material(&self, material: &Option<MaterialProperties>) -> EngineeringResult<MaterialProperties> {
        material.clone().ok_or_else(move || EngineeringError::MissingParameter {
            parameter: "material".to_string(),
            calculator: self.calculator_id().to_string(),
        })
    }
    
    /// Get additional parameter with validation
    fn get_additional_param(
        &self,
        params: &EngineeringParameters,
        name: &str,
        min: Option<f64>,
        max: Option<f64>,
    ) -> EngineeringResult<f64> {
        let value = params.additional.as_ref()
            .and_then(|a| a.get(name).copied())
            .ok_or_else(|| EngineeringError::MissingParameter {
                parameter: name.to_string(),
                calculator: self.calculator_id().to_string(),
            })?;
        
        if let Some(min_val) = min {
            if value < min_val {
                return Err(EngineeringError::InvalidParameter {
                    parameter: name.to_string(),
                    value: value.to_string(),
                    reason: format!("Must be >= {}", min_val),
                });
            }
        }
        
        if let Some(max_val) = max {
            if value > max_val {
                return Err(EngineeringError::InvalidParameter {
                    parameter: name.to_string(),
                    value: value.to_string(),
                    reason: format!("Must be <= {}", max_val),
                });
            }
        }
        
        Ok(value)
    }
    
    /// Get calculator ID for error messages
    fn calculator_id(&self) -> &str;
}

/// Trait for calculators that support multiple design codes
pub trait MultiCodeCalculator: EngineerCalculator {
    /// Get list of supported design codes
    fn supported_codes(&self) -> Vec<DesignCode>;
    
    /// Get active design code from parameters or default
    fn active_code(&self, params: &EngineeringParameters) -> DesignCode {
        params.design_code
            .as_ref()
            .and_then(|code_str| self.parse_code(code_str))
            .unwrap_or_else(|| self.default_code())
    }
    
    /// Parse design code string
    fn parse_code(&self, code: &str) -> Option<DesignCode>;
    
    /// Default design code if none specified
    fn default_code(&self) -> DesignCode;
}

/// Trait for structural calculators with load combinations
pub trait StructuralCalculator: EngineerCalculator {
    /// Calculate factored loads based on design code
    fn calculate_factored_loads(&self, loads: &LoadCase, code: &DesignCode) -> FactoredLoads;
    
    /// Get resistance factors for design code
    fn resistance_factors(&self, code: &DesignCode) -> ResistanceFactors;
    
    /// Check serviceability limits
    fn check_serviceability(&self, deflection: f64, span: f64) -> ServiceabilityCheck;
}

/// Trait for calculators that generate cost estimates
pub trait CostEstimator: EngineerCalculator {
    /// Calculate material costs
    fn estimate_material_cost(&self, params: &EngineeringParameters) -> EngineeringResult<f64>;
    
    /// Calculate labor costs
    fn estimate_labor_cost(&self, params: &EngineeringParameters) -> EngineeringResult<f64>;
    
    /// Calculate total project cost
    fn estimate_total_cost(&self, params: &EngineeringParameters) -> EngineeringResult<CostBreakdown> {
        Ok(CostBreakdown {
            material: self.estimate_material_cost(params)?,
            labor: self.estimate_labor_cost(params)?,
            equipment: 0.0,
            overhead: 0.0,
        })
    }
}

/// Helper struct for factored loads
#[derive(Debug, Clone)]
pub struct FactoredLoads {
    pub dead: f64,
    pub live: f64,
    pub wind: f64,
    pub seismic: f64,
    pub total: f64,
}

/// Helper struct for resistance factors
#[derive(Debug, Clone)]
pub struct ResistanceFactors {
    pub flexure: f64,
    pub shear: f64,
    pub compression: f64,
    pub tension: f64,
}

/// Serviceability check result
#[derive(Debug, Clone)]
pub struct ServiceabilityCheck {
    pub deflection_actual: f64,
    pub deflection_limit: f64,
    pub passes: bool,
    pub ratio: f64,
}

/// Cost breakdown structure
#[derive(Debug, Clone)]
pub struct CostBreakdown {
    pub material: f64,
    pub labor: f64,
    pub equipment: f64,
    pub overhead: f64,
}

impl CostBreakdown {
    pub fn total(&self) -> f64 {
        self.material + self.labor + self.equipment + self.overhead
    }
}

/// Registry trait for managing calculators
pub trait CalculatorRegistry {
    /// Register a calculator
    fn register(&mut self, calculator: Box<dyn EngineerCalculator>);
    
    /// Find calculator by ID
    fn find(&self, id: &str) -> Option<&dyn EngineerCalculator>;
    
    /// Get all calculators
    fn all(&self) -> Vec<&dyn EngineerCalculator>;
    
    /// Get calculators by category
    fn by_category(&self, category: CalculatorCategory) -> Vec<&dyn EngineerCalculator>;
    
    /// Get catalogue for API
    fn catalogue(&self) -> EngineeringCalculatorCatalogue;
}

/// Macro for implementing basic ParameterValidator
#[macro_export]
macro_rules! impl_parameter_validator_eng {
    ($type:ty, $id:expr) => {
        impl ParameterValidator for $type {
            fn calculator_id(&self) -> &str {
                $id
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockCalculator;
    
    impl ParameterValidator for MockCalculator {
        fn calculator_id(&self) -> &str {
            "mock_calculator"
        }
    }
    
    #[test]
    fn test_validate_dimension() {
        let calc = MockCalculator;
        
        // Valid dimension
        let result = calc.validate_dimension("height", Some(5.0), 0.0, 10.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
        
        // Below minimum
        let result = calc.validate_dimension("height", Some(-1.0), 0.0, 10.0);
        assert!(result.is_err());
        
        // Above maximum
        let result = calc.validate_dimension("height", Some(15.0), 0.0, 10.0);
        assert!(result.is_err());
        
        // Missing
        let result = calc.validate_dimension("height", None, 0.0, 10.0);
        assert!(result.is_err());
    }
}