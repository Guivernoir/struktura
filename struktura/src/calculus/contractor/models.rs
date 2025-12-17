use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// ENUMS AND CONSTANTS
// ============================================================================

/// Contracting discipline categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculatorCategory {
    Bidding,
    Scheduling,
    Estimation,
    Management,
}

impl CalculatorCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bidding => "bidding",
            Self::Scheduling => "scheduling",
            Self::Estimation => "estimation",
            Self::Management => "management",
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Bidding => "Bidding",
            Self::Scheduling => "Scheduling",
            Self::Estimation => "Estimation",
            Self::Management => "Management",
        }
    }
}

/// Regulation codes and standards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegulationCode {
    // Construction
    IBC,
    NEC,
    OSHA,
    LEED,
    
    // General
    ISO,
    ASTM,
    
    // Management
    PMP,
    Agile,
}

impl RegulationCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IBC => "IBC",
            Self::NEC => "NEC",
            Self::OSHA => "OSHA",
            Self::LEED => "LEED",
            Self::ISO => "ISO",
            Self::ASTM => "ASTM",
            Self::PMP => "PMP",
            Self::Agile => "Agile",
        }
    }
}

// ============================================================================
// INPUT MODELS
// ============================================================================

/// Material properties for contracting calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialProperties {
    pub material_type: String,
    
    // Cost properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waste_factor: Option<f64>,
    
    // Physical properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,                // kg/m³
    
    // Other properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,
}

impl Default for MaterialProperties {
    fn default() -> Self {
        Self {
            material_type: "Concrete".to_string(),
            unit_cost: None,
            waste_factor: None,
            density: None,
            availability: None,
        }
    }
}

/// Resource requirements definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub labor_hours: f64,
    pub equipment_hours: f64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcontractor_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overhead: Option<f64>,
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            labor_hours: 1.0,
            equipment_hours: 1.0,
            material_quantity: None,
            subcontractor_cost: None,
            overhead: None,
        }
    }
}

/// Safety factors for design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyFactors {
    pub cost_factor: f64,
    pub time_factor: f64,
    pub risk_reduction_factor: f64,
    pub importance_factor: f64,
}

impl Default for SafetyFactors {
    fn default() -> Self {
        Self {
            cost_factor: 1.1,
            time_factor: 1.2,
            risk_reduction_factor: 0.9,
            importance_factor: 1.0,
        }
    }
}

/// Comprehensive contracting parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractingParameters {
    /// Geometric dimensions (meters, etc.)
    pub dimensions: HashMap<String, f64>,
    
    /// Material properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<MaterialProperties>,
    
    /// Resource conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceRequirements>,
    
    /// Safety factors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_factors: Option<SafetyFactors>,
    
    /// Regulation code: "IBC", "OSHA", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulation_code: Option<String>,
    
    /// Environmental conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposure_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity: Option<f64>,
    
    /// Additional calculator-specific parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional: Option<HashMap<String, f64>>,
    
    /// Optional project metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_metadata: Option<ProjectMetadata>,
}

/// Project metadata for tracking and documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contractor_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_date: Option<String>,
}

// ============================================================================
// OUTPUT MODELS
// ============================================================================

/// Single contracting result item with uncertainty and criticality
#[derive(Debug, Clone, Serialize)]
pub struct ContractingResultItem {
    pub label: String,
    pub value: f64,
    pub unit: String,
    
    /// Tolerance as fraction (e.g., 0.05 = ±5%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
    
    /// Formatted value string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_value: Option<String>,
    
    /// Indicates if this is a critical result
    pub is_critical: bool,
}

/// Project analysis result
#[derive(Debug, Clone, Serialize)]
pub struct ProjectAnalysisResult {
    pub total_cost: f64,
    pub total_duration: f64,
    pub risk_level: f64,
    pub compliance_score: f64,
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

/// Structured warning with severity and details
#[derive(Debug, Clone, Serialize)]
pub struct ContractingWarning {
    pub severity: WarningSeverity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_parameter: Option<String>,
}

/// Contracting calculation response
#[derive(Debug, Serialize)]
pub struct ContractingCalculationResponse {
    pub calculation_type: String,
    pub results: Vec<ContractingResultItem>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<ProjectAnalysisResult>,
    
    /// Legacy: Vec<String> for backward compatibility
    pub warnings: Vec<String>,
    
    /// New structured warnings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_warnings: Option<Vec<ContractingWarning>>,
    
    pub recommendations: Vec<String>,
    pub compliance_notes: Vec<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_metadata: Option<CalculationMetadata>,
}

#[derive(Debug, Serialize)]
pub struct CalculationMetadata {
    pub timestamp: String,
    pub calculator_version: String,
    pub regulation_code_used: String,
    pub requires_certification_review: bool,
}

// ============================================================================
// METADATA MODELS
// ============================================================================

/// Parameter metadata for API discovery
#[derive(Debug, Clone, Serialize)]
pub struct ParameterMetadata {
    pub name: String,
    pub path: String,              // e.g., "dimensions.area"
    pub data_type: ParameterType,
    pub unit: String,
    pub description: String,
    pub required: bool,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typical_range: Option<(f64, f64)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_rules: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    Number,
    Integer,
    String,
    Boolean,
    Enum(Vec<String>),
    Array,
    Object,
}

/// Calculator metadata for API catalogue
#[derive(Debug, Clone, Serialize)]
pub struct ContractingCalculatorMetadata {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub regulation_codes: Vec<String>,
    
    /// Enhanced parameter metadata with full specifications
    pub parameters: Vec<ParameterMetadata>,
    
    /// Legacy field for backward compatibility
    pub required_parameters: Vec<String>,
    pub optional_parameters: Vec<String>,
    
    pub typical_applications: Vec<String>,
    pub requires_certification_review: bool,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity_level: Option<ComplexityLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_time: Option<String>, // e.g., "< 1s", "1-5s"
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ComplexityLevel {
    Basic,       // Simple formulas, minimal iteration
    Intermediate,// Multiple steps, some iteration
    Advanced,    // Complex algorithms, heavy computation
}

/// Category information for API
#[derive(Debug, Clone, Serialize)]
pub struct ContractingCategoryInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requires_certification: bool,
    pub icon: Option<String>,
}

/// Complete calculator catalogue for API
#[derive(Debug, Serialize)]
pub struct ContractingCalculatorCatalogue {
    pub version: String,
    pub categories: Vec<ContractingCategoryInfo>,
    pub calculators: Vec<ContractingCalculatorMetadata>,
    pub disclaimer: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_index: Option<SearchIndex>,
}

#[derive(Debug, Serialize)]
pub struct SearchIndex {
    pub tags: HashMap<String, Vec<String>>, // tag -> calculator IDs
    pub keywords: HashMap<String, Vec<String>>, // keyword -> calculator IDs
}

// ============================================================================
// REQUEST MODELS
// ============================================================================

/// Request structure for contracting calculations
#[derive(Debug, Deserialize)]
pub struct ContractingCalculationRequest {
    pub calculation_type: String,
    pub parameters: ContractingParameters,
    
    /// Optional: Request specific output format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<OutputFormat>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Standard,
    Detailed,   // Include all intermediate calculations
    Summary,    // Only critical results
}

// ============================================================================
// HELPER IMPLEMENTATIONS
// ============================================================================

impl ContractingCalculatorMetadata {
    /// Builder pattern for metadata construction
    pub fn builder(id: impl Into<String>, name: impl Into<String>) -> MetadataBuilder {
        MetadataBuilder::new(id, name)
    }
}

pub struct MetadataBuilder {
    id: String,
    name: String,
    category: String,
    description: String,
    regulation_codes: Vec<String>,
    parameters: Vec<ParameterMetadata>,
    typical_applications: Vec<String>,
    requires_certification_review: bool,
    complexity_level: Option<ComplexityLevel>,
}

impl MetadataBuilder {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            category: "general".to_string(),
            description: String::new(),
            regulation_codes: Vec::new(),
            parameters: Vec::new(),
            typical_applications: Vec::new(),
            requires_certification_review: false,
            complexity_level: None,
        }
    }
    
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }
    
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
    
    pub fn regulation_code(mut self, code: impl Into<String>) -> Self {
        self.regulation_codes.push(code.into());
        self
    }
    
    pub fn parameter(mut self, param: ParameterMetadata) -> Self {
        self.parameters.push(param);
        self
    }
    
    pub fn requires_certification(mut self) -> Self {
        self.requires_certification_review = true;
        self
    }
    
    pub fn complexity(mut self, level: ComplexityLevel) -> Self {
        self.complexity_level = Some(level);
        self
    }
    
    pub fn build(self) -> ContractingCalculatorMetadata {
        let required_parameters: Vec<String> = self.parameters.iter()
            .filter(|p| p.required)
            .map(|p| p.path.clone())
            .collect();

        let optional_parameters: Vec<String> = self.parameters.iter()
            .filter(|p| !p.required)
            .map(|p| p.path.clone())
            .collect();
        
        ContractingCalculatorMetadata {
            id: self.id,
            name: self.name,
            category: self.category,
            description: self.description,
            regulation_codes: self.regulation_codes,
            parameters: self.parameters,
            required_parameters,
            optional_parameters,
            typical_applications: self.typical_applications,
            requires_certification_review: self.requires_certification_review,
            complexity_level: self.complexity_level,
            calculation_time: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculator_category_serialization() {
        let cat = CalculatorCategory::Bidding;
        assert_eq!(cat.as_str(), "bidding");
        assert_eq!(cat.display_name(), "Bidding");
    }
}