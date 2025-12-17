use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;

/// Insulation material costs (USD per m²)
const FIBERGLASS_BATT_R13_PER_M2: f64 = 4.20;   // 2x4 walls
const FIBERGLASS_BATT_R19_PER_M2: f64 = 5.80;   // 2x6 walls
const FIBERGLASS_BATT_R30_PER_M2: f64 = 8.50;   // Ceiling
const SPRAY_FOAM_PER_M2: f64 = 22.00;           // Premium option
const VAPOR_BARRIER_PER_M2: f64 = 1.20;
const INSTALLATION_LABOR_PER_M2: f64 = 3.50;

pub struct InsulationCalculator;

#[async_trait]
impl BeginnerCalculator for InsulationCalculator {
    fn id(&self) -> &str {
        "insulation"
    }

    fn name(&self) -> &str {
        "Wall/Ceiling Insulation Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Interiors
    }

    fn metadata(&self) -> BeginnerCalculatorMetadata {
        let parameters = vec![
            ParameterMetadata {
                name: "width".to_string(),
                path: "width".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Wall/ceiling width".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(15.0),
                typical_range: Some((2.0, 8.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Wall/ceiling length".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(20.0),
                typical_range: Some((3.0, 12.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Cavity depth (0.089 for 2x4, 0.140 for 2x6)".to_string(),
                required: true,
                min_value: Some(0.089),
                max_value: Some(0.305),
                typical_range: Some((0.089, 0.254)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate insulation materials for walls or ceilings with R-value recommendations based on cavity depth.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 1.0, 15.0)?;
        self.validate_dimension("length", params.length, 1.0, 20.0)?;
        self.validate_dimension("height", params.height, 0.089, 0.305)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let area = params.width * params.length;
        
        // Determine insulation type and R-value based on cavity depth
        let (insulation_type, r_value, cost_per_m2) = if params.height <= 0.095 {
            // 2x4 cavity (~3.5 inches)
            ("Fiberglass Batt R-13", "R-13", FIBERGLASS_BATT_R13_PER_M2)
        } else if params.height <= 0.150 {
            // 2x6 cavity (~5.5 inches)
            ("Fiberglass Batt R-19", "R-19", FIBERGLASS_BATT_R19_PER_M2)
        } else {
            // Deeper cavity (ceiling)
            ("Fiberglass Batt R-30", "R-30", FIBERGLASS_BATT_R30_PER_M2)
        };
        
        // Strategic advisories
        if params.height <= 0.095 {
            warnings.push("R-13 insulation in 2x4 walls provides minimal thermal resistance. Consider 2x6 framing for better efficiency.".to_string());
        }
        if area > 80.0 {
            warnings.push("Large insulation projects benefit from professional installation to ensure proper coverage and avoid compression.".to_string());
        }
        
        // Material calculations
        let insulation_cost = area * cost_per_m2;
        let vapor_barrier_cost = area * VAPOR_BARRIER_PER_M2;
        
        // Spray foam alternative (premium)
        let spray_foam_cost = area * SPRAY_FOAM_PER_M2;
        
        let total_material_batt = insulation_cost + vapor_barrier_cost;
        
        // Labor
        let labor_hours = area * 0.08; // ~5 minutes per m²
        let installation_cost = area * INSTALLATION_LABOR_PER_M2;
        
        let total_batt = total_material_batt + installation_cost;
        let total_spray_foam = spray_foam_cost; // Spray foam includes vapor barrier
        
        // Energy savings estimate (rough)
        let annual_savings = area * 1.20; // ~$1.20/m²/year for proper insulation
        
        let results = vec![
            BeginnerResultItem {
                label: "Area to Insulate".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Cavity Depth".to_string(),
                value: params.height,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Recommended Insulation".to_string(),
                value: 0.0, // Placeholder for string
                unit: insulation_type.to_string(),
            },
            BeginnerResultItem {
                label: "R-Value".to_string(),
                value: 0.0,
                unit: r_value.to_string(),
            },
            BeginnerResultItem {
                label: "Fiberglass Batt Cost".to_string(),
                value: insulation_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Vapor Barrier Cost".to_string(),
                value: vapor_barrier_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material (Batt + Barrier)".to_string(),
                value: total_material_batt,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Installation Labor Hours".to_string(),
                value: labor_hours,
                unit: "hours".to_string(),
            },
            BeginnerResultItem {
                label: "Installation Cost".to_string(),
                value: installation_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Cost (Batt System)".to_string(),
                value: total_batt,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Spray Foam Alternative Cost".to_string(),
                value: total_spray_foam,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Estimated Annual Energy Savings".to_string(),
                value: annual_savings,
                unit: "USD/year".to_string(),
            },
        ];

        Ok(BeginnerCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            warnings,
        })
    }
}

impl ParameterValidator for InsulationCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_2x4_wall_insulation() {
        let calc = InsulationCalculator;
        let params = BeginnerParameters {
            width: 4.0,
            length: 3.0,
            height: 0.089,  // 2x4 cavity
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_2x6_wall_insulation() {
        let calc = InsulationCalculator;
        let params = BeginnerParameters {
            width: 4.0,
            length: 3.0,
            height: 0.140,  // 2x6 cavity
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }
}