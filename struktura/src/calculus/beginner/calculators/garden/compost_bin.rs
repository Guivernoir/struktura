use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

pub struct CompostBinCalculator;

#[async_trait]
impl BeginnerCalculator for CompostBinCalculator {
    fn id(&self) -> &str {
        "compost_bin"
    }

    fn name(&self) -> &str {
        "Compost Bin Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Garden
    }

    fn metadata(&self) -> BeginnerCalculatorMetadata {
        let parameters = vec![
            ParameterMetadata {
                name: "width".to_string(),
                path: "width".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Single bin width (typically 1.0-1.2m)".to_string(),
                required: true,
                min_value: Some(0.9),
                max_value: Some(1.5),
                typical_range: Some((1.0, 1.2)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Single bin length (typically 1.0-1.2m)".to_string(),
                required: true,
                min_value: Some(0.9),
                max_value: Some(1.5),
                typical_range: Some((1.0, 1.2)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Bin height (typically 0.9-1.2m for hot composting)".to_string(),
                required: true,
                min_value: Some(0.8),
                max_value: Some(1.5),
                typical_range: Some((0.9, 1.2)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculates materials for 3-bin composting systems".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 0.9, 1.5)?;
        self.validate_dimension("length", params.length, 0.9, 1.5)?;
        self.validate_dimension("height", params.height, 0.8, 1.5)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();

        // Single bin volume
        let single_volume = params.width * params.length * params.height;
        let total_volume = single_volume * 3.0;

        // Optimal for hot composting
        if single_volume < 1.0 {
            warnings.push("Bins smaller than 1m³ may not reach hot composting temperatures (55-65°C). Consider larger dimensions.".to_string());
        }

        // Materials: Assume cedar for durability
        // Walls: 4 sides per bin, but shared walls in 3-bin system
        let num_front_back = 3.0 * 2.0; // Front and back for each bin
        let num_sides = 4.0; // Outer sides + internal dividers
        let board_height = 0.15; // Assume 15cm boards
        let rows_per_wall = (params.height / board_height).ceil();
        let board_length_front_back = params.width;
        let board_length_sides = params.length;
        let total_boards_front_back = num_front_back * rows_per_wall;
        let total_boards_sides = num_sides * rows_per_wall;
        let total_lumber_m = (total_boards_front_back * board_length_front_back) + (total_boards_sides * board_length_sides);
        let lumber_cost = total_lumber_m * CEDAR_BOARD_COST_PER_M;

        // Wire mesh for ventilation (bottom and sides)
        let mesh_area_per_bin = (params.width * params.length) + (2.0 * params.width * params.height) + (2.0 * params.length * params.height);
        let total_mesh_area = mesh_area_per_bin * 3.0 * 0.5; // Half mesh for ventilation
        let mesh_cost = total_mesh_area * 5.0; // Assume $5/m²

        // Hinges for lids (2 per bin)
        let hinges = 6.0;
        let hinges_cost = hinges * 4.50;

        // Ventilation holes: Assume drilled in wood, spacing every 15cm
        let holes_per_wall = ((params.height / 0.15) * (params.width / 0.15)) as f64;

        // Capacity: Assume 4 turns per year
        let annual_capacity_m3 = total_volume * 4.0;

        // Waste factor
        let waste_lumber = total_lumber_m * WASTE_FACTOR_LUMBER;

        let total_cost = lumber_cost + mesh_cost + hinges_cost;

        let results = vec![
            BeginnerResultItem {
                label: "Single Bin Volume".to_string(),
                value: single_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Total System Volume".to_string(),
                value: total_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Annual Composting Capacity".to_string(),
                value: annual_capacity_m3,
                unit: "m³/year".to_string(),
            },
            BeginnerResultItem {
                label: "Cedar Lumber Required".to_string(),
                value: total_lumber_m + waste_lumber,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Wire Mesh Area".to_string(),
                value: total_mesh_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Hinges for Lids".to_string(),
                value: hinges,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Ventilation Holes per Wall".to_string(),
                value: holes_per_wall,
                unit: "holes".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
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

impl ParameterValidator for CompostBinCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_standard_compost_bin() {
        let calc = CompostBinCalculator;
        let params = BeginnerParameters {
            width: 1.0,
            length: 1.0,
            height: 1.0,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }
}