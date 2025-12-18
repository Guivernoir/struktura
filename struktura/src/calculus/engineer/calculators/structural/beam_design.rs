use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::steel_properties::*;
use super::resistance_factors::*;
use super::load_factors::*;
use super::deflection_limits::*;
use super::helpers::*;

pub struct BeamDesignCalculator;

impl ParameterValidator for BeamDesignCalculator {
    fn calculator_id(&self) -> &str {
        "beam_design"
    }
}

#[async_trait]
impl EngineerCalculator for BeamDesignCalculator {
    fn id(&self) -> &str {
        "beam_design"
    }

    fn name(&self) -> &str {
        "Steel Beam Design"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Structural
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("beam_design", "Steel Beam Design")
            .category("structural")
            .description("Design steel beam for flexure, shear, and deflection per AISC 360")
            .design_code("AISC 360")
            .design_code("ASCE 7")
            .parameter(ParameterMetadata {
                name: "Span Length".to_string(),
                path: "dimensions.length".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Beam span length".to_string(),
                required: true,
                default_value: Some(6.0),
                min_value: Some(1.0),
                max_value: Some(20.0),
                typical_range: Some((3.0, 10.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Dead Load".to_string(),
                path: "loads.dead_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kN/m".to_string(),
                description: "Uniform dead load".to_string(),
                required: true,
                default_value: Some(10.0),
                min_value: Some(1.0),
                max_value: Some(50.0),
                typical_range: Some((5.0, 20.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Live Load".to_string(),
                path: "loads.live_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kN/m".to_string(),
                description: "Uniform live load".to_string(),
                required: true,
                default_value: Some(15.0),
                min_value: Some(1.0),
                max_value: Some(50.0),
                typical_range: Some((5.0, 25.0)),
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
                name: "Support Condition".to_string(),
                path: "additional.support_condition".to_string(),
                data_type: ParameterType::Enum(vec!["simple".to_string(), "continuous".to_string()]),
                unit: "".to_string(),
                description: "simple or continuous".to_string(),
                required: false,
                default_value: None,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["simple or continuous".to_string()]),
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let span = self.validate_dimension("length", params.dimensions.get("length").copied(), 1.0, 20.0)?;
        if let Some(loads) = &params.loads {
            if loads.dead_load < 1.0 || loads.live_load < 1.0 {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "loads".to_string(),
                    value: format!("dead: {}, live: {}", loads.dead_load, loads.live_load),
                    reason: "Loads too low".to_string(),
                });
            }
        }

        if let Some(material) = &params.material {
            if let Some(fy) = material.yield_strength {
                if fy < 200.0 {
                    return Err(EngineeringError::InvalidParameter {
                        parameter: "yield_strength".to_string(),
                        value: fy.to_string(),
                        reason: "Below typical steel grades".to_string(),
                    });
                }
            }
        }

        let support = params.additional.as_ref().and_then(|a| a.get("support_condition")).map(|v| v.to_string()).unwrap_or("simple".to_string());
        if support != "simple" && support != "continuous" {
            return Err(EngineeringError::InvalidParameter {
                parameter: "support_condition".to_string(),
                value: support,
                reason: "Invalid support type".to_string(),
            });
        }

        if span > 12.0 {
            return Err(EngineeringError::DomainError {
                field: "span".to_string(),
                message: "Long span - consider truss alternative".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let span = params.dimensions.get("length").copied().unwrap_or(6.0);
        let dead = params.loads.as_ref().map(|l| l.dead_load).unwrap_or(10.0);
        let live = params.loads.as_ref().map(|l| l.live_load).unwrap_or(15.0);
        let fy = params.material.as_ref().and_then(|m| m.yield_strength).unwrap_or(FY_A992);
        let support = params.additional.as_ref().and_then(|a| a.get("support_condition")).map(|v| v.to_string()).unwrap_or("simple".to_string());

        let wu = factored_load_basic(dead, live);
        let mu = if support == "simple" {
            wu * span.powi(2) / 8.0
        } else {
            wu * span.powi(2) / 12.0 // Approximate for continuous
        };

        let req_section_mod = mu * 1000.0 / (PHI_FLEXURE * fy); // cm³
        let shear_max = wu * span / 2.0;
        let def_live = 5.0 * live * 1000.0 * (span * 100.0).powi(4) / (384.0 * E_STEEL * 1e9 * req_section_mod / 100.0); // mm, approximate

        let (passes_def, util_def) = check_deflection(def_live, span, L_OVER_360);

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if !passes_def {
            warnings.push(format!("Excessive deflection. Utilization: {:.2}", util_def));
            recommendations.push("Increase section size or use camber".to_string());
        }

        if shear_max > 1000.0 {
            warnings.push("High shear - check web thickness".to_string());
        }

        compliance_notes.push("Design per AISC 360 LRFD".to_string());
        compliance_notes.push("Verify lateral torsional buckling".to_string());
        compliance_notes.push("Check serviceability for vibrations if applicable".to_string());

        let results = vec![
            EngineeringResultItem::new("Factored Moment", mu, "kNm")
                .critical()
                .with_format(format!("{:.1} kNm", mu)),
            EngineeringResultItem::new("Required Sx", req_section_mod, "cm³")
                .critical()
                .with_format(format!("{:.0} cm³", req_section_mod)),
            EngineeringResultItem::new("Max Shear", shear_max, "kN")
                .with_format(format!("{:.1} kN", shear_max)),
            EngineeringResultItem::new("Live Deflection", def_live, "mm")
                .with_format(format!("{:.1} mm", def_live)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "beam_design".to_string(),
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

