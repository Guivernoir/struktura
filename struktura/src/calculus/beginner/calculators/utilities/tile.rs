use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

/// Tile material costs (USD)
const TILE_STANDARD_SIZE: f64 = 0.30;          // 30cm x 30cm
const TILE_COST_PER_M2: f64 = 28.50;
const GROUT_COST_PER_M2: f64 = 2.80;
const THINSET_COST_PER_M2: f64 = 4.25;
const TILE_SPACER_COST_PER_BOX: f64 = 8.50;
const UNDERLAYMENT_COST_PER_M2: f64 = 6.50;    // Cement board

/// Installation costs
const TILE_LABOR_PER_HOUR: f64 = 48.00;        // Skilled trade
const TILE_INSTALL_HOURS_PER_M2: f64 = 0.80;   // 48 minutes per m²
const TILE_SAW_RENTAL_DAILY: f64 = 85.00;

pub struct TileCountCalculator;

#[async_trait]
impl BeginnerCalculator for TileCountCalculator {
    fn id(&self) -> &str {
        "tile_count"
    }

    fn name(&self) -> &str {
        "Tile Installation Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Utilities
    }

    fn metadata(&self) -> BeginnerCalculatorMetadata {
        let parameters = vec![
            ParameterMetadata {
                name: "width".to_string(),
                path: "width".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Area width".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(12.0),
                typical_range: Some((1.0, 8.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Area length".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(15.0),
                typical_range: Some((1.0, 12.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Not used (set to 1.0)".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(1.0),
                typical_range: Some((1.0, 1.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate tiles, thinset, grout, and installation costs for floor or wall tiling projects.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 1.0, 12.0)?;
        self.validate_dimension("length", params.length, 1.0, 15.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let area = params.width * params.length;
        
        // Strategic assessment
        if area > 50.0 {
            warnings.push("Large tiling projects (>50m²) require professional installation for level substrate and consistent grout lines.".to_string());
        }
        if area < 5.0 {
            warnings.push("Small tile jobs have high per-unit costs. Consider bundling multiple small areas into one project.".to_string());
        }
        
        warnings.push("Ensure substrate is level, clean, and properly prepared. Uneven floors will telegraph through tile.".to_string());
        
        // Tile calculations
        let tile_area_each = TILE_STANDARD_SIZE * TILE_STANDARD_SIZE;
        let tiles_needed_base = area / tile_area_each;
        let tiles_needed_with_waste = (tiles_needed_base * (1.0 + WASTE_FACTOR_TILE)).ceil();
        
        // Material costs
        let tile_cost = area * TILE_COST_PER_M2 * (1.0 + WASTE_FACTOR_TILE);
        let thinset_cost = area * THINSET_COST_PER_M2;
        let grout_cost = area * GROUT_COST_PER_M2;
        
        // Additional materials
        let spacer_boxes = (tiles_needed_with_waste / 100.0).ceil();
        let spacer_cost = spacer_boxes * TILE_SPACER_COST_PER_BOX;
        
        // Underlayment (for wood subfloors or waterproofing)
        let underlayment_cost = area * UNDERLAYMENT_COST_PER_M2;
        
        let total_material = tile_cost + thinset_cost + grout_cost + spacer_cost + underlayment_cost;
        
        // Labor estimate
        let labor_hours = area * TILE_INSTALL_HOURS_PER_M2;
        let labor_cost = labor_hours * TILE_LABOR_PER_HOUR;
        
        // Tool rental (tile saw if DIY or small pro job)
        let rental_days = (labor_hours / 8.0).ceil();
        let tool_rental = if area > 10.0 { 
            rental_days * TILE_SAW_RENTAL_DAILY 
        } else { 
            0.0 
        };
        
        let total_project = total_material + labor_cost + tool_rental;
        
        // Additional tactical advice
        if area > 20.0 {
            warnings.push("Consider using leveling clips/spacers for large format tiles to prevent lippage between tiles.".to_string());
        }
        
        let results = vec![
            BeginnerResultItem {
                label: "Total Floor/Wall Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Tiles Required (base)".to_string(),
                value: tiles_needed_base,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Tiles Required (with 12% waste)".to_string(),
                value: tiles_needed_with_waste,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Tile Size Used".to_string(),
                value: TILE_STANDARD_SIZE * 100.0, // Convert to cm
                unit: "cm".to_string(),
            },
            BeginnerResultItem {
                label: "Spacer Boxes Needed".to_string(),
                value: spacer_boxes,
                unit: "boxes".to_string(),
            },
            BeginnerResultItem {
                label: "Tile Cost".to_string(),
                value: tile_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Thinset Mortar Cost".to_string(),
                value: thinset_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Grout Cost".to_string(),
                value: grout_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Underlayment/Waterproofing Cost".to_string(),
                value: underlayment_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Spacers & Small Materials".to_string(),
                value: spacer_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
                value: total_material,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Estimated Labor Hours".to_string(),
                value: labor_hours,
                unit: "hours".to_string(),
            },
            BeginnerResultItem {
                label: "Estimated Labor Cost".to_string(),
                value: labor_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Tool Rental (if needed)".to_string(),
                value: tool_rental,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Project Cost".to_string(),
                value: total_project,
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

impl ParameterValidator for TileCountCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bathroom_floor() {
        let calc = TileCountCalculator;
        let params = BeginnerParameters {
            width: 2.5,
            length: 3.0,
            height: 1.0,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_large_area_warning() {
        let calc = TileCountCalculator;
        let params = BeginnerParameters {
            width: 10.0,
            length: 8.0,
            height: 1.0,
            additional: None,
        };
        
        let result = calc.calculate(params).await.unwrap();
        assert!(!result.warnings.is_empty());
    }
}