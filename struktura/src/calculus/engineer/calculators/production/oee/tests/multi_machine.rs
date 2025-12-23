//! Multi-machine system analysis tests
//! 
//! Tests aggregation methods and bottleneck detection

use super::*;
use crate::calculus::engineer::calculators::production::oee::{
    engine::{
        calculate_oee,
        multi_machine::{
            aggregate_system_oee, compare_aggregation_methods, quick_system_analysis,
            AggregationMethod, MachineOeeData,
        },
    },
    OeeResult,
};

fn create_machine_data(
    machine_id: &str,
    availability: f64,
    performance: f64,
    quality: f64,
) -> MachineOeeData {
    // Calculate realistic production based on time and cycle time
    let planned_hours = 8.0;
    let running_hours = planned_hours * availability;
    let downtime_hours = planned_hours - running_hours;
    
    // Ideal cycle time: 25 seconds
    let ideal_cycle_secs = 25.0;
    let running_seconds = running_hours * 3600.0;
    
    // Theoretical max units at ideal cycle time
    let theoretical_max = (running_seconds / ideal_cycle_secs).floor() as u32;
    
    // Actual production considering performance
    let actual_total = (theoretical_max as f64 * performance).floor() as u32;
    
    // Apply quality to get good/scrap split
    let good_units = (actual_total as f64 * quality).floor() as u32;
    let scrap_units = actual_total - good_units;
    
    let input = TestFixture::basic()
        .with_time_allocations(running_hours as u64, downtime_hours as u64)
        .with_production(actual_total, good_units, scrap_units, 0)
        .build();
    
    let result = calculate_oee(input).expect("Should create machine result");
    
    MachineOeeData {
        machine_id: machine_id.to_string(),
        machine_name: Some(format!("Machine {}", machine_id)),
        result,
        sequence_position: None,
        is_bottleneck: false,
    }
}

#[test]
fn test_simple_average_aggregation() {
    let machines = vec![
        create_machine_data("M001", 0.90, 0.95, 0.95), // OEE ~81%
        create_machine_data("M002", 0.80, 0.90, 0.90), // OEE ~65%
        create_machine_data("M003", 0.85, 0.92, 0.93), // OEE ~73%
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::SimpleAverage);
    
    // System OEE should be average of three
    assert_oee_in_range(analysis.system_oee, 0.70, 0.80);
}

#[test]
fn test_minimum_aggregation() {
    let machines = vec![
        create_machine_data("M001", 0.95, 0.95, 0.95), // High OEE
        create_machine_data("M002", 0.60, 0.70, 0.80), // Bottleneck (low OEE)
        create_machine_data("M003", 0.90, 0.92, 0.93), // High OEE
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::Minimum);
    
    // System OEE should be minimum (M002)
    assert!(
        analysis.system_oee < 0.40,
        "System OEE should be limited by worst machine"
    );
}

#[test]
fn test_multiplicative_aggregation() {
    let machines = vec![
        create_machine_data("M001", 0.90, 0.90, 0.90), // OEE = 0.729
        create_machine_data("M002", 0.90, 0.90, 0.90), // OEE = 0.729
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::Multiplicative);
    
    // System OEE = 0.729 * 0.729 = 0.531
    assert_approx_eq(
        analysis.system_oee,
        0.729 * 0.729,
        0.05,
        "Multiplicative aggregation"
    );
}

#[test]
fn test_bottleneck_detection() {
    let machines = vec![
        create_machine_data("M001", 0.95, 0.95, 0.95), // Good
        create_machine_data("M002", 0.50, 0.60, 0.70), // Bottleneck
        create_machine_data("M003", 0.92, 0.93, 0.94), // Good
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::TimeWeighted);
    
    // Should identify M002 as bottleneck
    assert!(
        !analysis.bottleneck_analysis.primary_bottlenecks.is_empty(),
        "Should identify bottlenecks"
    );
    
    let bottleneck = &analysis.bottleneck_analysis.primary_bottlenecks[0];
    assert_eq!(bottleneck.machine_id, "M002");
}

#[test]
fn test_system_metrics_aggregation() {
    let machines = vec![
        create_machine_data("M001", 0.90, 0.90, 0.90),
        create_machine_data("M002", 0.80, 0.85, 0.95),
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::SimpleAverage);
    
    // Check system-level metrics
    assert!(analysis.system_metrics.avg_availability > 0.0);
    assert!(analysis.system_metrics.avg_performance > 0.0);
    assert!(analysis.system_metrics.avg_quality > 0.0);
    assert!(analysis.system_metrics.total_production > 0);
    assert!(!analysis.system_metrics.best_machine_id.is_empty());
    assert!(!analysis.system_metrics.worst_machine_id.is_empty());
}

#[test]
fn test_best_worst_machine_identification() {
    let machines = vec![
        create_machine_data("M001", 0.95, 0.95, 0.95), // Best
        create_machine_data("M002", 0.60, 0.70, 0.80), // Worst
        create_machine_data("M003", 0.85, 0.88, 0.90), // Middle
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::SimpleAverage);
    
    assert_eq!(analysis.system_metrics.best_machine_id, "M001");
    assert_eq!(analysis.system_metrics.worst_machine_id, "M002");
}

#[test]
fn test_single_machine_system() {
    let machines = vec![create_machine_data("M001", 0.85, 0.90, 0.95)];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::SimpleAverage);
    
    // System OEE should equal single machine OEE
    let expected_oee = 0.85 * 0.90 * 0.95;
    assert_approx_eq(analysis.system_oee, expected_oee, 0.05, "Single machine system");
}

#[test]
fn test_empty_machine_list() {
    let machines = vec![];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::SimpleAverage);
    
    // Should handle gracefully
    assert_eq!(analysis.system_oee, 0.0);
    assert!(analysis.machines.is_empty());
}

#[test]
fn test_compare_aggregation_methods() {
    let machines = vec![
        create_machine_data("M001", 0.90, 0.90, 0.90),
        create_machine_data("M002", 0.80, 0.85, 0.95),
        create_machine_data("M003", 0.85, 0.88, 0.92),
    ];
    
    let comparison = compare_aggregation_methods(machines);
    
    // Should have results for all 5 methods
    assert_eq!(comparison.len(), 5);
    
    // Verify expected ordering (generally)
    let simple = comparison.get(&AggregationMethod::SimpleAverage).unwrap();
    let minimum = comparison.get(&AggregationMethod::Minimum).unwrap();
    let multiplicative = comparison.get(&AggregationMethod::Multiplicative).unwrap();
    
    // Minimum should be lowest
    assert!(
        minimum < simple,
        "Minimum should be most conservative"
    );
    
    // Multiplicative should be lower than simple for serial coupling
    assert!(
        multiplicative < simple,
        "Multiplicative accounts for serial coupling"
    );
}

#[test]
fn test_bottleneck_recommended_action() {
    let machines = vec![
        create_machine_data("M001", 0.50, 0.90, 0.95), // Availability bottleneck
        create_machine_data("M002", 0.90, 0.50, 0.95), // Performance bottleneck
        create_machine_data("M003", 0.90, 0.90, 0.50), // Quality bottleneck
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::Minimum);
    
    // Should identify appropriate actions for each bottleneck
    for bottleneck in &analysis.bottleneck_analysis.primary_bottlenecks {
        assert!(!bottleneck.recommended_action_key.is_empty());
    }
}

#[test]
fn test_system_capacity_calculation() {
    let machines = vec![
        create_machine_data("M001", 0.90, 0.90, 0.90),
        create_machine_data("M002", 0.85, 0.85, 0.85),
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::Minimum);
    
    // Should calculate system capacity
    if let Some(capacity) = analysis.bottleneck_analysis.system_capacity_limit {
        assert!(capacity > 0.0, "System capacity should be positive");
    }
}

#[test]
fn test_throughput_gain_calculation() {
    let machines = vec![
        create_machine_data("M001", 0.95, 0.95, 0.95), // Good
        create_machine_data("M002", 0.60, 0.70, 0.80), // Bottleneck
        create_machine_data("M003", 0.92, 0.93, 0.94), // Good
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::TimeWeighted);
    
    // Should calculate potential throughput gain
    assert!(
        analysis.bottleneck_analysis.potential_throughput_gain > 0.0,
        "Should identify improvement opportunity"
    );
}

#[test]
fn test_quick_system_analysis() {
    let machines = vec![
        create_machine_data("M001", 0.90, 0.90, 0.90),
        create_machine_data("M002", 0.85, 0.87, 0.92),
    ];
    
    // Quick analysis should use TimeWeighted (default)
    let analysis = quick_system_analysis(machines);
    
    assert_eq!(analysis.aggregation_method, AggregationMethod::TimeWeighted);
}

#[test]
fn test_confidence_tracking_system_level() {
    let machines = vec![
        create_machine_data("M001", 0.90, 0.90, 0.90),
        create_machine_data("M002", 0.85, 0.87, 0.92),
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::SimpleAverage);
    
    // System should track confidence (lowest of all machines)
    // With test fixtures using explicit values, should be High
    assert_eq!(
        analysis.confidence,
        crate::calculus::engineer::calculators::production::oee::domain::Confidence::High
    );
}

#[test]
fn test_machine_sequence_positions() {
    let mut machines = vec![
        create_machine_data("M001", 0.90, 0.90, 0.90),
        create_machine_data("M002", 0.85, 0.87, 0.92),
        create_machine_data("M003", 0.88, 0.89, 0.91),
    ];
    
    // Set sequence positions
    machines[0].sequence_position = Some(1);
    machines[1].sequence_position = Some(2);
    machines[2].sequence_position = Some(3);
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::Multiplicative);
    
    // Should preserve sequence information
    assert!(analysis.machines.iter().all(|m| m.sequence_position.is_some()));
}

#[test]
fn test_aggregation_deterministic() {
    let machines1 = vec![
        create_machine_data("M001", 0.90, 0.90, 0.90),
        create_machine_data("M002", 0.85, 0.87, 0.92),
    ];
    
    let machines2 = vec![
        create_machine_data("M001", 0.90, 0.90, 0.90),
        create_machine_data("M002", 0.85, 0.87, 0.92),
    ];
    
    let analysis1 = aggregate_system_oee(machines1, AggregationMethod::SimpleAverage);
    let analysis2 = aggregate_system_oee(machines2, AggregationMethod::SimpleAverage);
    
    // Should produce identical results
    assert_approx_eq(
        analysis1.system_oee,
        analysis2.system_oee,
        0.0001,
        "Deterministic aggregation"
    );
}

#[test]
fn test_all_perfect_machines() {
    let machines = vec![
        create_machine_data("M001", 1.0, 1.0, 1.0),
        create_machine_data("M002", 1.0, 1.0, 1.0),
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::SimpleAverage);
    
    // System OEE should be very high (>85%)
    assert!(
        analysis.system_oee > 0.85,
        "Perfect system should have high OEE: got {}",
        analysis.system_oee
    );
    
    // No bottlenecks
    assert!(
        analysis.bottleneck_analysis.primary_bottlenecks.is_empty(),
        "No bottlenecks in perfect system"
    );
}

#[test]
fn test_all_failed_machines() {
    let machines = vec![
        create_machine_data("M001", 0.0, 0.50, 0.50),
        create_machine_data("M002", 0.0, 0.50, 0.50),
    ];
    
    let analysis = aggregate_system_oee(machines, AggregationMethod::SimpleAverage);
    
    // System OEE should be 0%
    assert_approx_eq(analysis.system_oee, 0.0, 0.01, "Failed system");
}