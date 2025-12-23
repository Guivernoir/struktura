//! Assumption Ledger: Complete traceability system
//! 
//! This is the "trust builder" - every assumption, every source,
//! every threshold, every warning, all in one auditable structure.
//! 
//! Per the README: "Always accessible from results."

pub mod assumption_tracking;

use crate::calculus::engineer::calculators::production::oee::assumptions::InputValue;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// A single tracked assumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumptionEntry {
    /// Assumption identifier (translation key)
    pub assumption_key: String,
    /// Human-readable description (translation key)
    pub description_key: String,
    /// The actual value
    pub value: serde_json::Value,
    /// How it was obtained
    pub source: String,  // "explicit", "inferred", "default"
    /// When it was recorded
    pub timestamp: DateTime<Utc>,
    /// Impact level on results
    pub impact: ImpactLevel,
    /// Related assumptions (dependencies)
    pub related_assumptions: Vec<String>,
}

/// Impact level of an assumption on results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImpactLevel {
    Critical,  // Changes this significantly affect OEE
    High,      // Material impact on results
    Medium,    // Moderate impact
    Low,       // Minor impact
    Info,      // Informational only
}

/// Warning recorded during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerWarning {
    /// Warning code
    pub code: String,
    /// Warning message (translation key)
    pub message_key: String,
    /// Parameters for translation
    pub params: serde_json::Value,
    /// Severity
    pub severity: WarningSeverity,
    /// Related assumptions
    pub related_assumptions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WarningSeverity {
    High,
    Medium,
    Low,
}

/// Threshold configuration record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdRecord {
    /// Threshold name (translation key)
    pub threshold_key: String,
    /// Value used
    pub value: f64,
    /// Unit (translation key)
    pub unit_key: String,
    /// Why this threshold was used
    pub rationale_key: String,
}

/// Complete assumption ledger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumptionLedger {
    /// When this analysis was performed
    pub analysis_timestamp: DateTime<Utc>,
    /// All tracked assumptions
    pub assumptions: Vec<AssumptionEntry>,
    /// All warnings raised
    pub warnings: Vec<LedgerWarning>,
    /// All thresholds used
    pub thresholds: Vec<ThresholdRecord>,
    /// Input source statistics
    pub source_statistics: SourceStatistics,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Statistics about input sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceStatistics {
    pub explicit_count: usize,
    pub inferred_count: usize,
    pub default_count: usize,
    pub total_count: usize,
    pub explicit_percentage: f64,
    pub inferred_percentage: f64,
    pub default_percentage: f64,
}

impl AssumptionLedger {
    pub fn new() -> Self {
        Self {
            analysis_timestamp: Utc::now(),
            assumptions: Vec::new(),
            warnings: Vec::new(),
            thresholds: Vec::new(),
            source_statistics: SourceStatistics {
                explicit_count: 0,
                inferred_count: 0,
                default_count: 0,
                total_count: 0,
                explicit_percentage: 0.0,
                inferred_percentage: 0.0,
                default_percentage: 0.0,
            },
            metadata: HashMap::new(),
        }
    }
    
    pub fn add_assumption(&mut self, entry: AssumptionEntry) {
        self.assumptions.push(entry);
        self.recalculate_statistics();
    }
    
    pub fn add_warning(&mut self, warning: LedgerWarning) {
        self.warnings.push(warning);
    }
    
    pub fn add_threshold(&mut self, threshold: ThresholdRecord) {
        self.thresholds.push(threshold);
    }
    
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
    
    fn recalculate_statistics(&mut self) {
        let mut explicit = 0;
        let mut inferred = 0;
        let mut default = 0;
        
        for assumption in &self.assumptions {
            match assumption.source.as_str() {
                "explicit" => explicit += 1,
                "inferred" => inferred += 1,
                "default" => default += 1,
                _ => {}
            }
        }
        
        let total = explicit + inferred + default;
        let total_f = total as f64;
        
        self.source_statistics = SourceStatistics {
            explicit_count: explicit,
            inferred_count: inferred,
            default_count: default,
            total_count: total,
            explicit_percentage: if total > 0 { (explicit as f64 / total_f) * 100.0 } else { 0.0 },
            inferred_percentage: if total > 0 { (inferred as f64 / total_f) * 100.0 } else { 0.0 },
            default_percentage: if total > 0 { (default as f64 / total_f) * 100.0 } else { 0.0 },
        };
    }
    
    /// Get critical assumptions only
    pub fn critical_assumptions(&self) -> Vec<&AssumptionEntry> {
        self.assumptions
            .iter()
            .filter(|a| a.impact == ImpactLevel::Critical)
            .collect()
    }
    
    /// Get high-severity warnings
    pub fn high_severity_warnings(&self) -> Vec<&LedgerWarning> {
        self.warnings
            .iter()
            .filter(|w| w.severity == WarningSeverity::High)
            .collect()
    }
    
    /// Get all default values used (red flag for data quality)
    pub fn default_values_used(&self) -> Vec<&AssumptionEntry> {
        self.assumptions
            .iter()
            .filter(|a| a.source == "default")
            .collect()
    }
}

impl Default for AssumptionLedger {
    fn default() -> Self {
        Self::new()
    }
}
