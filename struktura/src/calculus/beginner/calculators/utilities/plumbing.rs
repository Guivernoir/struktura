use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

/// Pipe material costs (USD per m)
const COPPER_PIPE_12MM_PER_M: f64 = 8.50;      // 1/2" water supply
const COPPER_PIPE_19MM_PER_M: f64 = 12.00;     // 3/4" water supply
const PEX_PIPE_12MM_PER_M: f64 = 2.80;         // 1/2" PEX (flexible)
const PEX_PIPE_19MM_PER_M: f64 = 3.50;         // 3/4" PEX
const PVC_DRAIN_50MM_PER_M: f64 = 4.20;        // 2" drain
const PVC_DRAIN_75MM_PER_M: f64 = 6.50;        // 3" drain
const PVC_DRAIN_100MM_PER_M: f64 = 8.80;       // 4" drain

/// Fittings and connections
const FITTING_COST_SMALL: f64 = 2.50;          // Elbow, coupling
const FITTING_COST_LARGE: f64 = 8.50;          // Tee, valve
const SHUTOFF_VALVE_COST: f64 = 12.50;
const DRAIN_FITTING_COST: f64 = 5.50;

/// Plumbing labor
const PLUMBER_HOURLY: f64 = 95.00;

pub struct PipeRunCalculator;

#[async_trait]
impl BeginnerCalculator for PipeRunCalculator {
    fn id(&self) -> &str {
        "pipe_run"
    }

    fn name(&self) -> &str {
        "Water Supply Pipe Calculator"
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
                description: "Horizontal pipe run distance".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(30.0),
                typical_range: Some((3.0, 15.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Additional pipe length or fixtures".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(20.0),
                typical_range: Some((0.0, 10.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Vertical rise (if applicable)".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(10.0),
                typical_range: Some((0.0, 4.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate water supply pipe materials (copper or PEX) for residential plumbing runs.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string()],
            optional_parameters: vec!["length".to_string(), "height".to_string()],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 1.0, 30.0)?;
        self.validate_dimension("length", params.length, 0.0, 20.0)?;
        self.validate_dimension("height", params.height, 0.0, 10.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        // Total pipe run
        let total_pipe_length = params.width + params.length + params.height;
        
        // Add waste factor for cuts and fittings
        let pipe_with_waste = total_pipe_length * (1.0 + WASTE_FACTOR_PIPE);
        
        // Strategic intelligence
        warnings.push("All plumbing work must comply with local building codes and be performed by licensed plumber if required.".to_string());
        
        if total_pipe_length > 20.0 {
            warnings.push("Long pipe runs may experience pressure drop. Consider larger diameter pipe or pressure booster.".to_string());
        }
        
        if params.height > 3.0 {
            warnings.push("Significant vertical runs require proper support brackets every 1.2m to prevent sagging.".to_string());
        }
        
        // Material options: Copper vs PEX
        // Copper (traditional, rigid)
        let copper_pipe_cost = pipe_with_waste * COPPER_PIPE_12MM_PER_M;
        let copper_fittings_estimate = (total_pipe_length / 2.0) * FITTING_COST_SMALL; // Fitting every 2m
        let copper_total = copper_pipe_cost + copper_fittings_estimate + SHUTOFF_VALVE_COST;
        
        // PEX (modern, flexible)
        let pex_pipe_cost = pipe_with_waste * PEX_PIPE_12MM_PER_M;
        let pex_fittings_estimate = (total_pipe_length / 3.0) * FITTING_COST_SMALL; // Fewer fittings needed
        let pex_manifold_cost = 85.0; // If using manifold system
        let pex_total = pex_pipe_cost + pex_fittings_estimate + pex_manifold_cost + SHUTOFF_VALVE_COST;
        
        // Labor estimate
        let labor_hours = (total_pipe_length * 0.20) + 1.5; // 12min per meter + setup
        let labor_cost = labor_hours * PLUMBER_HOURLY;
        
        let total_copper_installed = copper_total + labor_cost;
        let total_pex_installed = pex_total + labor_cost;
        
        // Support brackets
        let num_supports = (total_pipe_length / PIPE_SUPPORT_SPACING).ceil();
        let support_cost = num_supports * 3.50;
        
        warnings.push("PEX is easier to install, freezes better, and costs less but check local code approval.".to_string());
        warnings.push("Copper is traditional, durable, and universally code-approved but requires soldering skills.".to_string());
        
        let results = vec![
            BeginnerResultItem {
                label: "Horizontal Run".to_string(),
                value: params.width,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Additional Length".to_string(),
                value: params.length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Vertical Rise".to_string(),
                value: params.height,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Total Pipe Length".to_string(),
                value: total_pipe_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Pipe with 10% Waste".to_string(),
                value: pipe_with_waste,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Support Brackets Needed".to_string(),
                value: num_supports,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Copper Pipe Cost (1/2\")".to_string(),
                value: copper_pipe_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Copper Fittings & Valve".to_string(),
                value: copper_fittings_estimate + SHUTOFF_VALVE_COST,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Copper Materials".to_string(),
                value: copper_total,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "PEX Pipe Cost (1/2\")".to_string(),
                value: pex_pipe_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "PEX Fittings & Manifold".to_string(),
                value: pex_fittings_estimate + pex_manifold_cost + SHUTOFF_VALVE_COST,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total PEX Materials".to_string(),
                value: pex_total,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Support Brackets Cost".to_string(),
                value: support_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Labor Hours".to_string(),
                value: labor_hours,
                unit: "hours".to_string(),
            },
            BeginnerResultItem {
                label: "Labor Cost".to_string(),
                value: labor_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Installed (Copper)".to_string(),
                value: total_copper_installed + support_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Installed (PEX)".to_string(),
                value: total_pex_installed + support_cost,
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

impl ParameterValidator for PipeRunCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

pub struct DrainLineCalculator;

#[async_trait]
impl BeginnerCalculator for DrainLineCalculator {
    fn id(&self) -> &str {
        "drain_line"
    }

    fn name(&self) -> &str {
        "Drain Line Calculator"
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
                description: "Drain pipe run length".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(25.0),
                typical_range: Some((2.0, 12.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "mm".to_string(),
                description: "Drain pipe diameter (50, 75, or 100mm)".to_string(),
                required: true,
                min_value: Some(50.0),
                max_value: Some(100.0),
                typical_range: Some((50.0, 100.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Vertical drop (if applicable)".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(8.0),
                typical_range: Some((0.0, 3.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate PVC drain line materials and slope requirements for residential waste systems.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string()],
            optional_parameters: vec!["height".to_string()],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 1.0, 25.0)?;
        
        // Validate pipe diameter selection
        let diameter = params.length;
        if diameter != 50.0 && diameter != 75.0 && diameter != 100.0 {
            return Err(BeginnerError::InvalidParameter {
                parameter: "length (diameter)".to_string(),
                value: diameter.to_string(),
                reason: "Must be 50, 75, or 100mm".to_string(),
            });
        }
        
        self.validate_dimension("height", params.height, 0.0, 8.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let pipe_length = params.width;
        let pipe_diameter_mm = params.length;
        let vertical_drop = params.height;
        
        // Total run including vertical
        let total_length = pipe_length + vertical_drop;
        let pipe_with_waste = total_length * (1.0 + WASTE_FACTOR_PIPE);
        
        // Required slope for drainage (1/4 inch per foot = 2%)
        let required_slope = pipe_length * 0.02;
        
        // Strategic assessment
        warnings.push("Drain lines require minimum 1/4\" per foot (2%) slope for proper drainage per plumbing code.".to_string());
        
        if pipe_length > 15.0 {
            warnings.push("Long drain runs may require cleanout access points every 15m per code requirements.".to_string());
        }
        
        if vertical_drop > 4.0 {
            warnings.push("Long vertical drops require proper venting to prevent siphoning and maintain trap seals.".to_string());
        }
        
        // Determine pipe cost based on diameter
        let (diameter_str, cost_per_m) = match pipe_diameter_mm as i32 {
            50 => ("50mm (2\")", PVC_DRAIN_50MM_PER_M),
            75 => ("75mm (3\")", PVC_DRAIN_75MM_PER_M),
            100 => ("100mm (4\")", PVC_DRAIN_100MM_PER_M),
            _ => ("50mm (2\")", PVC_DRAIN_50MM_PER_M), // Default fallback
        };
        
        let pipe_cost = pipe_with_waste * cost_per_m;
        
        // Fittings estimate
        let num_fittings = (total_length / 2.5).ceil(); // Fitting every 2.5m
        let fitting_cost = num_fittings * DRAIN_FITTING_COST;
        
        // P-trap and cleanout
        let trap_cost = 15.50;
        let cleanout_cost = if pipe_length > 10.0 { 28.50 } else { 0.0 };
        
        // Hangers/supports
        let num_supports = (total_length / PIPE_SUPPORT_SPACING).ceil();
        let support_cost = num_supports * 4.50;
        
        let total_material = pipe_cost + fitting_cost + trap_cost + cleanout_cost + support_cost;
        
        // Labor
        let labor_hours = (total_length * 0.25) + 2.0; // 15min per meter + setup
        let labor_cost = labor_hours * PLUMBER_HOURLY;
        
        let total_project = total_material + labor_cost;
        
        // Guidance on diameter selection
        let diameter_guidance = match pipe_diameter_mm as i32 {
            50 => "Suitable for sinks and lavatories",
            75 => "Suitable for bathtubs, showers, and multiple fixtures",
            100 => "Suitable for toilets and main drain lines",
            _ => "Standard drain size",
        };
        
        warnings.push(format!("Selected diameter: {} - {}", diameter_str, diameter_guidance));
        
        let results = vec![
            BeginnerResultItem {
                label: "Horizontal Run".to_string(),
                value: pipe_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Pipe Diameter".to_string(),
                value: pipe_diameter_mm,
                unit: "mm".to_string(),
            },
            BeginnerResultItem {
                label: "Vertical Drop".to_string(),
                value: vertical_drop,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Total Pipe Length".to_string(),
                value: total_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Required Slope (2%)".to_string(),
                value: required_slope,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Pipe with 10% Waste".to_string(),
                value: pipe_with_waste,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Fittings Needed".to_string(),
                value: num_fittings,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Support Hangers".to_string(),
                value: num_supports,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "PVC Drain Pipe Cost".to_string(),
                value: pipe_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Fittings Cost".to_string(),
                value: fitting_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "P-Trap Cost".to_string(),
                value: trap_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Cleanout Cost".to_string(),
                value: cleanout_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Support Hangers Cost".to_string(),
                value: support_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
                value: total_material,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Labor Hours".to_string(),
                value: labor_hours,
                unit: "hours".to_string(),
            },
            BeginnerResultItem {
                label: "Labor Cost".to_string(),
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

impl ParameterValidator for DrainLineCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}