//! Domain layer: Core business logic for OEE calculations
//! 
//! This is where assumptions become insights.
//! Every calculation is traceable, every metric is explainable.
//! Attribution only - no causality claims.

pub mod economics;
pub mod extended;
pub mod loss_tree;
pub mod metrics;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Represents how a value was derived - critical for traceability
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueSource {
    /// User provided this value explicitly
    Explicit,
    /// We calculated/inferred this from other inputs
    Inferred,
    /// System default was used
    Default,
}

/// A calculated metric with full traceability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedMetric {
    /// The metric's name (translation key)
    pub name_key: String,
    /// The calculated value
    pub value: f64,
    /// Unit of measurement (translation key)
    pub unit_key: String,
    /// Formula used (translation key)
    pub formula_key: String,
    /// Parameters that went into the formula
    pub formula_params: HashMap<String, f64>,
    /// How confident we are in this (based on input sources)
    pub confidence: Confidence,
}

/// Confidence level based on input quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Ord, Eq, PartialOrd)]
pub enum Confidence {
    High,    // All inputs explicit
    Medium,  // Mix of explicit/inferred
    Low,     // Significant defaults used
}

/// A time-based loss allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LossAllocation {
    /// Category name (translation key)
    pub category_key: String,
    /// Duration of this loss
    pub duration: Duration,
    /// Percentage of total time
    pub percentage: f64,
    /// Sub-categories (hierarchical)
    pub sub_allocations: Vec<LossAllocation>,
}

/// Economic impact with uncertainty bounds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicImpact {
    /// Description (translation key)
    pub description_key: String,
    /// Low estimate
    pub low_estimate: f64,
    /// Central estimate
    pub central_estimate: f64,
    /// High estimate
    pub high_estimate: f64,
    /// Currency code (ISO 4217)
    pub currency: String,
    /// Assumptions used (translation keys)
    pub assumptions: Vec<String>,
}
