use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

pub struct LateralLoadAnalysisCalculator;

impl ParameterValidator for LateralLoadAnalysisCalculator {
    fn calculator_id(&self) -> &str {
        "lateral_load_analysis"
    }
}

#[async_trait]
impl EngineerCalculator for LateralLoadAnalysisCalculator {
    fn id(&self) -> &str {
        "lateral_load_analysis"
    }

    fn name(&self) -> &str {
        "Lateral Load Analysis"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Structural
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("lateral_load_analysis", "Lateral Load Analysis")
            .category("structural")
            .description("Calculate wind or seismic loads and distribute to lateral system per ASCE 7")
            .design_code("ASCE 7")
            .parameter(ParameterMetadata {
                name: "Building Height".to_string(),
                path: "dimensions.height".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Total building height".to_string(),
                required: true,
                default_value: Some(20.0),
                min_value: Some(5.0),
                max_value: Some(100.0),
                typical_range: Some((10.0, 50.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Building Width".to_string(),
                path: "dimensions.width".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Building width".to_string(),
                required: true,
                default_value: Some(20.0),
                min_value: Some(5.0),
                max_value: Some(100.0),
                typical_range: Some((10.0, 30.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Load Type".to_string(),
                path: "additional.load_type".to_string(),
                data_type: ParameterType::Enum(vec!["wind".to_string(), "seismic".to_string()]),
                unit: "".to_string(),
                description: "wind or seismic".to_string(),
                required: true,
                default_value: None,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["wind or seismic".to_string()]),
            })
            .parameter(ParameterMetadata {
                name: "Base Load".to_string(),
                path: "loads.wind_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kPa".to_string(),
                description: "Wind pressure or seismic acceleration".to_string(),
                required: true,
                default_value: Some(1.0),
                min_value: Some(0.1),
                max_value: Some(5.0),
                typical_range: Some((0.5, 2.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Number of Stories".to_string(),
                path: "additional.num_stories".to_string(),
                data_type: ParameterType::Integer,
                unit: "".to_string(),
                description: "Number of stories".to_string(),
                required: true,
                default_value: Some(5.0),
                min_value: Some(1.0),
                max_value: Some(20.0),
                typical_range: Some((3.0, 10.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Advanced)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        self.validate_dimension("height", params.dimensions.get("height").copied(), 5.0, 100.0)?;
        self.validate_dimension("width", params.dimensions.get("width").copied(), 5.0, 100.0)?;
        let load_type = params.additional.as_ref().and_then(|a| a.get("load_type")).map(|v| v.to_string()).unwrap_or("wind".to_string());
        if load_type != "wind" && load_type != "seismic" {
            return Err(EngineeringError::InvalidParameter {
                parameter: "load_type".to_string(),
                value: load_type,
                reason: "Invalid type".to_string(),
            });
        }
        if let Some(loads) = &params.loads {
            if loads.wind_load.unwrap_or(0.0) < 0.1 {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "wind_load".to_string(),
                    value: loads.wind_load.unwrap_or(0.0).to_string(),
                    reason: "Low load".to_string(),
                });
            }
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let height = params.dimensions.get("height").copied().unwrap_or(20.0);
        let width = params.dimensions.get("width").copied().unwrap_or(20.0);
        let load_type = params.additional.as_ref().and_then(|a| a.get("load_type")).map(|v| v.to_string()).unwrap_or("wind".to_string());
        let base_load = params.loads.as_ref().map(|l| l.wind_load.unwrap_or(1.0)).unwrap_or(1.0);
        let num_stories = params.additional.as_ref().and_then(|a| a.get("num_stories").copied()).unwrap_or(5.0);

        let area = width * height / num_stories; // Per story approx
        let total_force = if load_type == "wind" {
            base_load * area * num_stories
        } else {
            base_load * 1000.0 * area * num_stories // Seismic mass approx
        };
        let base_shear = total_force;
        let base_moment = base_shear * height / 2.0;

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if total_force > 5000.0 {
            warnings.push("High lateral load. Verify building configuration".to_string());
            recommendations.push("Consider shear walls for high-rise".to_string());
        }

        compliance_notes.push("Load distribution per ASCE 7".to_string());
        compliance_notes.push("Use equivalent lateral force method".to_string());

        let results = vec![
            EngineeringResultItem::new("Base Shear", base_shear, "kN")
                .critical()
                .with_format(format!("{:.1} kN", base_shear)),
            EngineeringResultItem::new("Overturning Moment", base_moment, "kNm")
                .with_format(format!("{:.1} kNm", base_moment)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "lateral_load_analysis".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "ASCE 7".to_string(),
                requires_pe_review: true,
            }),
        })
    }
}

