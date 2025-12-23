//! Validation layer: Enforces internal mathematical coherence only.
//! 
//! Bright-line rule: We validate logic, not realism.
//! If numbers are mathematically impossible, we flag it.
//! If they're merely "unlikely," we shut up.

pub mod logical;
pub mod ranges;
pub mod warnings;

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Severity levels for validation issues
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    /// Blocks calculation - mathematical impossibility
    Fatal,
    /// Suggests data quality issue - calculation proceeds
    Warning,
    /// Informational only - no action needed
    Info,
}

/// A validation issue with translation-ready messaging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Translation key for the message
    pub message_key: String,
    /// Parameters for translation interpolation
    pub params: serde_json::Value,
    /// Severity level
    pub severity: Severity,
    /// Field path (e.g., "time_allocations[2].duration")
    pub field_path: Option<String>,
    /// Error code for programmatic handling
    pub code: String,
}

impl ValidationIssue {
    pub fn fatal(code: &str, message_key: &str, params: serde_json::Value) -> Self {
        Self {
            message_key: message_key.to_string(),
            params,
            severity: Severity::Fatal,
            field_path: None,
            code: code.to_string(),
        }
    }

    pub fn warning(code: &str, message_key: &str, params: serde_json::Value) -> Self {
        Self {
            message_key: message_key.to_string(),
            params,
            severity: Severity::Warning,
            field_path: None,
            code: code.to_string(),
        }
    }

    pub fn info(code: &str, message_key: &str, params: serde_json::Value) -> Self {
        Self {
            message_key: message_key.to_string(),
            params,
            severity: Severity::Info,
            field_path: None,
            code: code.to_string(),
        }
    }

    pub fn with_field(mut self, field_path: &str) -> Self {
        self.field_path = Some(field_path.to_string());
        self
    }
}

/// Complete validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            issues: Vec::new(),
        }
    }

    pub fn add_issue(&mut self, issue: ValidationIssue) {
        if issue.severity == Severity::Fatal {
            self.is_valid = false;
        }
        self.issues.push(issue);
    }

    pub fn merge(&mut self, other: ValidationResult) {
        self.is_valid = self.is_valid && other.is_valid;
        self.issues.extend(other.issues);
    }

    pub fn has_fatal_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Fatal)
    }

    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Warning)
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}
