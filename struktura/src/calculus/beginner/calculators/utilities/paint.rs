use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

/// Paint material costs (USD)
const PAINT_COVERAGE_M2_PER_LITER: f64 = 12.0;  // Good quality paint
const PAINT_COST_PER_LITER: f64 = 18.50;
const PRIMER_COVERAGE_M2_PER_LITER: f64 = 14.0;
const PRIMER_COST_PER_LITER: f64 = 14.00;

/// Painting labor rates
const PAINTING_LABOR_PER_HOUR: f64 = 38.00;
const PAINTING_RATE_M2_PER_HOUR: f64 = 15.0;
const PREP_TIME_BASE_HOURS: f64 = 2.0;

/// Supply costs
const BRUSH_ROLLER_KIT: f64 = 25.00;
const DROP_CLOTH_COST: f64 = 12.00;
const TAPE_COST_PER_ROLL: f64 = 4.50;
const TRAY_LINER_COST: f64 = 3.00;

pub struct PaintCoverageCalculator;

#[async_trait]
impl BeginnerCalculator for PaintCoverageCalculator {
    fn id(&self) -> &str {
        "paint_coverage"
    }

    fn name(&self) -> &str {
        "Paint Coverage Calculator"
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
                description: "Room width".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(10.0),
                typical_range: Some((2.0, 6.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Room length".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(10.0),
                typical_range: Some((2.0, 6.0)),
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
            description: "Calculate paint and primer requirements for rooms, including walls and ceiling with opening deductions.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 1.0, 10.0)?;
        self.validate_dimension("length", params.length, 1.0, 10.0)?;
        self.validate_dimension("height", params.height, 2.0, 4.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        // Calculate wall area (4 walls)
        let perimeter = 2.0 * (params.width + params.length);
        let wall_area = perimeter * params.height;
        
        // Ceiling area
        let ceiling_area = params.width * params.length;
        let total_area = wall_area + ceiling_area;
        
        // Strategic assessment
        if total_area > 150.0 {
            warnings.push("Large paint projects (>150m²) may require multiple days and professional equipment (airless sprayers).".to_string());
        }
        if params.height > 3.0 {
            warnings.push("High ceilings (>3m) require scaffolding or extension poles. Factor in additional time and equipment.".to_string());
        }
        
        // Typical deductions for doors and windows (10% of wall area)
        let openings_deduction = wall_area * 0.10;
        let net_wall_area = wall_area - openings_deduction;
        let net_total_area = net_wall_area + ceiling_area;
        
        // Paint requirements (2 coats standard)
        let paint_liters_walls = (net_wall_area * 2.0) / PAINT_COVERAGE_M2_PER_LITER;
        let paint_liters_ceiling = (ceiling_area * 2.0) / PAINT_COVERAGE_M2_PER_LITER;
        let total_paint_liters = (paint_liters_walls + paint_liters_ceiling).ceil();
        
        // Primer (1 coat) - essential for new drywall or color changes
        let primer_liters = (net_total_area / PRIMER_COVERAGE_M2_PER_LITER).ceil();
        
        // Material costs
        let paint_cost = total_paint_liters * PAINT_COST_PER_LITER;
        let primer_cost = primer_liters * PRIMER_COST_PER_LITER;
        
        // Supplies (brushes, rollers, tape, drop cloths)
        let num_rooms = 1.0;
        let tape_rolls = (perimeter / 15.0).ceil().max(2.0);
        let supplies_cost = BRUSH_ROLLER_KIT + (DROP_CLOTH_COST * num_rooms) + 
                           (tape_rolls * TAPE_COST_PER_ROLL) + TRAY_LINER_COST;
        
        let total_material = paint_cost + primer_cost + supplies_cost;
        
        // Labor estimate
        let labor_hours = (net_total_area / PAINTING_RATE_M2_PER_HOUR) + PREP_TIME_BASE_HOURS;
        let labor_cost = labor_hours * PAINTING_LABOR_PER_HOUR;
        
        let total_project = total_material + labor_cost;
        
        // Helpful intelligence
        if ceiling_area > 20.0 {
            warnings.push("Ceiling painting benefits from a quality roller with extension pole to reduce fatigue and improve speed.".to_string());
        }
        
        let results = vec![
            BeginnerResultItem {
                label: "Total Wall Area (4 walls)".to_string(),
                value: wall_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Ceiling Area".to_string(),
                value: ceiling_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Openings Deduction".to_string(),
                value: openings_deduction,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Net Paintable Area".to_string(),
                value: net_total_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Paint Required (2 coats)".to_string(),
                value: total_paint_liters,
                unit: "liters".to_string(),
            },
            BeginnerResultItem {
                label: "Primer Required (1 coat)".to_string(),
                value: primer_liters,
                unit: "liters".to_string(),
            },
            BeginnerResultItem {
                label: "Paint Cost".to_string(),
                value: paint_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Primer Cost".to_string(),
                value: primer_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Supplies & Tools".to_string(),
                value: supplies_cost,
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

impl ParameterValidator for PaintCoverageCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_standard_room() {
        let calc = PaintCoverageCalculator;
        let params = BeginnerParameters {
            width: 4.0,
            length: 5.0,
            height: 2.44,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_large_room_warning() {
        let calc = PaintCoverageCalculator;
        let params = BeginnerParameters {
            width: 10.0,
            length: 10.0,
            height: 3.0,
            additional: None,
        };
        
        let result = calc.calculate(params).await.unwrap();
        assert!(!result.warnings.is_empty());
    }
}