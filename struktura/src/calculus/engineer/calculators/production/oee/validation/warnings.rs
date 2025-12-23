//! Warnings: Non-fatal issues that suggest data quality problems
//! 
//! These don't block calculation but alert users to potential issues

use super::*;
use serde_json::json;

/// Checks for suspiciously high scrap rate
pub fn check_high_scrap_rate(
    scrap_units: u32,
    total_units: u32,
    threshold: f64,  // e.g., 0.2 for 20%
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    if total_units > 0 {
        let scrap_rate = scrap_units as f64 / total_units as f64;
        
        if scrap_rate > threshold {
            result.add_issue(
                ValidationIssue::warning(
                    "HIGH_SCRAP_RATE",
                    "validation.warning.high_scrap_rate",
                    json!({
                        "scrap_units": scrap_units,
                        "total_units": total_units,
                        "scrap_rate": (scrap_rate * 100.0).round(),
                        "threshold": (threshold * 100.0).round(),
                    }),
                )
                .with_field("production.scrap_units"),
            );
        }
    }
    
    result
}

/// Checks for suspiciously low utilization
pub fn check_low_utilization(
    running_time: Duration,
    planned_time: Duration,
    threshold: f64,  // e.g., 0.3 for 30%
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    if planned_time.as_secs() > 0 {
        let utilization = running_time.as_secs_f64() / planned_time.as_secs_f64();
        
        if utilization < threshold {
            result.add_issue(
                ValidationIssue::warning(
                    "LOW_UTILIZATION",
                    "validation.warning.low_utilization",
                    json!({
                        "running_seconds": running_time.as_secs(),
                        "planned_seconds": planned_time.as_secs(),
                        "utilization": (utilization * 100.0).round(),
                        "threshold": (threshold * 100.0).round(),
                    }),
                )
                .with_field("time_allocations"),
            );
        }
    }
    
    result
}

/// Checks for missing reason codes
pub fn check_missing_reason_codes(
    records_with_reasons: usize,
    total_records: usize,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    if total_records > 0 && records_with_reasons < total_records {
        let missing_count = total_records - records_with_reasons;
        let missing_percentage = (missing_count as f64 / total_records as f64) * 100.0;
        
        result.add_issue(
            ValidationIssue::info(
                "MISSING_REASON_CODES",
                "validation.info.missing_reason_codes",
                json!({
                    "missing_count": missing_count,
                    "total_records": total_records,
                    "missing_percentage": missing_percentage.round(),
                }),
            )
            .with_field("downtimes"),
        );
    }
    
    result
}

/// Checks for data source inconsistencies
pub fn check_input_source_quality(
    explicit_count: usize,
    inferred_count: usize,
    default_count: usize,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    let total = explicit_count + inferred_count + default_count;
    
    if total > 0 {
        let default_percentage = (default_count as f64 / total as f64) * 100.0;
        
        // Warning: High percentage of defaults (>30%)
        if default_percentage > 30.0 {
            result.add_issue(
                ValidationIssue::warning(
                    "HIGH_DEFAULT_USAGE",
                    "validation.warning.high_default_usage",
                    json!({
                        "default_count": default_count,
                        "total_inputs": total,
                        "default_percentage": default_percentage.round(),
                    }),
                ),
            );
        }
        
        // Info: Report input source distribution
        result.add_issue(
            ValidationIssue::info(
                "INPUT_SOURCE_DISTRIBUTION",
                "validation.info.input_source_distribution",
                json!({
                    "explicit_count": explicit_count,
                    "inferred_count": inferred_count,
                    "default_count": default_count,
                    "explicit_percentage": ((explicit_count as f64 / total as f64) * 100.0).round(),
                }),
            ),
        );
    }
    
    result
}

/// Checks for unusual shift patterns
pub fn check_unusual_shift_duration(
    planned_time: Duration,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    let hours = planned_time.as_secs_f64() / 3600.0;
    
    // Info: Unusually short shift (<2 hours)
    if hours < 2.0 {
        result.add_issue(
            ValidationIssue::info(
                "SHORT_ANALYSIS_WINDOW",
                "validation.info.short_analysis_window",
                json!({
                    "duration_hours": hours.round(),
                }),
            )
            .with_field("planned_production_time"),
        );
    }
    
    // Info: Unusually long shift (>24 hours)
    if hours > 24.0 {
        result.add_issue(
            ValidationIssue::info(
                "LONG_ANALYSIS_WINDOW",
                "validation.info.long_analysis_window",
                json!({
                    "duration_hours": hours.round(),
                }),
            )
            .with_field("planned_production_time"),
        );
    }
    
    result
}