//! Calculation engine: The orchestrator
//! 
//! This is where Input becomes Result.
//! Pure functions, no side effects, complete traceability.

pub mod decomposition;
pub mod leverage;
pub mod oee;
pub mod sensitivity;
pub mod temporal_scrap;
pub mod multi_machine;

use crate::calculus::engineer::calculators::production::oee::{
    domain::{self, Confidence, ValueSource},
    ledger::{assumption_tracking::AssumptionTracker, AssumptionLedger, ImpactLevel},
    validation::{self, ValidationResult},
    OeeInput, OeeResult,
};

/// Main calculation pipeline
pub fn calculate_oee(input: OeeInput) -> Result<OeeResult, EngineError> {
    // Step 1: Validate inputs
    let validation_result = validate_input(&input)?;
    
    // Step 2: Build assumption ledger
    let mut ledger = build_ledger(&input);
    
    // Step 3: Determine input confidence
    let confidence = determine_confidence(&input);
    
    // Step 4: Calculate core metrics
    let core_metrics = oee::calculate_core_metrics_from_input(&input, confidence.clone());
    
    // Step 5: Calculate extended metrics
    let extended_metrics = oee::calculate_extended_metrics_from_input(&input, confidence.clone());
    
    // Step 6: Build loss tree
    let loss_tree = decomposition::build_loss_tree(&input);
    
    // Step 7: Add validation warnings to ledger
    transfer_validation_to_ledger(&validation_result, &mut ledger);
    
    Ok(OeeResult {
        core_metrics,
        extended_metrics,
        loss_tree,
        economic_analysis: None, // Calculated separately if parameters provided
        ledger,
        validation: validation_result,
    })
}

/// Calculate with economic analysis
pub fn calculate_oee_with_economics(
    input: OeeInput,
    economic_params: domain::economics::EconomicParameters,
) -> Result<OeeResult, EngineError> {
    let mut result = calculate_oee(input.clone())?;
    
    // Calculate economic impact
    let lost_units = calculate_lost_units(&input);
    let scrap_units = *input.production.scrap_units.value();
    let rework_units = *input.production.reworked_units.value();
    let downtime_hours = input.time_model.total_downtime().as_secs_f64() / 3600.0;
    let theoretical_units_per_hour = calculate_theoretical_rate(&input);
    
    // Assume 0.1 hours per rework unit (configurable in production)
    let avg_rework_time = 0.1;
    
    let economic_analysis = domain::economics::analyze_economics(
        lost_units,
        scrap_units,
        rework_units,
        downtime_hours,
        theoretical_units_per_hour,
        avg_rework_time,
        &economic_params,
    );
    
    result.economic_analysis = Some(economic_analysis);
    
    Ok(result)
}

/// Validate complete input
fn validate_input(input: &OeeInput) -> Result<ValidationResult, EngineError> {
    let mut result = ValidationResult::new();
    
    // Time allocations
    result.merge(validation::logical::validate_time_allocations(
        *input.time_model.planned_production_time.value(),
        &input.time_model.allocations.iter()
            .map(|a| *a.duration.value())
            .collect::<Vec<_>>(),
    ));
    
    // Production counts
    result.merge(validation::logical::validate_production_counts(
        *input.production.total_units.value(),
        *input.production.good_units.value(),
        *input.production.scrap_units.value(),
        *input.production.reworked_units.value(),
    ));
    
    // Cycle times
    result.merge(validation::logical::validate_cycle_times(
        *input.cycle_time.ideal_cycle_time.value(),
        input.cycle_time.average_cycle_time.as_ref().map(|a| *a.value()),
    ));
    
    // Capacity constraints
    let running_time = input.time_model.running_time();
    result.merge(validation::logical::validate_capacity_constraints(
        *input.production.total_units.value(),
        running_time,
        *input.cycle_time.ideal_cycle_time.value(),
    ));
    
    // Downtime vs time allocation consistency
    result.merge(validation::logical::validate_downtime_records(
        &input.downtimes.records.iter()
            .map(|r| *r.duration.value())
            .collect::<Vec<_>>(),
        input.time_model.total_downtime(),
    ));
    
    // Warnings for data quality
    result.merge(validation::warnings::check_high_scrap_rate(
        *input.production.scrap_units.value(),
        *input.production.total_units.value(),
        input.thresholds.high_scrap_rate_threshold,
    ));
    
    result.merge(validation::warnings::check_low_utilization(
        running_time,
        *input.time_model.planned_production_time.value(),
        input.thresholds.low_utilization_threshold,
    ));
    
    // Check if we can proceed
    if result.has_fatal_errors() {
        return Err(EngineError::ValidationFailed(result));
    }
    
    Ok(result)
}

/// Build complete assumption ledger
fn build_ledger(input: &OeeInput) -> AssumptionLedger {
    let mut tracker = AssumptionTracker::new();
    
    // Track time inputs
    tracker.track_duration(
        "planned_production_time",
        "ledger.assumptions.planned_time",
        &input.time_model.planned_production_time,
        ImpactLevel::Critical,
    );
    
    // Track production counts
    tracker.track_count(
        "total_units",
        "ledger.assumptions.total_units",
        &input.production.total_units,
        ImpactLevel::Critical,
    );
    
    tracker.track_count(
        "good_units",
        "ledger.assumptions.good_units",
        &input.production.good_units,
        ImpactLevel::Critical,
    );
    
    tracker.track_count(
        "scrap_units",
        "ledger.assumptions.scrap_units",
        &input.production.scrap_units,
        ImpactLevel::High,
    );
    
    tracker.track_count(
        "reworked_units",
        "ledger.assumptions.reworked_units",
        &input.production.reworked_units,
        ImpactLevel::Medium,
    );
    
    // Track cycle time
    tracker.track_duration(
        "ideal_cycle_time",
        "ledger.assumptions.ideal_cycle_time",
        &input.cycle_time.ideal_cycle_time,
        ImpactLevel::Critical,
    );
    
    // Track thresholds
    tracker.track_threshold(
        "micro_stoppage_threshold",
        input.thresholds.micro_stoppage_threshold.as_secs_f64(),
        "units.seconds",
        "ledger.thresholds.micro_stoppage_rationale",
    );
    
    tracker.track_threshold(
        "speed_loss_threshold",
        input.thresholds.speed_loss_threshold,
        "units.percentage",
        "ledger.thresholds.speed_loss_rationale",
    );
    
    // Add metadata
    let mut ledger = tracker.finish();
    ledger.add_metadata("machine_id", &input.machine.machine_id);
    if let Some(line) = &input.machine.line_id {
        ledger.add_metadata("line_id", line);
    }
    if let Some(product) = &input.machine.product_id {
        ledger.add_metadata("product_id", product);
    }
    
    ledger
}

/// Determine overall confidence based on input sources
fn determine_confidence(input: &OeeInput) -> Confidence {
    let mut explicit_count = 0;
    let mut inferred_count = 0;
    let mut default_count = 0;
    
    // Check critical inputs
    if input.time_model.planned_production_time.is_explicit() {
        explicit_count += 1;
    } else if input.time_model.planned_production_time.is_inferred() {
        inferred_count += 1;
    } else {
        default_count += 1;
    }
    
    if input.production.total_units.is_explicit() {
        explicit_count += 1;
    } else if input.production.total_units.is_inferred() {
        inferred_count += 1;
    } else {
        default_count += 1;
    }
    
    if input.production.good_units.is_explicit() {
        explicit_count += 1;
    } else if input.production.good_units.is_inferred() {
        inferred_count += 1;
    } else {
        default_count += 1;
    }
    
    // Decision logic
    if default_count > 0 {
        Confidence::Low
    } else if inferred_count > explicit_count {
        Confidence::Medium
    } else {
        Confidence::High
    }
}

/// Transfer validation warnings to ledger
fn transfer_validation_to_ledger(validation: &ValidationResult, ledger: &mut AssumptionLedger) {
    use crate::calculus::engineer::calculators::production::oee::ledger::WarningSeverity;
    use crate::calculus::engineer::calculators::production::oee::validation::Severity;
    
    for issue in &validation.issues {
        let severity = match issue.severity {
            Severity::Fatal => WarningSeverity::High,
            Severity::Warning => WarningSeverity::Medium,
            Severity::Info => WarningSeverity::Low,
        };
        
        ledger.add_warning(crate::calculus::engineer::calculators::production::oee::ledger::LedgerWarning {
            code: issue.code.clone(),
            message_key: issue.message_key.clone(),
            params: issue.params.clone(),
            severity,
            related_assumptions: Vec::new(),
        });
    }
}

/// Calculate lost units (theoretical max - actual)
fn calculate_lost_units(input: &OeeInput) -> u32 {
    let running_time = input.time_model.running_time();
    let ideal_cycle_time = *input.cycle_time.ideal_cycle_time.value();
    
    if ideal_cycle_time.as_secs() == 0 {
        return 0;
    }
    
    let theoretical_max = (running_time.as_secs_f64() / ideal_cycle_time.as_secs_f64()).floor() as u32;
    let actual = *input.production.total_units.value();
    
    theoretical_max.saturating_sub(actual)
}

/// Calculate theoretical production rate (units/hour)
fn calculate_theoretical_rate(input: &OeeInput) -> f64 {
    let ideal_cycle_time = *input.cycle_time.ideal_cycle_time.value();
    
    if ideal_cycle_time.as_secs() == 0 {
        return 0.0;
    }
    
    3600.0 / ideal_cycle_time.as_secs_f64()
}

/// Engine error types
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("Validation failed")]
    ValidationFailed(ValidationResult),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Calculation error: {0}")]
    CalculationError(String),
}
