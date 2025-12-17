use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

const FENCE_PANEL_WIDTH: f64 = 2.4; // Standard 8ft panel
const FENCE_PANEL_COST: f64 = 45.0;
const FENCE_POST_SPACING: f64 = 2.4;
const FENCE_POST_COST: f64 = 18.0;
const CONCRETE_PER_POST_M3: f64 = 0.035; // ~35 liters per post
const GATE_COST: f64 = 120.0;

pub struct FenceCalculator;

#[async_trait]
impl BeginnerCalculator for FenceCalculator {
    fn id(&self) -> &str {
        "fence"
    }

    fn name(&self) -> &str {
        "Fence Builder"
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
                description: "Total fence length (perimeter)".to_string(),
                required: true,
                min_value: Some(3.0),
                max_value: Some(200.0),
                typical_range: Some((10.0, 50.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Fence height".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(3.0),
                typical_range: Some((1.8, 2.4)),
            },
            ParameterMetadata {
                name: "width".to_string(),
                path: "width".to_string(),
                data_type: "number".to_string(),
                unit: "gates".to_string(),
                description: "Number of gates (0-5)".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(5.0),
                typical_range: Some((1.0, 2.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate fence panels, posts, concrete, and gates for perimeter fencing.".to_string(),
            parameters,
            required_parameters: vec!["length".to_string(), "height".to_string()],
            optional_parameters: vec!["width".to_string()],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        if params.length <= 0.0 || params.height <= 0.0 {
            return Err(BeginnerError::DomainError {
                field: "dimensions".to_string(),
                message: "Length and height must be positive".to_string(),
            });
        }
        if params.width < 0.0 || params.width > 5.0 {
            return Err(BeginnerError::DomainError {
                field: "gates".to_string(),
                message: "Number of gates must be between 0 and 5".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        let num_gates = params.width.round(); // Using width as gate count
        
        if params.height > 2.4 {
            warnings.push("Fences >2.4m may require building permits and engineering review.".to_string());
        }
        if params.length > 100.0 {
            warnings.push("Long fence runs (>100m) may require survey and property line verification.".to_string());
        }
        
        // Account for gate openings (typically 1.2m each)
        let gate_length = num_gates * 1.2;
        let fence_panel_length = params.length - gate_length;
        
        // Fence panels and posts
        let num_panels = (fence_panel_length / FENCE_PANEL_WIDTH).ceil();
        let num_posts = num_panels + 1.0 + (num_gates * 2.0); // Extra posts for gates
        
        let panel_cost = num_panels * FENCE_PANEL_COST;
        let post_cost = num_posts * FENCE_POST_COST;
        let gate_cost = num_gates * GATE_COST;
        
        // Concrete for post setting
        let concrete_volume = num_posts * CONCRETE_PER_POST_M3;
        let concrete_cost = concrete_volume * CONCRETE_COST_PER_M3;
        
        // Hardware (hinges, latches, screws)
        let hardware_cost = (num_panels * 2.5) + (num_gates * 25.0);
        
        let total_material_cost = panel_cost + post_cost + gate_cost + 
                                  concrete_cost + hardware_cost;
        
        // Labor: 2 hours per panel section + 3 hours per gate
        let labor_hours = (num_panels * 2.0) + (num_gates * 3.0);
        let labor_cost = labor_hours * GENERAL_LABOR_RATE;
        
        let total_project_cost = total_material_cost + labor_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Total Fence Length".to_string(),
                value: params.length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Fence Panels Required".to_string(),
                value: num_panels,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Posts Required".to_string(),
                value: num_posts,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Gates Required".to_string(),
                value: num_gates,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Concrete for Posts".to_string(),
                value: concrete_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Panel Cost".to_string(),
                value: panel_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Post Cost".to_string(),
                value: post_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Gate Cost".to_string(),
                value: gate_cost,
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

impl ParameterValidator for FenceCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}