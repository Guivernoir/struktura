//! Integration tests
//! 
//! End-to-end tests of complete workflows

use super::*;
use crate::calculus::engineer::calculators::production::oee::{
    domain::economics::EconomicParameters,
    engine::{
        calculate_oee, calculate_oee_with_economics,
        leverage::calculate_leverage,
        multi_machine::{aggregate_system_oee, AggregationMethod, MachineOeeData},
        sensitivity::quick_sensitivity_analysis,
    },
};

#[test]
fn test_complete_oee_workflow() {
    // End-to-end test: input -> calculation -> all analyses
    let input = TestFixture::basic()
        .with_production(1000, 900, 80, 20)
        .with_time_allocations(7, 1)
        .with_downtime(1800, true) // 30 min breakdown
        .with_downtime(1800, false) // 30 min other
        .build();
    
    // Calculate base OEE
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    
    // Verify core metrics calculated
    assert_valid_percentage(result.core_metrics.availability.value, "Availability");
    assert_valid_percentage(result.core_metrics.performance.value, "Performance");
    assert_valid_percentage(result.core_metrics.quality.value, "Quality");
    assert_valid_percentage(result.core_metrics.oee.value, "OEE");
    
    // Verify extended metrics
    assert_valid_percentage(result.extended_metrics.utilization.value, "Utilization");
    assert!(result.extended_metrics.mtbf.is_some(), "Should calculate MTBF");
    
    // Verify loss tree
    assert!(!result.loss_tree.root.children.is_empty());
    
    // Verify ledger populated
    assert!(!result.ledger.assumptions.is_empty());
    
    // Run sensitivity analysis
    let sensitivity = quick_sensitivity_analysis(&input, &result.core_metrics);
    assert_eq!(sensitivity.results.len(), 6);
    
    // Run leverage analysis
    let leverage = calculate_leverage(&input, &result.core_metrics);
    assert!(!leverage.is_empty(), "Should identify leverage opportunities");
}

#[test]
fn test_oee_with_economics_workflow() {
    let input = TestFixture::basic()
        .with_production(1000, 850, 150, 0)
        .build();
    
    let economic_params = EconomicParameters::from_point_estimates(
        50.0,  // unit_price
        20.0,  // marginal_contribution
        10.0,  // material_cost
        30.0,  // labor_cost_per_hour
        "USD",
    );
    
    let result = calculate_oee_with_economics(input, economic_params)
        .expect("Calculation with economics should succeed");
    
    // Should have economic analysis
    assert!(
        result.economic_analysis.is_some(),
        "Should include economic analysis"
    );
    
    let econ = result.economic_analysis.unwrap();
    
    // Should calculate all cost components
    assert!(econ.throughput_loss.central_estimate > 0.0);
    assert!(econ.material_waste.central_estimate > 0.0);
    assert!(econ.total_impact.central_estimate > 0.0);
    
    // Total should be sum of components
    let manual_total = econ.throughput_loss.central_estimate
        + econ.material_waste.central_estimate
        + econ.rework_cost.central_estimate
        + econ.opportunity_cost.central_estimate;
    
    assert_approx_eq(
        econ.total_impact.central_estimate,
        manual_total,
        1.0,
        "Total impact"
    );
}

#[test]
fn test_multi_machine_complete_workflow() {
    // Create results for 3 machines
    let machines = vec![
        {
            let cycle_time = 25;
            let running_hours = 7;
            let theoretical = (running_hours * 3600 / cycle_time) as u32;
            let actual = (theoretical as f64 * 0.95) as u32; // 95% performance
            
            let input = TestFixture::basic()
                .with_time_allocations(running_hours, 1)
                .with_production(actual, (actual as f64 * 0.95) as u32, (actual as f64 * 0.05) as u32, 0)
                .build();
            let result = calculate_oee(input).expect("Machine 1 calc");
            MachineOeeData {
                machine_id: "M001".to_string(),
                machine_name: Some("Press 1".to_string()),
                result,
                sequence_position: Some(1),
                is_bottleneck: false,
            }
        },
        {
            let cycle_time = 25;
            let running_hours = 5; // More downtime
            let theoretical = (running_hours * 3600 / cycle_time) as u32;
            let actual = (theoretical as f64 * 0.90) as u32;
            
            let input = TestFixture::basic()
                .with_time_allocations(running_hours, 3)
                .with_production(actual, (actual as f64 * 0.80) as u32, (actual as f64 * 0.20) as u32, 0)
                .build();
            let result = calculate_oee(input).expect("Machine 2 calc");
            MachineOeeData {
                machine_id: "M002".to_string(),
                machine_name: Some("Press 2".to_string()),
                result,
                sequence_position: Some(2),
                is_bottleneck: false,
            }
        },
        {
            let cycle_time = 25;
            let running_hours = 7;
            let theoretical = (running_hours * 3600 / cycle_time) as u32;
            let actual = (theoretical as f64 * 0.98) as u32;
            
            let input = TestFixture::basic()
                .with_time_allocations(running_hours, 1)
                .with_production(actual, (actual as f64 * 0.97) as u32, (actual as f64 * 0.03) as u32, 0)
                .build();
            let result = calculate_oee(input).expect("Machine 3 calc");
            MachineOeeData {
                machine_id: "M003".to_string(),
                machine_name: Some("Assembly".to_string()),
                result,
                sequence_position: Some(3),
                is_bottleneck: false,
            }
        },
    ];
    
    // Aggregate system
    let system = aggregate_system_oee(machines, AggregationMethod::TimeWeighted);
    
    // Verify system-level results
    assert!(system.system_oee > 0.0);
    assert_eq!(system.machines.len(), 3);
    
    // Should identify M002 as bottleneck (worst OEE)
    assert!(!system.bottleneck_analysis.primary_bottlenecks.is_empty());
    
    let bottleneck = &system.bottleneck_analysis.primary_bottlenecks[0];
    assert_eq!(bottleneck.machine_id, "M002");
}

#[test]
fn test_sensitivity_to_action_workflow() {
    // Workflow: Calculate OEE -> Sensitivity Analysis -> Identify Priority Action
    let input = TestFixture::basic()
        .with_production(1000, 700, 300, 0) // High scrap - quality issue
        .build();
    
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    let sensitivity = quick_sensitivity_analysis(&input, &result.core_metrics);
    
    // Find most sensitive parameter
    let most_sensitive = sensitivity.results.iter()
        .max_by(|a, b| {
            a.oee_delta.abs().partial_cmp(&b.oee_delta.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .expect("Should have most sensitive parameter");
    
    // With high scrap, scrap reduction should be high impact
    assert!(most_sensitive.oee_delta.abs() > 2.0, "Should have significant impact");
    
    // Verify impact is primarily on quality
    assert!(
        most_sensitive.metric_changes.quality_delta.abs() >
        most_sensitive.metric_changes.availability_delta.abs(),
        "Quality should be primary impact area"
    );
}

#[test]
fn test_teep_complete_workflow() {
    // Workflow with TEEP for strategic planning
    // 5-day operation (40 hours planned) in full week
    let input = TestFixture::basic()
        .with_planned_time(40) // 40 hours planned (5 days)
        .with_time_allocations(38, 2) // 38 hours operating, 2 hours downtime
        .with_production(5000, 4800, 200, 0) // Realistic for 38 hours
        .with_teep(168) // Full week (7 days)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Should have both OEE and TEEP
    let oee = result.core_metrics.oee.value * 100.0;
    let teep = result.extended_metrics.teep.expect("TEEP should be calculated").value * 100.0;
    
    // TEEP should be much lower than OEE (low loading factor)
    assert!(
        teep < oee * 0.30,
        "TEEP should reflect low weekly utilization: TEEP={}, OEE={}",
        teep,
        oee
    );
    
    // This identifies strategic opportunity: extend operating hours
}

#[test]
fn test_assumption_ledger_audit_trail() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Verify complete audit trail
    assert!(!result.ledger.assumptions.is_empty());
    assert!(!result.ledger.thresholds.is_empty());
    
    // Check source statistics
    assert!(result.ledger.source_statistics.total_count > 0);
    
    // Should track critical assumptions
    let critical = result.ledger.critical_assumptions();
    assert!(!critical.is_empty(), "Should track critical assumptions");
    
    // All assumptions should have timestamps
    for assumption in &result.ledger.assumptions {
        assert!(
            assumption.timestamp.timestamp() > 0,
            "Should have valid timestamp"
        );
    }
}

#[test]
fn test_validation_integration() {
    // Test validation throughout workflow
    // Use realistic data that triggers warnings but not fatal errors
    let cycle_time = 25;
    let running_hours = 3; // Low utilization
    let theoretical = (running_hours * 3600 / cycle_time) as u32; // 432 units max
    let actual = (theoretical as f64 * 0.95) as u32; // 410 units
    
    let input = TestFixture::basic()
        .with_time_allocations(running_hours, 5) // Low utilization
        .with_production(actual, (actual as f64 * 0.70) as u32, (actual as f64 * 0.30) as u32, 0) // High scrap
        .build();
    
    let result = calculate_oee(input).expect("Should complete with warnings");
    
    // Should have multiple warnings
    assert!(result.validation.has_warnings());
    assert!(!result.validation.has_fatal_errors());
    
    // Warnings should be in ledger
    assert!(
        !result.ledger.warnings.is_empty(),
        "Warnings should be in ledger"
    );
}

#[test]
fn test_complete_reporting_workflow() {
    // Simulate generating reports for different audiences
    let input = TestFixture::basic()
        .with_production(1000, 850, 100, 50)
        .with_time_allocations(7, 1)
        .with_downtime(1800, true)
        .build();
    
    let economic_params = EconomicParameters::from_point_estimates(
        100.0, 40.0, 20.0, 35.0, "USD",
    );
    
    let result = calculate_oee_with_economics(input.clone(), economic_params)
        .expect("Calculation should succeed");
    
    // Engineering report data
    let engineering_data = (
        &result.core_metrics,
        &result.extended_metrics,
        &result.loss_tree,
        &result.ledger,
    );
    assert!(engineering_data.0.oee.value > 0.0);
    
    // Executive report data (simplified)
    let executive_data = (
        result.core_metrics.oee.value * 100.0,
        result.economic_analysis.as_ref().unwrap().total_impact.central_estimate,
    );
    assert!(executive_data.0 > 0.0);
    assert!(executive_data.1 > 0.0);
    
    // Educational data (with explanations)
    assert!(!result.core_metrics.oee.formula_params.is_empty());
}

#[test]
fn test_serialization_complete_result() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Complete result should be serializable
    let json = serde_json::to_string(&result);
    assert!(json.is_ok(), "Complete result should be serializable");
    
    // Should be deserializable
    if let Ok(json_str) = json {
        let deserialized: Result<
            crate::calculus::engineer::calculators::production::oee::OeeResult,
            _
        > = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok(), "Should be deserializable");
    }
}

#[test]
fn test_deterministic_complete_workflow() {
    // Complete workflow should be deterministic
    let input1 = TestFixture::basic()
        .with_production(1000, 900, 100, 0)
        .build();
    let input2 = TestFixture::basic()
        .with_production(1000, 900, 100, 0)
        .build();
    
    let result1 = calculate_oee(input1).expect("Calc 1");
    let result2 = calculate_oee(input2).expect("Calc 2");
    
    // Core metrics should match
    assert_approx_eq(
        result1.core_metrics.oee.value,
        result2.core_metrics.oee.value,
        0.0001,
        "Deterministic OEE"
    );
    
    // Extended metrics should match
    assert_approx_eq(
        result1.extended_metrics.utilization.value,
        result2.extended_metrics.utilization.value,
        0.0001,
        "Deterministic utilization"
    );
}

#[test]
fn test_performance_reasonable_time() {
    use std::time::Instant;
    
    let input = TestFixture::basic().build();
    
    let start = Instant::now();
    let _result = calculate_oee(input).expect("Calculation should succeed");
    let duration = start.elapsed();
    
    // Should complete in reasonable time (<100ms for basic calculation)
    assert!(
        duration.as_millis() < 100,
        "Calculation should be fast: {:?}",
        duration
    );
}

#[test]
fn test_multiple_analyses_workflow() {
    // Run all available analyses
    let input = TestFixture::basic()
        .with_production(1000, 850, 100, 50)
        .with_teep(24)
        .build();
    
    let economic_params = EconomicParameters::from_point_estimates(
        100.0, 40.0, 20.0, 35.0, "USD",
    );
    
    // 1. Core calculation with economics
    let result = calculate_oee_with_economics(input.clone(), economic_params)
        .expect("Base calculation");
    
    // 2. Sensitivity
    let sensitivity = quick_sensitivity_analysis(&input, &result.core_metrics);
    
    // 3. Leverage
    let leverage = calculate_leverage(&input, &result.core_metrics);
    
    // All should complete successfully
    assert!(result.core_metrics.oee.value > 0.0);
    assert_eq!(sensitivity.results.len(), 6);
    assert!(!leverage.is_empty());
    assert!(result.economic_analysis.is_some());
    assert!(result.extended_metrics.teep.is_some());
}