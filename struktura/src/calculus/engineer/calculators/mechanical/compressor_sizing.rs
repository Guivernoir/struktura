use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::constants::*;

pub struct CompressorSizingCalculator;

impl ParameterValidator for CompressorSizingCalculator {
    fn calculator_id(&self) -> &str {
        "compressor_sizing"
    }
}

#[async_trait]
impl EngineerCalculator for CompressorSizingCalculator {
    fn id(&self) -> &str {
        "compressor_sizing"
    }

    fn name(&self) -> &str {
        "Compressor Sizing"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Mechanical
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("compressor_sizing", "Compressor Sizing")
            .category("mechanical")
            .description("Calculate required compressor power and displacement for reciprocating or centrifugal compressors")
            .design_code("ASME PTC 10")
            .parameter(ParameterMetadata {
                name: "Inlet Pressure".to_string(),
                path: "additional.p_in".to_string(),
                data_type: ParameterType::Number,
                unit: "kPa".to_string(),
                description: "Inlet pressure (absolute)".to_string(),
                required: true,
                default_value: Some(101.325),
                min_value: Some(10.0),
                max_value: Some(10000.0),
                typical_range: Some((100.0, 500.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Outlet Pressure".to_string(),
                path: "additional.p_out".to_string(),
                data_type: ParameterType::Number,
                unit: "kPa".to_string(),
                description: "Outlet pressure (absolute)".to_string(),
                required: true,
                default_value: Some(500.0),
                min_value: Some(100.0),
                max_value: Some(20000.0),
                typical_range: Some((200.0, 1000.0)),
                validation_rules: Some(vec!["Must be > p_in".to_string()]),
            })
            .parameter(ParameterMetadata {
                name: "Flow Rate".to_string(),
                path: "additional.flow_rate".to_string(),
                data_type: ParameterType::Number,
                unit: "m³/min".to_string(),
                description: "Volumetric flow rate at inlet conditions".to_string(),
                required: true,
                default_value: Some(10.0),
                min_value: Some(0.1),
                max_value: Some(1000.0),
                typical_range: Some((1.0, 100.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Gas Constant".to_string(),
                path: "additional.gas_constant".to_string(),
                data_type: ParameterType::Number,
                unit: "J/(kg·K)".to_string(),
                description: "Specific gas constant (R_specific = R/MW)".to_string(),
                required: false,
                default_value: Some(287.0), // Air
                min_value: Some(50.0),
                max_value: Some(500.0),
                typical_range: Some((200.0, 400.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Specific Heat Ratio".to_string(),
                path: "additional.k".to_string(),
                data_type: ParameterType::Number,
                unit: "dimensionless".to_string(),
                description: "k = Cp/Cv".to_string(),
                required: false,
                default_value: Some(1.4), // Air
                min_value: Some(1.1),
                max_value: Some(1.7),
                typical_range: Some((1.2, 1.6)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Efficiency".to_string(),
                path: "additional.efficiency".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Isentropic efficiency".to_string(),
                required: false,
                default_value: Some(75.0),
                min_value: Some(50.0),
                max_value: Some(90.0),
                typical_range: Some((60.0, 85.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let p_in = self.get_additional_param(params, "p_in", Some(10.0), Some(10000.0))?;
        let p_out = self.get_additional_param(params, "p_out", Some(100.0), Some(20000.0))?;
        self.get_additional_param(params, "flow_rate", Some(0.1), Some(1000.0))?;
        self.get_additional_param(params, "gas_constant", Some(50.0), Some(500.0))?;
        self.get_additional_param(params, "k", Some(1.1), Some(1.7))?;
        self.get_additional_param(params, "efficiency", Some(50.0), Some(90.0))?;

        if p_out <= p_in {
            return Err(EngineeringError::InvalidParameter {
                parameter: "p_out".to_string(),
                value: p_out.to_string(),
                reason: "Must be > p_in".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let p_in = self.get_additional_param(&params, "p_in", None, None)?;
        let p_out = self.get_additional_param(&params, "p_out", None, None)?;
        let flow_rate_m3min = self.get_additional_param(&params, "flow_rate", None, None)?;
        let r = params.additional.as_ref().and_then(|a| a.get("gas_constant").copied()).unwrap_or(287.0);
        let k = params.additional.as_ref().and_then(|a| a.get("k").copied()).unwrap_or(1.4);
        let eff = params.additional.as_ref().and_then(|a| a.get("efficiency").copied()).unwrap_or(75.0) / 100.0;

        let pressure_ratio = p_out / p_in;
        let isentropic_work = (k / (k - 1.0)) * r * 293.0 * (pressure_ratio.powf((k - 1.0)/k) - 1.0) / 1000.0; // kJ/kg
        let actual_work = isentropic_work / eff;
        let mass_flow = (flow_rate_m3min / 60.0) * (p_in * 1000.0 / (r * 293.0)); // kg/s (assume T=20°C)
        let power = actual_work * mass_flow;

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if pressure_ratio > 10.0 {
            warnings.push(format!("High pressure ratio ({:.1}). Consider multi-stage compression.", pressure_ratio));
            recommendations.push("Add intercooling for ratios >5".to_string());
        }

        compliance_notes.push("Calculation for ideal gas compression".to_string());
        compliance_notes.push("Verify with compressor maps for accurate selection".to_string());

        let results = vec![
            EngineeringResultItem::new("Power Required", power, "kW")
                .critical()
                .with_format(format!("{:.1} kW", power)),
            EngineeringResultItem::new("Pressure Ratio", pressure_ratio, "dimensionless")
                .with_format(format!("{:.2}", pressure_ratio)),
            EngineeringResultItem::new("Isentropic Work", isentropic_work, "kJ/kg")
                .with_format(format!("{:.1} kJ/kg", isentropic_work)),
            EngineeringResultItem::new("Mass Flow Rate", mass_flow, "kg/s")
                .with_format(format!("{:.3} kg/s", mass_flow)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "compressor_sizing".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "ASME PTC 10".to_string(),
                requires_pe_review: false,
            }),
        })
    }
}

