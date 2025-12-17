use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

pub struct RaisedGardenBedCalculator;

#[async_trait]
impl BeginnerCalculator for RaisedGardenBedCalculator {
    fn id(&self) -> &str {
        "raised_garden_bed"
    }

    fn name(&self) -> &str {
        "Raised Garden Bed Calculator"
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
                description: "Bed width (typically 1.0-1.2m for easy reach)".to_string(),
                required: true,
                min_value: Some(0.6),
                max_value: Some(2.0),
                typical_range: Some((1.0, 1.2)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Bed length".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(10.0),
                typical_range: Some((2.4, 6.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Bed height above ground (30-45cm optimal)".to_string(),
                required: true,
                min_value: Some(0.20),
                max_value: Some(0.80),
                typical_range: Some((0.30, 0.45)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate materials for agricultural raised beds optimized for vegetable production with proper drainage.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 0.6, 2.0)?;
        self.validate_dimension("length", params.length, 1.0, 10.0)?;
        self.validate_dimension("height", params.height, 0.20, 0.80)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let area = params.width * params.length;
        let volume = area * params.height;
        
        // Agricultural intelligence
        if params.width > 1.2 {
            warnings.push("Beds wider than 1.2m are difficult to reach from sides. Consider splitting into multiple beds with pathways.".to_string());
        }
        if params.height < 0.30 {
            warnings.push("Shallow beds (<30cm) limit root crops. Increase height for carrots, potatoes, and deep-rooted vegetables.".to_string());
        }
        if params.length > 8.0 {
            warnings.push("Long beds (>8m) should have cross-access paths every 4m for maintenance efficiency.".to_string());
        }
        
        // Optimal soil mix for vegetables (40% topsoil, 30% compost, 20% coco coir/peat, 10% perlite/vermiculite)
        let topsoil_volume = volume * 0.40;
        let compost_volume = volume * 0.30;
        let premium_mix_volume = volume * 0.30; // Coco coir + perlite mix
        
        let topsoil_cost = topsoil_volume * TOPSOIL_COST_PER_M3;
        let compost_cost = compost_volume * COMPOST_COST_PER_M3;
        let premium_mix_cost = premium_mix_volume * (PREMIUM_SOIL_COST_PER_M3 * 1.2);
        let total_soil_cost = topsoil_cost + compost_cost + premium_mix_cost;
        
        // Construction materials
        let perimeter = 2.0 * (params.width + params.length);
        
        // Use 2x lumber (5cm x 20cm boards)
        let board_height = 0.20;
        let num_board_layers = (params.height / board_height).ceil();
        let side_boards = perimeter * num_board_layers;
        
        // Corner posts (10x10cm, extra length for ground anchoring)
        let corner_posts = params.height * 4.0 * 1.2;
        
        // Internal supports (every 1.2m)
        let num_supports = ((params.length / 1.2).floor() - 1.0).max(0.0);
        let support_posts = num_supports * params.height * 2.0; // Both sides
        
        let lumber_cost = (side_boards + corner_posts + support_posts) * TREATED_LUMBER_COST_PER_M;
        
        // Hardware wire mesh bottom (gopher protection)
        let mesh_area = area * 1.05;
        let mesh_cost = mesh_area * 4.50;
        
        // Screws and brackets
        let hardware_cost = (side_boards * 0.60) + (num_supports * 8.0);
        
        // Drainage gravel base (5cm layer)
        let gravel_volume = area * 0.05;
        let gravel_cost = gravel_volume * GRAVEL_COST_PER_M3;
        
        let total_construction = lumber_cost + mesh_cost + hardware_cost + gravel_cost;
        let total_project = total_soil_cost + total_construction;
        
        // Yield estimate (rough)
        let estimated_yield_kg = area * 8.0; // ~8kg per m² for mixed vegetables
        
        warnings.push("Position beds in full sun (6+ hours daily) with north-south orientation for even light distribution.".to_string());
        
        let results = vec![
            BeginnerResultItem {
                label: "Bed Area (planting surface)".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Soil Volume Required".to_string(),
                value: volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Topsoil (40%)".to_string(),
                value: topsoil_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Compost (30%)".to_string(),
                value: compost_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Premium Mix (30%)".to_string(),
                value: premium_mix_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Board Layers Needed".to_string(),
                value: num_board_layers,
                unit: "layers".to_string(),
            },
            BeginnerResultItem {
                label: "Internal Support Posts".to_string(),
                value: num_supports * 2.0,
                unit: "posts".to_string(),
            },
            BeginnerResultItem {
                label: "Drainage Gravel".to_string(),
                value: gravel_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Soil Mix Cost".to_string(),
                value: total_soil_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Lumber & Posts".to_string(),
                value: lumber_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Mesh & Hardware".to_string(),
                value: mesh_cost + hardware_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Drainage Gravel Cost".to_string(),
                value: gravel_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Project Cost".to_string(),
                value: total_project,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Est. Annual Yield (mixed veg)".to_string(),
                value: estimated_yield_kg,
                unit: "kg".to_string(),
            },
        ];

        Ok(BeginnerCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            warnings,
        })
    }
}

impl ParameterValidator for RaisedGardenBedCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}