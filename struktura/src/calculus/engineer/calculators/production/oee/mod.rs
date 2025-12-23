//! OEE Calculator - Production Loss & OEE Engineering Framework
//! 
//! A deterministic, assumption-driven calculator for production analysis.
//! This is a System of Reasoning, not a System of Record.

pub mod api;
pub mod assumptions;
pub mod domain;
pub mod engine;
pub mod ledger;
pub mod validation;

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Complete OEE analysis input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OeeInput {
    pub window: assumptions::AnalysisWindow,
    pub machine: assumptions::MachineContext,
    pub time_model: assumptions::time::TimeModel,
    pub production: assumptions::counts::ProductionSummary,
    pub cycle_time: assumptions::cycle::CycleTimeModel,
    pub downtimes: assumptions::downtime::DowntimeCollection,
    pub thresholds: assumptions::thresholds::ThresholdConfiguration,
}

/// Complete OEE analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OeeResult {
    /// Core OEE metrics (Availability, Performance, Quality, OEE)
    pub core_metrics: domain::metrics::CoreMetrics,
    
    /// Extended metrics (TEEP, Utilization, MTBF, etc.)
    pub extended_metrics: domain::extended::ExtendedMetrics,
    
    /// Loss tree decomposition
    pub loss_tree: domain::loss_tree::LossTree,
    
    /// Economic analysis (if parameters provided)
    pub economic_analysis: Option<domain::economics::EconomicAnalysis>,
    
    /// Complete assumption ledger
    pub ledger: ledger::AssumptionLedger,
    
    /// Validation result
    pub validation: validation::ValidationResult,
}

/// Economic parameters for cost analysis (optional)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicParameters {
    pub unit_price: (f64, f64, f64),
    pub marginal_contribution: (f64, f64, f64),
    pub material_cost: (f64, f64, f64),
    pub labor_cost_per_hour: (f64, f64, f64),
    pub currency: String,
}
