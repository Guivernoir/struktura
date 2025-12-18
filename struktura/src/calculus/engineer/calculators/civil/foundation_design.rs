use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::soil_properties::*;

pub struct FoundationDesignCalculator;

impl ParameterValidator for FoundationDesignCalculator {
    fn calculator_id(&self) -> &str {
        "foundation_design"
    }
}

#[async_trait]
impl EngineerCalculator for FoundationDesignCalculator {
    fn id(&self) -> &str {
        "foundation_design"
    }

    fn name(&self) -> &str {
        "Shallow Foundation Design"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Civil
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("foundation_design", "Shallow Foundation Design")
            .category("civil")
            .description("Design isolated footing for bearing capacity and settlement per Terzaghi method")
            .design_code("ACI 318")
            .design_code("AASHTO LRFD")
            .parameter(ParameterMetadata {
                name: "Column Load".to_string(),
                path: "loads.dead_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kN".to_string(),
                description: "Total service load on foundation".to_string(),
                required: true,
                default_value: Some(1000.0),
                min_value: Some(100.0),
                max_value: Some(10000.0),
                typical_range: Some((500.0, 2000.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Soil Bearing Capacity".to_string(),
                path: "additional.bearing_capacity".to_string(),
                data_type: ParameterType::Number,
                unit: "kPa".to_string(),
                description: "Allowable soil bearing capacity".to_string(),
                required: true,
                default_value: Some(150.0),
                min_value: Some(50.0),
                max_value: Some(500.0),
                typical_range: Some((100.0, 300.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Footing Depth".to_string(),
                path: "dimensions.depth".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Embedment depth".to_string(),
                required: false,
                default_value: Some(1.5),
                min_value: Some(0.5),
                max_value: Some(3.0),
                typical_range: Some((1.0, 2.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Safety Factor Bearing".to_string(),
                path: "safety_factors.bearing".to_string(),
                data_type: ParameterType::Number,
                unit: "dimensionless".to_string(),
                description: "FOS for bearing capacity".to_string(),
                required: false,
                default_value: Some(3.0),
                min_value: Some(2.0),
                max_value: Some(4.0),
                typical_range: Some((2.5, 3.5)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        if let Some(loads) = &params.loads {
            if loads.dead_load < 100.0 {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "dead_load".to_string(),
                    value: loads.dead_load.to_string(),
                    reason: "Low load for isolated footing".to_string(),
                });
            }
        }

        self.get_additional_param(params, "bearing_capacity", Some(50.0), Some(500.0))?;
        self.validate_dimension("depth", params.dimensions.get("depth").copied(), 0.5, 3.0)?;

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let load = params.loads.as_ref().map(|l| l.dead_load).unwrap_or(1000.0);
        let q_all = self.get_additional_param(&params, "bearing_capacity", None, None)?;
        let depth = params.dimensions.get("depth").copied().unwrap_or(1.5);
        let fos = params.safety_factors.as_ref().and_then(|s| s.bearing).unwrap_or(3.0);

        let area_req = load / q_all;
        let size = area_req.sqrt();
        let settlement_est = load / (q_all * fos) * 25.4; // mm, heuristic

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if size > 3.0 {
            warnings.push(format!("Large footing ({:.1}m). Consider mat foundation.", size));
            recommendations.push("Verify eccentric loading if applicable".to_string());
        }

        if depth < 1.0 {
            warnings.push("Shallow embedment. Frost protection may be required.".to_string());
        }

        compliance_notes.push("Bearing capacity per Terzaghi equation".to_string());
        compliance_notes.push("Settlement estimate approximate - perform detailed analysis".to_string());
        compliance_notes.push("Design reinforcement per ACI 318".to_string());

        let results = vec![
            EngineeringResultItem::new("Required Area", area_req, "m²")
                .critical()
                .with_format(format!("{:.1} m²", area_req)),
            EngineeringResultItem::new("Footing Size (square)", size, "m")
                .critical()
                .with_format(format!("{:.2} m", size)),
            EngineeringResultItem::new("Estimated Settlement", settlement_est, "mm")
                .with_format(format!("{:.1} mm", settlement_est)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "foundation_design".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "ACI 318".to_string(),
                requires_pe_review: true,
            }),
        })
    }
}

