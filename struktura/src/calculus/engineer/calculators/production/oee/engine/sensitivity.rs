//! Sensitivity analysis and what-if scenarios
//! 
//! Test how results change with input variations.
//! Helps identify which inputs have the most leverage on OEE.

use crate::calculus::engineer::calculators::production::oee::{
    domain::metrics::CoreMetrics,
    engine::oee::calculate_core_metrics_from_input,
    OeeInput,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Sensitivity analysis result for a single parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityResult {
    /// Parameter varied (translation key)
    pub parameter_key: String,
    /// Baseline value (original)
    pub baseline_value: f64,
    /// Variation tested (±%)
    pub variation_percent: f64,
    /// New value after variation
    pub varied_value: f64,
    /// Baseline OEE (%)
    pub baseline_oee: f64,
    /// New OEE after variation (%)
    pub varied_oee: f64,
    /// Impact on OEE (absolute percentage points)
    pub oee_delta: f64,
    /// Impact classification
    pub impact_level: SensitivityImpact,
    /// Detailed metric changes
    pub metric_changes: MetricChanges,
}

/// Classification of sensitivity impact
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SensitivityImpact {
    Critical,  // >5% OEE swing
    High,      // 2-5% OEE swing
    Medium,    // 0.5-2% OEE swing
    Low,       // <0.5% OEE swing
}

/// Detailed changes in all metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricChanges {
    pub availability_delta: f64,
    pub performance_delta: f64,
    pub quality_delta: f64,
}

/// Complete sensitivity analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityAnalysis {
    pub baseline_oee: f64,
    pub results: Vec<SensitivityResult>,
    pub most_sensitive_parameter: String,
    pub least_sensitive_parameter: String,
}

/// Run comprehensive sensitivity analysis on key parameters
pub fn analyze_sensitivity(
    input: &OeeInput,
    baseline_metrics: &CoreMetrics,
    variation_percent: f64,
) -> SensitivityAnalysis {
    let mut results = Vec::new();
    let baseline_oee = baseline_metrics.oee.value * 100.0;
    
    // Test planned time sensitivity
    results.push(analyze_planned_time_sensitivity(
        input,
        baseline_metrics,
        variation_percent,
    ));
    
    // Test downtime sensitivity
    results.push(analyze_downtime_sensitivity(
        input,
        baseline_metrics,
        variation_percent,
    ));
    
    // Test cycle time sensitivity
    results.push(analyze_cycle_time_sensitivity(
        input,
        baseline_metrics,
        variation_percent,
    ));
    
    // Test production count sensitivity
    results.push(analyze_production_count_sensitivity(
        input,
        baseline_metrics,
        variation_percent,
    ));
    
    // Test good units sensitivity
    results.push(analyze_good_units_sensitivity(
        input,
        baseline_metrics,
        variation_percent,
    ));
    
    // Test scrap sensitivity
    results.push(analyze_scrap_sensitivity(
        input,
        baseline_metrics,
        variation_percent,
    ));
    
    // Find most and least sensitive parameters
    let most_sensitive = results
        .iter()
        .max_by(|a, b| {
            a.oee_delta.abs().partial_cmp(&b.oee_delta.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|r| r.parameter_key.clone())
        .unwrap_or_else(|| "unknown".to_string());
    
    let least_sensitive = results
        .iter()
        .min_by(|a, b| {
            a.oee_delta.abs().partial_cmp(&b.oee_delta.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|r| r.parameter_key.clone())
        .unwrap_or_else(|| "unknown".to_string());
    
    SensitivityAnalysis {
        baseline_oee,
        results,
        most_sensitive_parameter: most_sensitive,
        least_sensitive_parameter: least_sensitive,
    }
}

/// Analyze planned time sensitivity (+10% increase)
fn analyze_planned_time_sensitivity(
    input: &OeeInput,
    baseline_metrics: &CoreMetrics,
    variation_percent: f64,
) -> SensitivityResult {
    let baseline_value = input.time_model.planned_production_time.value().as_secs_f64();
    let varied_value = baseline_value * (1.0 + variation_percent / 100.0);
    
    // Create modified input
    let mut modified_input = input.clone();
    let new_duration = Duration::from_secs_f64(varied_value);
    modified_input.time_model.planned_production_time = 
        crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Explicit(new_duration);
    
    // Recalculate metrics
    let modified_metrics = calculate_core_metrics_from_input(
        &modified_input,
        baseline_metrics.oee.confidence.clone(),
    );
    
    let baseline_oee = baseline_metrics.oee.value * 100.0;
    let varied_oee = modified_metrics.oee.value * 100.0;
    let oee_delta = varied_oee - baseline_oee;
    
    SensitivityResult {
        parameter_key: "sensitivity.planned_time".to_string(),
        baseline_value,
        variation_percent,
        varied_value,
        baseline_oee,
        varied_oee,
        oee_delta,
        impact_level: classify_impact(oee_delta),
        metric_changes: MetricChanges {
            availability_delta: (modified_metrics.availability.value - baseline_metrics.availability.value) * 100.0,
            performance_delta: (modified_metrics.performance.value - baseline_metrics.performance.value) * 100.0,
            quality_delta: (modified_metrics.quality.value - baseline_metrics.quality.value) * 100.0,
        },
    }
}

/// Analyze downtime sensitivity (-10% reduction)
/// FIXED: Properly transfers saved downtime back to running time
fn analyze_downtime_sensitivity(
    input: &OeeInput,
    baseline_metrics: &CoreMetrics,
    variation_percent: f64,
) -> SensitivityResult {
    let baseline_value = input.time_model.total_downtime().as_secs_f64();
    // For downtime, we want to see impact of REDUCING it
    let varied_value = baseline_value * (1.0 - variation_percent / 100.0);
    
    let mut modified_input = input.clone();
    
    // Calculate time saved by reducing downtime
    let scale_factor = if baseline_value > 0.0 {
        varied_value / baseline_value
    } else {
        1.0
    };
    
    let mut running_time_addition = Duration::ZERO;
    
    // Scale all downtime allocations proportionally and accumulate saved time
    for allocation in &mut modified_input.time_model.allocations {
        if allocation.state != crate::calculus::engineer::calculators::production::oee::assumptions::MachineState::Running {
            let current = *allocation.duration.value();
            let new_duration = Duration::from_secs_f64(current.as_secs_f64() * scale_factor);
            let saved = current.saturating_sub(new_duration);
            running_time_addition += saved;
            allocation.duration = crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Inferred(new_duration);
        }
    }
    
    // Add saved time to running time (critical fix)
    for allocation in &mut modified_input.time_model.allocations {
        if allocation.state == crate::calculus::engineer::calculators::production::oee::assumptions::MachineState::Running {
            let current = *allocation.duration.value();
            let new_duration = current + running_time_addition;
            allocation.duration = crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Inferred(new_duration);
            break;
        }
    }
    
    let modified_metrics = calculate_core_metrics_from_input(
        &modified_input,
        baseline_metrics.oee.confidence.clone(),
    );
    
    let baseline_oee = baseline_metrics.oee.value * 100.0;
    let varied_oee = modified_metrics.oee.value * 100.0;
    let oee_delta = varied_oee - baseline_oee;
    
    SensitivityResult {
        parameter_key: "sensitivity.downtime".to_string(),
        baseline_value,
        variation_percent: -variation_percent, // Negative because we're reducing
        varied_value,
        baseline_oee,
        varied_oee,
        oee_delta,
        impact_level: classify_impact(oee_delta),
        metric_changes: MetricChanges {
            availability_delta: (modified_metrics.availability.value - baseline_metrics.availability.value) * 100.0,
            performance_delta: (modified_metrics.performance.value - baseline_metrics.performance.value) * 100.0,
            quality_delta: (modified_metrics.quality.value - baseline_metrics.quality.value) * 100.0,
        },
    }
}

/// Analyze cycle time sensitivity (-10% improvement)
/// FIXED: Recalculates theoretical production at improved cycle time
fn analyze_cycle_time_sensitivity(
    input: &OeeInput,
    baseline_metrics: &CoreMetrics,
    variation_percent: f64,
) -> SensitivityResult {
    let baseline_value = input.cycle_time.ideal_cycle_time.value().as_secs_f64();
    // For cycle time, we want to see impact of IMPROVING it (reducing)
    let varied_value = baseline_value * (1.0 - variation_percent / 100.0);
    
    let mut modified_input = input.clone();
    let new_duration = Duration::from_secs_f64(varied_value);
    modified_input.cycle_time.ideal_cycle_time = 
        crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Explicit(new_duration);
    
    // Critical fix: With faster cycle time, theoretical capacity increases
    // Recalculate what production would be at the improved cycle time
    let running_time = modified_input.time_model.running_time();
    let new_theoretical_max = if varied_value > 0.0 {
        (running_time.as_secs_f64() / varied_value).floor() as u32
    } else {
        0
    };
    
    // Scale production proportionally (maintaining same performance and quality rates)
    let old_total = *input.production.total_units.value();
    let production_ratio = if old_total > 0 {
        new_theoretical_max as f64 / old_total as f64
    } else {
        1.0
    };
    
    // Apply ratio to all production counts (but cap at theoretical max)
    let new_total = (old_total as f64 * production_ratio).floor().min(new_theoretical_max as f64) as u32;
    let quality_rate = if old_total > 0 {
        *input.production.good_units.value() as f64 / old_total as f64
    } else {
        1.0
    };
    let new_good = (new_total as f64 * quality_rate).floor() as u32;
    let new_scrap = new_total.saturating_sub(new_good);
    
    modified_input.production.total_units = 
        crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Inferred(new_total);
    modified_input.production.good_units = 
        crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Inferred(new_good);
    modified_input.production.scrap_units = 
        crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Inferred(new_scrap);
    
    let modified_metrics = calculate_core_metrics_from_input(
        &modified_input,
        baseline_metrics.oee.confidence.clone(),
    );
    
    let baseline_oee = baseline_metrics.oee.value * 100.0;
    let varied_oee = modified_metrics.oee.value * 100.0;
    let oee_delta = varied_oee - baseline_oee;
    
    SensitivityResult {
        parameter_key: "sensitivity.cycle_time".to_string(),
        baseline_value,
        variation_percent: -variation_percent,
        varied_value,
        baseline_oee,
        varied_oee,
        oee_delta,
        impact_level: classify_impact(oee_delta),
        metric_changes: MetricChanges {
            availability_delta: (modified_metrics.availability.value - baseline_metrics.availability.value) * 100.0,
            performance_delta: (modified_metrics.performance.value - baseline_metrics.performance.value) * 100.0,
            quality_delta: (modified_metrics.quality.value - baseline_metrics.quality.value) * 100.0,
        },
    }
}

/// Analyze production count sensitivity (+10% increase)
fn analyze_production_count_sensitivity(
    input: &OeeInput,
    baseline_metrics: &CoreMetrics,
    variation_percent: f64,
) -> SensitivityResult {
    let baseline_value = *input.production.total_units.value() as f64;
    let varied_value = (baseline_value * (1.0 + variation_percent / 100.0)).round();
    
    let mut modified_input = input.clone();
    let new_total = varied_value as u32;
    
    // Scale good units proportionally
    let current_quality = *input.production.good_units.value() as f64 / baseline_value;
    let new_good = (new_total as f64 * current_quality).round() as u32;
    
    modified_input.production.total_units = 
        crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Explicit(new_total);
    modified_input.production.good_units = 
        crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Inferred(new_good);
    
    let modified_metrics = calculate_core_metrics_from_input(
        &modified_input,
        baseline_metrics.oee.confidence.clone(),
    );
    
    let baseline_oee = baseline_metrics.oee.value * 100.0;
    let varied_oee = modified_metrics.oee.value * 100.0;
    let oee_delta = varied_oee - baseline_oee;
    
    SensitivityResult {
        parameter_key: "sensitivity.production_count".to_string(),
        baseline_value,
        variation_percent,
        varied_value,
        baseline_oee,
        varied_oee,
        oee_delta,
        impact_level: classify_impact(oee_delta),
        metric_changes: MetricChanges {
            availability_delta: (modified_metrics.availability.value - baseline_metrics.availability.value) * 100.0,
            performance_delta: (modified_metrics.performance.value - baseline_metrics.performance.value) * 100.0,
            quality_delta: (modified_metrics.quality.value - baseline_metrics.quality.value) * 100.0,
        },
    }
}

/// Analyze good units sensitivity (+10% quality improvement)
fn analyze_good_units_sensitivity(
    input: &OeeInput,
    baseline_metrics: &CoreMetrics,
    variation_percent: f64,
) -> SensitivityResult {
    let baseline_value = *input.production.good_units.value() as f64;
    let varied_value = (baseline_value * (1.0 + variation_percent / 100.0))
        .round()
        .min(*input.production.total_units.value() as f64); // Can't exceed total
    
    let mut modified_input = input.clone();
    let new_good = varied_value as u32;
    
    // Reduce scrap correspondingly
    let increase = new_good.saturating_sub(*input.production.good_units.value());
    let new_scrap = input.production.scrap_units.value().saturating_sub(increase);
    
    modified_input.production.good_units = 
        crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Explicit(new_good);
    modified_input.production.scrap_units = 
        crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Inferred(new_scrap);
    
    let modified_metrics = calculate_core_metrics_from_input(
        &modified_input,
        baseline_metrics.oee.confidence.clone(),
    );
    
    let baseline_oee = baseline_metrics.oee.value * 100.0;
    let varied_oee = modified_metrics.oee.value * 100.0;
    let oee_delta = varied_oee - baseline_oee;
    
    SensitivityResult {
        parameter_key: "sensitivity.good_units".to_string(),
        baseline_value,
        variation_percent,
        varied_value,
        baseline_oee,
        varied_oee,
        oee_delta,
        impact_level: classify_impact(oee_delta),
        metric_changes: MetricChanges {
            availability_delta: (modified_metrics.availability.value - baseline_metrics.availability.value) * 100.0,
            performance_delta: (modified_metrics.performance.value - baseline_metrics.performance.value) * 100.0,
            quality_delta: (modified_metrics.quality.value - baseline_metrics.quality.value) * 100.0,
        },
    }
}

/// Analyze scrap sensitivity (-10% scrap reduction)
fn analyze_scrap_sensitivity(
    input: &OeeInput,
    baseline_metrics: &CoreMetrics,
    variation_percent: f64,
) -> SensitivityResult {
    let baseline_value = *input.production.scrap_units.value() as f64;
    // Reduce scrap
    let varied_value = (baseline_value * (1.0 - variation_percent / 100.0)).round().max(0.0);
    
    let mut modified_input = input.clone();
    let new_scrap = varied_value as u32;
    
    // Add difference to good units
    let scrap_reduction = input.production.scrap_units.value().saturating_sub(new_scrap);
    let new_good = *input.production.good_units.value() + scrap_reduction;
    
    modified_input.production.scrap_units = 
        crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Explicit(new_scrap);
    modified_input.production.good_units = 
        crate::calculus::engineer::calculators::production::oee::assumptions::InputValue::Inferred(new_good);
    
    let modified_metrics = calculate_core_metrics_from_input(
        &modified_input,
        baseline_metrics.oee.confidence.clone(),
    );
    
    let baseline_oee = baseline_metrics.oee.value * 100.0;
    let varied_oee = modified_metrics.oee.value * 100.0;
    let oee_delta = varied_oee - baseline_oee;
    
    SensitivityResult {
        parameter_key: "sensitivity.scrap_units".to_string(),
        baseline_value,
        variation_percent: -variation_percent,
        varied_value,
        baseline_oee,
        varied_oee,
        oee_delta,
        impact_level: classify_impact(oee_delta),
        metric_changes: MetricChanges {
            availability_delta: (modified_metrics.availability.value - baseline_metrics.availability.value) * 100.0,
            performance_delta: (modified_metrics.performance.value - baseline_metrics.performance.value) * 100.0,
            quality_delta: (modified_metrics.quality.value - baseline_metrics.quality.value) * 100.0,
        },
    }
}

/// Classify impact level based on OEE delta
fn classify_impact(oee_delta: f64) -> SensitivityImpact {
    let abs_delta = oee_delta.abs();
    
    if abs_delta > 5.0 {
        SensitivityImpact::Critical
    } else if abs_delta > 2.0 {
        SensitivityImpact::High
    } else if abs_delta > 0.5 {
        SensitivityImpact::Medium
    } else {
        SensitivityImpact::Low
    }
}

/// Run quick sensitivity analysis with default ±10% variation
pub fn quick_sensitivity_analysis(
    input: &OeeInput,
    baseline_metrics: &CoreMetrics,
) -> SensitivityAnalysis {
    analyze_sensitivity(input, baseline_metrics, 10.0)
}