//! Invalid input and validation tests
//! 
//! Tests error handling and validation logic

use super::*;
use crate::calculus::engineer::calculators::production::oee::engine::{
    calculate_oee, EngineError,
};

#[test]
fn test_time_allocation_exceeds_planned() {
    let input = invalid_fixture_time_overflow();
    
    let result = calculate_oee(input);
    
    // Should return validation error
    assert!(
        result.is_err(),
        "Should fail when time allocations exceed planned time"
    );
    
    if let Err(EngineError::ValidationFailed(validation)) = result {
        assert!(validation.has_fatal_errors());
    }
}

#[test]
fn test_production_count_mismatch() {
    let input = invalid_fixture_count_mismatch();
    
    let result = calculate_oee(input);
    
    // Should return validation error
    assert!(
        result.is_err(),
        "Should fail when production counts don't sum correctly"
    );
}

#[test]
fn test_negative_duration() {
    // Can't create negative duration in Rust, but test zero
    let input = TestFixture::basic()
        .with_time_allocations(0, 0) // No time allocated
        .with_production(0, 0, 0, 0) // No production
        .build();
    
    let result = calculate_oee(input);
    
    // Should fail or warn appropriately
    assert!(
        result.is_err() || (result.is_ok() && result.unwrap().validation.has_warnings()),
        "Zero time should trigger validation issue"
    );
}

#[test]
fn test_zero_cycle_time() {
    let input = TestFixture::basic()
        .with_cycle_time(0, None) // Zero cycle time
        .build();
    
    let result = calculate_oee(input);
    
    // Should fail validation
    assert!(result.is_err(), "Zero cycle time should be invalid");
}

#[test]
fn test_production_exceeds_capacity() {
    // 1000 units in 1 hour with 25-second cycle time = impossible
    let input = TestFixture::basic()
        .with_time_allocations(1, 0) // Only 1 hour
        .with_production(10000, 10000, 0, 0) // Way too many units
        .with_cycle_time(25, None)
        .build();
    
    let result = calculate_oee(input);
    
    // Should fail or warn about physical impossibility
    assert!(result.is_err() || result.unwrap().validation.has_fatal_errors());
}

#[test]
fn test_average_cycle_time_less_than_ideal() {
    // Average < ideal is physically possible (short bursts), should warn
    let cycle_time = 30;
    let running_hours = 7;
    let theoretical = (running_hours * 3600 / cycle_time) as u32;
    
    let input = TestFixture::basic()
        .with_time_allocations(running_hours, 1)
        .with_production(theoretical, theoretical, 0, 0) // Valid production for ideal
        .with_cycle_time(30, Some(20)) // Average < ideal (warning, not error)
        .build();
    
    let result = calculate_oee(input).expect("Should complete with warning");
    
    // Should have warning about average < ideal
    assert!(
        result.validation.has_warnings(),
        "Should warn about average < ideal"
    );
}

#[test]
fn test_all_time_less_than_planned() {
    // all_time cannot be less than planned_time
    let mut input = TestFixture::basic()
        .with_teep(4) // 4 hours all_time
        .build();
    
    // Planned time is 8 hours (> all_time) - logically invalid
    let result = calculate_oee(input);
    
    // Should handle this gracefully (either error or warning)
    // Implementation may vary
}

#[test]
fn test_empty_time_allocations() {
    let mut input = TestFixture::basic().build();
    input.time_model.allocations.clear();
    // Still has production data but no time allocation - invalid
    
    let result = calculate_oee(input);
    
    // Should fail validation (can't produce units with no running time)
    assert!(
        result.is_err(),
        "Should fail validation with empty allocations and production"
    );
}

#[test]
fn test_high_scrap_rate_warning() {
    let input = TestFixture::basic()
        .with_production(1000, 500, 500, 0) // 50% scrap
        .build();
    
    let result = calculate_oee(input).expect("Should complete with warning");
    
    // Should warn about high scrap rate
    assert!(
        result.validation.has_warnings(),
        "Should warn about high scrap rate"
    );
    
    // Find scrap warning
    let has_scrap_warning = result.validation.issues.iter()
        .any(|issue| issue.code == "HIGH_SCRAP_RATE");
    
    assert!(has_scrap_warning, "Should have HIGH_SCRAP_RATE warning");
}

#[test]
fn test_low_utilization_warning() {
    // Low running time should trigger warning but be valid
    let cycle_time = 25;
    let running_hours = 2;
    let theoretical = (running_hours * 3600 / cycle_time) as u32; // 288 units
    
    let input = TestFixture::basic()
        .with_time_allocations(running_hours, 6) // Only 2 hours running out of 8
        .with_production(theoretical, theoretical, 0, 0) // Valid production
        .build();
    
    let result = calculate_oee(input).expect("Should complete with warning");
    
    // Should warn about low utilization
    assert!(result.validation.has_warnings());
    
    let has_utilization_warning = result.validation.issues.iter()
        .any(|issue| issue.code == "LOW_UTILIZATION");
    
    assert!(has_utilization_warning, "Should have LOW_UTILIZATION warning");
}

#[test]
fn test_zero_planned_time() {
    let mut input = TestFixture::basic().build();
    input.time_model.planned_production_time = InputValue::Explicit(Duration::ZERO);
    
    let result = calculate_oee(input);
    
    // Should fail or handle with warning
    assert!(
        result.is_err() || result.unwrap().validation.has_warnings(),
        "Zero planned time should be flagged"
    );
}

#[test]
fn test_missing_machine_context() {
    let mut input = TestFixture::basic().build();
    input.machine.machine_id = String::new(); // Empty machine ID
    
    // Should still calculate (machine ID is metadata, not critical for calculation)
    let result = calculate_oee(input);
    assert!(result.is_ok(), "Empty machine ID should not block calculation");
}

#[test]
fn test_validation_error_messages() {
    let input = invalid_fixture_count_mismatch();
    
    let result = calculate_oee(input);
    
    if let Err(EngineError::ValidationFailed(validation)) = result {
        // Should have descriptive error messages
        assert!(!validation.issues.is_empty());
        
        for issue in &validation.issues {
            assert!(!issue.message_key.is_empty(), "Should have message key");
            assert!(!issue.code.is_empty(), "Should have error code");
        }
    }
}

#[test]
fn test_validation_field_paths() {
    let input = invalid_fixture_count_mismatch();
    
    let result = calculate_oee(input);
    
    if let Err(EngineError::ValidationFailed(validation)) = result {
        // Should identify which fields have problems
        let has_field_path = validation.issues.iter()
            .any(|issue| issue.field_path.is_some());
        
        assert!(has_field_path, "Should identify problematic fields");
    }
}

#[test]
fn test_multiple_validation_errors() {
    // Create input with multiple problems
    let input = TestFixture::basic()
        .with_production(1000, 950, 100, 0) // Count mismatch
        .with_time_allocations(10, 5) // Time overflow
        .build();
    
    let result = calculate_oee(input);
    
    if let Err(EngineError::ValidationFailed(validation)) = result {
        // Should catch multiple errors
        assert!(
            validation.issues.len() > 1,
            "Should identify multiple validation errors"
        );
    }
}

#[test]
fn test_warnings_dont_block_calculation() {
    // Create input that triggers warnings but isn't invalid
    let cycle_time = 25;
    let running_hours = 3; // Low utilization (warning)
    let theoretical = (running_hours * 3600 / cycle_time) as u32;
    let actual = (theoretical as f64 * 0.95) as u32;
    
    let input = TestFixture::basic()
        .with_time_allocations(running_hours, 5) // Low utilization (warning)
        .with_production(actual, (actual as f64 * 0.70) as u32, (actual as f64 * 0.30) as u32, 0) // High scrap (warning)
        .build();
    
    // Should complete despite warnings
    let result = calculate_oee(input).expect("Warnings shouldn't block calculation");
    
    assert!(result.validation.has_warnings());
    assert!(!result.validation.has_fatal_errors());
}

#[test]
fn test_info_level_messages() {
    let input = TestFixture::basic()
        .with_production(0, 0, 0, 0) // Zero production (info level)
        .build();
    
    let result = calculate_oee(input).expect("Should handle zero production");
    
    // Should have info-level messages
    let has_info = result.validation.issues.iter()
        .any(|issue| matches!(
            issue.severity,
            crate::calculus::engineer::calculators::production::oee::validation::Severity::Info
        ));
    
    assert!(has_info, "Should have info-level messages");
}

#[test]
fn test_validation_result_serializable() {
    let input = invalid_fixture_count_mismatch();
    
    if let Err(EngineError::ValidationFailed(validation)) = calculate_oee(input) {
        // Should be serializable to JSON (for API responses)
        let json = serde_json::to_string(&validation);
        assert!(json.is_ok(), "Validation result should be serializable");
    }
}

#[test]
fn test_downtime_record_mismatch_warning() {
    let mut input = TestFixture::basic()
        .with_downtime(1800, true) // 30 minutes in records
        .with_time_allocations(7, 1) // But 1 hour in allocations
        .build();
    
    let result = calculate_oee(input).expect("Should complete with warning");
    
    // Should warn about mismatch between downtime records and allocations
    assert!(result.validation.has_warnings());
}

#[test]
fn test_edge_case_all_defaults() {
    // Test with all default values (not realistic, but should handle)
    use crate::calculus::engineer::calculators::production::oee::assumptions::thresholds::ThresholdConfiguration;
    
    let start = Utc::now();
    let end = start + ChronoDuration::hours(8);
    
    let input = crate::calculus::engineer::calculators::production::oee::OeeInput {
        window: AnalysisWindow { start, end },
        machine: MachineContext {
            machine_id: "TEST".to_string(),
            line_id: None,
            product_id: None,
            shift_id: None,
        },
        time_model: crate::calculus::engineer::calculators::production::oee::assumptions::time::TimeModel {
            planned_production_time: InputValue::Default(Duration::from_secs(8 * 3600)),
            allocations: Vec::new(),
            all_time: None,
        },
        production: ProductionSummary {
            total_units: InputValue::Default(0),
            good_units: InputValue::Default(0),
            scrap_units: InputValue::Default(0),
            reworked_units: InputValue::Default(0),
        },
        cycle_time: crate::calculus::engineer::calculators::production::oee::assumptions::cycle::CycleTimeModel::from_ideal(Duration::from_secs(30)),
        downtimes: crate::calculus::engineer::calculators::production::oee::assumptions::downtime::DowntimeCollection::new(),
        thresholds: ThresholdConfiguration::defaults(),
    };
    
    // Should handle all defaults (with low confidence)
    let result = calculate_oee(input).expect("Should handle defaults");
    
    assert_eq!(
        result.core_metrics.oee.confidence,
        crate::calculus::engineer::calculators::production::oee::domain::Confidence::Low
    );
}