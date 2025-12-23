//! Assumption tracking utilities
//! 
//! Helper functions to create ledger entries from input values.

use super::*;
use crate::calculus::engineer::calculators::production::oee::assumptions::InputValue;
use chrono::Utc;
use serde_json::json;

/// Track a duration input
pub fn track_duration(
    key: &str,
    description_key: &str,
    value: &InputValue<Duration>,
    impact: ImpactLevel,
) -> AssumptionEntry {
    AssumptionEntry {
        assumption_key: key.to_string(),
        description_key: description_key.to_string(),
        value: json!({
            "seconds": value.value().as_secs(),
            "formatted": format_duration(value.value()),
        }),
        source: value.source_type().to_string(),
        timestamp: Utc::now(),
        impact,
        related_assumptions: Vec::new(),
    }
}

/// Track a count input
pub fn track_count(
    key: &str,
    description_key: &str,
    value: &InputValue<u32>,
    impact: ImpactLevel,
) -> AssumptionEntry {
    AssumptionEntry {
        assumption_key: key.to_string(),
        description_key: description_key.to_string(),
        value: json!(value.value()),
        source: value.source_type().to_string(),
        timestamp: Utc::now(),
        impact,
        related_assumptions: Vec::new(),
    }
}

/// Track a threshold configuration
pub fn track_threshold(
    key: &str,
    value: f64,
    unit_key: &str,
    rationale_key: &str,
) -> ThresholdRecord {
    ThresholdRecord {
        threshold_key: key.to_string(),
        value,
        unit_key: unit_key.to_string(),
        rationale_key: rationale_key.to_string(),
    }
}

/// Create a warning from validation issues
pub fn create_warning(
    code: &str,
    message_key: &str,
    params: serde_json::Value,
    severity: WarningSeverity,
    related: Vec<String>,
) -> LedgerWarning {
    LedgerWarning {
        code: code.to_string(),
        message_key: message_key.to_string(),
        params,
        severity,
        related_assumptions: related,
    }
}

/// Format duration for human readability
fn format_duration(duration: &Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Batch track multiple assumptions
pub struct AssumptionTracker {
    ledger: AssumptionLedger,
}

impl AssumptionTracker {
    pub fn new() -> Self {
        Self {
            ledger: AssumptionLedger::new(),
        }
    }
    
    pub fn track_duration(
        &mut self,
        key: &str,
        description_key: &str,
        value: &InputValue<Duration>,
        impact: ImpactLevel,
    ) -> &mut Self {
        self.ledger.add_assumption(track_duration(key, description_key, value, impact));
        self
    }
    
    pub fn track_count(
        &mut self,
        key: &str,
        description_key: &str,
        value: &InputValue<u32>,
        impact: ImpactLevel,
    ) -> &mut Self {
        self.ledger.add_assumption(track_count(key, description_key, value, impact));
        self
    }
    
    pub fn track_threshold(
        &mut self,
        key: &str,
        value: f64,
        unit_key: &str,
        rationale_key: &str,
    ) -> &mut Self {
        self.ledger.add_threshold(track_threshold(key, value, unit_key, rationale_key));
        self
    }
    
    pub fn add_warning(
        &mut self,
        code: &str,
        message_key: &str,
        params: serde_json::Value,
        severity: WarningSeverity,
        related: Vec<String>,
    ) -> &mut Self {
        self.ledger.add_warning(create_warning(code, message_key, params, severity, related));
        self
    }
    
    pub fn finish(self) -> AssumptionLedger {
        self.ledger
    }
}

impl Default for AssumptionTracker {
    fn default() -> Self {
        Self::new()
    }
}