use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::steel_properties::*;
use super::resistance_factors::*;

pub struct ConnectionDesignCalculator;

impl ParameterValidator for ConnectionDesignCalculator {
    fn calculator_id(&self) -> &str {
        "connection_design"
    }
}

#[async_trait]
impl EngineerCalculator for ConnectionDesignCalculator {
    fn id(&self) -> &str {
        "connection_design"
    }

    fn name(&self) -> &str {
        "Bolted Connection Design"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Structural
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("connection_design", "Bolted Connection Design")
            .category("structural")
            .description("Design bolted connection for shear and tension per AISC 360")
            .design_code("AISC 360")
            .parameter(ParameterMetadata {
                name: "Shear Load".to_string(),
                path: "loads.shear_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kN".to_string(),
                description: "Design shear force".to_string(),
                required: true,
                default_value: Some(200.0),
                min_value: Some(10.0),
                max_value: Some(1000.0),
                typical_range: Some((50.0, 300.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Tension Load".to_string(),
                path: "loads.tension_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kN".to_string(),
                description: "Design tension force".to_string(),
                required: false,
                default_value: Some(0.0),
                min_value: Some(0.0),
                max_value: Some(1000.0),
                typical_range: Some((0.0, 200.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Bolt Grade".to_string(),
                path: "additional.bolt_grade".to_string(),
                data_type: ParameterType::Enum(vec!["A325".to_string(), "A490".to_string()]),
                unit: "".to_string(),
                description: "A325 or A490".to_string(),
                required: false,
                default_value: None,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["A325 or A490".to_string()]),
            })
            .parameter(ParameterMetadata {
                name: "Bolt Diameter".to_string(),
                path: "dimensions.diameter".to_string(),
                data_type: ParameterType::Number,
                unit: "mm".to_string(),
                description: "Bolt diameter".to_string(),
                required: false,
                default_value: Some(20.0),
                min_value: Some(12.0),
                max_value: Some(36.0),
                typical_range: Some((16.0, 24.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        if let Some(loads) = &params.loads {
            if loads.shear_load < Some(10.0) {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "shear_load".to_string(),
                    value: loads.shear_load.expect("No given value, defaulting").to_string(),
                    reason: "Low shear".to_string(),
                });
            }
        }

        let grade = params.additional.as_ref().and_then(|a| a.get("bolt_grade")).map(|v| v.to_string()).unwrap_or("A325".to_string());
        if grade != "A325" && grade != "A490" {
            return Err(EngineeringError::InvalidParameter {
                parameter: "bolt_grade".to_string(),
                value: grade,
                reason: "Invalid grade".to_string(),
            });
        }

        self.validate_dimension("diameter", params.dimensions.get("diameter").copied(), 12.0, 36.0)?;

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let vu = params.loads.as_ref().map(|l| l.shear_load.unwrap_or(200.0)).unwrap_or(200.0);
        let tu = params.loads.as_ref().map(|l| l.tension_load.unwrap_or(0.0)).unwrap_or(0.0);
        let grade = params.additional.as_ref().and_then(|a| a.get("bolt_grade")).map(|v| v.to_string()).unwrap_or("A325".to_string());
        let d = params.dimensions.get("diameter").copied().unwrap_or(20.0) / 1000.0;

        let fub = if grade == "A325" { 830.0 } else { 1130.0 }; // MPa
        let area = std::f64::consts::PI * (d / 2.0).powi(2);
        let rn_shear = 0.5 * fub * area * 1000.0; // kN per bolt
        let num_bolts_shear = (vu / (PHI_SHEAR * rn_shear)).ceil();

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if tu > 0.0 {
            warnings.push("Tension present - check combined interaction".to_string());
            recommendations.push("Use pretensioned bolts for tension".to_string());
        }

        if num_bolts_shear > 8.0 {
            warnings.push("Many bolts required. Consider welded connection".to_string());
        }

        compliance_notes.push("Design per AISC 360 Chapter J".to_string());
        compliance_notes.push("Assume threads excluded from shear plane".to_string());
        compliance_notes.push("Check bearing and tearout".to_string());

        let results = vec![
            EngineeringResultItem::new("Bolt Shear Capacity", rn_shear, "kN/bolt")
                .with_format(format!("{:.1} kN/bolt", rn_shear)),
            EngineeringResultItem::new("Required Bolts", num_bolts_shear, "")
                .critical()
                .with_format(format!("{:.0}", num_bolts_shear)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "connection_design".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::engineer::test_utils::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_connection_design() {
        let calc = ConnectionDesignCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("diameter", 20.0),
        ]);
        params.loads = Some(LoadCase {
            shear_load: Some(200.0),
            tension_load: Some(0.0),
            ..Default::default()
        });
        let mut additional = HashMap::new();
        additional.insert("bolt_grade".to_string(), "A325".to_string());
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() >= 2);
    }
}