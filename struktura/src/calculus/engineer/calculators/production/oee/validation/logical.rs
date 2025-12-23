//! Logical validation: Mathematical coherence checks
//! 
//! Rules enforced:
//! - Sum(parts) = Total
//! - Time allocations ≤ planned time
//! - Production counts coherent
//! - Physical impossibilities (units vs capacity)

use super::*;
use serde_json::json;

/// Validates time allocation coherence
pub fn validate_time_allocations(
    planned_time: Duration,
    time_allocations: &[Duration],
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    let total_allocated: Duration = time_allocations.iter().sum();
    
    // Fatal: Time allocations exceed planned time
    if total_allocated > planned_time {
        result.add_issue(
            ValidationIssue::fatal(
                "TIME_ALLOCATION_EXCEEDS_PLANNED",
                "validation.error.time_allocation_exceeds_planned",
                json!({
                    "allocated_seconds": total_allocated.as_secs(),
                    "planned_seconds": planned_time.as_secs(),
                    "excess_seconds": (total_allocated - planned_time).as_secs(),
                }),
            )
            .with_field("time_allocations"),
        );
    }
    
    // Warning: Time allocations significantly less than planned (>5% gap)
    let allocated_ratio = total_allocated.as_secs_f64() / planned_time.as_secs_f64();
    if allocated_ratio < 0.95 && !time_allocations.is_empty() {
        result.add_issue(
            ValidationIssue::warning(
                "TIME_ALLOCATION_GAP",
                "validation.warning.time_allocation_gap",
                json!({
                    "allocated_seconds": total_allocated.as_secs(),
                    "planned_seconds": planned_time.as_secs(),
                    "gap_seconds": (planned_time - total_allocated).as_secs(),
                    "gap_percentage": ((1.0 - allocated_ratio) * 100.0).round(),
                }),
            )
            .with_field("time_allocations"),
        );
    }
    
    result
}

/// Validates production count coherence
pub fn validate_production_counts(
    total_units: u32,
    good_units: u32,
    scrap_units: u32,
    reworked_units: u32,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    let parts_sum = good_units + scrap_units + reworked_units;
    
    // Fatal: Parts don't sum to total
    if parts_sum != total_units {
        result.add_issue(
            ValidationIssue::fatal(
                "PRODUCTION_COUNT_MISMATCH",
                "validation.error.production_count_mismatch",
                json!({
                    "total_units": total_units,
                    "good_units": good_units,
                    "scrap_units": scrap_units,
                    "reworked_units": reworked_units,
                    "parts_sum": parts_sum,
                    "difference": (parts_sum as i64) - (total_units as i64),
                }),
            )
            .with_field("production"),
        );
    }
    
    // Info: Zero production
    if total_units == 0 {
        result.add_issue(
            ValidationIssue::info(
                "ZERO_PRODUCTION",
                "validation.info.zero_production",
                json!({}),
            )
            .with_field("production.total_units"),
        );
    }
    
    result
}

/// Validates cycle time relationships
pub fn validate_cycle_times(
    ideal_cycle_time: Duration,
    average_cycle_time: Option<Duration>,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    if let Some(avg) = average_cycle_time {
        // Warning: Average cycle time less than ideal (physically impossible for sustained production)
        if avg < ideal_cycle_time {
            result.add_issue(
                ValidationIssue::warning(
                    "CYCLE_TIME_BELOW_IDEAL",
                    "validation.warning.cycle_time_below_ideal",
                    json!({
                        "ideal_seconds": ideal_cycle_time.as_secs_f64(),
                        "average_seconds": avg.as_secs_f64(),
                        "difference_seconds": (ideal_cycle_time - avg).as_secs_f64(),
                    }),
                )
                .with_field("cycle_time.average_cycle_time"),
            );
        }
        
        // Warning: Average significantly higher than ideal (>50% - suggests major performance issues)
        let ratio = avg.as_secs_f64() / ideal_cycle_time.as_secs_f64();
        if ratio > 1.5 {
            result.add_issue(
                ValidationIssue::warning(
                    "CYCLE_TIME_SIGNIFICANTLY_HIGHER",
                    "validation.warning.cycle_time_significantly_higher",
                    json!({
                        "ideal_seconds": ideal_cycle_time.as_secs_f64(),
                        "average_seconds": avg.as_secs_f64(),
                        "ratio": (ratio * 100.0).round(),
                    }),
                )
                .with_field("cycle_time.average_cycle_time"),
            );
        }
    }
    
    result
}

/// Validates theoretical capacity vs actual production
pub fn validate_capacity_constraints(
    total_units: u32,
    running_time: Duration,
    ideal_cycle_time: Duration,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    if ideal_cycle_time.as_secs() == 0 {
        result.add_issue(
            ValidationIssue::fatal(
                "ZERO_CYCLE_TIME",
                "validation.error.zero_cycle_time",
                json!({}),
            )
            .with_field("cycle_time.ideal_cycle_time"),
        );
        return result;
    }
    
    let theoretical_max = (running_time.as_secs_f64() / ideal_cycle_time.as_secs_f64()).floor() as u32;
    
    // Fatal: Production exceeds theoretical capacity
    if total_units > theoretical_max {
        result.add_issue(
            ValidationIssue::fatal(
                "PRODUCTION_EXCEEDS_CAPACITY",
                "validation.error.production_exceeds_capacity",
                json!({
                    "total_units": total_units,
                    "theoretical_max": theoretical_max,
                    "running_seconds": running_time.as_secs(),
                    "ideal_cycle_seconds": ideal_cycle_time.as_secs_f64(),
                    "excess_units": total_units - theoretical_max,
                }),
            )
            .with_field("production.total_units"),
        );
    }
    
    result
}

/// Validates downtime records consistency
pub fn validate_downtime_records(
    downtime_records: &[Duration],
    total_stopped_time: Duration,
) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    let records_sum: Duration = downtime_records.iter().sum();
    
    // Warning: Downtime records don't match stopped time allocation
    let diff_seconds = (records_sum.as_secs() as i64) - (total_stopped_time.as_secs() as i64);
    if diff_seconds.abs() > 60 {  // Allow 1-minute tolerance
        result.add_issue(
            ValidationIssue::warning(
                "DOWNTIME_RECORD_MISMATCH",
                "validation.warning.downtime_record_mismatch",
                json!({
                    "records_sum_seconds": records_sum.as_secs(),
                    "stopped_time_seconds": total_stopped_time.as_secs(),
                    "difference_seconds": diff_seconds,
                }),
            )
            .with_field("downtimes"),
        );
    }
    
    result
}
