use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

/// Beginner calculation error types with surgical precision
#[derive(Debug, Clone)]
pub enum BeginnerError {
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
    
    /// Calculation domain error (e.g., negative dimensions)
    DomainError {
        field: String,
        message: String,
    },
    
    /// Generic calculation error
    CalculationError(String),
}

impl fmt::Display for BeginnerError {
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
            Self::CalculationError(msg) => {
                write!(f, "Calculation error: {}", msg)
            }
        }
    }
}

impl std::error::Error for BeginnerError {}

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

impl BeginnerError {
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
                        "Check available calculators at /api/v1/calculus/beginner/catalogue".to_string(),
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
                        format!("See calculator metadata for '{}' at /api/v1/calculus/beginner/catalogue", calculator),
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

/// Axum response implementation
impl IntoResponse for BeginnerError {
    fn into_response(self) -> Response {
        let (status, error_response) = self.to_response();
        (status, Json(error_response)).into_response()
    }
}

/// Result type alias for beginner calculations
pub type BeginnerResult<T> = Result<T, BeginnerError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_parameter_error() {
        let err = BeginnerError::MissingParameter {
            parameter: "width".to_string(),
            calculator: "planter_box".to_string(),
        };
        
        let (status, response) = err.to_response();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.error_type, "missing_parameter");
        assert!(!response.suggestions.is_empty());
    }
}