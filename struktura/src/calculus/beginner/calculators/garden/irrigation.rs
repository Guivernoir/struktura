use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

pub struct DripIrrigationCalculator;

#[async_trait]
impl BeginnerCalculator for DripIrrigationCalculator {
    fn id(&self) -> &str {
        "drip_irrigation"
    }

    fn name(&self) -> &str {
        "Drip Irrigation Calculator"
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
                description: "Bed width".to_string(),
                required: true,
                min_value: Some(0.5),
                max_value: Some(3.0),
                typical_range: Some((1.0, 2.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Bed length".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(30.0),
                typical_range: Some((5.0, 20.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Plant spacing (typically 0.3-0.6m)".to_string(),
                required: true,
                min_value: Some(0.2),
                max_value: Some(1.0),
                typical_range: Some((0.3, 0.6)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate drip line length, emitters, and water needs.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 0.5, 3.0)?;
        self.validate_dimension("length", params.length, 2.0, 30.0)?;
        self.validate_dimension("height", params.height, 0.2, 1.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();

        // Assume rows along length, spacing = height
        let num_rows = (params.width / params.height).floor() + 1.0;
        let drip_line_length = num_rows * params.length;
        let drip_cost = drip_line_length * DRIP_TUBING_COST_PER_M;

        // Emitters per row
        let emitters_per_row = (params.length / params.height).ceil();
        let total_emitters = emitters_per_row * num_rows;
        let emitters_cost = total_emitters * EMITTER_COST_EACH;

        // Pressure ~20-30 PSI
        let pressure_req = 25.0;

        // Water consumption assume 1 GPH per emitter, 30min/day
        let daily_water_gal = (total_emitters * 1.0 * 0.5) / 3.785; // liters

        let total_cost = drip_cost + emitters_cost;

        let results = vec![
            BeginnerResultItem {
                label: "Drip Line Length".to_string(),
                value: drip_line_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Total Emitters".to_string(),
                value: total_emitters,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Pressure Requirement".to_string(),
                value: pressure_req,
                unit: "PSI".to_string(),
            },
            BeginnerResultItem {
                label: "Daily Water Estimate".to_string(),
                value: daily_water_gal,
                unit: "liters".to_string(),
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

impl ParameterValidator for DripIrrigationCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

pub struct SprinklerCoverageCalculator;

#[async_trait]
impl BeginnerCalculator for SprinklerCoverageCalculator {
    fn id(&self) -> &str {
        "sprinkler_coverage"
    }

    fn name(&self) -> &str {
        "Sprinkler Coverage Calculator"
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
                description: "Area width".to_string(),
                required: true,
                min_value: Some(5.0),
                max_value: Some(50.0),
                typical_range: Some((10.0, 30.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Area length".to_string(),
                required: true,
                min_value: Some(5.0),
                max_value: Some(50.0),
                typical_range: Some((10.0, 30.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Head radius (typically 3-5m)".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(10.0),
                typical_range: Some((3.0, 5.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate sprinkler heads, spacing, and flow.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 5.0, 50.0)?;
        self.validate_dimension("length", params.length, 5.0, 50.0)?;
        self.validate_dimension("height", params.height, 2.0, 10.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();

        // Head spacing = radius * 1.0 (for overlap)
        let spacing = params.height;
        let num_heads_width = (params.width / spacing).ceil();
        let num_heads_length = (params.length / spacing).ceil();
        let total_heads = num_heads_width * num_heads_length;
        let heads_cost = total_heads * SPRINKLER_HEAD_COST;

        // GPM per head assume 2 GPM
        let gpm_per_head = 2.0;
        let total_gpm = total_heads * gpm_per_head;

        let total_cost = heads_cost;

        let results = vec![
            BeginnerResultItem {
                label: "Total Heads".to_string(),
                value: total_heads,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Head Spacing".to_string(),
                value: spacing,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Total GPM".to_string(),
                value: total_gpm,
                unit: "GPM".to_string(),
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

impl ParameterValidator for SprinklerCoverageCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_drip_irrigation() {
        let calc = DripIrrigationCalculator;
        let params = BeginnerParameters {
            width: 1.5,
            length: 10.0,
            height: 0.3,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sprinkler_coverage() {
        let calc = SprinklerCoverageCalculator;
        let params = BeginnerParameters {
            width: 20.0,
            length: 30.0,
            height: 4.0,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }
}