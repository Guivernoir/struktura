use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

pub struct HVACLoadCalculationCalculator;

impl ParameterValidator for HVACLoadCalculationCalculator {
    fn calculator_id(&self) -> &str {
        "hvac_load_calculation"
    }
}

#[async_trait]
impl EngineerCalculator for HVACLoadCalculationCalculator {
    fn id(&self) -> &str {
        "hvac_load_calculation"
    }

    fn name(&self) -> &str {
        "HVAC Cooling/Heating Load Calculation"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Mechanical
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("hvac_load_calculation", "HVAC Cooling/Heating Load Calculation")
            .category("mechanical")
            .description("Estimate building cooling and heating loads using ASHRAE methods")
            .design_code("ASHRAE 90.1")
            .design_code("ASHRAE Fundamentals")
            .parameter(ParameterMetadata {
                name: "Building Area".to_string(),
                path: "dimensions.area".to_string(),
                data_type: ParameterType::Number,
                unit: "m²".to_string(),
                description: "Total floor area".to_string(),
                required: true,
                default_value: Some(1000.0),
                min_value: Some(10.0),
                max_value: Some(100000.0),
                typical_range: Some((100.0, 5000.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Outdoor Temperature".to_string(),
                path: "additional.outdoor_temp".to_string(),
                data_type: ParameterType::Number,
                unit: "°C".to_string(),
                description: "Design outdoor temperature".to_string(),
                required: true,
                default_value: Some(35.0),
                min_value: Some(-50.0),
                max_value: Some(50.0),
                typical_range: Some((25.0, 40.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Indoor Temperature".to_string(),
                path: "additional.indoor_temp".to_string(),
                data_type: ParameterType::Number,
                unit: "°C".to_string(),
                description: "Desired indoor temperature".to_string(),
                required: true,
                default_value: Some(24.0),
                min_value: Some(18.0),
                max_value: Some(30.0),
                typical_range: Some((22.0, 26.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Wall U-Value".to_string(),
                path: "additional.wall_u".to_string(),
                data_type: ParameterType::Number,
                unit: "W/(m²·K)".to_string(),
                description: "Wall heat transfer coefficient".to_string(),
                required: false,
                default_value: Some(0.5),
                min_value: Some(0.1),
                max_value: Some(2.0),
                typical_range: Some((0.2, 0.8)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Window Area Ratio".to_string(),
                path: "additional.window_ratio".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Window to wall area ratio".to_string(),
                required: false,
                default_value: Some(20.0),
                min_value: Some(0.0),
                max_value: Some(80.0),
                typical_range: Some((10.0, 40.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Occupancy".to_string(),
                path: "additional.occupancy".to_string(),
                data_type: ParameterType::Number,
                unit: "persons/m²".to_string(),
                description: "Occupancy density".to_string(),
                required: false,
                default_value: Some(0.1),
                min_value: Some(0.0),
                max_value: Some(1.0),
                typical_range: Some((0.05, 0.2)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let area = self.validate_dimension("area", params.dimensions.get("area").copied(), 10.0, 100000.0)?;
        self.get_additional_param(params, "outdoor_temp", Some(-50.0), Some(50.0))?;
        self.get_additional_param(params, "indoor_temp", Some(18.0), Some(30.0))?;
        self.get_additional_param(params, "wall_u", Some(0.1), Some(2.0))?;
        self.get_additional_param(params, "window_ratio", Some(0.0), Some(80.0))?;
        self.get_additional_param(params, "occupancy", Some(0.0), Some(1.0))?;

        if area < 100.0 {
            return Err(EngineeringError::DomainError {
                field: "area".to_string(),
                message: "Small building - simplified method may not apply".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let area = params.dimensions.get("area").copied().unwrap_or(1000.0);
        let outdoor_temp = self.get_additional_param(&params, "outdoor_temp", None, None)?;
        let indoor_temp = self.get_additional_param(&params, "indoor_temp", None, None)?;
        let wall_u = params.additional.as_ref().and_then(|a| a.get("wall_u").copied()).unwrap_or(0.5);
        let window_ratio = params.additional.as_ref().and_then(|a| a.get("window_ratio").copied()).unwrap_or(20.0) / 100.0;
        let occupancy = params.additional.as_ref().and_then(|a| a.get("occupancy").copied()).unwrap_or(0.1);

        // Simplified load calculation
        let dt = (outdoor_temp - indoor_temp).abs();
        let envelope_area = area * 2.5; // Assume height 2.5m, perimeter approx
        let conduction_load = wall_u * envelope_area * dt * (1.0 - window_ratio);
        let window_load = 2.0 * envelope_area * window_ratio * dt; // Higher U for windows
        let solar_load = 200.0 * area * 0.5; // Approximate solar gain
        let internal_load = 100.0 * occupancy * area; // W/person
        let ventilation_load = 500.0 * occupancy * area * dt / 20.0; // Approximate

        let total_load = conduction_load + window_load + solar_load + internal_load + ventilation_load;
        let load_tons = total_load / 12000.0 / 3.517; // Convert W to tons (approx)

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if dt > 20.0 {
            warnings.push(format!("Large temperature difference ({:.1}°C). Verify insulation.", dt));
            recommendations.push("Improve building envelope U-values".to_string());
        }

        if occupancy > 0.2 {
            warnings.push("High occupancy density. Verify ventilation requirements".to_string());
        }

        compliance_notes.push("Simplified load calculation per ASHRAE methods".to_string());
        compliance_notes.push("For detailed analysis, use CLTD/CLF method".to_string());

        let results = vec![
            EngineeringResultItem::new("Total Load", total_load / 1000.0, "kW")
                .critical()
                .with_format(format!("{:.1} kW", total_load / 1000.0)),
            EngineeringResultItem::new("Load in Tons", load_tons, "tons")
                .with_format(format!("{:.1} tons", load_tons)),
            EngineeringResultItem::new("Conduction Load", conduction_load / 1000.0, "kW"),
            EngineeringResultItem::new("Internal Load", internal_load / 1000.0, "kW"),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "hvac_load_calculation".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "ASHRAE 90.1".to_string(),
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
    async fn test_hvac_load() {
        let calc = HVACLoadCalculationCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("area", 1000.0),
        ]);
        
        let mut additional = HashMap::new();
        additional.insert("outdoor_temp".to_string(), 35.0);
        additional.insert("indoor_temp".to_string(), 24.0);
        additional.insert("wall_u".to_string(), 0.5);
        additional.insert("window_ratio".to_string(), 20.0);
        additional.insert("occupancy".to_string(), 0.1);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() >= 4);
    }

    #[test]
    fn test_small_area() {
        let calc = HVACLoadCalculationCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("area", 50.0), // Small
        ]);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}