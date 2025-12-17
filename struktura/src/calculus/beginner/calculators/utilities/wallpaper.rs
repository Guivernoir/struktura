use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

/// Wallpaper specifications
const WALLPAPER_ROLL_WIDTH: f64 = 0.53;        // 21 inches standard
const WALLPAPER_ROLL_LENGTH: f64 = 10.05;      // 33 feet standard
const WALLPAPER_COST_PER_ROLL: f64 = 35.00;    // Mid-range paper

/// Installation materials
const ADHESIVE_COST_PER_LITER: f64 = 12.50;
const ADHESIVE_COVERAGE_M2_PER_LITER: f64 = 25.0;
const WALLPAPER_LABOR_PER_HOUR: f64 = 42.00;
const WALLPAPER_INSTALL_HOURS_PER_M2: f64 = 0.40; // 24 minutes per m²

pub struct WallpaperCalculator;

#[async_trait]
impl BeginnerCalculator for WallpaperCalculator {
    fn id(&self) -> &str {
        "wallpaper"
    }

    fn name(&self) -> &str {
        "Wallpaper Calculator"
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
                min_value: Some(2.0),
                max_value: Some(10.0),
                typical_range: Some((3.0, 6.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Room length".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(12.0),
                typical_range: Some((3.0, 8.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Wall height".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(3.5),
                typical_range: Some((2.4, 3.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate wallpaper rolls, adhesive, and installation costs for room walls.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 2.0, 10.0)?;
        self.validate_dimension("length", params.length, 2.0, 12.0)?;
        self.validate_dimension("height", params.height, 2.0, 3.5)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let perimeter = 2.0 * (params.width + params.length);
        let wall_area = perimeter * params.height;
        
        // Deduct standard openings (doors and windows)
        let openings_deduction = STANDARD_DOOR_AREA * 2.0 + STANDARD_WINDOW_AREA * 2.0;
        let net_wall_area = (wall_area - openings_deduction).max(0.0);
        
        // Strategic intelligence
        warnings.push("Wallpaper requires smooth, primed walls. Old wallpaper must be completely removed first.".to_string());
        
        if params.height > 3.0 {
            warnings.push("High walls (>3m) make wallpaper alignment difficult. Consider professional installation.".to_string());
        }
        
        // Calculate rolls needed
        // Account for pattern matching which increases waste
        let usable_length_per_roll = WALLPAPER_ROLL_LENGTH * (1.0 - WASTE_FACTOR_WALLPAPER);
        let area_per_roll = WALLPAPER_ROLL_WIDTH * usable_length_per_roll;
        let rolls_needed = (net_wall_area / area_per_roll).ceil();
        
        // Material costs
        let wallpaper_cost = rolls_needed * WALLPAPER_COST_PER_ROLL;
        
        let adhesive_liters = (net_wall_area / ADHESIVE_COVERAGE_M2_PER_LITER).ceil();
        let adhesive_cost = adhesive_liters * ADHESIVE_COST_PER_LITER;
        
        // Tools and supplies
        let supplies_cost = 35.0; // Smoothing tools, sponges, razor, level
        
        let total_material = wallpaper_cost + adhesive_cost + supplies_cost;
        
        // Labor (wallpaper is skilled work)
        let labor_hours = net_wall_area * WALLPAPER_INSTALL_HOURS_PER_M2 + 1.5; // +1.5h prep
        let labor_cost = labor_hours * WALLPAPER_LABOR_PER_HOUR;
        
        let total_project = total_material + labor_cost;
        
        // Additional warnings
        if rolls_needed > 10.0 {
            warnings.push("Order all rolls from same production batch to ensure consistent color. Dye lots can vary significantly.".to_string());
        }
        
        warnings.push("Wallpaper installation requires patience and precision. First-timers should practice in closets or low-visibility areas.".to_string());
        
        let results = vec![
            BeginnerResultItem {
                label: "Total Wall Perimeter".to_string(),
                value: perimeter,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Gross Wall Area".to_string(),
                value: wall_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Openings Deduction".to_string(),
                value: openings_deduction,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Net Wall Area".to_string(),
                value: net_wall_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Wallpaper Rolls Needed".to_string(),
                value: rolls_needed,
                unit: "rolls".to_string(),
            },
            BeginnerResultItem {
                label: "Adhesive Required".to_string(),
                value: adhesive_liters,
                unit: "liters".to_string(),
            },
            BeginnerResultItem {
                label: "Wallpaper Cost".to_string(),
                value: wallpaper_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Adhesive Cost".to_string(),
                value: adhesive_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Tools & Supplies".to_string(),
                value: supplies_cost,
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

impl ParameterValidator for WallpaperCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}