use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

const RETAINING_BLOCK_COVERAGE_M2: f64 = 0.09; // 30cm x 30cm face
const RETAINING_BLOCK_COST: f64 = 5.50;
const GEOGRID_COST_PER_M2: f64 = 8.25;
const DRAINAGE_PIPE_COST_PER_M: f64 = 4.50;

pub struct RetainingWallCalculator;

#[async_trait]
impl BeginnerCalculator for RetainingWallCalculator {
    fn id(&self) -> &str {
        "retaining_wall"
    }

    fn name(&self) -> &str {
        "Retaining Wall Builder"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Outdoors
    }

    fn metadata(&self) -> BeginnerCalculatorMetadata {
        let parameters = vec![
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Wall length".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(50.0),
                typical_range: Some((3.0, 20.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Wall height (max 1.2m for DIY)".to_string(),
                required: true,
                min_value: Some(0.3),
                max_value: Some(1.5),
                typical_range: Some((0.5, 1.0)),
            },
            ParameterMetadata {
                name: "width".to_string(),
                path: "width".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Base width (typically 0.4-0.6m)".to_string(),
                required: true,
                min_value: Some(0.3),
                max_value: Some(1.0),
                typical_range: Some((0.4, 0.6)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate retaining wall blocks, base materials, drainage, and reinforcement for landscaping walls.".to_string(),
            parameters,
            required_parameters: vec!["length".to_string(), "height".to_string(), "width".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        if params.length <= 0.0 || params.height <= 0.0 || params.width <= 0.0 {
            return Err(BeginnerError::DomainError {
                field: "dimensions".to_string(),
                message: "All dimensions must be positive".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        if params.height > 1.2 {
            warnings.push("CRITICAL: Walls >1.2m typically require engineering approval and are not suitable for DIY.".to_string());
        }
        if params.height > 0.9 && params.height <= 1.2 {
            warnings.push("Walls >0.9m may require geogrid reinforcement and building permits.".to_string());
        }
        
        // Wall face area
        let face_area = params.length * params.height;
        
        // Retaining blocks with 8% waste
        let blocks_needed = (face_area / RETAINING_BLOCK_COVERAGE_M2).ceil() * 1.08;
        let block_cost = blocks_needed * RETAINING_BLOCK_COST;
        
        // Base gravel (15cm deep, width of base)
        let base_volume = params.length * params.width * 0.15;
        let base_cost = base_volume * GRAVEL_COST_PER_M3;
        
        // Backfill drainage gravel (30cm behind wall)
        let backfill_volume = params.length * 0.30 * params.height;
        let backfill_cost = backfill_volume * GRAVEL_COST_PER_M3;
        
        // Geogrid reinforcement (needed for walls >0.9m)
        let geogrid_area = if params.height > 0.9 {
            params.length * params.width * (params.height / 0.3).ceil() // Every 30cm
        } else {
            0.0
        };
        let geogrid_cost = geogrid_area * GEOGRID_COST_PER_M2;
        
        // Drainage pipe at base
        let drainage_cost = params.length * DRAINAGE_PIPE_COST_PER_M;
        
        // Cap blocks (top course, optional but recommended)
        let cap_blocks = (params.length / 0.45).ceil(); // 45cm cap blocks
        let cap_cost = cap_blocks * 12.0;
        
        let total_material_cost = block_cost + base_cost + backfill_cost + 
                                  geogrid_cost + drainage_cost + cap_cost;
        
        // Labor: 1.5 hours per m² of wall face
        let labor_hours = face_area * 1.5;
        let labor_cost = labor_hours * SKILLED_LABOR_RATE; // Requires skill
        
        let total_project_cost = total_material_cost + labor_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Wall Face Area".to_string(),
                value: face_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Retaining Blocks (incl. 8% waste)".to_string(),
                value: blocks_needed,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Base Gravel Volume".to_string(),
                value: base_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Backfill Drainage Gravel".to_string(),
                value: backfill_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Geogrid Reinforcement Area".to_string(),
                value: geogrid_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Cap Blocks".to_string(),
                value: cap_blocks,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Retaining Block Cost".to_string(),
                value: block_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Base & Backfill Cost".to_string(),
                value: base_cost + backfill_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Drainage & Reinforcement".to_string(),
                value: drainage_cost + geogrid_cost,
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

impl ParameterValidator for RetainingWallCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}