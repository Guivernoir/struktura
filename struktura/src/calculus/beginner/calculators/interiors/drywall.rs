use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

/// Drywall material costs (USD)
const DRYWALL_COST_PER_SHEET: f64 = 14.50;
const DRYWALL_COMPOUND_PER_M2: f64 = 0.45;      // kg per m²
const DRYWALL_COMPOUND_COST_PER_KG: f64 = 1.80;
const DRYWALL_TAPE_PER_SHEET: f64 = 3.5;        // meters of tape per sheet
const DRYWALL_TAPE_COST_PER_M: f64 = 0.25;
const DRYWALL_SCREW_COST_PER_M2: f64 = 0.85;
const CORNER_BEAD_COST_PER_M: f64 = 1.85;

pub struct DrywallCountCalculator;

#[async_trait]
impl BeginnerCalculator for DrywallCountCalculator {
    fn id(&self) -> &str {
        "drywall_count"
    }

    fn name(&self) -> &str {
        "Drywall Material Calculator"
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
                description: "Wall/ceiling width".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(20.0),
                typical_range: Some((2.0, 15.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Wall/ceiling length".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(30.0),
                typical_range: Some((2.0, 20.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Not used for area calculation (set to 1.0)".to_string(),
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
            description: "Calculate drywall sheets and finishing materials including compound, tape, screws, and corner bead.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 1.0, 20.0)?;
        self.validate_dimension("length", params.length, 1.0, 30.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let total_area = params.width * params.length;
        
        // Intelligence briefing on project complexity
        if total_area > 100.0 {
            warnings.push("Large drywall projects (>100m²) benefit from professional installation to ensure quality joints and finish.".to_string());
        }
        if total_area < 5.0 {
            warnings.push("Small projects may have higher per-unit costs due to material minimums. Consider bundling with other work.".to_string());
        }
        
        // Sheet calculations with waste factor
        let sheets_base = (total_area / DRYWALL_SHEET_AREA).ceil();
        let sheets_with_waste = (sheets_base * (1.0 + WASTE_FACTOR_DRYWALL)).ceil();
        let sheet_cost = sheets_with_waste * DRYWALL_COST_PER_SHEET;
        
        // Joint compound (mud) - multiple coats
        let compound_kg = total_area * DRYWALL_COMPOUND_PER_M2;
        let compound_cost = compound_kg * DRYWALL_COMPOUND_COST_PER_KG;
        
        // Paper tape for all joints
        let tape_length = sheets_with_waste * DRYWALL_TAPE_PER_SHEET;
        let tape_cost = tape_length * DRYWALL_TAPE_COST_PER_M;
        
        // Screws (standard spacing)
        let screw_cost = total_area * DRYWALL_SCREW_COST_PER_M2;
        
        // Corner bead (10% of perimeter typically needs corners)
        let perimeter = 2.0 * (params.width + params.length);
        let corner_bead_length = perimeter * 0.10;
        let corner_bead_cost = corner_bead_length * CORNER_BEAD_COST_PER_M;
        
        let total_material_cost = sheet_cost + compound_cost + tape_cost + screw_cost + corner_bead_cost;
        
        // Labor estimate (professional taper)
        let labor_hours = total_area * 0.5; // 30 minutes per m²
        let labor_cost = labor_hours * 42.0; // $42/hour
        
        let results = vec![
            BeginnerResultItem {
                label: "Total Area to Cover".to_string(),
                value: total_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Drywall Sheets (base)".to_string(),
                value: sheets_base,
                unit: "sheets".to_string(),
            },
            BeginnerResultItem {
                label: "Drywall Sheets (with 15% waste)".to_string(),
                value: sheets_with_waste,
                unit: "sheets".to_string(),
            },
            BeginnerResultItem {
                label: "Joint Compound Required".to_string(),
                value: compound_kg,
                unit: "kg".to_string(),
            },
            BeginnerResultItem {
                label: "Paper Tape Required".to_string(),
                value: tape_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Corner Bead Length".to_string(),
                value: corner_bead_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Drywall Sheets Cost".to_string(),
                value: sheet_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Finishing Materials Cost".to_string(),
                value: compound_cost + tape_cost + corner_bead_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Fasteners Cost".to_string(),
                value: screw_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
                value: total_material_cost,
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

impl ParameterValidator for DrywallCountCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_small_room() {
        let calc = DrywallCountCalculator;
        let params = BeginnerParameters {
            width: 3.0,
            length: 4.0,
            height: 1.0,  // Not used
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_large_project_warning() {
        let calc = DrywallCountCalculator;
        let params = BeginnerParameters {
            width: 15.0,
            length: 10.0,
            height: 1.0,
            additional: None,
        };
        
        let result = calc.calculate(params).await.unwrap();
        assert!(!result.warnings.is_empty());
    }
}