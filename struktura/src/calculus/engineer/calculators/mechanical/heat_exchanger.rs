use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::heat_exchanger_values::*;
use super::helpers::*;
use super::fluid_properties::*;
use super::constants::*;

pub struct HeatExchangerCalculator;

impl ParameterValidator for HeatExchangerCalculator {
    fn calculator_id(&self) -> &str {
        "heat_exchanger"
    }
}

#[async_trait]
impl EngineerCalculator for HeatExchangerCalculator {
    fn id(&self) -> &str {
        "heat_exchanger"
    }

    fn name(&self) -> &str {
        "Heat Exchanger Design and Sizing"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Mechanical
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("heat_exchanger", "Heat Exchanger Design and Sizing")
            .category("mechanical")
            .description("Calculate heat transfer rate, LMTD, required area, and effectiveness for shell-and-tube or plate heat exchangers per TEMA standards")
            .design_code("TEMA")
            .design_code("ASME BPVC")
            .parameter(ParameterMetadata {
                name: "Hot Inlet Temperature".to_string(),
                path: "additional.t_hot_in".to_string(),
                data_type: ParameterType::Number,
                unit: "°C".to_string(),
                description: "Hot fluid inlet temperature".to_string(),
                required: true,
                default_value: Some(80.0),
                min_value: Some(0.0),
                max_value: Some(500.0),
                typical_range: Some((50.0, 200.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Hot Outlet Temperature".to_string(),
                path: "additional.t_hot_out".to_string(),
                data_type: ParameterType::Number,
                unit: "°C".to_string(),
                description: "Hot fluid outlet temperature".to_string(),
                required: true,
                default_value: Some(50.0),
                min_value: Some(0.0),
                max_value: Some(500.0),
                typical_range: Some((30.0, 150.0)),
                validation_rules: Some(vec!["Must be < t_hot_in".to_string()]),
            })
            .parameter(ParameterMetadata {
                name: "Cold Inlet Temperature".to_string(),
                path: "additional.t_cold_in".to_string(),
                data_type: ParameterType::Number,
                unit: "°C".to_string(),
                description: "Cold fluid inlet temperature".to_string(),
                required: true,
                default_value: Some(20.0),
                min_value: Some(0.0),
                max_value: Some(300.0),
                typical_range: Some((10.0, 50.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Cold Outlet Temperature".to_string(),
                path: "additional.t_cold_out".to_string(),
                data_type: ParameterType::Number,
                unit: "°C".to_string(),
                description: "Cold fluid outlet temperature".to_string(),
                required: true,
                default_value: Some(40.0),
                min_value: Some(0.0),
                max_value: Some(300.0),
                typical_range: Some((20.0, 100.0)),
                validation_rules: Some(vec!["Must be > t_cold_in".to_string()]),
            })
            .parameter(ParameterMetadata {
                name: "Hot Mass Flow".to_string(),
                path: "additional.mass_flow_hot".to_string(),
                data_type: ParameterType::Number,
                unit: "kg/s".to_string(),
                description: "Hot fluid mass flow rate".to_string(),
                required: true,
                default_value: Some(10.0),
                min_value: Some(0.1),
                max_value: Some(1000.0),
                typical_range: Some((1.0, 100.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Cold Mass Flow".to_string(),
                path: "additional.mass_flow_cold".to_string(),
                data_type: ParameterType::Number,
                unit: "kg/s".to_string(),
                description: "Cold fluid mass flow rate".to_string(),
                required: true,
                default_value: Some(15.0),
                min_value: Some(0.1),
                max_value: Some(1000.0),
                typical_range: Some((1.0, 100.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Overall Heat Transfer Coefficient".to_string(),
                path: "additional.u_value".to_string(),
                data_type: ParameterType::Number,
                unit: "W/(m²·K)".to_string(),
                description: "Overall heat transfer coefficient (U)".to_string(),
                required: false,
                default_value: Some(850.0),
                min_value: Some(10.0),
                max_value: Some(5000.0),
                typical_range: Some((100.0, 2500.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let t_hot_in = self.get_additional_param(params, "t_hot_in", Some(0.0), Some(500.0))?;
        let t_hot_out = self.get_additional_param(params, "t_hot_out", Some(0.0), Some(500.0))?;
        let t_cold_in = self.get_additional_param(params, "t_cold_in", Some(0.0), Some(300.0))?;
        let t_cold_out = self.get_additional_param(params, "t_cold_out", Some(0.0), Some(300.0))?;
        self.get_additional_param(params, "mass_flow_hot", Some(0.1), Some(1000.0))?;
        self.get_additional_param(params, "mass_flow_cold", Some(0.1), Some(1000.0))?;
        self.get_additional_param(params, "u_value", Some(10.0), Some(5000.0))?;

        if t_hot_out >= t_hot_in {
            return Err(EngineeringError::InvalidParameter {
                parameter: "t_hot_out".to_string(),
                value: t_hot_out.to_string(),
                reason: "Must be < t_hot_in".to_string(),
            });
        }

        if t_cold_out <= t_cold_in {
            return Err(EngineeringError::InvalidParameter {
                parameter: "t_cold_out".to_string(),
                value: t_cold_out.to_string(),
                reason: "Must be > t_cold_in".to_string(),
            });
        }

        if t_hot_out <= t_cold_out {
            return Err(EngineeringError::DomainError {
                field: "temperatures".to_string(),
                message: "Temperature cross detected - impossible for counterflow".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let t_hot_in = self.get_additional_param(&params, "t_hot_in", None, None)?;
        let t_hot_out = self.get_additional_param(&params, "t_hot_out", None, None)?;
        let t_cold_in = self.get_additional_param(&params, "t_cold_in", None, None)?;
        let t_cold_out = self.get_additional_param(&params, "t_cold_out", None, None)?;
        let mass_flow_hot = self.get_additional_param(&params, "mass_flow_hot", None, None)?;
        let mass_flow_cold = self.get_additional_param(&params, "mass_flow_cold", None, None)?;
        let u_value = params.additional.as_ref().and_then(|a| a.get("u_value").copied()).unwrap_or(U_WATER_WATER);

        // Assume water for both sides
        let c_hot = WATER_SPECIFIC_HEAT * mass_flow_hot;
        let c_cold = WATER_SPECIFIC_HEAT * mass_flow_cold;
        let c_min = c_hot.min(c_cold);
        let c_ratio = c_min / c_hot.max(c_cold);

        let lmtd = lmtd_counterflow(t_hot_in, t_hot_out, t_cold_in, t_cold_out);
        let heat_rate = heat_transfer_kw(mass_flow_hot, WATER_SPECIFIC_HEAT, t_hot_in - t_hot_out);
        let required_area = heat_rate * 1000.0 / (u_value * lmtd);
        let ntu_val = ntu(u_value * required_area, c_min);
        let effectiveness = effectiveness_from_ntu_counterflow(ntu_val, c_ratio);

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if effectiveness < 0.5 {
            warnings.push(format!("Low effectiveness ({:.2}). Consider different configuration.", effectiveness));
            recommendations.push("Switch to parallel flow or multiple passes".to_string());
        }

        if required_area > 100.0 {
            recommendations.push("Large heat exchanger area. Consider multiple units in parallel".to_string());
        }

        compliance_notes.push("Design per TEMA standards for shell-and-tube exchangers".to_string());
        compliance_notes.push("Verify fouling factors and clean-side allocation".to_string());

        let results = vec![
            EngineeringResultItem::new("Heat Transfer Rate", heat_rate, "kW")
                .critical()
                .with_format(format!("{:.1} kW", heat_rate)),
            EngineeringResultItem::new("LMTD", lmtd, "°C")
                .with_format(format!("{:.1} °C", lmtd)),
            EngineeringResultItem::new("Required Area", required_area, "m²")
                .critical()
                .with_format(format!("{:.1} m²", required_area)),
            EngineeringResultItem::new("Effectiveness", effectiveness * 100.0, "%")
                .with_format(format!("{:.1}%", effectiveness * 100.0)),
            EngineeringResultItem::new("NTU", ntu_val, "dimensionless")
                .with_format(format!("{:.2}", ntu_val)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "heat_exchanger".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "TEMA".to_string(),
                requires_pe_review: false,
            }),
        })
    }
}

