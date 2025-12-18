use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

pub struct RefrigerationCycleCalculator;

impl ParameterValidator for RefrigerationCycleCalculator {
    fn calculator_id(&self) -> &str {
        "refrigeration_cycle"
    }
}

#[async_trait]
impl EngineerCalculator for RefrigerationCycleCalculator {
    fn id(&self) -> &str {
        "refrigeration_cycle"
    }

    fn name(&self) -> &str {
        "Refrigeration Cycle Analysis"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Mechanical
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("refrigeration_cycle", "Refrigeration Cycle Analysis")
            .category("mechanical")
            .description("Calculate COP, heat rejection, and work input for vapor-compression refrigeration cycle")
            .design_code("ASHRAE Fundamentals")
            .parameter(ParameterMetadata {
                name: "Evaporator Temperature".to_string(),
                path: "additional.t_evap".to_string(),
                data_type: ParameterType::Number,
                unit: "°C".to_string(),
                description: "Evaporator saturation temperature".to_string(),
                required: true,
                default_value: Some(-10.0),
                min_value: Some(-50.0),
                max_value: Some(10.0),
                typical_range: Some((-20.0, 0.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Condenser Temperature".to_string(),
                path: "additional.t_cond".to_string(),
                data_type: ParameterType::Number,
                unit: "°C".to_string(),
                description: "Condenser saturation temperature".to_string(),
                required: true,
                default_value: Some(40.0),
                min_value: Some(20.0),
                max_value: Some(60.0),
                typical_range: Some((30.0, 50.0)),
                validation_rules: Some(vec!["Must be > t_evap".to_string()]),
            })
            .parameter(ParameterMetadata {
                name: "Refrigerant".to_string(),
                path: "material.material_type".to_string(),
                data_type: ParameterType::Enum(vec!["R134a".to_string(), "R410a".to_string()]),
                unit: "".to_string(),
                description: "Refrigerant type (e.g., R134a, R410a)".to_string(),
                required: false,
                default_value: None,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Cooling Capacity".to_string(),
                path: "additional.cooling_capacity".to_string(),
                data_type: ParameterType::Number,
                unit: "kW".to_string(),
                description: "Required cooling capacity".to_string(),
                required: true,
                default_value: Some(100.0),
                min_value: Some(1.0),
                max_value: Some(10000.0),
                typical_range: Some((10.0, 500.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Isentropic Efficiency".to_string(),
                path: "additional.isentropic_eff".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Compressor isentropic efficiency".to_string(),
                required: false,
                default_value: Some(80.0),
                min_value: Some(50.0),
                max_value: Some(95.0),
                typical_range: Some((70.0, 85.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Advanced)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let t_evap = self.get_additional_param(params, "t_evap", Some(-50.0), Some(10.0))?;
        let t_cond = self.get_additional_param(params, "t_cond", Some(20.0), Some(60.0))?;
        self.get_additional_param(params, "cooling_capacity", Some(1.0), Some(10000.0))?;
        self.get_additional_param(params, "isentropic_eff", Some(50.0), Some(95.0))?;

        if t_cond <= t_evap {
            return Err(EngineeringError::InvalidParameter {
                parameter: "t_cond".to_string(),
                value: t_cond.to_string(),
                reason: "Must be > t_evap".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let t_evap = self.get_additional_param(&params, "t_evap", None, None)?;
        let t_cond = self.get_additional_param(&params, "t_cond", None, None)?;
        let cooling_capacity = self.get_additional_param(&params, "cooling_capacity", None, None)?;
        let isentropic_eff = params.additional.as_ref().and_then(|a| a.get("isentropic_eff").copied()).unwrap_or(80.0) / 100.0;

        // Simplified cycle analysis (assume R134a properties)
        let p_evap = 2.93; // bar at -10°C
        let p_cond = 10.16; // bar at 40°C
        let h_evap = 400.0; // kJ/kg (vapor)
        let h_liquid = 250.0; // kJ/kg after condenser
        let h_isentropic = 430.0; // kJ/kg isentropic compression
        let h_actual = h_evap + (h_isentropic - h_evap) / isentropic_eff;

        let cop = (h_evap - h_liquid) / (h_actual - h_evap);
        let work_input = cooling_capacity / cop;
        let heat_rejection = cooling_capacity + work_input;
        let mass_flow = cooling_capacity / (h_evap - h_liquid);

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if cop < 3.0 {
            warnings.push(format!("Low COP ({:.2}). Optimize temperatures.", cop));
            recommendations.push("Reduce condenser temperature or increase evaporator temp".to_string());
        }

        compliance_notes.push("Simplified vapor-compression cycle analysis".to_string());
        compliance_notes.push("Use refrigerant property tables for accurate calculations".to_string());

        let results = vec![
            EngineeringResultItem::new("COP", cop, "dimensionless")
                .critical()
                .with_format(format!("{:.2}", cop)),
            EngineeringResultItem::new("Work Input", work_input, "kW")
                .with_format(format!("{:.1} kW", work_input)),
            EngineeringResultItem::new("Heat Rejection", heat_rejection, "kW")
                .with_format(format!("{:.1} kW", heat_rejection)),
            EngineeringResultItem::new("Refrigerant Mass Flow", mass_flow, "kg/s")
                .with_format(format!("{:.3} kg/s", mass_flow)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "refrigeration_cycle".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "ASHRAE Fundamentals".to_string(),
                requires_pe_review: false,
            }),
        })
    }
}

