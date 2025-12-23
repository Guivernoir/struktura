//! Multi-machine system-level analysis
//! 
//! Aggregate OEE across multiple machines for line/plant-level insights.
//! Handles different aggregation strategies and identifies system bottlenecks.

use crate::calculus::engineer::calculators::production::oee::{
    domain::{metrics::CoreMetrics, Confidence},
    OeeResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// A single machine's OEE result with context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineOeeData {
    pub machine_id: String,
    pub machine_name: Option<String>,
    pub result: OeeResult,
    /// Machine's role in the line (for bottleneck analysis)
    pub sequence_position: Option<u32>,
    /// Is this machine a bottleneck?
    pub is_bottleneck: bool,
}

/// System-level OEE aggregation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOeeAnalysis {
    /// Overall system OEE (aggregated)
    pub system_oee: f64,
    /// Aggregation method used
    pub aggregation_method: AggregationMethod,
    /// Individual machine results
    pub machines: Vec<MachineOeeData>,
    /// System-level metrics
    pub system_metrics: SystemMetrics,
    /// Bottleneck analysis
    pub bottleneck_analysis: BottleneckAnalysis,
    /// Confidence in system-level results
    pub confidence: Confidence,
}

/// System-level aggregated metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Weighted average availability
    pub avg_availability: f64,
    /// Weighted average performance
    pub avg_performance: f64,
    /// Weighted average quality
    pub avg_quality: f64,
    /// Total planned time across all machines
    pub total_planned_time: Duration,
    /// Total downtime across all machines
    pub total_downtime: Duration,
    /// Total production across all machines
    pub total_production: u32,
    /// Total good units across all machines
    pub total_good_units: u32,
    /// Best performing machine
    pub best_machine_id: String,
    /// Worst performing machine
    pub worst_machine_id: String,
}

/// Bottleneck identification and impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckAnalysis {
    /// Primary bottleneck machine(s)
    pub primary_bottlenecks: Vec<BottleneckInfo>,
    /// System capacity constraint
    pub system_capacity_limit: Option<f64>, // units/hour
    /// Estimated throughput gain if bottleneck eliminated
    pub potential_throughput_gain: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckInfo {
    pub machine_id: String,
    pub oee: f64,
    pub throughput_impact: f64, // % impact on system throughput
    pub recommended_action_key: String,
}

/// Method for aggregating multiple machines into system OEE
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum AggregationMethod {
    /// Simple average (equal weight)
    SimpleAverage,
    /// Weighted by production volume
    ProductionWeighted,
    /// Weighted by planned time
    TimeWeighted,
    /// Minimum OEE (conservative, assumes serial line)
    Minimum,
    /// Multiplicative (assumes perfect serial coupling)
    Multiplicative,
}

/// Aggregate multiple machine results into system-level OEE
pub fn aggregate_system_oee(
    machines: Vec<MachineOeeData>,
    method: AggregationMethod,
) -> SystemOeeAnalysis {
    if machines.is_empty() {
        return create_empty_analysis(method);
    }
    
    // Calculate system OEE based on method
    let system_oee = match method {
        AggregationMethod::SimpleAverage => calculate_simple_average(&machines),
        AggregationMethod::ProductionWeighted => calculate_production_weighted(&machines),
        AggregationMethod::TimeWeighted => calculate_time_weighted(&machines),
        AggregationMethod::Minimum => calculate_minimum(&machines),
        AggregationMethod::Multiplicative => calculate_multiplicative(&machines),
    };
    
    // Calculate system-level metrics
    let system_metrics = calculate_system_metrics(&machines);
    
    // Perform bottleneck analysis
    let bottleneck_analysis = analyze_bottlenecks(&machines, system_oee);
    
    // Determine overall confidence (lowest of all machines)
    let confidence = machines
        .iter()
        .map(|m| m.result.core_metrics.oee.confidence.clone())
        .min()
        .unwrap_or(Confidence::Low);
    
    SystemOeeAnalysis {
        system_oee,
        aggregation_method: method,
        machines,
        system_metrics,
        bottleneck_analysis,
        confidence,
    }
}

/// Calculate simple average OEE
fn calculate_simple_average(machines: &[MachineOeeData]) -> f64 {
    let sum: f64 = machines
        .iter()
        .map(|m| m.result.core_metrics.oee.value)
        .sum();
    
    sum / machines.len() as f64
}

/// Calculate production-weighted average OEE
fn calculate_production_weighted(machines: &[MachineOeeData]) -> f64 {
    let mut weighted_sum = 0.0;
    let mut total_production = 0u32;
    
    for machine in machines {
        let production = machine.result.core_metrics.oee.formula_params
            .get("quality")
            .map(|q| (*q * 1000.0) as u32) // Approximate production from quality metric
            .unwrap_or(0);
        
        weighted_sum += machine.result.core_metrics.oee.value * production as f64;
        total_production += production;
    }
    
    if total_production > 0 {
        weighted_sum / total_production as f64
    } else {
        calculate_simple_average(machines)
    }
}

/// Calculate time-weighted average OEE
fn calculate_time_weighted(machines: &[MachineOeeData]) -> f64 {
    let mut weighted_sum = 0.0;
    let mut total_time = 0.0;
    
    for machine in machines {
        let planned_time = machine.result.core_metrics.availability.formula_params
            .get("planned_time_seconds")
            .copied()
            .unwrap_or(0.0);
        
        weighted_sum += machine.result.core_metrics.oee.value * planned_time;
        total_time += planned_time;
    }
    
    if total_time > 0.0 {
        weighted_sum / total_time
    } else {
        calculate_simple_average(machines)
    }
}

/// Calculate minimum OEE (conservative for serial lines)
fn calculate_minimum(machines: &[MachineOeeData]) -> f64 {
    machines
        .iter()
        .map(|m| m.result.core_metrics.oee.value)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0.0)
}

/// Calculate multiplicative OEE (serial coupling)
fn calculate_multiplicative(machines: &[MachineOeeData]) -> f64 {
    machines
        .iter()
        .map(|m| m.result.core_metrics.oee.value)
        .product()
}

/// Calculate aggregated system metrics
fn calculate_system_metrics(machines: &[MachineOeeData]) -> SystemMetrics {
    let mut total_planned_time = Duration::ZERO;
    let mut total_downtime = Duration::ZERO;
    let mut total_production = 0u32;
    let mut total_good_units = 0u32;
    
    let mut availability_sum = 0.0;
    let mut performance_sum = 0.0;
    let mut quality_sum = 0.0;
    
    let mut best_oee = 0.0;
    let mut worst_oee = 1.0;
    let mut best_machine_id = String::new();
    let mut worst_machine_id = String::new();
    
    for machine in machines {
        let metrics = &machine.result.core_metrics;
        
        // Aggregate time
        let planned = metrics.availability.formula_params
            .get("planned_time_seconds")
            .map(|s| Duration::from_secs_f64(*s))
            .unwrap_or(Duration::ZERO);
        let downtime = metrics.availability.formula_params
            .get("downtime_seconds")
            .map(|s| Duration::from_secs_f64(*s))
            .unwrap_or(Duration::ZERO);
        
        total_planned_time += planned;
        total_downtime += downtime;
        
        // Aggregate production
        let production = metrics.performance.formula_params
            .get("total_count")
            .map(|c| *c as u32)
            .unwrap_or(0);
        let good = metrics.quality.formula_params
            .get("good_count")
            .map(|c| *c as u32)
            .unwrap_or(0);
        
        total_production += production;
        total_good_units += good;
        
        // Sum metrics
        availability_sum += metrics.availability.value;
        performance_sum += metrics.performance.value;
        quality_sum += metrics.quality.value;
        
        // Track best/worst
        let oee = metrics.oee.value;
        if oee > best_oee {
            best_oee = oee;
            best_machine_id = machine.machine_id.clone();
        }
        if oee < worst_oee {
            worst_oee = oee;
            worst_machine_id = machine.machine_id.clone();
        }
    }
    
    let count = machines.len() as f64;
    
    SystemMetrics {
        avg_availability: availability_sum / count,
        avg_performance: performance_sum / count,
        avg_quality: quality_sum / count,
        total_planned_time,
        total_downtime,
        total_production,
        total_good_units,
        best_machine_id,
        worst_machine_id,
    }
}

/// Identify bottlenecks in the system
fn analyze_bottlenecks(machines: &[MachineOeeData], system_oee: f64) -> BottleneckAnalysis {
    let mut bottlenecks = Vec::new();
    
    // Sort machines by OEE (ascending)
    let mut sorted_machines: Vec<_> = machines.iter().collect();
    sorted_machines.sort_by(|a, b| {
        a.result.core_metrics.oee.value
            .partial_cmp(&b.result.core_metrics.oee.value)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Bottom 20% or machines below 70% OEE are potential bottlenecks
    let bottleneck_threshold = 0.70;
    let count_threshold = (machines.len() as f64 * 0.2).ceil() as usize;
    
    for machine in sorted_machines.iter().take(count_threshold) {
        let oee = machine.result.core_metrics.oee.value;
        
        if oee < bottleneck_threshold {
            // Estimate throughput impact
            let throughput_impact = (1.0 - oee) * 100.0;
            
            // Determine recommended action based on worst component
            let metrics = &machine.result.core_metrics;
            let recommended_action = if metrics.availability.value < metrics.performance.value.min(metrics.quality.value) {
                "bottleneck.action.reduce_downtime"
            } else if metrics.performance.value < metrics.quality.value {
                "bottleneck.action.improve_speed"
            } else {
                "bottleneck.action.improve_quality"
            };
            
            bottlenecks.push(BottleneckInfo {
                machine_id: machine.machine_id.clone(),
                oee,
                throughput_impact,
                recommended_action_key: recommended_action.to_string(),
            });
        }
    }
    
    // Calculate potential throughput gain
    let potential_gain = if let Some(worst) = sorted_machines.first() {
        let worst_oee = worst.result.core_metrics.oee.value;
        let best_oee = sorted_machines.last().map(|m| m.result.core_metrics.oee.value).unwrap_or(worst_oee);
        ((best_oee - worst_oee) / worst_oee) * 100.0
    } else {
        0.0
    };
    
    // Calculate system capacity (limited by slowest machine in serial line)
    let system_capacity_limit = calculate_system_capacity(machines);
    
    BottleneckAnalysis {
        primary_bottlenecks: bottlenecks,
        system_capacity_limit,
        potential_throughput_gain: potential_gain,
    }
}

/// Calculate theoretical system capacity (units/hour)
fn calculate_system_capacity(machines: &[MachineOeeData]) -> Option<f64> {
    if machines.is_empty() {
        return None;
    }
    
    // Find machine with lowest theoretical throughput
    let min_capacity = machines
        .iter()
        .filter_map(|m| {
            // Calculate theoretical rate from ideal cycle time
            let ideal_cycle = m.result.core_metrics.performance.formula_params
                .get("ideal_cycle_time_seconds")
                .copied()?;
            
            if ideal_cycle > 0.0 {
                Some(3600.0 / ideal_cycle) // units per hour
            } else {
                None
            }
        })
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
    min_capacity
}

/// Create empty analysis for edge case
fn create_empty_analysis(method: AggregationMethod) -> SystemOeeAnalysis {
    SystemOeeAnalysis {
        system_oee: 0.0,
        aggregation_method: method,
        machines: Vec::new(),
        system_metrics: SystemMetrics {
            avg_availability: 0.0,
            avg_performance: 0.0,
            avg_quality: 0.0,
            total_planned_time: Duration::ZERO,
            total_downtime: Duration::ZERO,
            total_production: 0,
            total_good_units: 0,
            best_machine_id: String::new(),
            worst_machine_id: String::new(),
        },
        bottleneck_analysis: BottleneckAnalysis {
            primary_bottlenecks: Vec::new(),
            system_capacity_limit: None,
            potential_throughput_gain: 0.0,
        },
        confidence: Confidence::Low,
    }
}

/// Compare different aggregation methods side-by-side
pub fn compare_aggregation_methods(machines: Vec<MachineOeeData>) -> HashMap<AggregationMethod, f64> {
    let methods = vec![
        AggregationMethod::SimpleAverage,
        AggregationMethod::ProductionWeighted,
        AggregationMethod::TimeWeighted,
        AggregationMethod::Minimum,
        AggregationMethod::Multiplicative,
    ];
    
    let mut results = HashMap::new();
    
    for method in methods {
        let analysis = aggregate_system_oee(machines.clone(), method.clone());
        results.insert(method, analysis.system_oee);
    }
    
    results
}

/// Quick system analysis with default time-weighted method
pub fn quick_system_analysis(machines: Vec<MachineOeeData>) -> SystemOeeAnalysis {
    aggregate_system_oee(machines, AggregationMethod::TimeWeighted)
}