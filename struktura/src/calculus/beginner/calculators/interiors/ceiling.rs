use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

/// Drop ceiling costs (USD)
const DROP_CEILING_TILE_PER_M2: f64 = 8.50;
const DROP_CEILING_GRID_PER_M2: f64 = 4.20;
const DROP_CEILING_WIRE_PER_M2: f64 = 1.80;
const DROP_CEILING_INSTALLATION_PER_M2: f64 = 12.00;

/// Drywall ceiling costs
const DRYWALL_CEILING_SHEET_COST: f64 = 14.50;
const DRYWALL_CEILING_COMPOUND_PER_M2: f64 = 0.50;
const DRYWALL_CEILING_COMPOUND_COST_PER_KG: f64 = 1.80;
const DRYWALL_CEILING_INSTALLATION_PER_M2: f64 = 18.00;

pub struct DropCeilingCalculator;

#[async_trait]
impl BeginnerCalculator for DropCeilingCalculator {
    fn id(&self) -> &str {
        "drop_ceiling"
    }

    fn name(&self) -> &str {
        "Drop Ceiling Calculator"
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
                description: "Room width".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(15.0),
                typical_range: Some((3.0, 8.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Room length".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(20.0),
                typical_range: Some((3.0, 12.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Drop distance from structure (typically 0.15-0.30m)".to_string(),
                required: true,
                min_value: Some(0.10),
                max_value: Some(0.60),
                typical_range: Some((0.15, 0.30)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate suspended drop ceiling materials including tiles, grid system, and hanging hardware.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 2.0, 15.0)?;
        self.validate_dimension("length", params.length, 2.0, 20.0)?;
        self.validate_dimension("height", params.height, 0.10, 0.60)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let area = params.width * params.length;
        let perimeter = 2.0 * (params.width + params.length);
        
        // Tactical assessment
        if params.height < 0.15 {
            warnings.push("Drop distance <0.15m provides minimal utility access. Consider if drop ceiling is necessary.".to_string());
        }
        if params.height > 0.40 {
            warnings.push("Large drop distances (>0.4m) may require additional structural support for grid system.".to_string());
        }
        if area > 80.0 {
            warnings.push("Large ceiling installations benefit from professional grid layout to minimize tile cuts.".to_string());
        }
        
        // Material calculations
        let tile_area = area * 1.05; // 5% waste for cuts
        let tile_cost = tile_area * DROP_CEILING_TILE_PER_M2;
        
        let grid_cost = area * DROP_CEILING_GRID_PER_M2;
        
        // Hanging wire (one every 1.2m in both directions)
        let wire_points = ((params.width / 1.2).ceil() * (params.length / 1.2).ceil()) as f64;
        let wire_cost = wire_points * DROP_CEILING_WIRE_PER_M2;
        
        // Wall angle trim (perimeter)
        let wall_angle_cost = perimeter * 2.50;
        
        let total_material = tile_cost + grid_cost + wire_cost + wall_angle_cost;
        
        // Installation
        let labor_hours = area * 0.15; // ~9 minutes per m²
        let installation_cost = area * DROP_CEILING_INSTALLATION_PER_M2;
        
        let total_cost = total_material + installation_cost;
        
        // Advantages
        warnings.push("Drop ceilings provide easy access to utilities and HVAC. Ideal for basements and commercial spaces.".to_string());
        
        let results = vec![
            BeginnerResultItem {
                label: "Ceiling Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Drop Distance".to_string(),
                value: params.height,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Ceiling Tiles (with 5% waste)".to_string(),
                value: tile_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Hanging Wire Points".to_string(),
                value: wire_points,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Ceiling Tile Cost".to_string(),
                value: tile_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Grid System Cost".to_string(),
                value: grid_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Hanging Hardware Cost".to_string(),
                value: wire_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Wall Angle Trim Cost".to_string(),
                value: wall_angle_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
                value: total_material,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Installation Hours".to_string(),
                value: labor_hours,
                unit: "hours".to_string(),
            },
            BeginnerResultItem {
                label: "Installation Cost".to_string(),
                value: installation_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Project Cost".to_string(),
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

impl ParameterValidator for DropCeilingCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

pub struct DrywallCeilingCalculator;

#[async_trait]
impl BeginnerCalculator for DrywallCeilingCalculator {
    fn id(&self) -> &str {
        "drywall_ceiling"
    }

    fn name(&self) -> &str {
        "Drywall Ceiling Calculator"
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
                description: "Room width".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(15.0),
                typical_range: Some((3.0, 8.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Room length".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(20.0),
                typical_range: Some((3.0, 12.0)),
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
            description: "Calculate drywall ceiling materials including sheets, compound, and installation. More permanent than drop ceiling.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 2.0, 15.0)?;
        self.validate_dimension("length", params.length, 2.0, 20.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let area = params.width * params.length;
        
        warnings.push("Ceiling drywall is physically demanding. Professional installation recommended for quality and safety.".to_string());
        
        if area > 60.0 {
            warnings.push("Large ceiling projects require scaffolding or specialized lifts. Factor in equipment rental costs.".to_string());
        }
        
        // Sheet calculations
        let sheets_needed = (area / DRYWALL_SHEET_AREA).ceil();
        let sheets_with_waste = (sheets_needed * 1.10).ceil(); // 10% waste
        let sheet_cost = sheets_with_waste * DRYWALL_CEILING_SHEET_COST;
        
        // Compound (ceilings need extra coats for finish)
        let compound_kg = area * DRYWALL_CEILING_COMPOUND_PER_M2;
        let compound_cost = compound_kg * DRYWALL_CEILING_COMPOUND_COST_PER_KG;
        
        // Tape and screws
        let tape_length = sheets_with_waste * 3.5;
        let tape_cost = tape_length * 0.25;
        let screw_cost = area * 1.20; // Ceiling needs more screws
        
        let total_material = sheet_cost + compound_cost + tape_cost + screw_cost;
        
        // Labor (ceiling work is slower)
        let labor_hours = area * 0.60; // 36 minutes per m²
        let installation_cost = area * DRYWALL_CEILING_INSTALLATION_PER_M2;
        
        let total_cost = total_material + installation_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Ceiling Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Drywall Sheets (with 10% waste)".to_string(),
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
                label: "Drywall Sheets Cost".to_string(),
                value: sheet_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Finishing Materials Cost".to_string(),
                value: compound_cost + tape_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Fasteners Cost".to_string(),
                value: screw_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
                value: total_material,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Installation Hours".to_string(),
                value: labor_hours,
                unit: "hours".to_string(),
            },
            BeginnerResultItem {
                label: "Installation Cost".to_string(),
                value: installation_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Project Cost".to_string(),
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

impl ParameterValidator for DrywallCeilingCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}