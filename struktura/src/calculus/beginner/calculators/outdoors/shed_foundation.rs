use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

const CONCRETE_BLOCK_COST: f64 = 4.25;
const PRESSURE_TREATED_SKID_COST_PER_M: f64 = 15.0;
const ANCHOR_BOLT_COST: f64 = 3.50;

pub struct ShedFoundationCalculator;

#[async_trait]
impl BeginnerCalculator for ShedFoundationCalculator {
    fn id(&self) -> &str {
        "shed_foundation"
    }

    fn name(&self) -> &str {
        "Shed Foundation Calculator"
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
                description: "Shed width".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(6.0),
                typical_range: Some((2.4, 4.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Shed length".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(8.0),
                typical_range: Some((3.0, 5.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "foundation_type".to_string(),
                description: "Foundation type (0=blocks, 1=skids, 2=slab)".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(2.0),
                typical_range: Some((0.0, 1.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate foundation materials for sheds: concrete blocks, skids, or slab foundation.".to_string(),
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
                parameter: "foundation_type".to_string(),
                value: params.height.to_string(),
                reason: "Must be 0 (blocks), 1 (skids), or 2 (slab)".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        let area = params.width * params.length;
        let foundation_type = params.height.round() as i32;
        
        if area > 20.0 {
            warnings.push("Sheds >20m² may require building permits in many jurisdictions.".to_string());
        }
        
        let (material_cost, labor_cost, results) = match foundation_type {
            0 => self.calculate_block_foundation(&params, area),
            1 => self.calculate_skid_foundation(&params, area),
            2 => self.calculate_slab_foundation(&params, area),
            _ => return Err(BeginnerError::InvalidParameter {
                parameter: "foundation_type".to_string(),
                value: foundation_type.to_string(),
                reason: "Invalid foundation type".to_string(),
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

impl ShedFoundationCalculator {
    fn calculate_block_foundation(&self, params: &BeginnerParameters, area: f64) -> (f64, f64, Vec<BeginnerResultItem>) {
        // Blocks at 1.2m spacing around perimeter + interior supports
        let perimeter_blocks = ((2.0 * (params.width + params.length)) / 1.2).ceil();
        let interior_supports = ((params.width / 1.2).floor() - 1.0) * ((params.length / 1.2).floor() - 1.0);
        let total_blocks = perimeter_blocks + interior_supports.max(0.0);
        
        let block_cost = total_blocks * CONCRETE_BLOCK_COST;
        
        // Gravel leveling pad
        let gravel_volume = area * 0.10;
        let gravel_cost = gravel_volume * GRAVEL_COST_PER_M3;
        
        let material_cost = block_cost + gravel_cost;
        let labor_cost = (area * 0.5) * GENERAL_LABOR_RATE; // 30 min per m²
        
        let results = vec![
            BeginnerResultItem {
                label: "Shed Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Foundation Type".to_string(),
                value: 0.0,
                unit: "Concrete Blocks".to_string(),
            },
            BeginnerResultItem {
                label: "Concrete Blocks Required".to_string(),
                value: total_blocks,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Leveling Gravel Volume".to_string(),
                value: gravel_volume,
                unit: "m³".to_string(),
            },
        ];
        
        (material_cost, labor_cost, results)
    }
    
    fn calculate_skid_foundation(&self, params: &BeginnerParameters, area: f64) -> (f64, f64, Vec<BeginnerResultItem>) {
        // 3 parallel skids for typical shed
        let num_skids = 3.0;
        let skid_length = params.length * num_skids;
        let skid_cost = skid_length * PRESSURE_TREATED_SKID_COST_PER_M;
        
        // Gravel base under skids
        let gravel_volume = params.length * 0.30 * num_skids * 0.10; // 30cm wide, 10cm deep
        let gravel_cost = gravel_volume * GRAVEL_COST_PER_M3;
        
        // Anchor stakes
        let anchors = num_skids * 4.0; // 4 per skid
        let anchor_cost = anchors * ANCHOR_BOLT_COST;
        
        let material_cost = skid_cost + gravel_cost + anchor_cost;
        let labor_cost = (area * 0.4) * GENERAL_LABOR_RATE; // 24 min per m²
        
        let results = vec![
            BeginnerResultItem {
                label: "Shed Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Foundation Type".to_string(),
                value: 1.0,
                unit: "Pressure-Treated Skids".to_string(),
            },
            BeginnerResultItem {
                label: "Number of Skids".to_string(),
                value: num_skids,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Total Skid Length".to_string(),
                value: skid_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Ground Anchors".to_string(),
                value: anchors,
                unit: "pieces".to_string(),
            },
        ];
        
        (material_cost, labor_cost, results)
    }
    
    fn calculate_slab_foundation(&self, params: &BeginnerParameters, area: f64) -> (f64, f64, Vec<BeginnerResultItem>) {
        let slab_thickness = 0.10; // 10cm slab
        
        let concrete_volume = area * slab_thickness * CONCRETE_WASTE_FACTOR;
        let concrete_cost = concrete_volume * CONCRETE_COST_PER_M3;
        
        let gravel_volume = area * GRAVEL_BASE_THICKNESS;
        let gravel_cost = gravel_volume * GRAVEL_COST_PER_M3;
        
        let rebar_weight = (area * slab_thickness) * REBAR_DENSITY_KG_PER_M3;
        let rebar_cost = rebar_weight * REBAR_COST_PER_KG;
        
        let material_cost = concrete_cost + gravel_cost + rebar_cost;
        let labor_cost = (area * 1.0) * SKILLED_LABOR_RATE; // 1 hour per m²
        
        let results = vec![
            BeginnerResultItem {
                label: "Shed Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Foundation Type".to_string(),
                value: 2.0,
                unit: "Concrete Slab".to_string(),
            },
            BeginnerResultItem {
                label: "Concrete Volume (with waste)".to_string(),
                value: concrete_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Gravel Base Volume".to_string(),
                value: gravel_volume,
                unit: "m³".to_string(),
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

impl ParameterValidator for ShedFoundationCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}