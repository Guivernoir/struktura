//! TEEP calculation tests
//! 
//! Tests Total Effective Equipment Performance

use super::*;
use crate::calculus::engineer::calculators::production::oee::engine::calculate_oee;

#[test]
fn test_teep_with_all_time() {
    let input = TestFixture::basic()
        .with_teep(24) // 24-hour calendar time
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // TEEP should be calculated
    assert!(
        result.extended_metrics.teep.is_some(),
        "TEEP should be calculated when all_time provided"
    );
    
    let teep = result.extended_metrics.teep.unwrap();
    
    // TEEP should be valid percentage
    assert_valid_percentage(teep.value, "TEEP");
    
    // TEEP should be less than or equal to OEE
    // (since loading factor ≤ 1.0)
    assert!(
        teep.value <= result.core_metrics.oee.value,
        "TEEP should be ≤ OEE"
    );
}

#[test]
fn test_teep_without_all_time() {
    let input = TestFixture::basic().build(); // No all_time
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // TEEP should NOT be calculated
    assert!(
        result.extended_metrics.teep.is_none(),
        "TEEP should not be calculated without all_time"
    );
}

#[test]
fn test_teep_formula() {
    // TEEP = (Operating Time / All Time) × Performance × Quality
    let input = TestFixture::basic()
        .with_time_allocations(8, 0) // 8 hours operating
        .with_teep(24) // 24-hour all time
        .with_production(1000, 950, 50, 0) // 95% quality
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    
    // Loading factor = 8 / 24 = 0.333
    let expected_loading = 8.0 / 24.0;
    let performance = result.core_metrics.performance.value;
    let quality = result.core_metrics.quality.value;
    let expected_teep = expected_loading * performance * quality;
    
    assert_approx_eq(
        teep.value,
        expected_teep,
        0.02,
        "TEEP = Loading × Performance × Quality"
    );
}

#[test]
fn test_teep_vs_oee() {
    // 12-hour shift (12 hours planned) in 24-hour day
    let input = TestFixture::basic()
        .with_planned_time(12) // 12-hour shift
        .with_time_allocations(12, 0) // 12 hours operating out of 24
        .with_teep(24)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    let oee = result.core_metrics.oee.value;
    
    // TEEP = OEE × Loading Factor
    // Loading = 12/24 = 0.5
    // So TEEP ≈ OEE × 0.5
    assert_approx_eq(
        teep.value,
        oee * 0.5,
        0.05,
        "TEEP = OEE × Loading Factor"
    );
}

#[test]
fn test_teep_with_8_hour_shift() {
    // Common scenario: 8-hour shift in 24-hour day
    let input = TestFixture::basic()
        .with_time_allocations(7, 1) // 7 hours running, 1 hour downtime (within 8-hour shift)
        .with_teep(24) // 24-hour calendar time
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    
    // Loading factor = 7/24 = 0.292
    // TEEP should reflect low loading factor
    assert!(
        teep.value < 0.35,
        "TEEP should reflect low loading factor: got {}",
        teep.value
    );
}

#[test]
fn test_teep_with_three_shifts() {
    // Three 8-hour shifts = 24 hours
    let input = TestFixture::basic()
        .with_time_allocations(8, 0) // Full shift running
        .with_teep(24)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    
    // Loading factor = 8/24 = 0.333
    let loading_factor = 8.0 / 24.0;
    let oee = result.core_metrics.oee.value;
    
    assert_approx_eq(
        teep.value,
        oee * loading_factor,
        0.02,
        "TEEP with three-shift operation"
    );
}

#[test]
fn test_teep_with_247_operation() {
    // Continuous 24/7 operation: 24-hour planned time
    let input = TestFixture::basic()
        .with_planned_time(24) // 24-hour operation
        .with_time_allocations(23, 1) // 23 hours running, 1 hour maintenance
        .with_teep(24) // 24-hour calendar time
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    
    // Loading factor = 23/24 ≈ 0.958
    // TEEP should be very close to OEE
    let oee = result.core_metrics.oee.value;
    assert!(
        teep.value > oee * 0.90,
        "TEEP should be close to OEE in 24/7 operation: TEEP={}, OEE={}",
        teep.value,
        oee
    );
}

#[test]
fn test_teep_formula_parameters() {
    let input = TestFixture::basic()
        .with_teep(24)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    
    // Should have formula parameters tracked
    assert!(
        !teep.formula_params.is_empty(),
        "TEEP should track formula parameters"
    );
    
    // Should include key components
    assert!(teep.formula_params.contains_key("operating_time_seconds"));
    assert!(teep.formula_params.contains_key("all_time_seconds"));
    assert!(teep.formula_params.contains_key("loading_factor"));
}

#[test]
fn test_teep_confidence() {
    let input = TestFixture::basic()
        .with_teep(24)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    
    // TEEP should inherit confidence from inputs
    assert_eq!(
        teep.confidence,
        crate::calculus::engineer::calculators::production::oee::domain::Confidence::High
    );
}

#[test]
fn test_teep_zero_operating_time() {
    let input = TestFixture::basic()
        .with_time_allocations(0, 8) // All downtime
        .with_teep(24)
        .with_production(0, 0, 0, 0)
        .build();
    
    let result = calculate_oee(input).expect("Should handle zero operating time");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    
    // TEEP should be 0
    assert_approx_eq(teep.value, 0.0, 0.01, "TEEP with zero operating time");
}

#[test]
fn test_teep_perfect_operation() {
    // Perfect 24/7 operation
    let cycle_time_secs = 25;
    let running_hours = 24;
    let theoretical_max = (running_hours * 3600 / cycle_time_secs) as u32; // 3456 units
    
    let input = TestFixture::basic()
        .with_planned_time(24) // 24-hour operation
        .with_time_allocations(24, 0) // Running 24/7
        .with_teep(24) // 24-hour calendar time
        .with_production(theoretical_max, theoretical_max, 0, 0) // Perfect production
        .with_cycle_time(cycle_time_secs as u64, None)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    
    // With 24/7 operation and perfect production, TEEP should approach 100%
    assert!(
        teep.value > 0.95,
        "TEEP should be very high with perfect 24/7 operation: got {}",
        teep.value
    );
}

#[test]
fn test_teep_loading_factor_calculation() {
    // 8-hour planned shift, 6 hours running
    let input = TestFixture::basic()
        .with_time_allocations(6, 2) // 6 hours running, 2 hours downtime
        .with_production(700, 700, 0, 0) // Realistic production for 6 hours
        .with_teep(24)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    
    // Extract loading factor from formula params
    let loading_factor = teep.formula_params
        .get("loading_factor")
        .expect("Should have loading factor");
    
    // Loading = 6/24 = 0.25
    assert_approx_eq(*loading_factor, 0.25, 0.01, "Loading factor");
}

#[test]
fn test_teep_with_weekend_operation() {
    // Simulate 5-day operation in 7-day week
    // 5 days × 8 hours = 40 hours planned
    let input = TestFixture::basic()
        .with_planned_time(40) // 40 hours planned (5 days)
        .with_time_allocations(38, 2) // 38 hours running, 2 hours downtime
        .with_production(5000, 4800, 200, 0) // Realistic for 38 hours
        .with_teep(168) // Full week (7 days)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    
    // Loading factor = 38/168 ≈ 0.226
    // TEEP should be much lower than OEE
    assert!(
        teep.value < result.core_metrics.oee.value * 0.30,
        "TEEP should reflect low weekly utilization: TEEP={}, OEE={}",
        teep.value,
        result.core_metrics.oee.value
    );
}

#[test]
fn test_teep_deterministic() {
    let input1 = TestFixture::basic().with_teep(24).build();
    let input2 = TestFixture::basic().with_teep(24).build();
    
    let result1 = calculate_oee(input1).expect("Calculation 1 should succeed");
    let result2 = calculate_oee(input2).expect("Calculation 2 should succeed");
    
    let teep1 = result1.extended_metrics.teep.expect("TEEP 1 should be calculated");
    let teep2 = result2.extended_metrics.teep.expect("TEEP 2 should be calculated");
    
    // Should produce identical results
    assert_approx_eq(teep1.value, teep2.value, 0.0001, "Deterministic TEEP");
}

#[test]
fn test_teep_translation_keys() {
    let input = TestFixture::basic()
        .with_teep(24)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated");
    
    // Should have proper translation keys
    assert_eq!(teep.name_key, "metrics.teep");
    assert_eq!(teep.unit_key, "units.percentage");
    assert_eq!(teep.formula_key, "formulas.teep");
}