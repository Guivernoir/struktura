//! Assumptions layer: Structured input handling
//! 
//! This is the "curation layer" - where we accept analyst-provided
//! summaries and track their provenance religiously.
//! 
//! Every value knows if it's Explicit, Inferred, or Default.
//! That distinction is the entire positioning.

pub mod counts;
pub mod cycle;
pub mod downtime;
pub mod thresholds;
pub mod time;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// How a value was obtained - the foundation of trust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputValue<T> {
    /// User explicitly provided this value
    Explicit(T),
    /// We derived this from other inputs
    Inferred(T),
    /// System default fallback
    Default(T),
}

impl<T> InputValue<T> {
    /// Get the actual value, regardless of source
    pub fn value(&self) -> &T {
        match self {
            InputValue::Explicit(v) => v,
            InputValue::Inferred(v) => v,
            InputValue::Default(v) => v,
        }
    }
    
    /// Get the value with ownership transfer
    pub fn into_value(self) -> T {
        match self {
            InputValue::Explicit(v) => v,
            InputValue::Inferred(v) => v,
            InputValue::Default(v) => v,
        }
    }
    
    /// Check if this is an explicit value
    pub fn is_explicit(&self) -> bool {
        matches!(self, InputValue::Explicit(_))
    }
    
    /// Check if this is inferred
    pub fn is_inferred(&self) -> bool {
        matches!(self, InputValue::Inferred(_))
    }
    
    /// Check if this is a default
    pub fn is_default(&self) -> bool {
        matches!(self, InputValue::Default(_))
    }
    
    /// Get the source type as string (for ledger)
    pub fn source_type(&self) -> &'static str {
        match self {
            InputValue::Explicit(_) => "explicit",
            InputValue::Inferred(_) => "inferred",
            InputValue::Default(_) => "default",
        }
    }
    
    /// Map the value to a different type
    pub fn map<U, F>(self, f: F) -> InputValue<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            InputValue::Explicit(v) => InputValue::Explicit(f(v)),
            InputValue::Inferred(v) => InputValue::Inferred(f(v)),
            InputValue::Default(v) => InputValue::Default(f(v)),
        }
    }
}

/// Analysis time window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisWindow {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl AnalysisWindow {
    pub fn duration(&self) -> Duration {
        let diff = self.end - self.start;
        Duration::from_secs(diff.num_seconds().max(0) as u64)
    }
}

/// Machine context for the analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineContext {
    pub machine_id: String,
    pub line_id: Option<String>,
    pub product_id: Option<String>,
    pub shift_id: Option<String>,
}

/// Machine operational states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MachineState {
    Running,
    Stopped,
    Setup,
    Starved,
    Blocked,
    Maintenance,
    Unknown,
}

impl MachineState {
    pub fn translation_key(&self) -> &'static str {
        match self {
            MachineState::Running => "state.running",
            MachineState::Stopped => "state.stopped",
            MachineState::Setup => "state.setup",
            MachineState::Starved => "state.starved",
            MachineState::Blocked => "state.blocked",
            MachineState::Maintenance => "state.maintenance",
            MachineState::Unknown => "state.unknown",
        }
    }
}

/// Hierarchical reason code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonCode {
    /// Hierarchical path, e.g., ["Mechanical", "Bearing Failure"]
    pub path: Vec<String>,
    pub is_failure: bool,
}

impl ReasonCode {
    pub fn new(path: Vec<String>) -> Self {
        Self { 
            path,
            is_failure: false,
        }
    }
    
    pub fn from_single(reason: &str) -> Self {
        Self {
            path: vec![reason.to_string()],
            is_failure: false,
        }
    }
    
    /// Get the most specific (leaf) reason
    pub fn leaf(&self) -> Option<&String> {
        self.path.last()
    }
    
    /// Get the root (top-level) reason
    pub fn root(&self) -> Option<&String> {
        self.path.first()
    }
    
    /// Get the full path as a string (e.g., "Mechanical > Bearing Failure")
    pub fn full_path(&self) -> String {
        self.path.join(" > ")
    }
}
