use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::fluid_properties::*;

pub struct ValveSizingCalculator;

impl ParameterValidator for ValveSizingCalculator {
    fn calculator_id(&self) -> &str {
        "valve_sizing"
    }
}

#[async_trait]
impl EngineerCalculator for ValveSizingCalculator {
    fn id(&self) -> &str {
        "valve_sizing"
    }

    fn name(&self) -> &str {
        "Valve Sizing (Cv Calculation)"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Mechanical
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("valve_sizing", "Valve Sizing (Cv Calculation)")
            .category("mechanical")
            .description("Calculate required valve Cv for liquid or gas service per ISA standards")
            .design_code("ISA 75.01")
            .parameter(ParameterMetadata {
                name: "Flow Rate".to_string(),
                path: "additional.flow_rate".to_string(),
                data_type: ParameterType::Number,
                unit: "gpm".to_string(),
                description: "Volumetric flow rate (for liquids)".to_string(),
                required: true,
                default_value: Some(100.0),
                min_value: Some(1.0),
                max_value: Some(10000.0),
                typical_range: Some((10.0, 500.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Pressure Drop".to_string(),
                path: "additional.dp".to_string(),
                data_type: ParameterType::Number,
                unit: "psi".to_string(),
                description: "Pressure drop across valve".to_string(),
                required: true,
                default_value: Some(10.0),
                min_value: Some(1.0),
                max_value: Some(1000.0),
                typical_range: Some((5.0, 50.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Specific Gravity".to_string(),
                path: "additional.sg".to_string(),
                data_type: ParameterType::Number,
                unit: "dimensionless".to_string(),
                description: "Specific gravity (relative to water)".to_string(),
                required: false,
                default_value: Some(1.0),
                min_value: Some(0.5),
                max_value: Some(2.0),
                typical_range: Some((0.8, 1.2)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Fluid Type".to_string(),
                path: "additional.fluid_type".to_string(),
                data_type: ParameterType::Enum(vec!["liquid".to_string(), "gas".to_string()]),
                unit: "".to_string(),
                description: "liquid or gas".to_string(),
                required: false,
                default_value: Some(0.0),
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["liquid or gas".to_string()]),
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        self.get_additional_param(params, "flow_rate", Some(1.0), Some(10000.0))?;
        let dp = self.get_additional_param(params, "dp", Some(1.0), Some(1000.0))?;
        self.get_additional_param(params, "sg", Some(0.5), Some(2.0))?;

        let fluid_type = params.additional.as_ref().and_then(|a| a.get("fluid_type")).map(|v| v.to_string()).unwrap_or("liquid".to_string());
        if fluid_type != "liquid" && fluid_type != "gas" {
            return Err(EngineeringError::InvalidParameter {
                parameter: "fluid_type".to_string(),
                value: fluid_type,
                reason: "Must be liquid or gas".to_string(),
            });
        }

        if dp < 5.0 {
            return Err(EngineeringError::DomainError {
                field: "dp".to_string(),
                message: "Low pressure drop - may not require control valve".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let flow_rate = self.get_additional_param(&params, "flow_rate", None, None)?;
        let dp = self.get_additional_param(&params, "dp", None, None)?;
        let sg = params.additional.as_ref().and_then(|a| a.get("sg").copied()).unwrap_or(1.0);
        let fluid_type = params.additional.as_ref().and_then(|a| a.get("fluid_type")).map(|v| v.to_string()).unwrap_or("liquid".to_string());

        let cv = if fluid_type == "liquid" {
            flow_rate / (dp / sg).sqrt()
        } else {
            // Gas, assume scfh, psid, sg gas
            flow_rate * (sg * 520.0).sqrt() / (816.0 * dp.sqrt()) // Simplified
        };

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if cv > 1000.0 {
            warnings.push(format!("Large Cv ({:.1}). Consider multiple valves.", cv));
            recommendations.push("Verify piping size compatibility".to_string());
        }

        compliance_notes.push("Cv calculation per ISA 75.01 for liquids".to_string());
        compliance_notes.push("For gases, use expanded equation with compressibility".to_string());

        let results = vec![
            EngineeringResultItem::new("Required Cv", cv, "dimensionless")
                .critical()
                .with_format(format!("{:.1}", cv)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "valve_sizing".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "ISA 75.01".to_string(),
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
    async fn test_valve_sizing_liquid() {
        let calc = ValveSizingCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("flow_rate".to_string(), 100.0);
        additional.insert("dp".to_string(), 10.0);
        additional.insert("sg".to_string(), 1.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() >= 1);
    }

    #[test]
    fn test_invalid_fluid_type() {
        let calc = ValveSizingCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("fluid_type".to_string(), "steam".to_string()); // Invalid
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}