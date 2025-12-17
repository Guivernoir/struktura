use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::helpers::*;
use super::fluid_properties::*;
use super::constants::*;

pub struct PipingPressureDropCalculator;

impl ParameterValidator for PipingPressureDropCalculator {
    fn calculator_id(&self) -> &str {
        "piping_pressure_drop"
    }
}

#[async_trait]
impl EngineerCalculator for PipingPressureDropCalculator {
    fn id(&self) -> &str {
        "piping_pressure_drop"
    }

    fn name(&self) -> &str {
        "Piping Pressure Drop Calculation"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Mechanical
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("piping_pressure_drop", "Piping Pressure Drop Calculation")
            .category("mechanical")
            .description("Calculate friction losses and pressure drop in piping systems using Darcy-Weisbach equation")
            .design_code("ASME B31.3")
            .design_code("ISO 5167")
            .parameter(ParameterMetadata {
                name: "Pipe Length".to_string(),
                path: "dimensions.length".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Length of pipe".to_string(),
                required: true,
                default_value: Some(100.0),
                min_value: Some(1.0),
                max_value: Some(10000.0),
                typical_range: Some((10.0, 500.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Pipe Diameter".to_string(),
                path: "dimensions.diameter".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Internal pipe diameter".to_string(),
                required: true,
                default_value: Some(0.1),
                min_value: Some(0.01),
                max_value: Some(2.0),
                typical_range: Some((0.05, 0.5)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Flow Rate".to_string(),
                path: "additional.flow_rate".to_string(),
                data_type: ParameterType::Number,
                unit: "m³/s".to_string(),
                description: "Volumetric flow rate".to_string(),
                required: true,
                default_value: Some(0.05),
                min_value: Some(0.001),
                max_value: Some(10.0),
                typical_range: Some((0.01, 1.0)),
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
                unit: "Pa·s".to_string(),
                description: "Dynamic viscosity".to_string(),
                required: false,
                default_value: Some(WATER_VISCOSITY),
                min_value: Some(1e-6),
                max_value: Some(1.0),
                typical_range: Some((0.0005, 0.01)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Pipe Roughness".to_string(),
                path: "additional.roughness".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Absolute pipe roughness".to_string(),
                required: false,
                default_value: Some(0.00015), // Commercial steel
                min_value: Some(1e-6),
                max_value: Some(0.001),
                typical_range: Some((0.00005, 0.0005)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let length = self.validate_dimension("length", params.dimensions.get("length").copied(), 1.0, 10000.0)?;
        let diameter = self.validate_dimension("diameter", params.dimensions.get("diameter").copied(), 0.01, 2.0)?;
        self.get_additional_param(params, "flow_rate", Some(0.001), Some(10.0))?;
        self.get_additional_param(params, "viscosity", Some(1e-6), Some(1.0))?;
        self.get_additional_param(params, "roughness", Some(1e-6), Some(0.001))?;

        if let Some(material) = &params.material {
            if material.density < Some(500.0) || material.density > Some(2000.0) {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "density".to_string(),
                    value: material.density.expect("No density provided, defaulting to water value").to_string(),
                    reason: "Unusual fluid density".to_string(),
                });
            }
        }

        if length / diameter > 10000.0 {
            return Err(EngineeringError::DomainError {
                field: "l_d_ratio".to_string(),
                message: "Extreme L/D ratio - verify inputs".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let length = params.dimensions.get("length").copied().unwrap_or(100.0);
        let diameter = params.dimensions.get("diameter").copied().unwrap_or(0.1);
        let flow_rate = self.get_additional_param(&params, "flow_rate", None, None)?;
        let density = params.material.as_ref().map(|m| m.density).unwrap_or(Some(WATER_DENSITY));
        let viscosity = params.additional.as_ref().and_then(|a| a.get("viscosity").copied()).unwrap_or(WATER_VISCOSITY);
        let roughness = params.additional.as_ref().and_then(|a| a.get("roughness").copied()).unwrap_or(0.00015);

        let velocity = flow_rate / (std::f64::consts::PI * (diameter / 2.0).powi(2));
        let re = reynolds_number(velocity, diameter, density.expect("No density provided, defaulting to water value"), viscosity);

        let friction = if re < 2300.0 {
            64.0 / re // Laminar
        } else {
            friction_factor_turbulent(re, roughness, diameter)
        };

        let pressure_drop = pressure_drop_pipe(friction, length, diameter, velocity, density.expect("No density provided, defaulting to water value")) / 1000.0; // kPa

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if velocity > 3.0 {
            warnings.push(format!("High velocity ({:.1} m/s). Risk of erosion.", velocity));
            recommendations.push("Increase pipe diameter to reduce velocity".to_string());
        } else if velocity < 0.5 {
            warnings.push(format!("Low velocity ({:.1} m/s). Risk of settling.", velocity));
            recommendations.push("Decrease pipe diameter or increase flow".to_string());
        }

        if re < 2300.0 {
            recommendations.push("Laminar flow - verify if turbulent assumption applies".to_string());
        }

        compliance_notes.push("Calculation per Darcy-Weisbach equation".to_string());
        compliance_notes.push("Add minor losses for fittings if applicable".to_string());

        let results = vec![
            EngineeringResultItem::new("Pressure Drop", pressure_drop, "kPa")
                .critical()
                .with_format(format!("{:.1} kPa", pressure_drop)),
            EngineeringResultItem::new("Velocity", velocity, "m/s")
                .with_format(format!("{:.2} m/s", velocity)),
            EngineeringResultItem::new("Reynolds Number", re, "dimensionless")
                .with_format(format!("{:.0}", re)),
            EngineeringResultItem::new("Friction Factor", friction, "dimensionless")
                .with_format(format!("{:.4}", friction)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "piping_pressure_drop".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "ASME B31.3".to_string(),
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
    async fn test_pressure_drop_calc() {
        let calc = PipingPressureDropCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("length", 100.0),
            ("diameter", 0.1),
        ]);
        
        let mut additional = HashMap::new();
        additional.insert("flow_rate".to_string(), 0.05);
        additional.insert("viscosity".to_string(), WATER_VISCOSITY);
        additional.insert("roughness".to_string(), 0.00015);
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
    fn test_extreme_ld_ratio() {
        let calc = PipingPressureDropCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("length", 10000.0),
            ("diameter", 0.01),
        ]);
        
        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}