use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

pub struct GravelPathCalculator;

#[async_trait]
impl BeginnerCalculator for GravelPathCalculator {
    fn id(&self) -> &str {
        "gravel_path"
    }

    fn name(&self) -> &str {
        "Gravel Path Calculator"
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
                description: "Path width (min 0.6m)".to_string(),
                required: true,
                min_value: Some(0.6),
                max_value: Some(2.0),
                typical_range: Some((0.8, 1.5)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Path length".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(50.0),
                typical_range: Some((5.0, 20.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Gravel depth (typically 0.10m)".to_string(),
                required: true,
                min_value: Some(0.05),
                max_value: Some(0.15),
                typical_range: Some((0.08, 0.12)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate gravel volume, landscape fabric, and edging for gravel paths.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 0.6, 2.0)?;
        self.validate_dimension("length", params.length, 2.0, 50.0)?;
        self.validate_dimension("height", params.height, 0.05, 0.15)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();

        let area = params.width * params.length;
        let gravel_volume = area * params.height;
        let gravel_cost = gravel_volume * GRAVEL_COST_PER_M3;

        // Base layer (compaction material, assume sand 5cm)
        let base_depth = 0.05;
        let base_volume = area * base_depth;
        let base_cost = base_volume * SAND_COST_PER_M3;

        // Landscape fabric
        let fabric_area = area * 1.1; // 10% overlap
        let fabric_cost = fabric_area * LANDSCAPE_FABRIC_COST_PER_M2;

        // Edge restraints
        let perimeter = 2.0 * params.length; // Sides only
        let edging_length = perimeter;
        let edging_cost = edging_length * EDGING_COST_PER_M;

        if params.width < PATH_WIDTH_MINIMUM {
            warnings.push("Paths narrower than 0.6m may be difficult to walk on. Consider widening.".to_string());
        }

        let total_cost = gravel_cost + base_cost + fabric_cost + edging_cost;

        let results = vec![
            BeginnerResultItem {
                label: "Path Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Gravel Volume".to_string(),
                value: gravel_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Base Layer Volume".to_string(),
                value: base_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Landscape Fabric".to_string(),
                value: fabric_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Edging Length".to_string(),
                value: edging_length,
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

impl ParameterValidator for GravelPathCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

pub struct SteppingStoneCalculator;

#[async_trait]
impl BeginnerCalculator for SteppingStoneCalculator {
    fn id(&self) -> &str {
        "stepping_stone"
    }

    fn name(&self) -> &str {
        "Stepping Stone Calculator"
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
                description: "Stone width (typically 0.4-0.6m)".to_string(),
                required: true,
                min_value: Some(0.3),
                max_value: Some(0.8),
                typical_range: Some((0.4, 0.6)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Path length".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(50.0),
                typical_range: Some((5.0, 20.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Stone thickness (typically 0.05m)".to_string(),
                required: true,
                min_value: Some(0.03),
                max_value: Some(0.1),
                typical_range: Some((0.04, 0.06)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate number of stepping stones, sand base, and recommendations.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 0.3, 0.8)?;
        self.validate_dimension("length", params.length, 2.0, 50.0)?;
        self.validate_dimension("height", params.height, 0.03, 0.1)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();

        // Stride length ~0.6m center to center
        let stride = 0.6;
        let num_stones = (params.length / stride).ceil();

        // Sand base per stone (0.05m depth, 20% larger area)
        let stone_area = params.width * params.width; // Assume square stones
        let sand_area_per_stone = stone_area * 1.2;
        let sand_depth = 0.05;
        let sand_volume = num_stones * sand_area_per_stone * sand_depth;
        let sand_cost = sand_volume * SAND_COST_PER_M3;

        // Stone cost assume $15 each
        let stone_cost_each = 15.0;
        let total_stone_cost = num_stones * stone_cost_each;

        let total_cost = sand_cost + total_stone_cost;

        let results = vec![
            BeginnerResultItem {
                label: "Number of Stones".to_string(),
                value: num_stones,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Sand Base Volume".to_string(),
                value: sand_volume,
                unit: "m³".to_string(),
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

impl ParameterValidator for SteppingStoneCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gravel_path() {
        let calc = GravelPathCalculator;
        let params = BeginnerParameters {
            width: 1.0,
            length: 10.0,
            height: 0.1,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stepping_stone() {
        let calc = SteppingStoneCalculator;
        let params = BeginnerParameters {
            width: 0.5,
            length: 10.0,
            height: 0.05,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }
}