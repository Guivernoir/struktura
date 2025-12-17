use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
};
use async_trait::async_trait;

/// Core trait for all beginner calculators
#[async_trait]
pub trait BeginnerCalculator: Send + Sync {
    /// Unique identifier for the calculator
    fn id(&self) -> &str;
    
    /// Human-readable name
    fn name(&self) -> &str;
    
    /// Category
    fn category(&self) -> CalculatorCategory;
    
    /// Full metadata
    fn metadata(&self) -> BeginnerCalculatorMetadata;
    
    /// Validate input parameters
    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()>;
    
    /// Perform the calculation
    async fn calculate(&self, params: BeginnerParameters) 
        -> BeginnerResult<BeginnerCalculationResponse>;
}

/// Parameter validator trait for reusable validation logic
pub trait ParameterValidator {
    /// Validate a single dimension parameter
    fn validate_dimension(
        &self,
        name: &str,
        value: f64,
        min: f64,
        max: f64,
    ) -> BeginnerResult<f64> {
        if value < min {
            Err(BeginnerError::InvalidParameter {
                parameter: name.to_string(),
                value: value.to_string(),
                reason: format!("Must be >= {}", min),
            })
        } else if value > max {
            Err(BeginnerError::InvalidParameter {
                parameter: name.to_string(),
                value: value.to_string(),
                reason: format!("Must be <= {}", max),
            })
        } else {
            Ok(value)
        }
    }
    
    /// Get calculator ID for error messages
    fn calculator_id(&self) -> &str;
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
        
        // Valid
        let result = calc.validate_dimension("height", 5.0, 0.0, 10.0);
        assert!(result.is_ok());
        
        // Invalid
        let result = calc.validate_dimension("height", -1.0, 0.0, 10.0);
        assert!(result.is_err());
    }
}