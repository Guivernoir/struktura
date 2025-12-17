use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

pub struct ThermalExpansionCalculator;

impl ParameterValidator for ThermalExpansionCalculator {
    fn calculator_id(&self) -> &str {
        "thermal_expansion"
    }
}

#[async_trait]
impl EngineerCalculator for ThermalExpansionCalculator {
    fn id(&self) -> &str {
        "thermal_expansion"
    }

    fn name(&self) -> &str {
        "Thermal Expansion Calculation"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Mechanical
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("thermal_expansion", "Thermal Expansion Calculation")
            .category("mechanical")
            .description("Calculate linear, volumetric, or area thermal expansion for materials")
            .design_code("ASTM E228")
            .parameter(ParameterMetadata {
                name: "Initial Length".to_string(),
                path: "dimensions.length".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Initial dimension".to_string(),
                required: true,
                default_value: Some(1.0),
                min_value: Some(0.001),
                max_value: Some(1000.0),
                typical_range: Some((0.1, 10.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Temperature Change".to_string(),
                path: "additional.delta_t".to_string(),
                data_type: ParameterType::Number,
                unit: "°C".to_string(),
                description: "Temperature change".to_string(),
                required: true,
                default_value: Some(100.0),
                min_value: Some(-500.0),
                max_value: Some(500.0),
                typical_range: Some((10.0, 200.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Expansion Coefficient".to_string(),
                path: "material.thermal_expansion".to_string(),
                data_type: ParameterType::Number,
                unit: "1/°C".to_string(),
                description: "Linear thermal expansion coefficient".to_string(),
                required: true,
                default_value: Some(12e-6), // Steel
                min_value: Some(1e-6),
                max_value: Some(50e-6),
                typical_range: Some((5e-6, 25e-6)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Expansion Type".to_string(),
                path: "additional.expansion_type".to_string(),
                data_type: ParameterType::Enum(vec!["linear".to_string(), "area".to_string(), "volume".to_string()]),
                unit: "".to_string(),
                description: "linear, area, or volume".to_string(),
                required: false,
                default_value: None,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["linear, area, volume".to_string()]),
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        self.validate_dimension("length", params.dimensions.get("length").copied(), 0.001, 1000.0)?;
        self.get_additional_param(params, "delta_t", Some(-500.0), Some(500.0))?;

        if let Some(material) = &params.material {
            if let Some(alpha) = material.thermal_expansion {
                if alpha < 1e-6 || alpha > 50e-6 {
                    return Err(EngineeringError::InvalidParameter {
                        parameter: "thermal_expansion".to_string(),
                        value: alpha.to_string(),
                        reason: "Unusual coefficient value".to_string(),
                    });
                }
            } else {
                return Err(EngineeringError::MissingParameter {
                    parameter: "thermal_expansion".to_string(),
                    calculator: self.calculator_id().to_string(),
                });
            }
        }

        let exp_type = params.additional.as_ref().and_then(|a| a.get("expansion_type")).map(|v| v.to_string()).unwrap_or("linear".to_string());
        if !["linear", "area", "volume"].contains(&exp_type.as_str()) {
            return Err(EngineeringError::InvalidParameter {
                parameter: "expansion_type".to_string(),
                value: exp_type,
                reason: "Invalid type".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let length = params.dimensions.get("length").copied().unwrap_or(1.0);
        let delta_t = self.get_additional_param(&params, "delta_t", None, None)?;
        let alpha = params.material.as_ref().and_then(|m| m.thermal_expansion).unwrap_or(12e-6);
        let exp_type = params.additional.as_ref().and_then(|a| a.get("expansion_type")).map(|v| v.to_string()).unwrap_or("linear".to_string());

        let factor = match exp_type.as_str() {
            "linear" => 1.0,
            "area" => 2.0,
            "volume" => 3.0,
            _ => 1.0,
        };

        let delta_l = length * alpha * delta_t * factor;

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if delta_t.abs() > 200.0 {
            warnings.push(format!("Large temperature change ({:.1}°C). Verify material limits.", delta_t));
            recommendations.push("Consider non-linear expansion at extreme temps".to_string());
        }

        if delta_l / length > 0.01 {
            warnings.push("Significant expansion (>1%). Design for accommodation".to_string());
        }

        compliance_notes.push("Linear thermal expansion per ASTM E228".to_string());
        compliance_notes.push("For alloys, use average coefficient over range".to_string());

        let results = vec![
            EngineeringResultItem::new("Expansion", delta_l * 1000.0, "mm")
                .critical()
                .with_format(format!("{:.2} mm", delta_l * 1000.0)),
            EngineeringResultItem::new("Relative Expansion", delta_l / length * 100.0, "%")
                .with_format(format!("{:.3}%", delta_l / length * 100.0)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "thermal_expansion".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "ASTM E228".to_string(),
                requires_pe_review: false,
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
    async fn test_thermal_expansion() {
        let calc = ThermalExpansionCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("length", 1.0),
        ]);
        
        let mut additional = HashMap::new();
        additional.insert("delta_t".to_string(), 100.0);
        params.additional = Some(additional);
        params.material = Some(MaterialProperties {
            material_type: "steel".to_string(),
            thermal_expansion: Some(12e-6),
            ..Default::default()
        });

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() >= 2);
    }

    #[test]
    fn test_missing_alpha() {
        let calc = ThermalExpansionCalculator;
        
        let mut params = minimal_parameters();
        // No thermal_expansion

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}