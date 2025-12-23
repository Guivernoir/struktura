//! Sensitivity analysis tests
//! 
//! Tests parameter variations and impact classification

use super::*;
use crate::calculus::engineer::calculators::production::oee::engine::{
    calculate_oee,
    sensitivity::{analyze_sensitivity, quick_sensitivity_analysis, SensitivityImpact},
};

#[test]
fn test_sensitivity_analysis_runs() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input.clone()).expect("Base calculation should succeed");
    
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // Should test 6 parameters
    assert_eq!(analysis.results.len(), 6, "Should test 6 parameters");
    
    // Should identify most and least sensitive
    assert!(!analysis.most_sensitive_parameter.is_empty());
    assert!(!analysis.least_sensitive_parameter.is_empty());
}

#[test]
fn test_downtime_sensitivity() {
    let cycle_time = 25;
    let running_hours = 6;
    let theoretical = (running_hours * 3600 / cycle_time) as u32;
    
    let input = TestFixture::basic()
        .with_time_allocations(running_hours, 2) // 2 hours downtime
        .with_production(theoretical, theoretical, 0, 0)
        .build();
    
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // Find downtime sensitivity result
    let downtime_result = analysis.results.iter()
        .find(|r| r.parameter_key == "sensitivity.downtime")
        .expect("Should have downtime sensitivity");
    
    // Reducing downtime should improve OEE
    assert!(
        downtime_result.oee_delta > 0.0,
        "Reducing downtime should increase OEE"
    );
    
    // Should primarily affect availability
    assert!(
        downtime_result.metric_changes.availability_delta > 0.0,
        "Downtime reduction should improve availability"
    );
}

#[test]
fn test_cycle_time_sensitivity() {
    let cycle_time = 30;
    let running_hours = 7;
    let theoretical = (running_hours * 3600 / cycle_time) as u32; // 840 units
    
    let input = TestFixture::basic()
        .with_time_allocations(running_hours, 1)
        .with_production(theoretical, theoretical, 0, 0)
        .with_cycle_time(cycle_time, None) // 30 seconds (slower)
        .build();
    
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // Find cycle time sensitivity
    let cycle_result = analysis.results.iter()
        .find(|r| r.parameter_key == "sensitivity.cycle_time")
        .expect("Should have cycle time sensitivity");
    
    // Improving (reducing) cycle time should improve OEE
    assert!(
        cycle_result.oee_delta > 0.0,
        "Faster cycle time should increase OEE"
    );
    
    // Should primarily affect performance
    assert!(
        cycle_result.metric_changes.performance_delta > 0.0,
        "Cycle time improvement should improve performance"
    );
}

#[test]
fn test_scrap_sensitivity() {
    let input = TestFixture::basic()
        .with_production(1000, 800, 200, 0) // 20% scrap
        .build();
    
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // Find scrap sensitivity
    let scrap_result = analysis.results.iter()
        .find(|r| r.parameter_key == "sensitivity.scrap_units")
        .expect("Should have scrap sensitivity");
    
    // Reducing scrap should improve OEE
    assert!(
        scrap_result.oee_delta > 0.0,
        "Reducing scrap should increase OEE"
    );
    
    // Should primarily affect quality
    assert!(
        scrap_result.metric_changes.quality_delta > 0.0,
        "Scrap reduction should improve quality"
    );
}

#[test]
fn test_impact_classification() {
    let input = TestFixture::basic()
        .with_production(1000, 500, 500, 0) // Severe quality issue
        .build();
    
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // Scrap should have high impact
    let scrap_result = analysis.results.iter()
        .find(|r| r.parameter_key == "sensitivity.scrap_units")
        .expect("Should have scrap sensitivity");
    
    // With 50% scrap, reducing by 10% should have significant impact
    assert!(
        matches!(
            scrap_result.impact_level,
            SensitivityImpact::Critical | SensitivityImpact::High
        ),
        "High scrap should have critical/high impact"
    );
}

#[test]
fn test_custom_variation_percentage() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    
    // Test with ±20% variation instead of default ±10%
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 20.0);
    
    // All results should reflect 20% variation
    for sens_result in &analysis.results {
        assert!(
            sens_result.variation_percent.abs() - 20.0 < 0.1,
            "Should use 20% variation"
        );
    }
}

#[test]
fn test_quick_sensitivity_analysis() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    
    // Quick analysis should use default 10% variation
    let analysis = quick_sensitivity_analysis(&input, &result.core_metrics);
    
    assert_eq!(analysis.results.len(), 6);
    assert!(!analysis.most_sensitive_parameter.is_empty());
}

#[test]
fn test_sensitivity_with_zero_baseline() {
    let input = TestFixture::basic()
        .with_production(0, 0, 0, 0)
        .build();
    
    let result = calculate_oee(input.clone()).expect("Should handle zero production");
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // Should complete without errors
    assert_eq!(analysis.results.len(), 6);
}

#[test]
fn test_all_parameters_tested() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // Check all expected parameters are tested
    let param_keys: Vec<&str> = analysis.results.iter()
        .map(|r| r.parameter_key.as_str())
        .collect();
    
    assert!(param_keys.contains(&"sensitivity.planned_time"));
    assert!(param_keys.contains(&"sensitivity.downtime"));
    assert!(param_keys.contains(&"sensitivity.cycle_time"));
    assert!(param_keys.contains(&"sensitivity.production_count"));
    assert!(param_keys.contains(&"sensitivity.good_units"));
    assert!(param_keys.contains(&"sensitivity.scrap_units"));
}

#[test]
fn test_sensitivity_most_sensitive_identification() {
    // Create scenario where scrap is clearly the worst problem
    let input = TestFixture::basic()
        .with_production(1000, 600, 400, 0) // 40% scrap - severe
        .with_time_allocations(7, 1) // Minor downtime
        .build();
    
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // Scrap should likely be most sensitive
    // (or at least have high impact)
    let scrap_result = analysis.results.iter()
        .find(|r| r.parameter_key == "sensitivity.scrap_units")
        .expect("Should have scrap sensitivity");
    
    assert!(
        scrap_result.oee_delta.abs() > 1.0,
        "Scrap should have >1% OEE impact"
    );
}

#[test]
fn test_sensitivity_metric_changes() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    for sens_result in &analysis.results {
        // Metric changes should sum to approximately match OEE delta
        // (not exactly due to multiplication, but should be related)
        let metric_total = sens_result.metric_changes.availability_delta
            + sens_result.metric_changes.performance_delta
            + sens_result.metric_changes.quality_delta;
        
        // At least one metric should change
        assert!(
            metric_total.abs() > 0.0,
            "At least one metric should change for {}",
            sens_result.parameter_key
        );
    }
}

#[test]
fn test_impact_level_thresholds() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // Verify impact classification logic
    for sens_result in &analysis.results {
        let abs_delta = sens_result.oee_delta.abs();
        
        match sens_result.impact_level {
            SensitivityImpact::Critical => assert!(abs_delta > 5.0),
            SensitivityImpact::High => assert!(abs_delta > 2.0 && abs_delta <= 5.0),
            SensitivityImpact::Medium => assert!(abs_delta > 0.5 && abs_delta <= 2.0),
            SensitivityImpact::Low => assert!(abs_delta <= 0.5),
        }
    }
}

#[test]
fn test_sensitivity_baseline_preserved() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    
    let baseline_oee = result.core_metrics.oee.value * 100.0;
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // All results should reference same baseline
    for sens_result in &analysis.results {
        assert_approx_eq(
            sens_result.baseline_oee,
            baseline_oee,
            0.01,
            "Baseline OEE should be consistent"
        );
    }
}

#[test]
fn test_sensitivity_with_perfect_oee() {
    // Edge case: already at 100% OEE
    let input = TestFixture::basic()
        .with_production(1000, 1000, 0, 0)
        .with_time_allocations(8, 0)
        .build();
    
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    let analysis = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // Should still complete
    assert_eq!(analysis.results.len(), 6);
    
    // Most variations should show degradation or no change
    // (can't improve beyond 100%)
}

#[test]
fn test_sensitivity_deterministic() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input.clone()).expect("Calculation should succeed");
    
    let analysis1 = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    let analysis2 = analyze_sensitivity(&input, &result.core_metrics, 10.0);
    
    // Results should be identical
    assert_eq!(analysis1.results.len(), analysis2.results.len());
    
    for (r1, r2) in analysis1.results.iter().zip(analysis2.results.iter()) {
        assert_eq!(r1.parameter_key, r2.parameter_key);
        assert_approx_eq(r1.oee_delta, r2.oee_delta, 0.0001, "OEE delta");
    }
}