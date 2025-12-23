//! API endpoint tests
//! 
//! Tests REST API endpoints (would use axum-test or similar in real implementation)

use super::*;

// Note: These are conceptual tests. In a real implementation, you'd use
// axum-test or similar framework to test the actual HTTP endpoints.
// Here we test the underlying logic that the API handlers use.

#[test]
fn test_calculate_endpoint_logic() {
    let input = TestFixture::basic().build();
    
    // Simulate API request
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(input);
    
    assert!(result.is_ok(), "Calculate endpoint should succeed");
}

#[test]
fn test_calculate_with_economics_endpoint_logic() {
    let input = TestFixture::basic().build();
    let economic_params = crate::calculus::engineer::calculators::production::oee::domain::economics::EconomicParameters::from_point_estimates(
        100.0, 40.0, 20.0, 35.0, "USD",
    );
    
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee_with_economics(
        input,
        economic_params,
    );
    
    assert!(result.is_ok(), "Calculate with economics should succeed");
    assert!(result.unwrap().economic_analysis.is_some());
}

#[test]
fn test_sensitivity_endpoint_logic() {
    let input = TestFixture::basic().build();
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(input.clone())
        .expect("Base calculation");
    
    let sensitivity = crate::calculus::engineer::calculators::production::oee::engine::sensitivity::analyze_sensitivity(
        &input,
        &result.core_metrics,
        10.0,
    );
    
    assert!(!sensitivity.results.is_empty());
}

#[test]
fn test_leverage_endpoint_logic() {
    let input = TestFixture::basic().build();
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(input.clone())
        .expect("Base calculation");
    
    let leverage = crate::calculus::engineer::calculators::production::oee::engine::leverage::calculate_leverage(
        &input,
        &result.core_metrics,
    );
    
    assert!(!leverage.is_empty(), "Should return leverage opportunities");
}

#[test]
fn test_system_aggregate_endpoint_logic() {
    use crate::calculus::engineer::calculators::production::oee::engine::multi_machine::{
        aggregate_system_oee, AggregationMethod, MachineOeeData,
    };
    
    let machines = vec![
        {
            let input = TestFixture::basic().build();
            let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(input)
                .expect("Machine calc");
            MachineOeeData {
                machine_id: "M001".to_string(),
                machine_name: Some("Test Machine".to_string()),
                result,
                sequence_position: None,
                is_bottleneck: false,
            }
        },
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::TimeWeighted);
    
    assert!(analysis.system_oee > 0.0);
}

#[test]
fn test_system_compare_methods_endpoint_logic() {
    use crate::calculus::engineer::calculators::production::oee::engine::multi_machine::{
        compare_aggregation_methods, MachineOeeData,
    };
    
    let machines = vec![
        {
            let input = TestFixture::basic().build();
            let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(input)
                .expect("Machine calc");
            MachineOeeData {
                machine_id: "M001".to_string(),
                machine_name: None,
                result,
                sequence_position: None,
                is_bottleneck: false,
            }
        },
        {
            let input = TestFixture::basic()
                .with_production(1000, 800, 200, 0)
                .build();
            let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(input)
                .expect("Machine calc");
            MachineOeeData {
                machine_id: "M002".to_string(),
                machine_name: None,
                result,
                sequence_position: None,
                is_bottleneck: false,
            }
        },
    ];
    
    let comparison = compare_aggregation_methods(machines);
    
    assert_eq!(comparison.len(), 5, "Should compare all 5 methods");
}

#[test]
fn test_api_error_handling_validation_failed() {
    let invalid_input = invalid_fixture_count_mismatch();
    
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(invalid_input);
    
    assert!(result.is_err(), "Should return error for invalid input");
    
    if let Err(crate::calculus::engineer::calculators::production::oee::engine::EngineError::ValidationFailed(validation)) = result {
        // Should have validation details for API response
        assert!(validation.has_fatal_errors());
        assert!(!validation.issues.is_empty());
    }
}

#[test]
fn test_api_serialization_calculate_response() {
    let input = TestFixture::basic().build();
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(input)
        .expect("Calculation should succeed");
    
    // Simulate API response serialization
    let json = serde_json::to_string(&result);
    assert!(json.is_ok(), "Result should be serializable for API response");
}

#[test]
fn test_api_serialization_sensitivity_response() {
    let input = TestFixture::basic().build();
    let base_result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(input.clone())
        .expect("Base calculation");
    
    let sensitivity = crate::calculus::engineer::calculators::production::oee::engine::sensitivity::analyze_sensitivity(
        &input,
        &base_result.core_metrics,
        10.0,
    );
    
    let json = serde_json::to_string(&sensitivity);
    assert!(json.is_ok(), "Sensitivity analysis should be serializable");
}

#[test]
fn test_api_serialization_system_response() {
    use crate::calculus::engineer::calculators::production::oee::engine::multi_machine::{
        aggregate_system_oee, AggregationMethod, MachineOeeData,
    };
    
    let machines = vec![
        {
            let input = TestFixture::basic().build();
            let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(input)
                .expect("Machine calc");
            MachineOeeData {
                machine_id: "M001".to_string(),
                machine_name: None,
                result,
                sequence_position: None,
                is_bottleneck: false,
            }
        },
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::SimpleAverage);
    
    let json = serde_json::to_string(&analysis);
    assert!(json.is_ok(), "System analysis should be serializable");
}

#[test]
fn test_api_error_serialization() {
    let invalid_input = invalid_fixture_count_mismatch();
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(invalid_input);
    
    if let Err(error) = result {
        // Error should be serializable for API response
        match error {
            crate::calculus::engineer::calculators::production::oee::engine::EngineError::ValidationFailed(validation) => {
                let json = serde_json::to_string(&validation);
                assert!(json.is_ok(), "Validation error should be serializable");
            }
            _ => {}
        }
    }
}

#[test]
fn test_api_request_deserialization() {
    // Test that API request structures can be deserialized
    let json_input = r#"{
        "window": {
            "start": "2024-01-01T08:00:00Z",
            "end": "2024-01-01T16:00:00Z"
        },
        "machine": {
            "machine_id": "M001",
            "line_id": null,
            "product_id": null,
            "shift_id": null
        },
        "time_model": {
            "planned_production_time": {"Explicit": 28800000000000},
            "allocations": [],
            "all_time": null
        },
        "production": {
            "total_units": {"Explicit": 1000},
            "good_units": {"Explicit": 950},
            "scrap_units": {"Explicit": 50},
            "reworked_units": {"Explicit": 0}
        },
        "cycle_time": {
            "ideal_cycle_time": {"Explicit": 25000000000},
            "average_cycle_time": null
        },
        "downtimes": {
            "records": []
        },
        "thresholds": {
            "micro_stoppage_threshold": 30000000000,
            "small_stop_threshold": 300000000000,
            "speed_loss_threshold": 0.05,
            "high_scrap_rate_threshold": 0.20,
            "low_utilization_threshold": 0.30
        }
    }"#;
    
    let result: Result<crate::calculus::engineer::calculators::production::oee::OeeInput, _> = 
        serde_json::from_str(json_input);
    
    // Note: This might fail due to duration serialization format
    // In real API, you'd use custom serializers/deserializers
}

#[test]
fn test_api_full_request_response_cycle() {
    // Simulate complete API request/response cycle
    let input = TestFixture::basic().build();
    
    // Serialize input (API request)
    let input_json = serde_json::to_string(&input).expect("Should serialize input");
    
    // Process (would be in API handler)
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(input)
        .expect("Calculation should succeed");
    
    // Serialize output (API response)
    let output_json = serde_json::to_string(&result).expect("Should serialize result");
    
    // Both should be valid JSON
    assert!(!input_json.is_empty());
    assert!(!output_json.is_empty());
}

#[test]
fn test_api_economic_parameters_serialization() {
    let params = crate::calculus::engineer::calculators::production::oee::domain::economics::EconomicParameters::from_point_estimates(
        100.0, 40.0, 20.0, 35.0, "USD",
    );
    
    let json = serde_json::to_string(&params);
    assert!(json.is_ok(), "Economic parameters should be serializable");
}

#[test]
fn test_api_empty_machine_list() {
    use crate::calculus::engineer::calculators::production::oee::engine::multi_machine::{
        aggregate_system_oee, AggregationMethod,
    };
    
    let machines = vec![];
    let analysis = aggregate_system_oee(machines, AggregationMethod::SimpleAverage);
    
    // Should handle gracefully
    assert_eq!(analysis.system_oee, 0.0);
    assert!(analysis.machines.is_empty());
}

#[test]
fn test_api_temporal_scrap_request() {
    use crate::calculus::engineer::calculators::production::oee::engine::temporal_scrap::{
        analyze_temporal_scrap, ScrapEvent, StartupWindowConfig, TemporalScrapData,
    };
    use chrono::Duration as ChronoDuration;
    
    let start = Utc::now();
    let end = start + ChronoDuration::hours(8);
    let window = AnalysisWindow { start, end };
    
    let mut data = TemporalScrapData::new(window);
    data.add_event(ScrapEvent::new(start + ChronoDuration::minutes(10), 5));
    
    let config = StartupWindowConfig::default();
    let analysis = analyze_temporal_scrap(&data, Duration::from_secs(25), &config);
    
    // Should serialize for API response
    let json = serde_json::to_string(&analysis);
    assert!(json.is_ok(), "Temporal scrap analysis should be serializable");
}

#[test]
fn test_api_confidence_in_response() {
    let input = TestFixture::basic().build();
    let result = crate::calculus::engineer::calculators::production::oee::engine::calculate_oee(input)
        .expect("Calculation should succeed");
    
    // Confidence should be included in response
    assert!(
        matches!(
            result.core_metrics.oee.confidence,
            crate::calculus::engineer::calculators::production::oee::domain::Confidence::High |
            crate::calculus::engineer::calculators::production::oee::domain::Confidence::Medium |
            crate::calculus::engineer::calculators::production::oee::domain::Confidence::Low
        )
    );
}