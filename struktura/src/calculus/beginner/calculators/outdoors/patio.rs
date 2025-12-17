use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

const PAVER_SIZE_M2: f64 = 0.04; // 20cm x 20cm standard paver
const PAVER_COST: f64 = 3.50;
const POLYMERIC_SAND_COVERAGE_M2: f64 = 15.0; // per 25kg bag
const POLYMERIC_SAND_COST: f64 = 28.0;
const EDGE_RESTRAINT_COST_PER_M: f64 = 6.75;

pub struct PatioCalculator;

#[async_trait]
impl BeginnerCalculator for PatioCalculator {
    fn id(&self) -> &str {
        "patio"
    }

    fn name(&self) -> &str {
        "Patio Paver Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Outdoors
    }

    fn metadata(&self) -> BeginnerCalculatorMetadata {
        let parameters = vec![
            ParameterMetadata {
                name: "width".to_string(),
                path: "width".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Patio width".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(15.0),
                typical_range: Some((3.0, 8.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Patio length".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(15.0),
                typical_range: Some((3.0, 10.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Base depth (typically 15-20cm)".to_string(),
                required: true,
                min_value: Some(0.10),
                max_value: Some(0.30),
                typical_range: Some((0.15, 0.20)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate pavers, base materials, sand, and edge restraints for patio construction.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        if params.width <= 0.0 || params.length <= 0.0 || params.height <= 0.0 {
            return Err(BeginnerError::DomainError {
                field: "dimensions".to_string(),
                message: "All dimensions must be positive".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        let area = params.width * params.length;
        let perimeter = 2.0 * (params.width + params.length);
        
        if params.height < 0.15 {
            warnings.push("Base depth <15cm may not provide adequate drainage and stability.".to_string());
        }
        if area > 50.0 {
            warnings.push("Large patios (>50m²) may require professional grading and drainage planning.".to_string());
        }
        
        // Pavers with 5% waste for cuts and breakage
        let pavers_needed = (area / PAVER_SIZE_M2).ceil() * 1.05;
        let paver_cost = pavers_needed * PAVER_COST;
        
        // Gravel base (bottom 2/3 of depth)
        let gravel_depth = params.height * 0.67;
        let gravel_volume = area * gravel_depth;
        let gravel_cost = gravel_volume * GRAVEL_COST_PER_M3;
        
        // Sand leveling course (top 1/3 of depth)
        let sand_depth = params.height * 0.33;
        let sand_volume = area * sand_depth;
        let sand_cost = sand_volume * SAND_COST_PER_M3;
        
        // Polymeric sand for joints
        let polymeric_sand_bags = (area / POLYMERIC_SAND_COVERAGE_M2).ceil();
        let polymeric_sand_cost = polymeric_sand_bags * POLYMERIC_SAND_COST;
        
        // Edge restraint
        let edge_restraint_cost = perimeter * EDGE_RESTRAINT_COST_PER_M;
        
        let total_material_cost = paver_cost + gravel_cost + sand_cost + 
                                  polymeric_sand_cost + edge_restraint_cost;
        
        // Labor estimation (0.8 hours per m²)
        let labor_hours = area * 0.8;
        let labor_cost = labor_hours * GENERAL_LABOR_RATE;
        
        let total_project_cost = total_material_cost + labor_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Patio Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Pavers Required (incl. 5% waste)".to_string(),
                value: pavers_needed,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Gravel Base Volume".to_string(),
                value: gravel_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Sand Leveling Volume".to_string(),
                value: sand_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Polymeric Sand Bags".to_string(),
                value: polymeric_sand_bags,
                unit: "bags".to_string(),
            },
            BeginnerResultItem {
                label: "Edge Restraint Length".to_string(),
                value: perimeter,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Paver Cost".to_string(),
                value: paver_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Base Materials Cost".to_string(),
                value: gravel_cost + sand_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Joint Sand & Edge Restraint".to_string(),
                value: polymeric_sand_cost + edge_restraint_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
                value: total_material_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Estimated Labor Cost".to_string(),
                value: labor_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Project Cost".to_string(),
                value: total_project_cost,
                unit: "USD".to_string(),
            },
        ];

        Ok(BeginnerCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            warnings,
        })
    }
}

impl ParameterValidator for PatioCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}