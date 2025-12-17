use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::pump_hydraulics::*;
use super::helpers::*;
use super::fluid_properties::*;
use super::constants::*;

pub struct PumpSizingCalculator;

impl ParameterValidator for PumpSizingCalculator {
    fn calculator_id(&self) -> &str {
        "pump_sizing"
    }
}

#[async_trait]
impl EngineerCalculator for PumpSizingCalculator {
    fn id(&self) -> &str {
        "pump_sizing"
    }

    fn name(&self) -> &str {
        "Pump Sizing and Selection"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Mechanical
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("pump_sizing", "Pump Sizing and Selection")
            .category("mechanical")
            .description("Calculate required pump head, power, NPSH, and efficiency for centrifugal pumps per API 610 standards")
            .design_code("API 610")
            .design_code("ISO 5199")
            .parameter(ParameterMetadata {
                name: "Flow Rate".to_string(),
                path: "additional.flow_rate".to_string(),
                data_type: ParameterType::Number,
                unit: "m³/h".to_string(),
                description: "Required volumetric flow rate".to_string(),
                required: true,
                default_value: Some(100.0),
                min_value: Some(1.0),
                max_value: Some(10000.0),
                typical_range: Some((10.0, 500.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Total Head".to_string(),
                path: "additional.total_head".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Total dynamic head (static + friction + velocity)".to_string(),
                required: true,
                default_value: Some(50.0),
                min_value: Some(1.0),
                max_value: Some(1000.0),
                typical_range: Some((10.0, 200.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Fluid Density".to_string(),
                path: "material.density".to_string(),
                data_type: ParameterType::Number,
                unit: "kg/m³".to_string(),
                description: "Fluid density".to_string(),
                required: false,
                default_value: Some(WATER_DENSITY),
                min_value: Some(500.0),
                max_value: Some(2000.0),
                typical_range: Some((800.0, 1200.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Fluid Viscosity".to_string(),
                path: "additional.viscosity".to_string(),
                data_type: ParameterType::Number,
                unit: "cP".to_string(),
                description: "Fluid dynamic viscosity".to_string(),
                required: false,
                default_value: Some(1.0),
                min_value: Some(0.1),
                max_value: Some(10000.0),
                typical_range: Some((0.5, 100.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Pump Efficiency".to_string(),
                path: "additional.pump_efficiency".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Assumed pump efficiency".to_string(),
                required: false,
                default_value: Some(EFF_MEDIUM_CENTRIFUGAL),
                min_value: Some(50.0),
                max_value: Some(90.0),
                typical_range: Some((60.0, 85.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "NPSH Available".to_string(),
                path: "additional.npsh_available".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Available Net Positive Suction Head".to_string(),
                required: false,
                default_value: Some(5.0),
                min_value: Some(0.0),
                max_value: Some(50.0),
                typical_range: Some((3.0, 10.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        self.get_additional_param(params, "flow_rate", Some(1.0), Some(10000.0))?;
        self.get_additional_param(params, "total_head", Some(1.0), Some(1000.0))?;
        self.get_additional_param(params, "viscosity", Some(0.1), Some(10000.0))?;
        self.get_additional_param(params, "pump_efficiency", Some(50.0), Some(90.0))?;
        self.get_additional_param(params, "npsh_available", Some(0.0), Some(50.0))?;

        if let Some(material) = &params.material {
            if material.density < Some(500.0) || material.density > Some(2000.0) {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "density".to_string(),
                    value: material.density.expect("No density provided, defaulting to water value").to_string(),
                    reason: "Unusual fluid density - verify".to_string(),
                });
            }
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let flow_rate_m3h = self.get_additional_param(&params, "flow_rate", None, None)?;
        let total_head = self.get_additional_param(&params, "total_head", None, None)?;
        let density = params.material.as_ref().map(|m| m.density).unwrap_or(Some(WATER_DENSITY));
        let viscosity_cp = params.additional.as_ref().and_then(|a| a.get("viscosity").copied()).unwrap_or(1.0);
        let pump_eff = params.additional.as_ref().and_then(|a| a.get("pump_efficiency").copied()).unwrap_or(EFF_MEDIUM_CENTRIFUGAL) / 100.0;
        let npsha = params.additional.as_ref().and_then(|a| a.get("npsh_available").copied()).unwrap_or(5.0);

        let flow_m3s = flow_rate_m3h / 3600.0;
        let hydraulic_power = hydraulic_power_kw(flow_m3s, total_head, density.expect("No density provided, defaulting to water value"));
        let brake_power = hydraulic_power / pump_eff;
        let npshr_approx = 2.0 + (flow_rate_m3h / 100.0).powf(0.5); // Rough estimate
        let npsh_margin = npsha - npshr_approx;

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if npsh_margin < 1.0 {
            warnings.push(format!("Insufficient NPSH margin ({:.1}m). Risk of cavitation.", npsh_margin));
            recommendations.push("Increase suction head or reduce vapor pressure".to_string());
        }

        if viscosity_cp > 100.0 {
            warnings.push(format!("High viscosity ({:.1} cP). Efficiency reduced - consider viscosity correction.", viscosity_cp));
            recommendations.push("Use viscosity correction charts for accurate sizing".to_string());
        }

        compliance_notes.push("Pump selection per API 610 for process pumps".to_string());
        compliance_notes.push("Verify NPSHr from manufacturer curve".to_string());

        let results = vec![
            EngineeringResultItem::new("Hydraulic Power", hydraulic_power, "kW")
                .critical()
                .with_format(format!("{:.1} kW", hydraulic_power)),
            EngineeringResultItem::new("Brake Power", brake_power, "kW")
                .critical()
                .with_format(format!("{:.1} kW", brake_power)),
            EngineeringResultItem::new("Required NPSHr (approx)", npshr_approx, "m")
                .with_format(format!("{:.1} m", npshr_approx)),
            EngineeringResultItem::new("NPSH Margin", npsh_margin, "m")
                .with_format(format!("{:.1} m", npsh_margin)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "pump_sizing".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "API 610".to_string(),
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
    async fn test_pump_sizing_basic() {
        let calc = PumpSizingCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("flow_rate".to_string(), 100.0);
        additional.insert("total_head".to_string(), 50.0);
        additional.insert("viscosity".to_string(), 1.0);
        additional.insert("pump_efficiency".to_string(), 75.0);
        additional.insert("npsh_available".to_string(), 5.0);
        params.additional = Some(additional);
        params.material = Some(MaterialProperties {
            material_type: "water".to_string(),
            density: Some(WATER_DENSITY),
            ..Default::default()
        });

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() >= 4);
    }

    #[test]
    fn test_high_viscosity_warning() {
        let calc = PumpSizingCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("flow_rate".to_string(), 100.0);
        additional.insert("total_head".to_string(), 50.0);
        additional.insert("viscosity".to_string(), 200.0); // High
        params.additional = Some(additional);

        let result = calc.calculate(params).await.unwrap();
        assert!(!result.warnings.is_empty());
    }
}