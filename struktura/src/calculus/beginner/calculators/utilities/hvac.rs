use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;

/// HVAC sizing constants
const BTU_PER_M2_BASE: f64 = 200.0;             // Base cooling/heating
const BTU_PER_M2_KITCHEN: f64 = 400.0;          // Higher for kitchens
const BTU_ADJUSTMENT_SUNNY: f64 = 1.10;         // 10% more for sunny rooms
const BTU_ADJUSTMENT_SHADED: f64 = 0.90;        // 10% less for shaded

/// Ductwork costs (USD per m)
const DUCT_COST_150MM: f64 = 12.50;             // 6 inch duct
const DUCT_COST_200MM: f64 = 18.00;             // 8 inch duct
const DUCT_COST_250MM: f64 = 24.50;             // 10 inch duct
const REGISTER_COST: f64 = 18.50;
const RETURN_VENT_COST: f64 = 22.00;

/// HVAC installation
const HVAC_LABOR_PER_HOUR: f64 = 95.00;

pub struct HVACSizingCalculator;

#[async_trait]
impl BeginnerCalculator for HVACSizingCalculator {
    fn id(&self) -> &str {
        "hvac_sizing"
    }

    fn name(&self) -> &str {
        "HVAC Sizing & Ductwork Calculator"
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
                max_value: Some(12.0),
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
                max_value: Some(15.0),
                typical_range: Some((3.0, 8.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Ceiling height".to_string(),
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
            description: "Calculate HVAC capacity requirements and basic ductwork materials for single rooms.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 2.0, 12.0)?;
        self.validate_dimension("length", params.length, 2.0, 15.0)?;
        self.validate_dimension("height", params.height, 2.0, 4.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let area = params.width * params.length;
        let volume = area * params.height;
        
        // Calculate BTU requirement (simplified Manual J)
        let base_btu = area * BTU_PER_M2_BASE;
        
        // For this basic calculator, assume average conditions
        let adjusted_btu = base_btu;
        
        // Convert BTU to tons (12,000 BTU = 1 ton)
        let tons_cooling = adjusted_btu / 12000.0;
        
        // Strategic warnings
        warnings.push("This is a simplified estimate. Professional Manual J calculation required for accurate sizing.".to_string());
        warnings.push("Factors like insulation, windows, climate zone, and occupancy significantly affect requirements.".to_string());
        
        if area > 50.0 {
            warnings.push("Large spaces may benefit from zoned HVAC systems with multiple units for better efficiency.".to_string());
        }
        
        if params.height > 3.0 {
            warnings.push("High ceilings increase heating/cooling volume. Consider ceiling fans to improve air circulation.".to_string());
        }
        
        // Ductwork estimation
        // Assume supply register every 3m along longer wall
        let num_supply_registers = (params.length.max(params.width) / 3.0).ceil().max(1.0);
        let num_return_vents = 1.0; // Typically 1 return per room
        
        // Duct run length (simplified: distance to main trunk + drops)
        let duct_run_length = num_supply_registers * 4.0; // Avg 4m per register
        
        // Duct size recommendation based on area
        let (duct_size_mm, duct_cost_per_m) = if area < 15.0 {
            (150.0, DUCT_COST_150MM)
        } else if area < 30.0 {
            (200.0, DUCT_COST_200MM)
        } else {
            (250.0, DUCT_COST_250MM)
        };
        
        // Material costs
        let duct_cost = duct_run_length * duct_cost_per_m;
        let register_cost = num_supply_registers * REGISTER_COST;
        let return_cost = num_return_vents * RETURN_VENT_COST;
        let damper_cost = num_supply_registers * 15.0; // Dampers for balancing
        
        let total_ductwork = duct_cost + register_cost + return_cost + damper_cost;
        
        // Labor (ductwork installation)
        let labor_hours = (duct_run_length * 0.5) + 2.0; // 30min per meter + setup
        let labor_cost = labor_hours * HVAC_LABOR_PER_HOUR;
        
        let total_project = total_ductwork + labor_cost;
        
        // Unit cost estimate (rough)
        let unit_cost_estimate = tons_cooling * 3500.0; // ~$3500 per ton installed
        
        warnings.push("Unit costs vary widely by efficiency rating (SEER), brand, and installation complexity.".to_string());
        
        let results = vec![
            BeginnerResultItem {
                label: "Room Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Room Volume".to_string(),
                value: volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Required BTU/hr (Cooling)".to_string(),
                value: adjusted_btu,
                unit: "BTU/hr".to_string(),
            },
            BeginnerResultItem {
                label: "Capacity (Tons)".to_string(),
                value: tons_cooling,
                unit: "tons".to_string(),
            },
            BeginnerResultItem {
                label: "Recommended Duct Size".to_string(),
                value: duct_size_mm,
                unit: "mm".to_string(),
            },
            BeginnerResultItem {
                label: "Supply Registers".to_string(),
                value: num_supply_registers,
                unit: "registers".to_string(),
            },
            BeginnerResultItem {
                label: "Return Vents".to_string(),
                value: num_return_vents,
                unit: "vents".to_string(),
            },
            BeginnerResultItem {
                label: "Ductwork Run Length".to_string(),
                value: duct_run_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Ductwork Material Cost".to_string(),
                value: duct_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Registers & Returns Cost".to_string(),
                value: register_cost + return_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Dampers & Fittings".to_string(),
                value: damper_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Ductwork Cost".to_string(),
                value: total_ductwork,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Installation Hours".to_string(),
                value: labor_hours,
                unit: "hours".to_string(),
            },
            BeginnerResultItem {
                label: "Installation Labor".to_string(),
                value: labor_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Ductwork Project".to_string(),
                value: total_project,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Est. HVAC Unit Cost (separate)".to_string(),
                value: unit_cost_estimate,
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

impl ParameterValidator for HVACSizingCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}