use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

const ASPHALT_COST_PER_M2: f64 = 25.0;
const ASPHALT_THICKNESS: f64 = 0.08; // 8cm
const EDGE_CURB_COST_PER_M: f64 = 18.0;

pub struct DrivewayCalculator;

#[async_trait]
impl BeginnerCalculator for DrivewayCalculator {
    fn id(&self) -> &str {
        "driveway"
    }

    fn name(&self) -> &str {
        "Driveway Calculator"
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
                description: "Driveway width (typically 3-4m)".to_string(),
                required: true,
                min_value: Some(2.5),
                max_value: Some(6.0),
                typical_range: Some((3.0, 4.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Driveway length".to_string(),
                required: true,
                min_value: Some(5.0),
                max_value: Some(50.0),
                typical_range: Some((10.0, 25.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "surface_type".to_string(),
                description: "Surface type (0=gravel, 1=asphalt, 2=concrete)".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(2.0),
                typical_range: Some((0.0, 2.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate materials for gravel, asphalt, or concrete driveways with base preparation.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        if params.width <= 0.0 || params.length <= 0.0 {
            return Err(BeginnerError::DomainError {
                field: "dimensions".to_string(),
                message: "Width and length must be positive".to_string(),
            });
        }
        if params.height < 0.0 || params.height > 2.0 {
            return Err(BeginnerError::InvalidParameter {
                parameter: "surface_type".to_string(),
                value: params.height.to_string(),
                reason: "Must be 0 (gravel), 1 (asphalt), or 2 (concrete)".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        let area = params.width * params.length;
        let perimeter = 2.0 * (params.width + params.length);
        let surface_type = params.height.round() as i32;
        
        if params.width < 3.0 {
            warnings.push("Driveway width <3m may be tight for larger vehicles.".to_string());
        }
        if area > 100.0 {
            warnings.push("Large driveways (>100m²) typically require professional grading and permits.".to_string());
        }
        
        let (material_cost, labor_cost, results) = match surface_type {
            0 => self.calculate_gravel_driveway(&params, area, perimeter),
            1 => self.calculate_asphalt_driveway(&params, area, perimeter),
            2 => self.calculate_concrete_driveway(&params, area, perimeter),
            _ => return Err(BeginnerError::InvalidParameter {
                parameter: "surface_type".to_string(),
                value: surface_type.to_string(),
                reason: "Invalid surface type".to_string(),
            }),
        };
        
        let total_project_cost = material_cost + labor_cost;
        
        let mut final_results = results;
        final_results.push(BeginnerResultItem {
            label: "Total Material Cost".to_string(),
            value: material_cost,
            unit: "USD".to_string(),
        });
        final_results.push(BeginnerResultItem {
            label: "Estimated Labor Cost".to_string(),
            value: labor_cost,
            unit: "USD".to_string(),
        });
        final_results.push(BeginnerResultItem {
            label: "Total Project Cost".to_string(),
            value: total_project_cost,
            unit: "USD".to_string(),
        });
        
        Ok(BeginnerCalculationResponse {
            calculation_type: self.id().to_string(),
            results: final_results,
            warnings,
        })
    }
}

impl DrivewayCalculator {
    fn calculate_gravel_driveway(&self, params: &BeginnerParameters, area: f64, perimeter: f64) -> (f64, f64, Vec<BeginnerResultItem>) {
        let gravel_depth = 0.15; // 15cm
        let gravel_volume = area * gravel_depth;
        let gravel_cost = gravel_volume * GRAVEL_COST_PER_M3;
        
        // Landscape fabric to prevent weeds
        let fabric_cost = area * 2.50;
        
        // Edge restraint (optional but recommended)
        let edge_cost = perimeter * 5.50;
        
        let material_cost = gravel_cost + fabric_cost + edge_cost;
        let labor_cost = (area * 0.3) * GENERAL_LABOR_RATE; // 18 min per m²
        
        let results = vec![
            BeginnerResultItem {
                label: "Driveway Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Surface Type".to_string(),
                value: 0.0,
                unit: "Gravel".to_string(),
            },
            BeginnerResultItem {
                label: "Gravel Volume (15cm depth)".to_string(),
                value: gravel_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Landscape Fabric Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Edge Restraint Length".to_string(),
                value: perimeter,
                unit: "m".to_string(),
            },
        ];
        
        (material_cost, labor_cost, results)
    }
    
    fn calculate_asphalt_driveway(&self, params: &BeginnerParameters, area: f64, perimeter: f64) -> (f64, f64, Vec<BeginnerResultItem>) {
        // Base gravel layer (20cm)
        let base_volume = area * 0.20;
        let base_cost = base_volume * GRAVEL_COST_PER_M3;
        
        // Asphalt surface
        let asphalt_cost = area * ASPHALT_COST_PER_M2;
        
        // Edge curbing
        let curb_cost = perimeter * EDGE_CURB_COST_PER_M;
        
        let material_cost = base_cost + asphalt_cost + curb_cost;
        let labor_cost = (area * 1.2) * SKILLED_LABOR_RATE; // 72 min per m²
        
        let results = vec![
            BeginnerResultItem {
                label: "Driveway Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Surface Type".to_string(),
                value: 1.0,
                unit: "Asphalt".to_string(),
            },
            BeginnerResultItem {
                label: "Base Gravel Volume".to_string(),
                value: base_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Asphalt Thickness".to_string(),
                value: ASPHALT_THICKNESS * 100.0,
                unit: "cm".to_string(),
            },
            BeginnerResultItem {
                label: "Edge Curbing Length".to_string(),
                value: perimeter,
                unit: "m".to_string(),
            },
        ];
        
        (material_cost, labor_cost, results)
    }
    
    fn calculate_concrete_driveway(&self, params: &BeginnerParameters, area: f64, perimeter: f64) -> (f64, f64, Vec<BeginnerResultItem>) {
        let concrete_thickness = 0.12; // 12cm for driveways
        
        // Base gravel
        let base_volume = area * GRAVEL_BASE_THICKNESS;
        let base_cost = base_volume * GRAVEL_COST_PER_M3;
        
        // Concrete with waste
        let concrete_volume = area * concrete_thickness * CONCRETE_WASTE_FACTOR;
        let concrete_cost = concrete_volume * CONCRETE_COST_PER_M3;
        
        // Rebar reinforcement
        let rebar_weight = (area * concrete_thickness) * REBAR_DENSITY_KG_PER_M3;
        let rebar_cost = rebar_weight * REBAR_COST_PER_KG;
        
        // Control joints and sealing
        let joint_cost = area * 1.50;
        
        let material_cost = base_cost + concrete_cost + rebar_cost + joint_cost;
        let labor_cost = (area * 1.5) * SKILLED_LABOR_RATE; // 90 min per m²
        
        let results = vec![
            BeginnerResultItem {
                label: "Driveway Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Surface Type".to_string(),
                value: 2.0,
                unit: "Concrete".to_string(),
            },
            BeginnerResultItem {
                label: "Base Gravel Volume".to_string(),
                value: base_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Concrete Volume (with waste)".to_string(),
                value: concrete_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Concrete Thickness".to_string(),
                value: concrete_thickness * 100.0,
                unit: "cm".to_string(),
            },
            BeginnerResultItem {
                label: "Rebar Weight".to_string(),
                value: rebar_weight,
                unit: "kg".to_string(),
            },
        ];
        
        (material_cost, labor_cost, results)
    }
}

impl ParameterValidator for DrivewayCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}