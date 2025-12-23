//! Core OEE metrics per ISO 22400-2
//! 
//! Availability × Performance × Quality = OEE
//! 
//! Every metric is traceable to its inputs and formula.

use super::*;
use serde_json::json;

/// Core OEE metrics bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMetrics {
    pub availability: TrackedMetric,
    pub performance: TrackedMetric,
    pub quality: TrackedMetric,
    pub oee: TrackedMetric,
}

/// Calculate availability: (Planned Time - Downtime) / Planned Time
pub fn calculate_availability(
    planned_time: Duration,
    downtime: Duration,
    inputs_confidence: Confidence,
) -> TrackedMetric {
    let planned_secs = planned_time.as_secs_f64();
    let downtime_secs = downtime.as_secs_f64();
    
    let value = if planned_secs > 0.0 {
        ((planned_secs - downtime_secs) / planned_secs).max(0.0).min(1.0)
    } else {
        0.0
    };
    
    TrackedMetric {
        name_key: "metrics.availability".to_string(),
        value,
        unit_key: "units.percentage".to_string(),
        formula_key: "formulas.availability".to_string(),
        formula_params: [
            ("planned_time_seconds".to_string(), planned_secs),
            ("downtime_seconds".to_string(), downtime_secs),
            ("operating_time_seconds".to_string(), planned_secs - downtime_secs),
        ].iter().cloned().collect(),
        confidence: inputs_confidence,
    }
}

/// Calculate performance: (Ideal Cycle Time × Total Count) / Operating Time
pub fn calculate_performance(
    ideal_cycle_time: Duration,
    total_count: u32,
    operating_time: Duration,
    inputs_confidence: Confidence,
) -> TrackedMetric {
    let ideal_secs = ideal_cycle_time.as_secs_f64();
    let operating_secs = operating_time.as_secs_f64();
    let ideal_production_time = ideal_secs * (total_count as f64);
    
    let value = if operating_secs > 0.0 {
        (ideal_production_time / operating_secs).max(0.0).min(1.0)
    } else {
        0.0
    };
    
    TrackedMetric {
        name_key: "metrics.performance".to_string(),
        value,
        unit_key: "units.percentage".to_string(),
        formula_key: "formulas.performance".to_string(),
        formula_params: [
            ("ideal_cycle_time_seconds".to_string(), ideal_secs),
            ("total_count".to_string(), total_count as f64),
            ("operating_time_seconds".to_string(), operating_secs),
            ("ideal_production_time".to_string(), ideal_production_time),
        ].iter().cloned().collect(),
        confidence: inputs_confidence,
    }
}

/// Calculate quality: Good Count / Total Count
pub fn calculate_quality(
    good_count: u32,
    total_count: u32,
    inputs_confidence: Confidence,
) -> TrackedMetric {
    let value = if total_count > 0 {
        (good_count as f64 / total_count as f64).max(0.0).min(1.0)
    } else {
        0.0
    };
    
    TrackedMetric {
        name_key: "metrics.quality".to_string(),
        value,
        unit_key: "units.percentage".to_string(),
        formula_key: "formulas.quality".to_string(),
        formula_params: [
            ("good_count".to_string(), good_count as f64),
            ("total_count".to_string(), total_count as f64),
        ].iter().cloned().collect(),
        confidence: inputs_confidence,
    }
}

/// Calculate OEE: Availability × Performance × Quality
pub fn calculate_oee(
    availability: &TrackedMetric,
    performance: &TrackedMetric,
    quality: &TrackedMetric,
) -> TrackedMetric {
    let value = availability.value * performance.value * quality.value;
    
    // Confidence is the lowest of the three components
    let confidence = [
        &availability.confidence,
        &performance.confidence,
        &quality.confidence,
    ].iter().min().unwrap().clone();
    
    TrackedMetric {
        name_key: "metrics.oee".to_string(),
        value,
        unit_key: "units.percentage".to_string(),
        formula_key: "formulas.oee".to_string(),
        formula_params: [
            ("availability".to_string(), availability.value),
            ("performance".to_string(), performance.value),
            ("quality".to_string(), quality.value),
        ].iter().cloned().collect(),
        confidence: confidence.clone(),
    }
}

/// Calculate all core metrics in one pass
pub fn calculate_core_metrics(
    planned_time: Duration,
    downtime: Duration,
    ideal_cycle_time: Duration,
    total_count: u32,
    good_count: u32,
    inputs_confidence: Confidence,
) -> CoreMetrics {
    let operating_time = planned_time - downtime;
    
    let availability = calculate_availability(planned_time, downtime, inputs_confidence.clone());
    let performance = calculate_performance(ideal_cycle_time, total_count, operating_time, inputs_confidence.clone());
    let quality = calculate_quality(good_count, total_count, inputs_confidence.clone());
    let oee = calculate_oee(&availability, &performance, &quality);
    
    CoreMetrics {
        availability,
        performance,
        quality,
        oee,
    }
}
