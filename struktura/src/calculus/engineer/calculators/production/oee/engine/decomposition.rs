//! Loss tree decomposition
//! 
//! Build hierarchical loss structures from input data.

use crate::calculus::engineer::calculators::production::oee::{
    domain::{loss_tree::LossTree, ValueSource},
    OeeInput,
};
use std::time::Duration;

/// Build complete loss tree from input
pub fn build_loss_tree(input: &OeeInput) -> LossTree {
    let planned_time = *input.time_model.planned_production_time.value();
    
    // Calculate loss components
    let breakdowns = calculate_breakdown_time(input);
    let setup_adjustments = calculate_setup_time(input);
    let small_stops = calculate_small_stops(input);
    let speed_losses = calculate_speed_losses(input);
    let startup_rejects = Duration::ZERO; // Would need temporal data
    let production_rejects = calculate_production_rejects(input);
    
    LossTree::build_six_big_losses(
        planned_time,
        breakdowns,
        setup_adjustments,
        small_stops,
        speed_losses,
        startup_rejects,
        production_rejects,
    )
}

/// Calculate breakdown time from downtime records
fn calculate_breakdown_time(input: &OeeInput) -> Duration {
    input.downtimes.records
        .iter()
        .filter(|r| is_breakdown(&r.reason))
        .map(|r| *r.duration.value())
        .sum()
}

/// Calculate setup/adjustment time
fn calculate_setup_time(input: &OeeInput) -> Duration {
    use crate::calculus::engineer::calculators::production::oee::assumptions::MachineState;
    
    input.time_model.allocations
        .iter()
        .filter(|a| a.state == MachineState::Setup)
        .map(|a| *a.duration.value())
        .sum()
}

/// Calculate small stops (micro-stoppages below threshold)
fn calculate_small_stops(input: &OeeInput) -> Duration {
    let threshold = input.thresholds.small_stop_threshold;
    
    input.downtimes.records
        .iter()
        .filter(|r| {
            let dur = *r.duration.value();
            dur < threshold && is_stoppage(&r.reason)
        })
        .map(|r| *r.duration.value())
        .sum()
}

/// Calculate speed losses (running below ideal speed)
fn calculate_speed_losses(input: &OeeInput) -> Duration {
    let running_time = input.time_model.running_time();
    let ideal_cycle = *input.cycle_time.ideal_cycle_time.value();
    let total_units = *input.production.total_units.value();
    
    if ideal_cycle.as_secs() == 0 {
        return Duration::ZERO;
    }
    
    let ideal_production_time = ideal_cycle * total_units;
    
    if running_time > ideal_production_time {
        running_time - ideal_production_time
    } else {
        Duration::ZERO
    }
}

/// Calculate time equivalent of production rejects
fn calculate_production_rejects(input: &OeeInput) -> Duration {
    let scrap_units = *input.production.scrap_units.value();
    let ideal_cycle = *input.cycle_time.ideal_cycle_time.value();
    
    ideal_cycle * scrap_units
}

/// Check if reason code indicates a breakdown
fn is_breakdown(reason: &crate::calculus::engineer::calculators::production::oee::assumptions::ReasonCode) -> bool {
    reason.is_failure
}

/// Check if reason code indicates a stoppage
fn is_stoppage(reason: &crate::calculus::engineer::calculators::production::oee::assumptions::ReasonCode) -> bool {
    !reason.is_failure
}
