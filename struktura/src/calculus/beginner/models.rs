use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// ENUMS AND CONSTANTS
// ============================================================================

/// Calculator categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculatorCategory {
    Garden,
    Interiors,
    Outdoors,
    Utilities,
}

impl CalculatorCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Garden => "garden",
            Self::Interiors => "interiors",
            Self::Outdoors => "outdoors",
            Self::Utilities => "utilities",
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Garden => "Garden & Landscaping",
            Self::Interiors => "Interior Construction",
            Self::Outdoors => "Outdoor Construction",
            Self::Utilities => "Utilities & Finishes",
        }
    }
}

/// Warning severity levels
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WarningSeverity {
    Low,
    Medium,
    High,
    Critical,
}

// ============================================================================
// INPUT MODELS
// ============================================================================

/// Beginner parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeginnerParameters {
    pub width: f64,
    pub length: f64,
    pub height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional: Option<HashMap<String, f64>>,
}

// Default impl if needed
impl Default for BeginnerParameters {
    fn default() -> Self {
        Self {
            width: 0.0,
            length: 0.0,
            height: 0.0,
            additional: None,
        }
    }
}

/// Request structure for beginner calculations
#[derive(Debug, Deserialize)]
pub struct BeginnerCalculationRequest {
    pub calculation_type: String,
    pub parameters: BeginnerParameters,
}

// ============================================================================
// OUTPUT MODELS
// ============================================================================

/// Single result item
#[derive(Debug, Clone, Serialize)]
pub struct BeginnerResultItem {
    pub label: String,
    pub value: f64,
    pub unit: String,
}

/// Calculation response
#[derive(Debug, Serialize)]
pub struct BeginnerCalculationResponse {
    pub calculation_type: String,
    pub results: Vec<BeginnerResultItem>,
    pub warnings: Vec<String>,
}

// ============================================================================
// METADATA MODELS
// ============================================================================

/// Parameter metadata
#[derive(Debug, Clone, Serialize)]
pub struct ParameterMetadata {
    pub name: String,
    pub path: String,
    pub data_type: String,
    pub unit: String,
    pub description: String,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typical_range: Option<(f64, f64)>,
}

/// Calculator metadata
#[derive(Debug, Clone, Serialize)]
pub struct BeginnerCalculatorMetadata {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub parameters: Vec<ParameterMetadata>,
    pub required_parameters: Vec<String>,
    pub optional_parameters: Vec<String>,
}

/// Category info
#[derive(Debug, Clone, Serialize)]
pub struct BeginnerCategoryInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
}

/// Catalogue
#[derive(Debug, Serialize)]
pub struct BeginnerCalculatorCatalogue {
    pub version: String,
    pub categories: Vec<BeginnerCategoryInfo>,
    pub calculators: Vec<BeginnerCalculatorMetadata>,
    pub disclaimer: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category() {
        let cat = CalculatorCategory::Garden;
        assert_eq!(cat.as_str(), "garden");
        assert_eq!(cat.display_name(), "Garden & Landscaping");
    }
}