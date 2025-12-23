//! Range validation: Boundary checks for numeric values
//! 
//! Ensures values are within physically/mathematically sensible bounds

use super::*;
use serde_json::json;

/// Validates percentage is in [0, 1] range
pub fn validate_percentage(
    value: f64,
    field_name: &str,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    if !(0.0..=1.0).contains(&value) {
        result.add_issue(
            ValidationIssue::fatal(
                "PERCENTAGE_OUT_OF_RANGE",
                "validation.error.percentage_out_of_range",
                json!({
                    "field": field_name,
                    "value": value,
                    "min": 0.0,
                    "max": 1.0,
                }),
            )
            .with_field(field_name),
        );
    }
    
    result
}

/// Validates duration is positive
pub fn validate_positive_duration(
    value: Duration,
    field_name: &str,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    if value.as_secs() == 0 && value.subsec_nanos() == 0 {
        result.add_issue(
            ValidationIssue::warning(
                "ZERO_DURATION",
                "validation.warning.zero_duration",
                json!({
                    "field": field_name,
                }),
            )
            .with_field(field_name),
        );
    }
    
    result
}

/// Validates count is non-negative
pub fn validate_non_negative_count(
    value: i64,
    field_name: &str,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    if value < 0 {
        result.add_issue(
            ValidationIssue::fatal(
                "NEGATIVE_COUNT",
                "validation.error.negative_count",
                json!({
                    "field": field_name,
                    "value": value,
                }),
            )
            .with_field(field_name),
        );
    }
    
    result
}

/// Validates a value is within a specified range
pub fn validate_range<T: PartialOrd + serde::Serialize>(
    value: T,
    min: T,
    max: T,
    field_name: &str,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    if value < min || value > max {
        result.add_issue(
            ValidationIssue::fatal(
                "VALUE_OUT_OF_RANGE",
                "validation.error.value_out_of_range",
                json!({
                    "field": field_name,
                    "value": value,
                    "min": min,
                    "max": max,
                }),
            )
            .with_field(field_name),
        );
    }
    
    result
}
