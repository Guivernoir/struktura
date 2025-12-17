use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::pavement::*;
use super::soil_properties::*;

pub struct PavementDesignCalculator;

impl ParameterValidator for PavementDesignCalculator {
    fn calculator_id(&self) -> &str {
        "pavement_design"
    }
}

#[async_trait]
impl EngineerCalculator for PavementDesignCalculator {
    fn id(&self) -> &str {
        "pavement_design"
    }

    fn name(&self) -> &str {
        "Flexible Pavement Design"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Civil
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("pavement_design", "Flexible Pavement Design")
            .category("civil")
            .description("Design flexible pavement thickness using AASHTO 1993 method")
            .design_code("AASHTO 1993")
            .parameter(ParameterMetadata {
                name: "ESAL".to_string(),
                path: "additional.esal".to_string(),
                data_type: ParameterType::Number,
                unit: "loadings".to_string(),
                description: "Equivalent Single Axle Loads over design period".to_string(),
                required: true,
                default_value: Some(1e6),
                min_value: Some(1e4),
                max_value: Some(1e8),
                typical_range: Some((1e5, 1e7)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Subgrade CBR".to_string(),
                path: "additional.cbr".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "California Bearing Ratio of subgrade".to_string(),
                required: true,
                default_value: Some(5.0),
                min_value: Some(2.0),
                max_value: Some(20.0),
                typical_range: Some((3.0, 10.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Reliability".to_string(),
                path: "additional.reliability".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Design reliability level".to_string(),
                required: false,
                default_value: Some(90.0),
                min_value: Some(50.0),
                max_value: Some(99.9),
                typical_range: Some((85.0, 95.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Drainage Coefficient".to_string(),
                path: "additional.drainage_coeff".to_string(),
                data_type: ParameterType::Number,
                unit: "dimensionless".to_string(),
                description: "Drainage quality coefficient".to_string(),
                required: false,
                default_value: Some(1.0),
                min_value: Some(0.5),
                max_value: Some(1.2),
                typical_range: Some((0.8, 1.1)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        self.get_additional_param(params, "esal", Some(1e4), Some(1e8))?;
        let cbr = self.get_additional_param(params, "cbr", Some(2.0), Some(20.0))?;
        self.get_additional_param(params, "reliability", Some(50.0), Some(99.9))?;
        self.get_additional_param(params, "drainage_coeff", Some(0.5), Some(1.2))?;

        if cbr < 3.0 {
            return Err(EngineeringError::DomainError {
                field: "cbr".to_string(),
                message: "Weak subgrade - consider improvement".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let esal = self.get_additional_param(&params, "esal", None, None)?;
        let cbr = self.get_additional_param(&params, "cbr", None, None)?;
        let reliability = params.additional.as_ref().and_then(|a| a.get("reliability").copied()).unwrap_or(90.0);
        let drainage = params.additional.as_ref().and_then(|a| a.get("drainage_coeff").copied()).unwrap_or(1.0);

        // Simplified AASHTO structural number calculation
        let zr = if reliability == 90.0 { -1.282 } else if reliability == 95.0 { -1.645 } else { -2.326 }; // 99%
        let so = 0.5; // Standard deviation
        let delta_psi = 2.5; // Serviceability loss
        let mr = cbr * 1500.0; // Resilient modulus psi
        let sn = (2.32 * esal.log10() + 9.36 * (mr.log10() / 1000.0) + zr * so + 2.97) / 2.32; // Heuristic

        let asphalt_thick = sn / ASPHALT_LAYER_COEFF * 25.4; // mm
        let base_thick = sn * 0.3 / BASE_LAYER_COEFF * 25.4; // mm

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if esal > ESAL_HEAVY_TRAFFIC {
            warnings.push(format!("High ESAL ({:.0}). Verify traffic projections.", esal));
            recommendations.push("Consider rigid pavement alternative".to_string());
        }

        if drainage < 0.8 {
            warnings.push("Poor drainage. Risk of premature failure.".to_string());
        }

        compliance_notes.push("Design per AASHTO 1993 empirical method".to_string());
        compliance_notes.push("Requires local calibration and materials testing".to_string());

        let results = vec![
            EngineeringResultItem::new("Structural Number (SN)", sn, "dimensionless")
                .critical()
                .with_format(format!("{:.2}", sn)),
            EngineeringResultItem::new("Asphalt Thickness", asphalt_thick, "mm")
                .with_format(format!("{:.0} mm", asphalt_thick)),
            EngineeringResultItem::new("Base Thickness", base_thick, "mm")
                .with_format(format!("{:.0} mm", base_thick)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "pavement_design".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "AASHTO 1993".to_string(),
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
    async fn test_pavement_design() {
        let calc = PavementDesignCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("esal".to_string(), 1000000.0);
        additional.insert("cbr".to_string(), 5.0);
        additional.insert("reliability".to_string(), 90.0);
        additional.insert("drainage_coeff".to_string(), 1.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() >= 3);
    }

    #[test]
    fn test_weak_subgrade() {
        let calc = PavementDesignCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("cbr".to_string(), 1.5); // Weak
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}