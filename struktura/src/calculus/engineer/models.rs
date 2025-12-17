use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// ENUMS AND CONSTANTS
// ============================================================================

/// Engineering discipline categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculatorCategory {
    Civil,
    Structural,
    Mechanical,
    Production,
    Geotechnical,
    Transportation,
    Hydraulic,
    Environmental,
}

impl CalculatorCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Civil => "civil",
            Self::Structural => "structural",
            Self::Mechanical => "mechanical",
            Self::Production => "production",
            Self::Geotechnical => "geotechnical",
            Self::Transportation => "transportation",
            Self::Hydraulic => "hydraulic",
            Self::Environmental => "environmental",
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Civil => "Civil Engineering",
            Self::Structural => "Structural Engineering",
            Self::Mechanical => "Mechanical Engineering",
            Self::Production => "Production Engineering",
            Self::Geotechnical => "Geotechnical Engineering",
            Self::Transportation => "Transportation Engineering",
            Self::Hydraulic => "Hydraulic Engineering",
            Self::Environmental => "Environmental Engineering",
        }
    }
}

/// Design codes and standards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DesignCode {
    // Structural
    ACI318,
    AISC360,
    ASCE7,
    EurocodeEC2,
    EurocodeEC3,
    
    // Civil/Geotechnical
    AASHTO,
    AASHTOLrfd,
    
    // Mechanical
    ASME,
    ASMEBPVC,
    API610,
    TEMA,
    
    // General
    ISO,
    ASTM,
    
    // Manufacturing
    CEMA,
    LeanManufacturing,
}

impl DesignCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ACI318 => "ACI 318",
            Self::AISC360 => "AISC 360",
            Self::ASCE7 => "ASCE 7",
            Self::EurocodeEC2 => "Eurocode 2",
            Self::EurocodeEC3 => "Eurocode 3",
            Self::AASHTO => "AASHTO",
            Self::AASHTOLrfd => "AASHTO LRFD",
            Self::ASME => "ASME",
            Self::ASMEBPVC => "ASME BPVC",
            Self::API610 => "API 610",
            Self::TEMA => "TEMA",
            Self::ISO => "ISO",
            Self::ASTM => "ASTM",
            Self::CEMA => "CEMA",
            Self::LeanManufacturing => "Lean Manufacturing",
        }
    }
}

// ============================================================================
// INPUT MODELS
// ============================================================================

/// Material properties for engineering calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialProperties {
    pub material_type: String,
    
    // Strength properties (MPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compressive_strength: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tensile_strength: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_strength: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimate_strength: Option<f64>,
    
    // Elastic properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_modulus: Option<f64>,        // GPa
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shear_modulus: Option<f64>,          // GPa
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poisson_ratio: Option<f64>,
    
    // Physical properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,                // kg/m³
    
    // Thermal properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thermal_conductivity: Option<f64>,   // W/(m·K)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thermal_expansion: Option<f64>,      // 1/°C
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specific_heat: Option<f64>,          // J/(kg·K)
}

impl Default for MaterialProperties {
    fn default() -> Self {
        Self {
            material_type: "Steel".to_string(),
            compressive_strength: None,
            tensile_strength: None,
            yield_strength: None,
            ultimate_strength: None,
            elastic_modulus: None,
            shear_modulus: None,
            poisson_ratio: None,
            density: None,
            thermal_conductivity: None,
            thermal_expansion: None,
            specific_heat: None,
        }
    }
}

/// Load case definition for structural analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadCase {
    pub dead_load: f64,        // kN or kN/m
    pub live_load: f64,        // kN or kN/m
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_load: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seismic_load: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snow_load: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_load: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension_load: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shear_load: Option<f64>,
    
    /// Load combination method: "LRFD", "ASD", "Eurocode"
    pub load_combination: String,
}

impl Default for LoadCase {
    fn default() -> Self {
        Self {
            dead_load: 1.0,
            live_load: 1.0,
            wind_load: None,
            seismic_load: None,
            snow_load: None,
            impact_load: None,
            tension_load: None,
            shear_load: None,
            load_combination: "LRFD".to_string(),
        }
    }
}

/// Safety factors for design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyFactors {
    pub dead_load_factor: f64,
    pub live_load_factor: f64,
    pub material_reduction_factor: f64,
    pub importance_factor: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overturning: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<f64>,
}

impl Default for SafetyFactors {
    fn default() -> Self {
        Self {
            dead_load_factor: 1.2,
            live_load_factor: 1.6,
            material_reduction_factor: 0.9,
            importance_factor: 1.0,
            overturning: None,
            bearing: None,
        }
    }
}

/// Comprehensive engineering parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineeringParameters {
    /// Geometric dimensions (meters, radians for angles)
    pub dimensions: HashMap<String, f64>,
    
    /// Material properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<MaterialProperties>,
    
    /// Loading conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loads: Option<LoadCase>,
    
    /// Safety factors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_factors: Option<SafetyFactors>,
    
    /// Design code: "ACI318", "AISC360", "Eurocode", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub design_code: Option<String>,
    
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
    pub engineer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_date: Option<String>,
}

// ============================================================================
// OUTPUT MODELS
// ============================================================================

/// Single engineering result item with uncertainty and criticality
#[derive(Debug, Clone, Serialize)]
pub struct EngineeringResultItem {
    pub label: String,
    pub value: f64,
    pub unit: String,
    
    /// Tolerance as fraction (e.g., 0.05 = ±5%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
    
    /// Whether this result is critical for safety/design
    pub is_critical: bool,
    
    /// Optional formatted display value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_value: Option<String>,
}

impl EngineeringResultItem {
    /// Create a new result with formatted display
    pub fn new(label: impl Into<String>, value: f64, unit: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value,
            unit: unit.into(),
            tolerance: None,
            is_critical: false,
            formatted_value: None,
        }
    }
    
    /// Builder pattern: mark as critical
    pub fn critical(mut self) -> Self {
        self.is_critical = true;
        self
    }
    
    /// Builder pattern: add tolerance
    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = Some(tolerance);
        self
    }
    
    /// Builder pattern: add formatted value
    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.formatted_value = Some(format.into());
        self
    }
}

/// Structural analysis result with detailed metrics
#[derive(Debug, Clone, Serialize)]
pub struct StructuralAnalysisResult {
    pub max_moment: f64,           // kN·m
    pub max_shear: f64,            // kN
    pub max_deflection: f64,       // mm
    pub utilization_ratio: f64,    // Demand/Capacity
    pub governing_limit_state: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stress_distribution: Option<Vec<StressPoint>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StressPoint {
    pub location: f64,       // Position along member
    pub stress: f64,         // MPa
    pub stress_type: String, // "tension", "compression", "shear"
}

/// Warning severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum WarningSeverity {
    Critical,    // Safety violation, cannot proceed
    High,        // Significant issue, strong recommendation
    Medium,      // Should address, but not critical
    Low,         // Informational, best practice
}

/// Structured warning with severity
#[derive(Debug, Clone, Serialize)]
pub struct EngineeringWarning {
    pub severity: WarningSeverity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_parameter: Option<String>,
}

impl EngineeringWarning {
    pub fn critical(message: impl Into<String>) -> Self {
        Self {
            severity: WarningSeverity::Critical,
            message: message.into(),
            affected_parameter: None,
        }
    }
    
    pub fn high(message: impl Into<String>) -> Self {
        Self {
            severity: WarningSeverity::High,
            message: message.into(),
            affected_parameter: None,
        }
    }
    
    pub fn with_parameter(mut self, param: impl Into<String>) -> Self {
        self.affected_parameter = Some(param.into());
        self
    }
}

/// Engineering calculation response
#[derive(Debug, Serialize)]
pub struct EngineeringCalculationResponse {
    pub calculation_type: String,
    pub results: Vec<EngineeringResultItem>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<StructuralAnalysisResult>,
    
    /// Legacy: Vec<String> for backward compatibility
    pub warnings: Vec<String>,
    
    /// New structured warnings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_warnings: Option<Vec<EngineeringWarning>>,
    
    pub recommendations: Vec<String>,
    pub compliance_notes: Vec<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_metadata: Option<CalculationMetadata>,
}

#[derive(Debug, Serialize)]
pub struct CalculationMetadata {
    pub timestamp: String,
    pub calculator_version: String,
    pub design_code_used: String,
    pub requires_pe_review: bool,
}

// ============================================================================
// METADATA MODELS
// ============================================================================

/// Parameter metadata for API discovery
#[derive(Debug, Clone, Serialize)]
pub struct ParameterMetadata {
    pub name: String,
    pub path: String,              // e.g., "dimensions.height"
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
pub struct EngineeringCalculatorMetadata {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub design_codes: Vec<String>,
    
    /// Enhanced parameter metadata with full specifications
    pub parameters: Vec<ParameterMetadata>,
    
    /// Legacy field for backward compatibility
    pub required_parameters: Vec<String>,
    pub optional_parameters: Vec<String>,
    
    pub typical_applications: Vec<String>,
    pub requires_pe_review: bool,
    
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
pub struct EngineeringCategoryInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requires_pe: bool,
    pub icon: Option<String>,
}

/// Complete calculator catalogue for API
#[derive(Debug, Serialize)]
pub struct EngineeringCalculatorCatalogue {
    pub version: String,
    pub categories: Vec<EngineeringCategoryInfo>,
    pub calculators: Vec<EngineeringCalculatorMetadata>,
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

/// Request structure for engineering calculations
#[derive(Debug, Deserialize)]
pub struct EngineeringCalculationRequest {
    pub calculation_type: String,
    pub parameters: EngineeringParameters,
    
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

impl EngineeringCalculatorMetadata {
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
    design_codes: Vec<String>,
    parameters: Vec<ParameterMetadata>,
    typical_applications: Vec<String>,
    requires_pe_review: bool,
    complexity_level: Option<ComplexityLevel>,
}

impl MetadataBuilder {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            category: "general".to_string(),
            description: String::new(),
            design_codes: Vec::new(),
            parameters: Vec::new(),
            typical_applications: Vec::new(),
            requires_pe_review: false,
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
    
    pub fn design_code(mut self, code: impl Into<String>) -> Self {
        self.design_codes.push(code.into());
        self
    }
    
    pub fn parameter(mut self, param: ParameterMetadata) -> Self {
        self.parameters.push(param);
        self
    }
    
    pub fn requires_pe(mut self) -> Self {
        self.requires_pe_review = true;
        self
    }
    
    pub fn complexity(mut self, level: ComplexityLevel) -> Self {
        self.complexity_level = Some(level);
        self
    }
    
    pub fn build(self) -> EngineeringCalculatorMetadata {
        //let (required, optional): (Vec<_>, Vec<_>) = self.parameters.iter()
        //    .partition(|p| p.required);

        let required_parameters: Vec<String> = self.parameters.iter()
            .filter(|p| p.required)
            .map(|p| p.path.clone())
            .collect();

        let optional_parameters: Vec<String> = self.parameters.iter()
            .filter(|p| !p.required)
            .map(|p| p.path.clone())
            .collect();
        
        EngineeringCalculatorMetadata {
            id: self.id,
            name: self.name,
            category: self.category,
            description: self.description,
            design_codes: self.design_codes,
            parameters: self.parameters,
            required_parameters,
            optional_parameters,
            typical_applications: self.typical_applications,
            requires_pe_review: self.requires_pe_review,
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
        let cat = CalculatorCategory::Civil;
        assert_eq!(cat.as_str(), "civil");
        assert_eq!(cat.display_name(), "Civil Engineering");
    }
    
    #[test]
    fn test_result_item_builder() {
        let result = EngineeringResultItem::new("Test", 42.0, "m")
            .critical()
            .with_tolerance(0.05)
            .with_format("42.00 m");
        
        assert!(result.is_critical);
        assert_eq!(result.tolerance, Some(0.05));
        assert_eq!(result.formatted_value, Some("42.00 m".to_string()));
    }
}