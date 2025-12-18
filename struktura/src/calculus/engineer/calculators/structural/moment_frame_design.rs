use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::steel_properties::*;

pub struct MomentFrameDesignCalculator;

impl ParameterValidator for MomentFrameDesignCalculator {
    fn calculator_id(&self) -> &str {
        "moment_frame_design"
    }
}

#[async_trait]
impl EngineerCalculator for MomentFrameDesignCalculator {
    fn id(&self) -> &str {
        "moment_frame_design"
    }

    fn name(&self) -> &str {
        "Moment Frame Design"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Structural
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("moment_frame_design", "Moment Frame Design")
            .category("structural")
            .description("Design steel moment frame for lateral loads per AISC 341 seismic provisions")
            .design_code("AISC 341")
            .design_code("ASCE 7")
            .parameter(ParameterMetadata {
                name: "Story Height".to_string(),
                path: "dimensions.height".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Typical story height".to_string(),
                required: true,
                default_value: Some(4.0),
                min_value: Some(2.5),
                max_value: Some(6.0),
                typical_range: Some((3.0, 4.5)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Number of Stories".to_string(),
                path: "additional.num_stories".to_string(),
                data_type: ParameterType::Integer,
                unit: "".to_string(),
                description: "Building stories".to_string(),
                required: true,
                default_value: Some(5.0),
                min_value: Some(1.0),
                max_value: Some(20.0),
                typical_range: Some((3.0, 10.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Seismic Load".to_string(),
                path: "loads.seismic_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kN".to_string(),
                description: "Base shear".to_string(),
                required: true,
                default_value: Some(1000.0),
                min_value: Some(100.0),
                max_value: Some(10000.0),
                typical_range: Some((500.0, 2000.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Bay Width".to_string(),
                path: "dimensions.width".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Frame bay width".to_string(),
                required: true,
                default_value: Some(6.0),
                min_value: Some(4.0),
                max_value: Some(12.0),
                typical_range: Some((5.0, 8.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Advanced)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        self.validate_dimension("height", params.dimensions.get("height").copied(), 2.5, 6.0)?;
        self.get_additional_param(params, "num_stories", Some(1.0), Some(20.0))?;
        if let Some(loads) = &params.loads {
            if loads.seismic_load < Some(100.0) {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "seismic_load".to_string(),
                    value: loads.seismic_load.expect("No given seismic load, defaulting").to_string(),
                    reason: "Low seismic load".to_string(),
                });
            }
        }
        self.validate_dimension("width", params.dimensions.get("width").copied(), 4.0, 12.0)?;

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let story_h = params.dimensions.get("height").copied().unwrap_or(4.0);
        let num_stories = params.additional.as_ref().and_then(|a| a.get("num_stories").copied()).unwrap_or(5.0);
        let v_base = params.loads.as_ref().map(|l| l.seismic_load).unwrap_or(Some(1000.0));
        let bay_w = params.dimensions.get("width").copied().unwrap_or(6.0);

        let story_shear = v_base.expect("No v_base given, defaulting") / num_stories; // Approximate uniform
        let moment_beam = story_shear * story_h / 2.0; // Fixed ends approx
        let drift_est = v_base.expect("No v_base given, defaulting") * (num_stories * story_h).powi(3) / (12.0 * E_STEEL * 1e6 * (bay_w / 10.0).powi(4)); // Approximate

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if drift_est > story_h / 400.0 {
            warnings.push(format!("Excessive drift ({:.1} mm). Stiffen frame.", drift_est * 1000.0));
            recommendations.push("Increase member sizes or add bracing".to_string());
        }

        compliance_notes.push("Preliminary sizing per AISC 341".to_string());
        compliance_notes.push("Perform P-delta analysis".to_string());
        compliance_notes.push("Design connections for ductility".to_string());

        let results = vec![
            EngineeringResultItem::new("Story Shear", story_shear, "kN")
                .with_format(format!("{:.1} kN", story_shear)),
            EngineeringResultItem::new("Beam Moment", moment_beam, "kNm")
                .critical()
                .with_format(format!("{:.1} kNm", moment_beam)),
            EngineeringResultItem::new("Estimated Drift", drift_est * 1000.0, "mm")
                .with_format(format!("{:.1} mm", drift_est * 1000.0)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "moment_frame_design".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "AISC 341".to_string(),
                requires_pe_review: true,
            }),
        })
    }
}

