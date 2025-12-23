//! Core OEE calculation tests
//! 
//! Tests the fundamental A × P × Q = OEE calculations

use super::*;
use crate::calculus::engineer::calculators::production::oee::{
    domain::Confidence,
    engine::calculate_oee,
};

#[test]
fn test_perfect_oee() {
    // Perfect scenario: no downtime, no scrap, ideal cycle time
    let input = TestFixture::basic()
        .with_production(1000, 1000, 0, 0)
        .with_time_allocations(8, 0)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Availability = 100% (no downtime)
    assert_approx_eq(result.core_metrics.availability.value, 1.0, 0.01, "Availability");
    
    // Performance should be 1.0 or close (1000 units in 8 hours at 25s cycle = 1152 theoretical)
    assert_valid_percentage(result.core_metrics.performance.value, "Performance");
    
    // Quality = 100% (no scrap)
    assert_approx_eq(result.core_metrics.quality.value, 1.0, 0.01, "Quality");
    
    // OEE should be high
    assert_oee_in_range(result.core_metrics.oee.value, 0.85, 1.0);
}

#[test]
fn test_zero_production() {
    let input = TestFixture::basic()
        .with_production(0, 0, 0, 0)
        .build();
    
    let result = calculate_oee(input).expect("Should handle zero production");
    
    // Should have validation warnings
    assert!(result.validation.has_warnings());
    
    // Metrics should handle zeros gracefully
    assert_valid_percentage(result.core_metrics.availability.value, "Availability");
    assert_valid_percentage(result.core_metrics.performance.value, "Performance");
    assert_valid_percentage(result.core_metrics.quality.value, "Quality");
}

#[test]
fn test_availability_calculation() {
    // Test with known downtime
    let input = TestFixture::basic()
        .with_time_allocations(7, 1) // 7 hours running, 1 hour down
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Availability = (8 - 1) / 8 = 87.5%
    assert_approx_eq(result.core_metrics.availability.value, 0.875, 0.01, "Availability");
}

#[test]
fn test_performance_calculation() {
    // Test performance with known cycle time
    let input = TestFixture::basic()
        .with_production(800, 800, 0, 0) // 800 units in 7 hours
        .with_cycle_time(25, None) // 25 seconds ideal
        .with_time_allocations(7, 1)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Theoretical: 7 hours = 25200 seconds / 25 = 1008 units possible
    // Actual: 800 units
    // Performance = (800 * 25) / 25200 = 0.793
    assert_approx_eq(result.core_metrics.performance.value, 0.793, 0.01, "Performance");
}

#[test]
fn test_quality_calculation() {
    let input = TestFixture::basic()
        .with_production(1000, 900, 100, 0) // 90% quality
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Quality = 900 / 1000 = 90%
    assert_approx_eq(result.core_metrics.quality.value, 0.90, 0.01, "Quality");
}

#[test]
fn test_oee_multiplication() {
    // Test that OEE = A × P × Q
    let input = TestFixture::basic()
        .with_production(800, 720, 80, 0) // 90% quality (720/800)
        .with_time_allocations(7, 1) // 87.5% availability
        .with_cycle_time(25, None)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    let a = result.core_metrics.availability.value;
    let p = result.core_metrics.performance.value;
    let q = result.core_metrics.quality.value;
    let oee = result.core_metrics.oee.value;
    
    // OEE should equal A × P × Q
    assert_approx_eq(oee, a * p * q, 0.001, "OEE = A × P × Q");
}

#[test]
fn test_confidence_tracking() {
    // Test with all explicit values
    let input = TestFixture::basic().build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Should have high confidence with explicit values
    assert_eq!(result.core_metrics.oee.confidence, Confidence::High);
}

#[test]
fn test_rework_units() {
    let input = TestFixture::basic()
        .with_production(1000, 900, 50, 50) // 50 reworked
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Rework should be tracked in extended metrics
    assert_approx_eq(
        result.extended_metrics.rework_rate.value,
        0.05,
        0.01,
        "Rework rate"
    );
}

#[test]
fn test_scrap_rate_high_warning() {
    // High scrap rate should trigger warning
    let input = TestFixture::basic()
        .with_production(1000, 700, 300, 0) // 30% scrap
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Should have warning about high scrap rate
    assert!(
        result.validation.has_warnings(),
        "Should warn about high scrap rate"
    );
    
    // But calculation should still complete
    assert_valid_percentage(result.core_metrics.quality.value, "Quality");
}

#[test]
fn test_low_utilization_warning() {
    // Low running time should trigger warning
    let cycle_time = 25;
    let running_hours = 2;
    let theoretical = (running_hours * 3600 / cycle_time) as u32;
    
    let input = TestFixture::basic()
        .with_time_allocations(running_hours, 6) // Only 2 hours running out of 8
        .with_production(theoretical, theoretical, 0, 0)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    assert!(
        result.validation.has_warnings(),
        "Should warn about low utilization"
    );
}

#[test]
fn test_extended_metrics_calculation() {
    let input = TestFixture::basic()
        .with_downtime(1800, true) // 30 min failure
        .with_downtime(1800, true) // 30 min failure
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // MTBF should be calculated
    assert!(
        result.extended_metrics.mtbf.is_some(),
        "MTBF should be calculated"
    );
    
    // MTTR should be calculated
    assert!(
        result.extended_metrics.mttr.is_some(),
        "MTTR should be calculated"
    );
}

#[test]
fn test_all_metrics_valid_percentages() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // All core metrics should be valid percentages
    assert_valid_percentage(result.core_metrics.availability.value, "Availability");
    assert_valid_percentage(result.core_metrics.performance.value, "Performance");
    assert_valid_percentage(result.core_metrics.quality.value, "Quality");
    assert_valid_percentage(result.core_metrics.oee.value, "OEE");
    
    // Extended metrics too
    assert_valid_percentage(result.extended_metrics.utilization.value, "Utilization");
    assert_valid_percentage(result.extended_metrics.scrap_rate.value, "Scrap rate");
    assert_valid_percentage(result.extended_metrics.rework_rate.value, "Rework rate");
}

#[test]
fn test_formula_parameters_recorded() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // OEE metric should have formula parameters
    assert!(
        !result.core_metrics.oee.formula_params.is_empty(),
        "OEE should have formula parameters"
    );
    
    // Should include A, P, Q
    assert!(result.core_metrics.oee.formula_params.contains_key("availability"));
    assert!(result.core_metrics.oee.formula_params.contains_key("performance"));
    assert!(result.core_metrics.oee.formula_params.contains_key("quality"));
}

#[test]
fn test_assumption_ledger_populated() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Ledger should track assumptions
    assert!(
        !result.ledger.assumptions.is_empty(),
        "Ledger should track assumptions"
    );
    
    // Should track critical inputs
    let critical = result.ledger.critical_assumptions();
    assert!(
        !critical.is_empty(),
        "Should have critical assumptions tracked"
    );
}

#[test]
fn test_loss_tree_structure() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Loss tree should have root node
    assert_eq!(
        result.loss_tree.root.category_key,
        "loss_tree.planned_time"
    );
    
    // Should have child categories
    assert!(
        !result.loss_tree.root.children.is_empty(),
        "Loss tree should have categories"
    );
}

#[test]
fn test_deterministic_calculation() {
    // Same input should produce same output
    let input1 = TestFixture::basic().build();
    let input2 = TestFixture::basic().build();
    
    let result1 = calculate_oee(input1).expect("Calculation 1 should succeed");
    let result2 = calculate_oee(input2).expect("Calculation 2 should succeed");
    
    // Results should be identical
    assert_approx_eq(
        result1.core_metrics.oee.value,
        result2.core_metrics.oee.value,
        0.0001,
        "Deterministic OEE"
    );
}

#[test]
fn test_edge_case_all_downtime() {
    let input = TestFixture::basic()
        .with_time_allocations(0, 8) // All downtime
        .with_production(0, 0, 0, 0)
        .build();
    
    let result = calculate_oee(input).expect("Should handle all downtime");
    
    // Availability should be 0
    assert_approx_eq(result.core_metrics.availability.value, 0.0, 0.01, "Availability");
    
    // OEE should be 0
    assert_approx_eq(result.core_metrics.oee.value, 0.0, 0.01, "OEE");
}

#[test]
fn test_edge_case_all_scrap() {
    let input = TestFixture::basic()
        .with_production(1000, 0, 1000, 0) // All scrap
        .build();
    
    let result = calculate_oee(input).expect("Should handle all scrap");
    
    // Quality should be 0
    assert_approx_eq(result.core_metrics.quality.value, 0.0, 0.01, "Quality");
    
    // OEE should be 0
    assert_approx_eq(result.core_metrics.oee.value, 0.0, 0.01, "OEE");
}