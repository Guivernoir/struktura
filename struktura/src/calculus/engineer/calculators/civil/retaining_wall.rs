use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::soil_properties::*;
use super::earth_pressure::*;
use super::concrete_properties::*;

pub struct RetainingWallCalculator;

impl ParameterValidator for RetainingWallCalculator {
    fn calculator_id(&self) -> &str {
        "retaining_wall"
    }
}

#[async_trait]
impl EngineerCalculator for RetainingWallCalculator {
    fn id(&self) -> &str {
        "retaining_wall"
    }

    fn name(&self) -> &str {
        "Retaining Wall Design"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Civil
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("retaining_wall", "Retaining Wall Design")
            .category("civil")
            .description("Design cantilever retaining wall for stability and strength per ACI 318 and geotechnical standards")
            .design_code("ACI 318")
            .design_code("AASHTO LRFD")
            .parameter(ParameterMetadata {
                name: "Wall Height".to_string(),
                path: "dimensions.height".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Total height of retaining wall".to_string(),
                required: true,
                default_value: Some(4.0),
                min_value: Some(1.0),
                max_value: Some(10.0),
                typical_range: Some((2.0, 6.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Soil Friction Angle".to_string(),
                path: "additional.friction_angle".to_string(),
                data_type: ParameterType::Number,
                unit: "degrees".to_string(),
                description: "Internal friction angle of backfill soil".to_string(),
                required: true,
                default_value: Some(30.0),
                min_value: Some(20.0),
                max_value: Some(40.0),
                typical_range: Some((25.0, 35.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Soil Unit Weight".to_string(),
                path: "additional.soil_unit_weight".to_string(),
                data_type: ParameterType::Number,
                unit: "kN/m³".to_string(),
                description: "Unit weight of backfill soil".to_string(),
                required: false,
                default_value: Some(UNIT_WEIGHT_SANDY),
                min_value: Some(15.0),
                max_value: Some(22.0),
                typical_range: Some((16.0, 20.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Surcharge Load".to_string(),
                path: "loads.live_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kPa".to_string(),
                description: "Uniform surcharge on backfill".to_string(),
                required: false,
                default_value: Some(0.0),
                min_value: Some(0.0),
                max_value: Some(20.0),
                typical_range: Some((0.0, 5.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Concrete Strength".to_string(),
                path: "material.compressive_strength".to_string(),
                data_type: ParameterType::Number,
                unit: "MPa".to_string(),
                description: "Concrete compressive strength (f'c)".to_string(),
                required: false,
                default_value: Some(30.0),
                min_value: Some(20.0),
                max_value: Some(50.0),
                typical_range: Some((25.0, 35.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Safety Factor Overturning".to_string(),
                path: "safety_factors.overturning".to_string(),
                data_type: ParameterType::Number,
                unit: "dimensionless".to_string(),
                description: "Required FOS against overturning".to_string(),
                required: false,
                default_value: Some(2.0),
                min_value: Some(1.5),
                max_value: Some(3.0),
                typical_range: Some((1.5, 2.5)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Advanced)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let height = self.validate_dimension("height", params.dimensions.get("height").copied(), 1.0, 10.0)?;
        self.get_additional_param(params, "friction_angle", Some(20.0), Some(40.0))?;
        self.get_additional_param(params, "soil_unit_weight", Some(15.0), Some(22.0))?;

        if let Some(loads) = &params.loads {
            if loads.live_load > 20.0 {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "live_load".to_string(),
                    value: loads.live_load.to_string(),
                    reason: "Excessive surcharge".to_string(),
                });
            }
        }

        if let Some(material) = &params.material {
            if let Some(fc) = material.compressive_strength {
                if fc < 20.0 {
                    return Err(EngineeringError::InvalidParameter {
                        parameter: "compressive_strength".to_string(),
                        value: fc.to_string(),
                        reason: "Below minimum for structural concrete".to_string(),
                    });
                }
            }
        }

        if height > 6.0 {
            return Err(EngineeringError::DomainError {
                field: "height".to_string(),
                message: "Tall wall - consider alternative design".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let height = params.dimensions.get("height").copied().unwrap_or(4.0);
        let phi = self.get_additional_param(&params, "friction_angle", None, None)?;
        let gamma = params.additional.as_ref().and_then(|a| a.get("soil_unit_weight").copied()).unwrap_or(UNIT_WEIGHT_SANDY);
        let surcharge = params.loads.as_ref().map(|l| l.live_load).unwrap_or(0.0);
        let fc = params.material.as_ref().and_then(|m| m.compressive_strength).unwrap_or(30.0);
        let fos_ot = params.safety_factors.as_ref().and_then(|s| s.overturning).unwrap_or(2.0);

        let ka = rankine_active(phi);
        let pa = 0.5 * ka * gamma * height.powi(2) + ka * surcharge * height;
        let overturn_moment = pa * (height / 3.0);
        let base_width_min = (overturn_moment * fos_ot / (0.5 * UNIT_WEIGHT_CONCRETE * height.powi(2))).sqrt();

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if base_width_min > height / 2.0 {
            warnings.push(format!("Wide base required ({:.2}m). Optimize design.", base_width_min));
            recommendations.push("Consider gravity wall or soil improvement".to_string());
        }

        compliance_notes.push("Design per ACI 318 for concrete sections".to_string());
        compliance_notes.push("Requires geotechnical verification".to_string());
        compliance_notes.push("Include drainage and waterproofing".to_string());

        let results = vec![
            EngineeringResultItem::new("Active Pressure", pa, "kN/m")
                .critical()
                .with_format(format!("{:.1} kN/m", pa)),
            EngineeringResultItem::new("Min Base Width", base_width_min, "m")
                .critical()
                .with_format(format!("{:.2} m", base_width_min)),
            EngineeringResultItem::new("Ka", ka, "dimensionless")
                .with_format(format!("{:.2}", ka)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "retaining_wall".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::engineer::test_utils::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_retaining_wall_basic() {
        let calc = RetainingWallCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("height", 4.0),
        ]);
        
        let mut additional = HashMap::new();
        additional.insert("friction_angle".to_string(), 30.0);
        additional.insert("soil_unit_weight".to_string(), 18.0);
        params.additional = Some(additional);
        params.loads = Some(LoadCase {
            live_load: 0.0,
            ..Default::default()
        });
        params.material = Some(MaterialProperties {
            compressive_strength: Some(30.0),
            ..Default::default()
        });
        params.safety_factors = Some(SafetyFactors {
            overturning: Some(2.0),
            ..Default::default()
        });

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() >= 3);
    }

    #[test]
    fn test_tall_wall() {
        let calc = RetainingWallCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("height", 8.0), // Tall
        ]);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}