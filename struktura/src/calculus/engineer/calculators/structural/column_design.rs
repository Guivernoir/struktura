use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::steel_properties::*;
use super::resistance_factors::*;
use super::load_factors::*;
use super::helpers::*;

pub struct ColumnDesignCalculator;

impl ParameterValidator for ColumnDesignCalculator {
    fn calculator_id(&self) -> &str {
        "column_design"
    }
}

#[async_trait]
impl EngineerCalculator for ColumnDesignCalculator {
    fn id(&self) -> &str {
        "column_design"
    }

    fn name(&self) -> &str {
        "Steel Column Design"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Structural
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("column_design", "Steel Column Design")
            .category("structural")
            .description("Design steel column for axial compression and buckling per AISC 360")
            .design_code("AISC 360")
            .design_code("ASCE 7")
            .parameter(ParameterMetadata {
                name: "Height".to_string(),
                path: "dimensions.height".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Column unsupported height".to_string(),
                required: true,
                default_value: Some(4.0),
                min_value: Some(1.0),
                max_value: Some(15.0),
                typical_range: Some((3.0, 6.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Axial Dead Load".to_string(),
                path: "loads.dead_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kN".to_string(),
                description: "Axial dead load".to_string(),
                required: true,
                default_value: Some(500.0),
                min_value: Some(100.0),
                max_value: Some(5000.0),
                typical_range: Some((200.0, 1000.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Axial Live Load".to_string(),
                path: "loads.live_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kN".to_string(),
                description: "Axial live load".to_string(),
                required: true,
                default_value: Some(300.0),
                min_value: Some(50.0),
                max_value: Some(3000.0),
                typical_range: Some((100.0, 500.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Steel Grade".to_string(),
                path: "material.yield_strength".to_string(),
                data_type: ParameterType::Number,
                unit: "MPa".to_string(),
                description: "Steel yield strength (Fy)".to_string(),
                required: false,
                default_value: Some(FY_A992),
                min_value: Some(200.0),
                max_value: Some(500.0),
                typical_range: Some((250.0, 345.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Effective Length Factor".to_string(),
                path: "additional.k_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "dimensionless".to_string(),
                description: "K factor for buckling (1.0 pinned-pinned)".to_string(),
                required: false,
                default_value: Some(1.0),
                min_value: Some(0.5),
                max_value: Some(2.0),
                typical_range: Some((0.65, 1.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let height = self.validate_dimension("height", params.dimensions.get("height").copied(), 1.0, 15.0)?;
        if let Some(loads) = &params.loads {
            if loads.dead_load < 100.0 {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "dead_load".to_string(),
                    value: loads.dead_load.to_string(),
                    reason: "Low axial load".to_string(),
                });
            }
        }

        self.get_additional_param(params, "k_factor", Some(0.5), Some(2.0))?;

        if height > 10.0 {
            return Err(EngineeringError::DomainError {
                field: "height".to_string(),
                message: "Tall column - consider bracing".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let height = params.dimensions.get("height").copied().unwrap_or(4.0);
        let dead = params.loads.as_ref().map(|l| l.dead_load).unwrap_or(500.0);
        let live = params.loads.as_ref().map(|l| l.live_load).unwrap_or(300.0);
        let fy = params.material.as_ref().and_then(|m| m.yield_strength).unwrap_or(FY_A992);
        let k = params.additional.as_ref().and_then(|a| a.get("k_factor").copied()).unwrap_or(1.0);

        let pu = 1.2 * dead + 1.6 * live;
        let lambda = k * height * 1000.0 / (fy.sqrt() * 10.0); // Approximate r from lambda
        let phi_pn = if lambda < 1.5 {
            PHI_COMPRESSION * 0.658f64.powf(lambda.powi(2)) * fy * (pu / fy) // Inelastic
        } else {
            PHI_COMPRESSION * 0.877 / lambda.powi(2) * fy * (pu / fy) // Elastic
        };

        let req_area = pu / phi_pn;

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if k > 1.0 {
            warnings.push("Fixed ends assumed. Verify alignment chart K".to_string());
        }

        if req_area > 5000.0 {
            warnings.push("Large section required. Consider HSS or built-up".to_string());
        }

        compliance_notes.push("Design per AISC 360 LRFD Chapter E".to_string());
        compliance_notes.push("Assume no slenderness in other axis".to_string());
        compliance_notes.push("Check P-delta if applicable".to_string());

        let results = vec![
            EngineeringResultItem::new("Factored Axial", pu, "kN")
                .critical()
                .with_format(format!("{:.0} kN", pu)),
            EngineeringResultItem::new("Required Area", req_area, "mm²")
                .critical()
                .with_format(format!("{:.0} mm²", req_area)),
            EngineeringResultItem::new("Slenderness Ratio", lambda * 100.0, "dimensionless") // Approximate
                .with_format(format!("{:.1}", lambda * 100.0)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "column_design".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "AISC 360".to_string(),
                requires_pe_review: true,
            }),
        })
    }
}

