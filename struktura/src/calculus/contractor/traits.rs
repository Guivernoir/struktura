use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
};
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

/// Core trait for all contracting calculators
/// 
/// Each calculator implements this trait to provide:
/// - Metadata for API discovery
/// - Parameter validation
/// - Calculation logic
/// - Result formatting
#[async_trait]
pub trait ContractorCalculator: Send + Sync {
    /// Unique identifier for the calculator
    fn id(&self) -> &str;
    
    /// Human-readable name
    fn name(&self) -> &str;
    
    /// Contracting discipline category
    fn category(&self) -> CalculatorCategory;
    
    /// Full metadata for API discovery
    fn metadata(&self) -> ContractingCalculatorMetadata;
    
    /// Validate input parameters before calculation
    /// 
    /// This should check:
    /// - Required parameters exist
    /// - Values are within practical constraints
    /// - Units are consistent
    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()>;
    
    /// Perform the contracting calculation
    /// 
    /// This is where the actual calculation happens.
    /// Should return warnings, recommendations, and compliance notes.
    async fn calculate(&self, params: ContractingParameters) 
        -> ContractingResult<ContractingCalculationResponse>;
    
    /// Optional: Pre-process parameters (unit conversion, normalization)
    fn preprocess(&self, params: &mut ContractingParameters) -> ContractingResult<()> {
        // Default: no preprocessing
        Ok(())
    }
    
    /// Optional: Post-process results (rounding, formatting)
    fn postprocess(&self, response: &mut ContractingCalculationResponse) -> ContractingResult<()> {
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
    ) -> ContractingResult<f64> {
        match value {
            Some(v) if v < min => Err(ContractingError::InvalidParameter {
                parameter: name.to_string(),
                value: v.to_string(),
                reason: format!("Must be >= {}", min),
            }),
            Some(v) if v > max => Err(ContractingError::InvalidParameter {
                parameter: name.to_string(),
                value: v.to_string(),
                reason: format!("Must be <= {}", max),
            }),
            Some(v) => Ok(v),
            None => Err(ContractingError::MissingParameter {
                parameter: name.to_string(),
                calculator: self.calculator_id().to_string(),
            }),
        }
    }
    
    /// Validate resource requirements exists and is reasonable
    fn validate_resources(&self, resources: &Option<ResourceRequirements>) -> ContractingResult<ResourceRequirements> {
        resources.clone().ok_or_else(move || ContractingError::MissingParameter {
            parameter: "resources".to_string(),
            calculator: self.calculator_id().to_string(),
        })
    }
    
    /// Validate material properties
    fn validate_material(&self, material: &Option<MaterialProperties>) -> ContractingResult<MaterialProperties> {
        material.clone().ok_or_else(move || ContractingError::MissingParameter {
            parameter: "material".to_string(),
            calculator: self.calculator_id().to_string(),
        })
    }
    
    /// Get additional parameter with validation
    fn get_additional_param(
        &self,
        params: &ContractingParameters,
        name: &str,
        min: Option<f64>,
        max: Option<f64>,
    ) -> ContractingResult<f64> {
        let value = params.additional.as_ref()
            .and_then(|a| a.get(name).copied())
            .ok_or_else(|| ContractingError::MissingParameter {
                parameter: name.to_string(),
                calculator: self.calculator_id().to_string(),
            })?;
        
        if let Some(min_val) = min {
            if value < min_val {
                return Err(ContractingError::InvalidParameter {
                    parameter: name.to_string(),
                    value: value.to_string(),
                    reason: format!("Must be >= {}", min_val),
                });
            }
        }
        
        if let Some(max_val) = max {
            if value > max_val {
                return Err(ContractingError::InvalidParameter {
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

/// Trait for calculators that support multiple regulation codes
pub trait MultiCodeCalculator: ContractorCalculator {
    /// Get list of supported regulation codes
    fn supported_codes(&self) -> Vec<RegulationCode>;
    
    /// Get active regulation code from parameters or default
    fn active_code(&self, params: &ContractingParameters) -> RegulationCode {
        params.regulation_code
            .as_ref()
            .and_then(|code_str| self.parse_code(code_str))
            .unwrap_or_else(|| self.default_code())
    }
    
    /// Parse regulation code string
    fn parse_code(&self, code: &str) -> Option<RegulationCode>;
    
    /// Default regulation code if none specified
    fn default_code(&self) -> RegulationCode;
}

/// Trait for calculators that generate cost estimates
pub trait CostEstimator: ContractorCalculator {
    /// Calculate material costs
    fn estimate_material_cost(&self, params: &ContractingParameters) -> ContractingResult<f64>;
    
    /// Calculate labor costs
    fn estimate_labor_cost(&self, params: &ContractingParameters) -> ContractingResult<f64>;
    
    /// Calculate total project cost
    fn estimate_total_cost(&self, params: &ContractingParameters) -> ContractingResult<CostBreakdown> {
        Ok(CostBreakdown {
            material: self.estimate_material_cost(params)?,
            labor: self.estimate_labor_cost(params)?,
            equipment: 0.0,
            overhead: 0.0,
        })
    }
}

/// Helper struct for factored costs
#[derive(Debug, Clone)]
pub struct FactoredCosts {
    pub material: f64,
    pub labor: f64,
    pub equipment: f64,
    pub overhead: f64,
    pub total: f64,
}

/// Helper struct for reduction factors
#[derive(Debug, Clone)]
pub struct ReductionFactors {
    pub material: f64,
    pub labor: f64,
    pub equipment: f64,
    pub overhead: f64,
}

/// Feasibility check result
#[derive(Debug, Clone)]
pub struct FeasibilityCheck {
    pub cost_actual: f64,
    pub cost_limit: f64,
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
    fn register(&mut self, calculator: Box<dyn ContractorCalculator>);
    
    /// Find calculator by ID
    fn find(&self, id: &str) -> Option<&dyn ContractorCalculator>;
    
    /// Get all calculators
    fn all(&self) -> Vec<&dyn ContractorCalculator>;
    
    /// Get calculators by category
    fn by_category(&self, category: CalculatorCategory) -> Vec<&dyn ContractorCalculator>;
    
    /// Get catalogue for API
    fn catalogue(&self) -> ContractingCalculatorCatalogue;
}

/// Macro for implementing basic ParameterValidator
#[macro_export]
macro_rules! impl_parameter_validator_con {
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
        let result = calc.validate_dimension("area", Some(5.0), 0.0, 10.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
        
        // Below minimum
        let result = calc.validate_dimension("area", Some(-1.0), 0.0, 10.0);
        assert!(result.is_err());
        
        // Above maximum
        let result = calc.validate_dimension("area", Some(15.0), 0.0, 10.0);
        assert!(result.is_err());
        
        // Missing
        let result = calc.validate_dimension("area", None, 0.0, 10.0);
        assert!(result.is_err());
    }
}