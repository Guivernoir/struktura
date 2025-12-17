use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

const RAFTER_SPACING: f64 = 0.6; // 60cm spacing
const CROSSBEAM_SPACING: f64 = 0.4; // 40cm spacing
const LATTICE_PANEL_COST: f64 = 35.0;

pub struct PergolaCalculator;

#[async_trait]
impl BeginnerCalculator for PergolaCalculator {
    fn id(&self) -> &str {
        "pergola"
    }

    fn name(&self) -> &str {
        "Pergola Builder"
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
                description: "Pergola width".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(8.0),
                typical_range: Some((3.0, 5.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Pergola length".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(10.0),
                typical_range: Some((3.0, 6.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Post height (clearance)".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(3.5),
                typical_range: Some((2.4, 3.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate posts, beams, rafters, and hardware for freestanding or attached pergolas.".to_string(),
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
        
        if area > 30.0 {
            warnings.push("Large pergolas (>30m²) may require engineering review and building permits.".to_string());
        }
        if params.height < 2.2 {
            warnings.push("Pergola height <2.2m may feel cramped for typical outdoor furniture and activities.".to_string());
        }
        
        // Posts: 4 corner posts minimum, add more for spans >3m
        let posts_per_side_width = (params.width / 3.0).ceil().max(2.0);
        let posts_per_side_length = (params.length / 3.0).ceil().max(2.0);
        let num_posts = posts_per_side_width * posts_per_side_length;
        
        // Post length includes 60cm for concrete footing
        let post_length = params.height + 0.6;
        let post_cost = num_posts * post_length * TREATED_4X4_COST_PER_M;
        
        // Main beams (run along length, supported by posts)
        let beam_length = params.length * posts_per_side_width; // One beam per post row
        let beam_cost = beam_length * TREATED_2X6_COST_PER_M * 2.0; // Double 2x6
        
        // Rafters (perpendicular to beams)
        let num_rafters = (params.length / RAFTER_SPACING).ceil() + 1.0;
        let rafter_length = params.width * num_rafters;
        let rafter_cost = rafter_length * TREATED_2X6_COST_PER_M;
        
        // Crossbeams (decorative, on top of rafters)
        let num_crossbeams = (params.width / CROSSBEAM_SPACING).ceil();
        let crossbeam_length = params.length * num_crossbeams;
        let crossbeam_cost = crossbeam_length * TREATED_2X4_COST_PER_M;
        
        // Concrete for post footings
        let concrete_per_footing = 0.04; // 40 liters per post
        let concrete_volume = num_posts * concrete_per_footing;
        let concrete_cost = concrete_volume * CONCRETE_COST_PER_M3;
        
        // Hardware (post anchors, brackets, bolts, screws)
        let hardware_cost = (num_posts * POST_ANCHOR_COST) + 
                           (num_rafters * 4.0) + // Rafter ties
                           (area * 2.5); // Misc screws/bolts
        
        let total_material_cost = post_cost + beam_cost + rafter_cost + 
                                  crossbeam_cost + concrete_cost + hardware_cost;
        
        // Labor: 3 hours per post + 1.5 hours per m² of coverage
        let labor_hours = (num_posts * 3.0) + (area * 1.5);
        let labor_cost = labor_hours * SKILLED_LABOR_RATE;
        
        let total_project_cost = total_material_cost + labor_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Pergola Coverage Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Posts Required".to_string(),
                value: num_posts,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Post Length (each)".to_string(),
                value: post_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Main Beam Length".to_string(),
                value: beam_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Rafters Required".to_string(),
                value: num_rafters,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Crossbeams Required".to_string(),
                value: num_crossbeams,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Concrete for Footings".to_string(),
                value: concrete_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Structural Lumber Cost".to_string(),
                value: post_cost + beam_cost + rafter_cost + crossbeam_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Concrete & Hardware".to_string(),
                value: concrete_cost + hardware_cost,
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

impl ParameterValidator for PergolaCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}