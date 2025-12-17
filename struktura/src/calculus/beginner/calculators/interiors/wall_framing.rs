use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

/// Material costs (USD)
const STUD_COST_EACH: f64 = 6.75;
const TOP_BOTTOM_PLATE_COST_PER_M: f64 = 4.50;
const HEADER_COST_PER_M: f64 = 12.00;
const BLOCKING_COST_PER_M: f64 = 4.50;
const HARDWARE_COST_PER_STUD: f64 = 2.50;

pub struct WallFramingCalculator;

#[async_trait]
impl BeginnerCalculator for WallFramingCalculator {
    fn id(&self) -> &str {
        "wall_framing"
    }

    fn name(&self) -> &str {
        "Wall Framing Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Interiors
    }

    fn metadata(&self) -> BeginnerCalculatorMetadata {
        let parameters = vec![
            ParameterMetadata {
                name: "width".to_string(),
                path: "width".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Wall thickness (typically stud width)".to_string(),
                required: true,
                min_value: Some(0.089),  // 2x4 stud
                max_value: Some(0.140),  // 2x6 stud
                typical_range: Some((0.089, 0.140)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Wall length".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(20.0),
                typical_range: Some((3.0, 12.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Wall height".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(4.0),
                typical_range: Some((2.4, 3.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate studs, plates, headers, and hardware for wall framing. Assumes standard 16-inch on-center spacing.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 0.089, 0.140)?;
        self.validate_dimension("length", params.length, 1.0, 20.0)?;
        self.validate_dimension("height", params.height, 2.0, 4.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        // Strategic intelligence assessment
        if params.height > 3.5 {
            warnings.push("Wall heights >3.5m may require engineered studs or additional bracing. Consult structural engineer.".to_string());
        }
        if params.length > 8.0 {
            warnings.push("Long walls (>8m) typically need mid-span blocking or additional structural support.".to_string());
        }
        if params.width < 0.089 {
            warnings.push("2x4 framing provides minimal insulation space. Consider 2x6 for exterior walls.".to_string());
        }
        
        // Calculate number of studs (on-center spacing + end studs)
        let num_studs = ((params.length / STUD_SPACING).ceil() + 1.0).max(2.0);
        
        // Total stud length needed
        let total_stud_length = num_studs * params.height;
        
        // Top and bottom plates (double top plate is standard practice)
        let plate_length = params.length * 3.0; // Bottom + 2x top plates
        
        // Headers for openings (assume one door opening if reasonable height)
        let has_opening = params.height >= 2.0 && params.height <= 3.0;
        let header_length = if has_opening { 1.2 } else { 0.0 };
        
        // Mid-wall blocking for structural rigidity
        let blocking_pieces = if params.length > 4.0 { 
            (params.length / 2.0).ceil() 
        } else { 
            0.0 
        };
        let blocking_length = blocking_pieces * params.width;
        
        // Cost calculations with surgical precision
        let stud_cost = num_studs * STUD_COST_EACH;
        let plate_cost = plate_length * TOP_BOTTOM_PLATE_COST_PER_M;
        let header_cost = header_length * HEADER_COST_PER_M;
        let blocking_cost = blocking_length * BLOCKING_COST_PER_M;
        let hardware_cost = num_studs * HARDWARE_COST_PER_STUD;
        
        let total_lumber_cost = stud_cost + plate_cost + header_cost + blocking_cost;
        let total_cost = total_lumber_cost + hardware_cost;
        
        // Labor estimate (professional framing crew)
        let labor_hours = (params.length * params.height) / 10.0; // 10m² per hour
        let labor_cost = labor_hours * 65.0; // $65/hour framing rate
        
        let results = vec![
            BeginnerResultItem {
                label: "Vertical Studs Required".to_string(),
                value: num_studs,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Total Stud Length".to_string(),
                value: total_stud_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Top/Bottom Plate Length".to_string(),
                value: plate_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Mid-Wall Blocking Pieces".to_string(),
                value: blocking_pieces,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Header Material (if needed)".to_string(),
                value: header_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Lumber Cost".to_string(),
                value: total_lumber_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Fasteners & Hardware".to_string(),
                value: hardware_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
                value: total_cost,
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
        ];

        Ok(BeginnerCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            warnings,
        })
    }
}

impl ParameterValidator for WallFramingCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_standard_wall() {
        let calc = WallFramingCalculator;
        let params = BeginnerParameters {
            width: 0.089,   // 2x4 stud
            length: 4.0,    // 4m wall
            height: 2.44,   // 8ft ceiling
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(!response.results.is_empty());
    }

    #[tokio::test]
    async fn test_long_wall_warning() {
        let calc = WallFramingCalculator;
        let params = BeginnerParameters {
            width: 0.089,
            length: 10.0,   // Long wall
            height: 2.44,
            additional: None,
        };
        
        let result = calc.calculate(params).await.unwrap();
        assert!(!result.warnings.is_empty());
    }
}