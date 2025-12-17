use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

use super::material_handling::*;

pub struct ConveyorBeltCalculator;

impl ParameterValidator for ConveyorBeltCalculator {
    fn calculator_id(&self) -> &str {
        "conveyor_belt"
    }
}

#[async_trait]
impl EngineerCalculator for ConveyorBeltCalculator {
    fn id(&self) -> &str {
        "conveyor_belt"
    }

    fn name(&self) -> &str {
        "Belt Conveyor Design and Sizing"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Production
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("conveyor_belt", "Belt Conveyor Design and Sizing")
            .category("production")
            .description("Calculate belt tensions, motor power, and capacity for material handling conveyors per CEMA standards")
            .design_code("CEMA")
            .design_code("ISO 5048")
            .parameter(ParameterMetadata {
                name: "Belt Length".to_string(),
                path: "dimensions.length".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Total center-to-center length of conveyor".to_string(),
                required: true,
                default_value: Some(50.0),
                min_value: Some(5.0),
                max_value: Some(500.0),
                typical_range: Some((20.0, 200.0)),
                validation_rules: Some(vec!["Must be positive".to_string()]),
            })
            .parameter(ParameterMetadata {
                name: "Belt Width".to_string(),
                path: "dimensions.width".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Belt width (standard sizes: 0.4, 0.5, 0.65, 0.8, 1.0, 1.2, 1.4 m)".to_string(),
                required: true,
                default_value: Some(0.8),
                min_value: Some(0.3),
                max_value: Some(3.0),
                typical_range: Some((0.5, 1.5)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Belt Speed".to_string(),
                path: "additional.belt_speed".to_string(),
                data_type: ParameterType::Number,
                unit: "m/s".to_string(),
                description: "Linear belt velocity".to_string(),
                required: true,
                default_value: Some(1.5),
                min_value: Some(0.5),
                max_value: Some(4.0),
                typical_range: Some((1.0, 2.5)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Material Density".to_string(),
                path: "material.density".to_string(),
                data_type: ParameterType::Number,
                unit: "kg/m³".to_string(),
                description: "Bulk density of material being conveyed".to_string(),
                required: false,
                default_value: Some(1600.0),
                min_value: Some(500.0),
                max_value: Some(3000.0),
                typical_range: Some((1000.0, 2000.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Inclination Angle".to_string(),
                path: "additional.inclination_angle".to_string(),
                data_type: ParameterType::Number,
                unit: "degrees".to_string(),
                description: "Belt inclination from horizontal (positive = upward)".to_string(),
                required: false,
                default_value: Some(0.0),
                min_value: Some(-20.0),
                max_value: Some(20.0),
                typical_range: Some((-10.0, 15.0)),
                validation_rules: Some(vec!["Steep angles (>18°) require cleated belts".to_string()]),
            })
            .parameter(ParameterMetadata {
                name: "Surcharge Angle".to_string(),
                path: "additional.surcharge_angle".to_string(),
                data_type: ParameterType::Number,
                unit: "degrees".to_string(),
                description: "Material surcharge angle (material piling on belt)".to_string(),
                required: false,
                default_value: Some(20.0),
                min_value: Some(0.0),
                max_value: Some(35.0),
                typical_range: Some((15.0, 25.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        // Validate dimensions
        self.validate_dimension("dimensions.length", 
            params.dimensions.get("length").copied(), 5.0, 500.0)?;
        
        self.validate_dimension("dimensions.width",
            params.dimensions.get("width").copied(), 0.3, 3.0)?;

        // Validate belt speed
        let belt_speed = self.get_additional_param(params, "belt_speed", Some(0.5), Some(4.0))?;

        // Validate inclination if provided
        if let Some(additional) = &params.additional {
            if let Some(angle) = additional.get("inclination_angle") {
                if angle.abs() > 20.0 {
                    return Err(EngineeringError::InvalidParameter {
                        parameter: "inclination_angle".to_string(),
                        value: angle.to_string(),
                        reason: "Inclination must be between -20° and +20°".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) 
        -> EngineeringResult<EngineeringCalculationResponse> {
        
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        // Extract parameters
        let belt_length = params.dimensions.get("length").copied().unwrap_or(50.0);
        let belt_width = params.dimensions.get("width").copied().unwrap_or(0.8);
        let belt_speed = params.additional.as_ref()
            .and_then(|a| a.get("belt_speed"))
            .copied()
            .unwrap_or(1.5);

        let material_density = params.material.as_ref()
            .and_then(|m| m.density)
            .unwrap_or(DENSITY_SAND);

        let inclination_angle = params.additional.as_ref()
            .and_then(|a| a.get("inclination_angle"))
            .copied()
            .unwrap_or(0.0);

        let surcharge_angle = params.additional.as_ref()
            .and_then(|a| a.get("surcharge_angle"))
            .copied()
            .unwrap_or(SURCHARGE_ANGLE_TYPICAL);

        let operating_hours = params.additional.as_ref()
            .and_then(|a| a.get("operating_hours_per_year"))
            .copied()
            .unwrap_or(5000.0);

        // Belt loading capacity (CEMA formula for troughed belt)
        let surcharge_factor = 1.0 + (surcharge_angle / 90.0) * 0.5;
        let load_cross_section = belt_width.powi(2) * 0.08 * surcharge_factor; // m²

        // Volumetric capacity
        let volumetric_capacity = load_cross_section * belt_speed * 3600.0; // m³/h

        // Mass capacity
        let mass_capacity = volumetric_capacity * material_density / 1000.0; // tonnes/h

        // Belt tensions
        let belt_weight_per_meter = belt_width * 15.0; // kg/m (approximate)
        let material_weight_per_meter = load_cross_section * material_density; // kg/m

        let gravity = 9.81;
        
        // Effective tension
        let te = (belt_weight_per_meter + material_weight_per_meter) * 
                 gravity * belt_length * 
                 (FRICTION_BELT_IDLER + inclination_angle.to_radians().sin());

        // Slack side tension (minimum to prevent sag)
        let t2 = 50.0 * belt_width * 1000.0; // N (empirical minimum)

        // Tight side tension
        let t1 = te + t2;

        // Required motor power
        let power_required = (te * belt_speed) / 1000.0; // kW
        let motor_power = power_required * 1.15; // 15% safety factor

        // Annual throughput
        let annual_throughput = mass_capacity * operating_hours;

        // Belt tension safety factor
        let belt_strength_required = t1 * 10.0; // N/mm width (10:1 safety factor)
        let belt_strength_per_mm = belt_strength_required / (belt_width * 1000.0);

        // Warnings and recommendations
        if inclination_angle > 18.0 {
            warnings.push(format!(
                "Steep inclination ({:.1}°). Material may roll back. Use cleated belt or reduce angle.",
                inclination_angle
            ));
        }

        if belt_speed > 3.0 {
            warnings.push(format!(
                "High belt speed ({:.1} m/s). Verify material spillage control and dust suppression.",
                belt_speed
            ));
            recommendations.push("Install dust suppression system at transfer points".to_string());
        }

        if motor_power > 75.0 {
            recommendations.push("Large motor (>75kW). Consider soft start or VFD to reduce starting current".to_string());
        }

        if belt_length > 100.0 {
            recommendations.push("Long conveyor. Install automatic take-up system for belt tensioning".to_string());
        }

        if mass_capacity > 500.0 {
            recommendations.push("High capacity system. Verify structural support design and idler spacing".to_string());
        }

        if belt_speed < BELT_SPEED_MIN {
            warnings.push("Belt speed below minimum recommended. Material may not discharge properly".to_string());
        }

        compliance_notes.push("Design per CEMA (Conveyor Equipment Manufacturers Association) standards".to_string());
        compliance_notes.push("Verify belt rating matches calculated tension with appropriate safety factor".to_string());
        compliance_notes.push("Install emergency pull cords and guards per OSHA requirements".to_string());
        compliance_notes.push("Consider impact idlers at loading zones to protect belt".to_string());
        compliance_notes.push("Ensure adequate drainage to prevent material buildup".to_string());

        let results = vec![
            EngineeringResultItem::new("Belt Length", belt_length, "m"),
            EngineeringResultItem::new("Belt Width", belt_width, "m"),
            EngineeringResultItem::new("Belt Speed", belt_speed, "m/s"),
            EngineeringResultItem::new("Volumetric Capacity", volumetric_capacity, "m³/h")
                .with_tolerance(0.10),
            EngineeringResultItem::new("Mass Capacity", mass_capacity, "tonnes/h")
                .with_tolerance(0.10)
                .critical(),
            EngineeringResultItem::new("Effective Tension (Te)", te / 1000.0, "kN")
                .with_tolerance(0.15)
                .critical(),
            EngineeringResultItem::new("Tight Side Tension (T1)", t1 / 1000.0, "kN")
                .with_tolerance(0.15)
                .critical(),
            EngineeringResultItem::new("Required Motor Power", motor_power, "kW")
                .with_tolerance(0.20)
                .critical()
                .with_format(format!("{:.1} kW", motor_power)),
            EngineeringResultItem::new("Belt Strength Required", belt_strength_per_mm, "N/mm")
                .critical(),
            EngineeringResultItem::new("Annual Throughput", annual_throughput, "tonnes/year")
                .with_tolerance(0.15),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "conveyor_belt".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "CEMA".to_string(),
                requires_pe_review: false,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::engineer::test_utils::*;

    #[tokio::test]
    async fn test_conveyor_basic_calculation() {
        let calc = ConveyorBeltCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("length", 50.0),
            ("width", 0.8),
        ]);
        
        let mut additional = HashMap::new();
        additional.insert("belt_speed".to_string(), 1.5);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() > 5);
        assert_eq!(response.calculation_type, "conveyor_belt");
    }

    #[test]
    fn test_steep_angle_warning() {
        let calc = ConveyorBeltCalculator;
        
        let mut params = minimal_parameters();
        params.dimensions.insert("length".to_string(), 50.0);
        params.dimensions.insert("width".to_string(), 0.8);
        
        let mut additional = HashMap::new();
        additional.insert("inclination_angle".to_string(), 25.0); // Too steep
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}