use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

pub struct SmallRetainingWallCalculator;

#[async_trait]
impl BeginnerCalculator for SmallRetainingWallCalculator {
    fn id(&self) -> &str {
        "retaining_wall"
    }

    fn name(&self) -> &str {
        "Small Retaining Wall Calculator"
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
                description: "Wall length".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(20.0),
                typical_range: Some((3.0, 10.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Block length (typically 0.3-0.4m)".to_string(),
                required: true,
                min_value: Some(0.2),
                max_value: Some(0.5),
                typical_range: Some((0.3, 0.4)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Wall height (max 1.2m)".to_string(),
                required: true,
                min_value: Some(0.3),
                max_value: Some(1.2),
                typical_range: Some((0.5, 1.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate materials for small retaining walls up to 1.2m.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 2.0, 20.0)?;
        self.validate_dimension("length", params.length, 0.2, 0.5)?;
        self.validate_dimension("height", params.height, 0.3, 1.2)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();

        if params.height > 1.0 {
            warnings.push("Walls over 1m may require engineering review and permits.".to_string());
        }

        // Blocks: assume block height 0.2m, length = params.length
        let block_height = 0.2;
        let rows = (params.height / block_height).ceil();
        let blocks_per_row = (params.width / params.length).ceil();
        let total_blocks = rows * blocks_per_row;
        let block_cost = total_blocks * 5.0; // $5/block

        // Base trench depth 10% height min 0.15m
        let trench_depth = (params.height * 0.1).max(0.15);
        let trench_volume = params.width * 0.3 * trench_depth; // assume 0.3m width
        let base_gravel = trench_volume;
        let base_cost = base_gravel * GRAVEL_COST_PER_M3;

        // Drainage gravel behind wall: 0.3m wide, full height
        let drainage_volume = params.width * 0.3 * params.height;
        let drainage_cost = drainage_volume * GRAVEL_COST_PER_M3;

        // Geotextile fabric
        let geo_area = params.width * params.height * 1.2; // extra
        let geo_cost = geo_area * GEOTEXTILE_COST_PER_M2;

        // Cap stones: top row
        let cap_stones = blocks_per_row;
        let cap_cost = cap_stones * 6.0;

        // Drainage pipe: 4" perforated, length = width
        let pipe_length = params.width;
        let pipe_cost = pipe_length * 2.5;

        let total_cost = block_cost + base_cost + drainage_cost + geo_cost + cap_cost + pipe_cost;

        let results = vec![
            BeginnerResultItem {
                label: "Total Blocks".to_string(),
                value: total_blocks,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Base Gravel".to_string(),
                value: base_gravel,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Drainage Gravel".to_string(),
                value: drainage_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Geotextile Fabric".to_string(),
                value: geo_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Cap Stones".to_string(),
                value: cap_stones,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Drainage Pipe".to_string(),
                value: pipe_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Total Cost".to_string(),
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

impl ParameterValidator for SmallRetainingWallCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retaining_wall() {
        let calc = SmallRetainingWallCalculator;
        let params = BeginnerParameters {
            width: 5.0,
            length: 0.4,
            height: 0.8,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }
}