//! Sensitivity analysis and what-if scenarios
//! 
//! Test how results change with input variations.

use crate::calculus::engineer::calculators::production::oee::OeeInput;
use serde::{Deserialize, Serialize};

/// Sensitivity analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityResult {
    /// Parameter varied (translation key)
    pub parameter_key: String,
    /// Baseline value
    pub baseline_value: f64,
    /// Variation tested (±%)
    pub variation_percent: f64,
    /// Impact on OEE (absolute points)
    pub oee_delta: f64,
    /// Impact classification
    pub impact_level: SensitivityImpact,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SensitivityImpact {
    Critical,  // >5% OEE swing
    High,      // 2-5% OEE swing
    Medium,    // 0.5-2% OEE swing
    Low,       // <0.5% OEE swing
}

/// Run sensitivity analysis on key parameters
pub fn analyze_sensitivity(input: &OeeInput) -> Vec<SensitivityResult> {
    vec![
        // Placeholder - would test ±10% variations on:
        // - Planned time
        // - Downtime
        // - Cycle time
        // - Production counts
        // Returns how much OEE changes
    ]
}
