//! OEE metric calculations (with TEEP support)
//! 
//! Bridge between input structures and domain calculations.
//! Now includes proper TEEP calculation when all_time is provided.

use crate::calculus::engineer::calculators::production::oee::{
    assumptions::{cycle::CycleTimeModel, counts::ProductionSummary, time::TimeModel},
    domain::{self, Confidence},
    OeeInput,
};

/// Calculate core metrics from complete input
pub fn calculate_core_metrics_from_input(
    input: &OeeInput,
    confidence: Confidence,
) -> domain::metrics::CoreMetrics {
    let planned_time = *input.time_model.planned_production_time.value();
    let downtime = input.time_model.total_downtime();
    let ideal_cycle_time = *input.cycle_time.ideal_cycle_time.value();
    let total_count = *input.production.total_units.value();
    let good_count = *input.production.good_units.value();
    
    domain::metrics::calculate_core_metrics(
        planned_time,
        downtime,
        ideal_cycle_time,
        total_count,
        good_count,
        confidence,
    )
}

/// Calculate extended metrics from complete input
pub fn calculate_extended_metrics_from_input(
    input: &OeeInput,
    confidence: Confidence,
) -> domain::extended::ExtendedMetrics {
    let planned_time = *input.time_model.planned_production_time.value();
    let operating_time = input.time_model.running_time();
    let total_count = *input.production.total_units.value();
    let scrap_count = *input.production.scrap_units.value();
    let rework_count = *input.production.reworked_units.value();
    
    // Calculate core metrics for TEEP
    let core = calculate_core_metrics_from_input(input, confidence.clone());
    
    // TEEP calculation (if we have all_time)
    let teep = if let Some(all_time) = input.time_model.get_all_time() {
        domain::extended::calculate_teep(
            operating_time,
            all_time,
            &core.performance,
            &core.quality,
            confidence.clone(),
        )
    } else {
        None
    };
    
    let utilization = domain::extended::calculate_utilization(
        operating_time,
        planned_time,
        confidence.clone(),
    );
    
    // MTBF/MTTR 
    let failure_count = count_failures(&input.downtimes);
    let mtbf = domain::extended::calculate_mtbf(operating_time, failure_count, confidence.clone());
    
    let total_repair_time = calculate_repair_time(&input.downtimes);
    let mttr = domain::extended::calculate_mttr(total_repair_time, failure_count, confidence.clone());
    
    let scrap_rate = domain::extended::calculate_scrap_rate(scrap_count, total_count, confidence.clone());
    let rework_rate = domain::extended::calculate_rework_rate(rework_count, total_count, confidence.clone());
    let net_operating_time = domain::extended::calculate_net_operating_time(operating_time, confidence);
    
    domain::extended::ExtendedMetrics {
        teep,
        utilization,
        mtbf,
        mttr,
        scrap_rate,
        rework_rate,
        net_operating_time,
    }
}

/// Count failures in downtime records 
fn count_failures(downtimes: &crate::calculus::engineer::calculators::production::oee::assumptions::downtime::DowntimeCollection) -> u32 {
    downtimes
        .records
        .iter()
        .filter(|r| {
            r.reason.is_failure
        })
        .count() as u32
}

/// Calculate total repair time (failures only)
fn calculate_repair_time(downtimes: &crate::calculus::engineer::calculators::production::oee::assumptions::downtime::DowntimeCollection) -> std::time::Duration {
    downtimes
        .records
        .iter()
        .filter(|r| {
            r.reason.is_failure
        })
        .map(|r| *r.duration.value())
        .sum()
}