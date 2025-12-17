use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::soil_properties::*;

pub struct SoilBearingCapacityCalculator;

impl ParameterValidator for SoilBearingCapacityCalculator {
    fn calculator_id(&self) -> &str {
        "soil_bearing_capacity"
    }
}

#[async_trait]
impl EngineerCalculator for SoilBearingCapacityCalculator {
    fn id(&self) -> &str {
        "soil_bearing_capacity"
    }

    fn name(&self) -> &str {
        "Soil Bearing Capacity"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Civil
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("soil_bearing_capacity", "Soil Bearing Capacity")
            .category("civil")
            .description("Calculate ultimate bearing capacity using Terzaghi or Meyerhof method")
            .design_code("USACE EM 1110-1-1905")
            .parameter(ParameterMetadata {
                name: "Cohesion".to_string(),
                path: "additional.cohesion".to_string(),
                data_type: ParameterType::Number,
                unit: "kPa".to_string(),
                description: "Soil cohesion (c)".to_string(),
                required: false,
                default_value: Some(0.0),
                min_value: Some(0.0),
                max_value: Some(100.0),
                typical_range: Some((0.0, 50.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Friction Angle".to_string(),
                path: "additional.friction_angle".to_string(),
                data_type: ParameterType::Number,
                unit: "degrees".to_string(),
                description: "Internal friction angle (φ)".to_string(),
                required: true,
                default_value: Some(30.0),
                min_value: Some(0.0),
                max_value: Some(45.0),
                typical_range: Some((25.0, 35.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Unit Weight".to_string(),
                path: "additional.unit_weight".to_string(),
                data_type: ParameterType::Number,
                unit: "kN/m³".to_string(),
                description: "Soil unit weight (γ)".to_string(),
                required: false,
                default_value: Some(UNIT_WEIGHT_SANDY),
                min_value: Some(15.0),
                max_value: Some(22.0),
                typical_range: Some((16.0, 20.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Footing Width".to_string(),
                path: "dimensions.width".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Footing width (B)".to_string(),
                required: true,
                default_value: Some(2.0),
                min_value: Some(0.5),
                max_value: Some(5.0),
                typical_range: Some((1.0, 3.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Embedment Depth".to_string(),
                path: "dimensions.depth".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Footing embedment depth (Df)".to_string(),
                required: false,
                default_value: Some(1.0),
                min_value: Some(0.0),
                max_value: Some(3.0),
                typical_range: Some((0.5, 1.5)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        self.get_additional_param(params, "friction_angle", Some(0.0), Some(45.0))?;
        self.get_additional_param(params, "cohesion", Some(0.0), Some(100.0))?;
        self.get_additional_param(params, "unit_weight", Some(15.0), Some(22.0))?;
        self.validate_dimension("width", params.dimensions.get("width").copied(), 0.5, 5.0)?;
        self.validate_dimension("depth", params.dimensions.get("depth").copied(), 0.0, 3.0)?;

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let c = params.additional.as_ref().and_then(|a| a.get("cohesion").copied()).unwrap_or(0.0);
        let phi = self.get_additional_param(&params, "friction_angle", None, None)?;
        let gamma = params.additional.as_ref().and_then(|a| a.get("unit_weight").copied()).unwrap_or(UNIT_WEIGHT_SANDY);
        let b = params.dimensions.get("width").copied().unwrap_or(2.0);
        let df = params.dimensions.get("depth").copied().unwrap_or(1.0);

        // Terzaghi bearing factors
        let nq = ((phi.tan() + 1.0).powi(2) / (2.0 * (45.0 - phi / 2.0).to_radians().tan().powi(2))).exp();
        let nc = if phi == 0.0 { 5.7 } else { (nq - 1.0) / phi.tan() };
        let ng = 1.5 * (nq - 1.0) * phi.tan();

        let q_ult = c * nc + gamma * df * nq + 0.5 * gamma * b * ng;

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if phi < 25.0 {
            warnings.push("Low friction angle. Cohesive soil dominant.".to_string());
        }

        if df / b < 0.5 {
            recommendations.push("Increase embedment for better capacity".to_string());
        }

        compliance_notes.push("Terzaghi equation for strip footing".to_string());
        compliance_notes.push("Apply shape and depth factors for square/round".to_string());
        compliance_notes.push("Use FOS 3.0 for allowable capacity".to_string());

        let results = vec![
            EngineeringResultItem::new("Ultimate Bearing Capacity", q_ult, "kPa")
                .critical()
                .with_format(format!("{:.0} kPa", q_ult)),
            EngineeringResultItem::new("Nc", nc, "dimensionless")
                .with_format(format!("{:.1}", nc)),
            EngineeringResultItem::new("Nq", nq, "dimensionless")
                .with_format(format!("{:.1}", nq)),
            EngineeringResultItem::new("Ng", ng, "dimensionless")
                .with_format(format!("{:.1}", ng)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "soil_bearing_capacity".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "USACE EM 1110-1-1905".to_string(),
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
    async fn test_bearing_capacity() {
        let calc = SoilBearingCapacityCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("width", 2.0),
            ("depth", 1.0),
        ]);
        let mut additional = HashMap::new();
        additional.insert("cohesion".to_string(), 0.0);
        additional.insert("friction_angle".to_string(), 30.0);
        additional.insert("unit_weight".to_string(), 18.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() >= 4);
    }
}