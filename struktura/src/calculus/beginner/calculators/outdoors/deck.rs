use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

// Deck-specific constants
const DECK_BOARD_COVERAGE_M2: f64 = 0.18; // Standard 140mm x 1.2m board
const DECK_JOIST_SPACING: f64 = 0.4; // 40cm joist spacing
const DECK_BOARD_COST: f64 = 12.50;

pub struct DeckCalculator;

#[async_trait]
impl BeginnerCalculator for DeckCalculator {
    fn id(&self) -> &str {
        "deck"
    }

    fn name(&self) -> &str {
        "Deck Builder"
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
                description: "Deck width".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(10.0),
                typical_range: Some((3.0, 6.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Deck length".to_string(),
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
                description: "Deck height (elevation)".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(3.0),
                typical_range: Some((0.3, 1.5)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate decking boards, joists, and support materials for outdoor decks.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        if params.width <= 0.0 || params.length <= 0.0 || params.height < 0.0 {
            return Err(BeginnerError::DomainError {
                field: "dimensions".to_string(),
                message: "Dimensions must be positive".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        let area = params.width * params.length;
        
        if area > 40.0 {
            warnings.push("Large deck areas (>40m²) may require structural engineer review and permits.".to_string());
        }
        if params.height > 1.5 {
            warnings.push("Elevated decks >1.5m typically require railings and additional support posts.".to_string());
        }
        
        let boards_needed = (area / DECK_BOARD_COVERAGE_M2).ceil() * 1.10;
        let board_cost = boards_needed * DECK_BOARD_COST;
        
        let num_joists = (params.length / DECK_JOIST_SPACING).ceil() + 2.0;
        let total_joist_length = num_joists * params.width;
        let joist_cost = total_joist_length * TREATED_2X6_COST_PER_M;
        
        let hardware_cost = area * HARDWARE_COST_PER_M2;
        
        let posts_needed = if params.height > 0.5 {
            ((params.width / 2.0).ceil() * (params.length / 2.0).ceil()).max(4.0)
        } else {
            0.0
        };
        let post_cost = posts_needed * 15.0;
        
        let total_cost = board_cost + joist_cost + hardware_cost + post_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Total Deck Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Decking Boards Required (incl. 10% waste)".to_string(),
                value: boards_needed,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Structural Joists Required".to_string(),
                value: num_joists,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Total Joist Length".to_string(),
                value: total_joist_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Support Posts Needed".to_string(),
                value: posts_needed,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Materials Cost (Boards)".to_string(),
                value: board_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Structural Cost (Joists)".to_string(),
                value: joist_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Hardware & Fasteners".to_string(),
                value: hardware_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Estimated Cost".to_string(),
                value: total_cost,
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

impl ParameterValidator for DeckCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}