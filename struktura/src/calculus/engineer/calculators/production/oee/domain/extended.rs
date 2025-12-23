//! Extended metrics beyond core OEE
//! 
//! TEEP, Utilization, MTBF/MTTR, scrap rates, etc.
//! The extra dials for people who want more granularity.

use super::*;

/// Extended metrics bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedMetrics {
    pub teep: Option<TrackedMetric>,
    pub utilization: TrackedMetric,
    pub mtbf: Option<TrackedMetric>,
    pub mttr: Option<TrackedMetric>,
    pub scrap_rate: TrackedMetric,
    pub rework_rate: TrackedMetric,
    pub net_operating_time: TrackedMetric,
}

/// Calculate TEEP: Total Effective Equipment Performance
/// TEEP = (Operating Time / All Time) × Performance × Quality
pub fn calculate_teep(
    operating_time: Duration,
    all_time: Duration,  // 24/7 theoretical time
    performance: &TrackedMetric,
    quality: &TrackedMetric,
    confidence: Confidence,
) -> Option<TrackedMetric> {
    let all_secs = all_time.as_secs_f64();
    
    if all_secs == 0.0 {
        return None;
    }
    
    let loading_factor = operating_time.as_secs_f64() / all_secs;
    let value = loading_factor * performance.value * quality.value;
    
    Some(TrackedMetric {
        name_key: "metrics.teep".to_string(),
        value,
        unit_key: "units.percentage".to_string(),
        formula_key: "formulas.teep".to_string(),
        formula_params: [
            ("operating_time_seconds".to_string(), operating_time.as_secs_f64()),
            ("all_time_seconds".to_string(), all_secs),
            ("loading_factor".to_string(), loading_factor),
            ("performance".to_string(), performance.value),
            ("quality".to_string(), quality.value),
        ].iter().cloned().collect(),
        confidence,
    })
}

/// Calculate Utilization: Operating Time / Planned Time
pub fn calculate_utilization(
    operating_time: Duration,
    planned_time: Duration,
    confidence: Confidence,
) -> TrackedMetric {
    let planned_secs = planned_time.as_secs_f64();
    
    let value = if planned_secs > 0.0 {
        (operating_time.as_secs_f64() / planned_secs).max(0.0).min(1.0)
    } else {
        0.0
    };
    
    TrackedMetric {
        name_key: "metrics.utilization".to_string(),
        value,
        unit_key: "units.percentage".to_string(),
        formula_key: "formulas.utilization".to_string(),
        formula_params: [
            ("operating_time_seconds".to_string(), operating_time.as_secs_f64()),
            ("planned_time_seconds".to_string(), planned_secs),
        ].iter().cloned().collect(),
        confidence,
    }
}

/// Calculate Mean Time Between Failures (illustrative - assumes modeled data)
pub fn calculate_mtbf(
    operating_time: Duration,
    failure_count: u32,
    confidence: Confidence,
) -> Option<TrackedMetric> {
    if failure_count == 0 {
        return None;
    }
    
    let value = operating_time.as_secs_f64() / (failure_count as f64);
    
    Some(TrackedMetric {
        name_key: "metrics.mtbf".to_string(),
        value,
        unit_key: "units.seconds".to_string(),
        formula_key: "formulas.mtbf".to_string(),
        formula_params: [
            ("operating_time_seconds".to_string(), operating_time.as_secs_f64()),
            ("failure_count".to_string(), failure_count as f64),
        ].iter().cloned().collect(),
        confidence,
    })
}

/// Calculate Mean Time To Repair (illustrative)
pub fn calculate_mttr(
    total_repair_time: Duration,
    failure_count: u32,
    confidence: Confidence,
) -> Option<TrackedMetric> {
    if failure_count == 0 {
        return None;
    }
    
    let value = total_repair_time.as_secs_f64() / (failure_count as f64);
    
    Some(TrackedMetric {
        name_key: "metrics.mttr".to_string(),
        value,
        unit_key: "units.seconds".to_string(),
        formula_key: "formulas.mttr".to_string(),
        formula_params: [
            ("total_repair_time_seconds".to_string(), total_repair_time.as_secs_f64()),
            ("failure_count".to_string(), failure_count as f64),
        ].iter().cloned().collect(),
        confidence,
    })
}

/// Calculate scrap rate
pub fn calculate_scrap_rate(
    scrap_count: u32,
    total_count: u32,
    confidence: Confidence,
) -> TrackedMetric {
    let value = if total_count > 0 {
        (scrap_count as f64 / total_count as f64).max(0.0).min(1.0)
    } else {
        0.0
    };
    
    TrackedMetric {
        name_key: "metrics.scrap_rate".to_string(),
        value,
        unit_key: "units.percentage".to_string(),
        formula_key: "formulas.scrap_rate".to_string(),
        formula_params: [
            ("scrap_count".to_string(), scrap_count as f64),
            ("total_count".to_string(), total_count as f64),
        ].iter().cloned().collect(),
        confidence,
    }
}

/// Calculate rework rate
pub fn calculate_rework_rate(
    rework_count: u32,
    total_count: u32,
    confidence: Confidence,
) -> TrackedMetric {
    let value = if total_count > 0 {
        (rework_count as f64 / total_count as f64).max(0.0).min(1.0)
    } else {
        0.0
    };
    
    TrackedMetric {
        name_key: "metrics.rework_rate".to_string(),
        value,
        unit_key: "units.percentage".to_string(),
        formula_key: "formulas.rework_rate".to_string(),
        formula_params: [
            ("rework_count".to_string(), rework_count as f64),
            ("total_count".to_string(), total_count as f64),
        ].iter().cloned().collect(),
        confidence,
    }
}

/// Calculate net operating time (for economic calculations)
pub fn calculate_net_operating_time(
    operating_time: Duration,
    confidence: Confidence,
) -> TrackedMetric {
    TrackedMetric {
        name_key: "metrics.net_operating_time".to_string(),
        value: operating_time.as_secs_f64(),
        unit_key: "units.seconds".to_string(),
        formula_key: "formulas.net_operating_time".to_string(),
        formula_params: [
            ("operating_time_seconds".to_string(), operating_time.as_secs_f64()),
        ].iter().cloned().collect(),
        confidence,
    }
}
