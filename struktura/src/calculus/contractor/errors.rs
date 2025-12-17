use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

/// Contracting calculation error types with surgical precision
#[derive(Debug, Clone)]
pub enum ContractingError {
    /// Calculator not found in registry
    CalculatorNotFound(String),
    
    /// Required parameter missing
    MissingParameter {
        parameter: String,
        calculator: String,
    },
    
    /// Invalid parameter value
    InvalidParameter {
        parameter: String,
        value: String,
        reason: String,
    },
    
    /// Calculation domain error (e.g., negative costs)
    DomainError {
        field: String,
        message: String,
    },
    
    /// Unit conversion error
    UnitError {
        from_unit: String,
        to_unit: String,
        message: String,
    },
    
    /// Regulation compliance violation
    ComplianceViolation {
        code: String,
        requirement: String,
        actual: String,
    },
    
    /// Numerical instability or convergence failure
    NumericalError(String),
    
    /// Safety factor violation
    SafetyViolation {
        parameter: String,
        required: f64,
        actual: f64,
    },
    
    /// Generic calculation error
    CalculationError(String),
}

impl fmt::Display for ContractingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CalculatorNotFound(calc) => {
                write!(f, "Calculator '{}' not found in registry", calc)
            }
            Self::MissingParameter { parameter, calculator } => {
                write!(
                    f,
                    "Required parameter '{}' missing for calculator '{}'",
                    parameter, calculator
                )
            }
            Self::InvalidParameter { parameter, value, reason } => {
                write!(
                    f,
                    "Invalid value '{}' for parameter '{}': {}",
                    value, parameter, reason
                )
            }
            Self::DomainError { field, message } => {
                write!(f, "Domain error in '{}': {}", field, message)
            }
            Self::UnitError { from_unit, to_unit, message } => {
                write!(
                    f,
                    "Unit conversion error from '{}' to '{}': {}",
                    from_unit, to_unit, message
                )
            }
            Self::ComplianceViolation { code, requirement, actual } => {
                write!(
                    f,
                    "Regulation code '{}' violation: required '{}', actual '{}'",
                    code, requirement, actual
                )
            }
            Self::NumericalError(msg) => {
                write!(f, "Numerical error: {}", msg)
            }
            Self::SafetyViolation { parameter, required, actual } => {
                write!(
                    f,
                    "Safety factor violation for '{}': required {:.2}, actual {:.2}",
                    parameter, required, actual
                )
            }
            Self::CalculationError(msg) => {
                write!(f, "Calculation error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ContractingError {}

/// Structured error response for API
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error_type: String,
    pub message: String,
    pub details: Option<ErrorDetails>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetails {
    pub field: Option<String>,
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub constraints: Option<Vec<String>>,
}

impl ContractingError {
    /// Convert error to HTTP response with appropriate status code and suggestions
    pub fn to_response(&self) -> (StatusCode, ErrorResponse) {
        match self {
            Self::CalculatorNotFound(calc) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    error_type: "calculator_not_found".to_string(),
                    message: self.to_string(),
                    details: Some(ErrorDetails {
                        field: Some("calculation_type".to_string()),
                        expected: None,
                        actual: Some(calc.clone()),
                        constraints: None,
                    }),
                    suggestions: vec![
                        "Check available calculators at /api/v1/calculus/contractor/catalogue".to_string(),
                        "Verify calculator ID matches exactly (case-sensitive)".to_string(),
                    ],
                },
            ),
            
            Self::MissingParameter { parameter, calculator } => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error_type: "missing_parameter".to_string(),
                    message: self.to_string(),
                    details: Some(ErrorDetails {
                        field: Some(parameter.clone()),
                        expected: Some("required value".to_string()),
                        actual: Some("null/undefined".to_string()),
                        constraints: None,
                    }),
                    suggestions: vec![
                        format!("Provide '{}' in request parameters", parameter),
                        format!("See calculator metadata for '{}' at /api/v1/calculus/contractor/catalogue", calculator),
                    ],
                },
            ),
            
            Self::InvalidParameter { parameter, value, reason } => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error_type: "invalid_parameter".to_string(),
                    message: self.to_string(),
                    details: Some(ErrorDetails {
                        field: Some(parameter.clone()),
                        expected: None,
                        actual: Some(value.clone()),
                        constraints: Some(vec![reason.clone()]),
                    }),
                    suggestions: vec![
                        format!("Verify '{}' meets constraints: {}", parameter, reason),
                        "Check units and magnitude are correct".to_string(),
                    ],
                },
            ),
            
            Self::DomainError { field, message } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse {
                    error_type: "domain_error".to_string(),
                    message: self.to_string(),
                    details: Some(ErrorDetails {
                        field: Some(field.clone()),
                        expected: None,
                        actual: None,
                        constraints: Some(vec![message.clone()]),
                    }),
                    suggestions: vec![
                        "Verify physical constraints are satisfied".to_string(),
                        format!("Check '{}' for realistic values", field),
                    ],
                },
            ),
            
            Self::SafetyViolation { parameter, required, actual } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse {
                    error_type: "safety_violation".to_string(),
                    message: self.to_string(),
                    details: Some(ErrorDetails {
                        field: Some(parameter.clone()),
                        expected: Some(format!(">= {:.2}", required)),
                        actual: Some(format!("{:.2}", actual)),
                        constraints: Some(vec!["Regulation safety requirement".to_string()]),
                    }),
                    suggestions: vec![
                        "Adjust resources or timeline".to_string(),
                        "Review requirements and factors".to_string(),
                        "Consider alternative approaches".to_string(),
                    ],
                },
            ),
            
            Self::ComplianceViolation { code, requirement, actual } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse {
                    error_type: "compliance_violation".to_string(),
                    message: self.to_string(),
                    details: Some(ErrorDetails {
                        field: None,
                        expected: Some(requirement.clone()),
                        actual: Some(actual.clone()),
                        constraints: Some(vec![format!("Per {}", code)]),
                    }),
                    suggestions: vec![
                        format!("Review {} requirements", code),
                        "Adjust plan to meet specifications".to_string(),
                        "Consult with experts for compliance verification".to_string(),
                    ],
                },
            ),
            
            Self::NumericalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_type: "numerical_error".to_string(),
                    message: msg.clone(),
                    details: None,
                    suggestions: vec![
                        "Try different input values".to_string(),
                        "Check for extreme values that may cause instability".to_string(),
                        "Contact support if problem persists".to_string(),
                    ],
                },
            ),
            
            Self::UnitError { from_unit, to_unit, message } => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error_type: "unit_error".to_string(),
                    message: self.to_string(),
                    details: Some(ErrorDetails {
                        field: None,
                        expected: Some(to_unit.clone()),
                        actual: Some(from_unit.clone()),
                        constraints: Some(vec![message.clone()]),
                    }),
                    suggestions: vec![
                        "Ensure all inputs use consistent units".to_string(),
                        "Refer to API documentation for expected units".to_string(),
                    ],
                },
            ),
            
            Self::CalculationError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_type: "calculation_error".to_string(),
                    message: msg.clone(),
                    details: None,
                    suggestions: vec![
                        "Verify input parameters are within reasonable ranges".to_string(),
                        "Contact support with request details".to_string(),
                    ],
                },
            ),
        }
    }
}

/// Axum response implementation with tactical precision
impl IntoResponse for ContractingError {
    fn into_response(self) -> Response {
        let (status, error_response) = self.to_response();
        (status, Json(error_response)).into_response()
    }
}

/// Result type alias for contracting calculations
pub type ContractingResult<T> = Result<T, ContractingError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_parameter_error() {
        let err = ContractingError::MissingParameter {
            parameter: "dimensions.area".to_string(),
            calculator: "bid_pricing".to_string(),
        };
        
        let (status, response) = err.to_response();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.error_type, "missing_parameter");
        assert!(!response.suggestions.is_empty());
    }
    
    #[test]
    fn test_safety_violation_error() {
        let err = ContractingError::SafetyViolation {
            parameter: "FOS_risk".to_string(),
            required: 2.0,
            actual: 1.5,
        };
        
        let (status, _) = err.to_response();
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }
}