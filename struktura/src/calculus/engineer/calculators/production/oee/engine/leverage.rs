//! Loss leverage analysis
//! 
//! Calculate theoretical impact of eliminating loss categories.

use crate::calculus::engineer::calculators::production::oee::{domain::metrics::CoreMetrics, OeeInput};
use serde::{Deserialize, Serialize};

/// Leverage impact analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeverageImpact {
    /// Loss category (translation key)
    pub category_key: String,
    /// OEE points gained if eliminated (e.g., 4.2 = +4.2%)
    pub oee_opportunity_points: f64,
    /// Additional throughput possible
    pub throughput_gain_units: u32,
    /// How sensitive this is to input assumptions
    pub sensitivity_score: f64,
}

/// Calculate leverage for major loss categories
pub fn calculate_leverage(input: &OeeInput, baseline: &CoreMetrics) -> Vec<LeverageImpact> {
    let mut impacts = Vec::new();
    
    // Downtime elimination
    let downtime_impact = calculate_downtime_elimination_impact(input, baseline);
    impacts.push(downtime_impact);
    
    // Speed loss elimination
    let speed_impact = calculate_speed_loss_elimination_impact(input, baseline);
    impacts.push(speed_impact);
    
    // Scrap elimination
    let scrap_impact = calculate_scrap_elimination_impact(input, baseline);
    impacts.push(scrap_impact);
    
    // Sort by OEE opportunity
    impacts.sort_by(|a, b| {
        b.oee_opportunity_points.partial_cmp(&a.oee_opportunity_points)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    
    impacts
}

fn calculate_downtime_elimination_impact(input: &OeeInput, baseline: &CoreMetrics) -> LeverageImpact {
    let planned_time = *input.time_model.planned_production_time.value();
    let current_availability = baseline.availability.value;
    
    // Perfect availability = 100%
    let availability_gain = 1.0 - current_availability;
    let oee_gain = availability_gain * baseline.performance.value * baseline.quality.value * 100.0;
    
    // Throughput gain
    let downtime = input.time_model.total_downtime();
    let ideal_cycle = *input.cycle_time.ideal_cycle_time.value();
    let throughput_gain = if ideal_cycle.as_secs() > 0 {
        (downtime.as_secs_f64() / ideal_cycle.as_secs_f64()).floor() as u32
    } else {
        0
    };
    
    LeverageImpact {
        category_key: "leverage.eliminate_downtime".to_string(),
        oee_opportunity_points: oee_gain,
        throughput_gain_units: throughput_gain,
        sensitivity_score: 0.9, // High sensitivity - depends on accurate downtime tracking
    }
}

fn calculate_speed_loss_elimination_impact(input: &OeeInput, baseline: &CoreMetrics) -> LeverageImpact {
    let current_performance = baseline.performance.value;
    
    // Perfect performance = 100%
    let performance_gain = 1.0 - current_performance;
    let oee_gain = baseline.availability.value * performance_gain * baseline.quality.value * 100.0;
    
    // Throughput gain
    let total_units = *input.production.total_units.value();
    let throughput_gain = ((total_units as f64) * performance_gain).floor() as u32;
    
    LeverageImpact {
        category_key: "leverage.eliminate_speed_loss".to_string(),
        oee_opportunity_points: oee_gain,
        throughput_gain_units: throughput_gain,
        sensitivity_score: 0.7, // Medium sensitivity - cycle time assumptions matter
    }
}

fn calculate_scrap_elimination_impact(input: &OeeInput, baseline: &CoreMetrics) -> LeverageImpact {
    let current_quality = baseline.quality.value;
    
    // Perfect quality = 100%
    let quality_gain = 1.0 - current_quality;
    let oee_gain = baseline.availability.value * baseline.performance.value * quality_gain * 100.0;
    
    // Throughput gain (scrap units become good)
    let scrap_units = *input.production.scrap_units.value();
    
    LeverageImpact {
        category_key: "leverage.eliminate_scrap".to_string(),
        oee_opportunity_points: oee_gain,
        throughput_gain_units: scrap_units,
        sensitivity_score: 0.8, // High sensitivity - depends on accurate count
    }
}
