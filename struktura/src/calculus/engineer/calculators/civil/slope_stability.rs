use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::soil_properties::*;
use super::earth_pressure::*;

pub struct SlopeStabilityCalculator;

impl ParameterValidator for SlopeStabilityCalculator {
    fn calculator_id(&self) -> &str {
        "slope_stability"
    }
}

#[async_trait]
impl EngineerCalculator for SlopeStabilityCalculator {
    fn id(&self) -> &str {
        "slope_stability"
    }

    fn name(&self) -> &str {
        "Slope Stability Analysis"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Civil
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("slope_stability", "Slope Stability Analysis")
            .category("civil")
            .description("Calculate factor of safety for infinite slope or Bishop method")
            .design_code("USACE EM 1110-2-1902")
            .parameter(ParameterMetadata {
                name: "Slope Angle".to_string(),
                path: "additional.slope_angle".to_string(),
                data_type: ParameterType::Number,
                unit: "degrees".to_string(),
                description: "Slope inclination".to_string(),
                required: true,
                default_value: Some(30.0),
                min_value: Some(1.0),
                max_value: Some(60.0),
                typical_range: Some((10.0, 45.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Soil Friction Angle".to_string(),
                path: "additional.friction_angle".to_string(),
                data_type: ParameterType::Number,
                unit: "degrees".to_string(),
                description: "Internal friction angle".to_string(),
                required: true,
                default_value: Some(30.0),
                min_value: Some(20.0),
                max_value: Some(40.0),
                typical_range: Some((25.0, 35.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Soil Cohesion".to_string(),
                path: "additional.cohesion".to_string(),
                data_type: ParameterType::Number,
                unit: "kPa".to_string(),
                description: "Soil cohesion".to_string(),
                required: false,
                default_value: Some(0.0),
                min_value: Some(0.0),
                max_value: Some(50.0),
                typical_range: Some((0.0, 20.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Soil Unit Weight".to_string(),
                path: "additional.unit_weight".to_string(),
                data_type: ParameterType::Number,
                unit: "kN/m³".to_string(),
                description: "Soil unit weight".to_string(),
                required: false,
                default_value: Some(UNIT_WEIGHT_SANDY),
                min_value: Some(15.0),
                max_value: Some(22.0),
                typical_range: Some((16.0, 20.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Slope Height".to_string(),
                path: "dimensions.height".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Slope height for finite analysis".to_string(),
                required: false,
                default_value: Some(10.0),
                min_value: Some(1.0),
                max_value: Some(50.0),
                typical_range: Some((5.0, 20.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Advanced)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let beta = self.get_additional_param(params, "slope_angle", Some(1.0), Some(60.0))?;
        let phi = self.get_additional_param(params, "friction_angle", Some(20.0), Some(40.0))?;

        if beta >= phi {
            return Err(EngineeringError::DomainError {
                field: "angles".to_string(),
                message: "Slope angle exceeds friction angle - unstable".to_string(),
            });
        }

        self.get_additional_param(params, "cohesion", Some(0.0), Some(50.0))?;
        self.get_additional_param(params, "unit_weight", Some(15.0), Some(22.0))?;

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let beta = self.get_additional_param(&params, "slope_angle", None, None)?;
        let phi = self.get_additional_param(&params, "friction_angle", None, None)?;
        let c = params.additional.as_ref().and_then(|a| a.get("cohesion").copied()).unwrap_or(0.0);
        let gamma = params.additional.as_ref().and_then(|a| a.get("unit_weight").copied()).unwrap_or(UNIT_WEIGHT_SANDY);
        let height = params.dimensions.get("height").copied().unwrap_or(10.0);

        // Infinite slope FOS
        let fos_infinite = (phi.tan() / beta.tan()) + (2.0 * c) / (gamma * height * beta.sin() * beta.cos());

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if fos_infinite < 1.5 {
            warnings.push(format!("Low FOS ({:.2}). Slope may be unstable.", fos_infinite));
            recommendations.push("Flatten slope or add reinforcement".to_string());
        }

        if c == 0.0 {
            recommendations.push("No cohesion assumed - conservative for granular soils".to_string());
        }

        compliance_notes.push("Infinite slope method for preliminary analysis".to_string());
        compliance_notes.push("Use Bishop or Spencer method for detailed design".to_string());

        let results = vec![
            EngineeringResultItem::new("Factor of Safety", fos_infinite, "dimensionless")
                .critical()
                .with_format(format!("{:.2}", fos_infinite)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "slope_stability".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "USACE EM 1110-2-1902".to_string(),
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
    async fn test_slope_stability() {
        let calc = SlopeStabilityCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("height", 10.0),
        ]);
        let mut additional = HashMap::new();
        additional.insert("slope_angle".to_string(), 30.0);
        additional.insert("friction_angle".to_string(), 35.0);
        additional.insert("cohesion".to_string(), 0.0);
        additional.insert("unit_weight".to_string(), 18.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() >= 1);
    }

    #[test]
    fn test_steep_slope() {
        let calc = SlopeStabilityCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("slope_angle".to_string(), 40.0);
        additional.insert("friction_angle".to_string(), 30.0); // Steep
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}