use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

pub struct PlanterBoxCalculator;

#[async_trait]
impl BeginnerCalculator for PlanterBoxCalculator {
    fn id(&self) -> &str {
        "planter_box"
    }

    fn name(&self) -> &str {
        "Planter Box Calculator"
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
                description: "Interior width of planter".to_string(),
                required: true,
                min_value: Some(0.3),
                max_value: Some(3.0),
                typical_range: Some((0.6, 2.5)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Interior length of planter".to_string(),
                required: true,
                min_value: Some(0.5),
                max_value: Some(6.0),
                typical_range: Some((1.0, 5.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Planter depth (typically 30-60cm)".to_string(),
                required: true,
                min_value: Some(0.1),
                max_value: Some(1.2),
                typical_range: Some((0.2, 0.8)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate soil volume, construction materials, and costs for raised planter boxes with optimal soil mix.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 0.3, 3.0)?;
        self.validate_dimension("length", params.length, 0.5, 6.0)?;
        self.validate_dimension("height", params.height, 0.1, 1.2)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        // Soil volume calculation
        let soil_volume = params.width * params.length * params.height;
        
        // Strategic horticultural assessment
        if params.height < MINIMUM_ROOT_DEPTH {
            warnings.push("Shallow planters (<20cm) limit root growth for most vegetables and perennials. Consider deeper construction.".to_string());
        }
        if params.height > 0.80 {
            warnings.push("Deep planters (>80cm) require significant soil volume and may need internal support structures or corner bracing.".to_string());
        }
        if soil_volume > 3.0 {
            warnings.push("Large planters (>3m³ soil) should have drainage holes every 30-40cm along the bottom to prevent waterlogging.".to_string());
        }
        if params.width < 0.40 {
            warnings.push("Narrow planters restrict plant spacing. Consider wider design for better root development.".to_string());
        }
        
        // Optimal soil mix calculation (60% topsoil, 30% compost, 10% premium mix)
        let topsoil_volume = soil_volume * 0.60;
        let compost_volume = soil_volume * 0.30;
        let premium_soil_volume = soil_volume * 0.10;
        
        let topsoil_cost = topsoil_volume * TOPSOIL_COST_PER_M3;
        let compost_cost = compost_volume * COMPOST_COST_PER_M3;
        let premium_soil_cost = premium_soil_volume * PREMIUM_SOIL_COST_PER_M3;
        let total_soil_cost = topsoil_cost + compost_cost + premium_soil_cost;
        
        // Box construction materials
        let perimeter = 2.0 * (params.width + params.length);
        
        // Side boards (assuming 15cm board height)
        let board_height = 0.15;
        let side_board_rows = (params.height / board_height).ceil();
        let side_board_length = perimeter * side_board_rows;
        
        // Corner posts (4 corners, 10cm x 10cm posts)
        let corner_post_length = params.height * 4.0 * 1.1; // +10% for ground anchoring
        
        // Bottom drainage slats (10cm spacing)
        let num_slats = (params.width / 0.10).ceil();
        let bottom_slat_length = params.length * num_slats;
        
        let total_cedar_length = side_board_length + bottom_slat_length;
        let cedar_cost = total_cedar_length * CEDAR_BOARD_COST_PER_M;
        let corner_post_cost = corner_post_length * TREATED_LUMBER_COST_PER_M;
        
        // Hardware and accessories
        let screws_cost = (side_board_length * 0.5) + (bottom_slat_length * 0.3); // ~$0.50 per meter for sides
        let landscape_fabric = params.width * params.length * LANDSCAPE_FABRIC_COST_PER_M2;
        
        // Optional: decorative mulch top layer (5cm depth)
        let mulch_volume = params.width * params.length * MULCH_LAYER_STANDARD;
        let mulch_cost = mulch_volume * MULCH_COST_PER_M3;
        
        let total_construction_cost = cedar_cost + corner_post_cost + screws_cost + landscape_fabric;
        let total_project_cost = total_soil_cost + total_construction_cost + mulch_cost;
        
        // Labor estimate (DIY-friendly project)
        let labor_hours = 3.0 + (perimeter * 0.15); // 3h base + time per meter
        
        warnings.push("Cedar naturally resists rot and insects. Alternative: use pressure-treated lumber with plastic liner for food safety.".to_string());
        
        let results = vec![
            BeginnerResultItem {
                label: "Planter Box Volume".to_string(),
                value: soil_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Topsoil Required (60%)".to_string(),
                value: topsoil_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Compost Required (30%)".to_string(),
                value: compost_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Premium Soil Mix (10%)".to_string(),
                value: premium_soil_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Side Board Rows Needed".to_string(),
                value: side_board_rows,
                unit: "rows".to_string(),
            },
            BeginnerResultItem {
                label: "Cedar Boards Length".to_string(),
                value: total_cedar_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Corner Post Length".to_string(),
                value: corner_post_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Mulch Volume (top layer)".to_string(),
                value: mulch_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Soil Mix Cost".to_string(),
                value: total_soil_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Cedar & Posts Cost".to_string(),
                value: cedar_cost + corner_post_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Hardware & Fabric".to_string(),
                value: screws_cost + landscape_fabric,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Mulch Cost".to_string(),
                value: mulch_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
                value: total_project_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Estimated Build Time (DIY)".to_string(),
                value: labor_hours,
                unit: "hours".to_string(),
            },
        ];

        Ok(BeginnerCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            warnings,
        })
    }
}

impl ParameterValidator for PlanterBoxCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_standard_planter() {
        let calc = PlanterBoxCalculator;
        let params = BeginnerParameters {
            width: 1.2,
            length: 2.4,
            height: 0.40,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_shallow_planter_warning() {
        let calc = PlanterBoxCalculator;
        let params = BeginnerParameters {
            width: 1.0,
            length: 2.0,
            height: 0.15,  // Shallow
            additional: None,
        };
        
        let result = calc.calculate(params).await.unwrap();
        assert!(!result.warnings.is_empty());
    }
}