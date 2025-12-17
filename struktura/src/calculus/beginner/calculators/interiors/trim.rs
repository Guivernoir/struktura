use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

/// Trim material costs (USD per m)
const BASEBOARD_MDF_PER_M: f64 = 3.20;
const BASEBOARD_WOOD_PER_M: f64 = 5.80;
const BASEBOARD_PAINT_PER_M: f64 = 0.65;
const BASEBOARD_INSTALLATION_PER_M: f64 = 4.50;

const CROWN_MOLDING_MDF_PER_M: f64 = 4.50;
const CROWN_MOLDING_WOOD_PER_M: f64 = 8.20;
const CROWN_MOLDING_INSTALLATION_PER_M: f64 = 7.50;

const DOOR_CASING_PER_M: f64 = 3.80;
const WINDOW_CASING_PER_M: f64 = 3.80;

pub struct BaseboardCalculator;

#[async_trait]
impl BeginnerCalculator for BaseboardCalculator {
    fn id(&self) -> &str {
        "baseboard"
    }

    fn name(&self) -> &str {
        "Baseboard Trim Calculator"
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
                description: "Number of doorways (will subtract from perimeter)".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(5.0),
                typical_range: Some((1.0, 3.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate baseboard trim materials including MDF or wood options, paint, and installation.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 2.0, 15.0)?;
        self.validate_dimension("length", params.length, 2.0, 20.0)?;
        self.validate_dimension("height", params.height, 0.0, 5.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let perimeter = 2.0 * (params.width + params.length);
        let num_doorways = params.height.floor();
        
        // Subtract doorway widths from perimeter
        let doorway_width = 0.91; // Standard door width
        let net_perimeter = (perimeter - (num_doorways * doorway_width)).max(0.0);
        
        // Add waste factor for cuts and miters
        let material_length = net_perimeter * (1.0 + WASTE_FACTOR_TRIM);
        
        // Tactical intelligence
        if num_doorways > 3.0 {
            warnings.push("Multiple doorways require careful miter cutting. Consider pre-cut corner blocks for easier installation.".to_string());
        }
        if material_length > 30.0 {
            warnings.push("Large baseboard projects benefit from a compound miter saw for clean, professional cuts.".to_string());
        }
        
        // Material costs (both options)
        let mdf_cost = material_length * BASEBOARD_MDF_PER_M;
        let wood_cost = material_length * BASEBOARD_WOOD_PER_M;
        
        // Paint and primer
        let paint_cost = material_length * BASEBOARD_PAINT_PER_M;
        
        // Hardware (nails, caulk, wood filler)
        let hardware_cost = material_length * 0.45;
        
        let total_mdf = mdf_cost + paint_cost + hardware_cost;
        let total_wood = wood_cost + paint_cost + hardware_cost;
        
        // Installation
        let labor_hours = material_length * 0.10; // ~6 minutes per meter
        let installation_cost = material_length * BASEBOARD_INSTALLATION_PER_M;
        
        let total_mdf_installed = total_mdf + installation_cost;
        let total_wood_installed = total_wood + installation_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Room Perimeter".to_string(),
                value: perimeter,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Number of Doorways".to_string(),
                value: num_doorways,
                unit: "openings".to_string(),
            },
            BeginnerResultItem {
                label: "Net Baseboard Length".to_string(),
                value: net_perimeter,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Material with 8% Waste".to_string(),
                value: material_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "MDF Baseboard Cost".to_string(),
                value: mdf_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Wood Baseboard Cost".to_string(),
                value: wood_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Paint & Primer Cost".to_string(),
                value: paint_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Hardware (nails, caulk, filler)".to_string(),
                value: hardware_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material (MDF)".to_string(),
                value: total_mdf,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material (Wood)".to_string(),
                value: total_wood,
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
                label: "Total Installed (MDF)".to_string(),
                value: total_mdf_installed,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Installed (Wood)".to_string(),
                value: total_wood_installed,
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

impl ParameterValidator for BaseboardCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

pub struct CrownMoldingCalculator;

#[async_trait]
impl BeginnerCalculator for CrownMoldingCalculator {
    fn id(&self) -> &str {
        "crown_molding"
    }

    fn name(&self) -> &str {
        "Crown Molding Calculator"
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
                description: "Ceiling height (informational only, set to 2.44 typical)".to_string(),
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
            description: "Calculate crown molding materials for ceiling-wall transition. Requires advanced miter cutting skills.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 2.0, 15.0)?;
        self.validate_dimension("length", params.length, 2.0, 20.0)?;
        self.validate_dimension("height", params.height, 2.0, 4.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let perimeter = 2.0 * (params.width + params.length);
        let material_length = perimeter * (1.0 + WASTE_FACTOR_TRIM);
        
        // Crown molding is professional territory
        warnings.push("Crown molding requires compound miter cuts at precise angles. Professional installation strongly recommended.".to_string());
        
        if params.height > 3.0 {
            warnings.push("High ceilings (>3m) require scaffolding or tall ladders. Safety equipment essential.".to_string());
        }
        if material_length > 25.0 {
            warnings.push("Large crown molding projects benefit from pre-cut corner blocks to simplify installation.".to_string());
        }
        
        // Material costs
        let mdf_cost = material_length * CROWN_MOLDING_MDF_PER_M;
        let wood_cost = material_length * CROWN_MOLDING_WOOD_PER_M;
        
        // Adhesive and finish
        let adhesive_cost = material_length * 0.80;
        let paint_cost = material_length * 0.75;
        
        let total_mdf = mdf_cost + adhesive_cost + paint_cost;
        let total_wood = wood_cost + adhesive_cost + paint_cost;
        
        // Installation (skilled labor)
        let labor_hours = material_length * 0.18; // ~11 minutes per meter
        let installation_cost = material_length * CROWN_MOLDING_INSTALLATION_PER_M;
        
        let total_mdf_installed = total_mdf + installation_cost;
        let total_wood_installed = total_wood + installation_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Room Perimeter".to_string(),
                value: perimeter,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Ceiling Height".to_string(),
                value: params.height,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Material with 8% Waste".to_string(),
                value: material_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "MDF Crown Molding Cost".to_string(),
                value: mdf_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Wood Crown Molding Cost".to_string(),
                value: wood_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Adhesive & Hardware".to_string(),
                value: adhesive_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Paint & Finish".to_string(),
                value: paint_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material (MDF)".to_string(),
                value: total_mdf,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material (Wood)".to_string(),
                value: total_wood,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Installation Hours".to_string(),
                value: labor_hours,
                unit: "hours".to_string(),
            },
            BeginnerResultItem {
                label: "Professional Installation Cost".to_string(),
                value: installation_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Installed (MDF)".to_string(),
                value: total_mdf_installed,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Installed (Wood)".to_string(),
                value: total_wood_installed,
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

impl ParameterValidator for CrownMoldingCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_baseboard_single_door() {
        let calc = BaseboardCalculator;
        let params = BeginnerParameters {
            width: 4.0,
            length: 5.0,
            height: 1.0,  // One doorway
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_crown_molding_standard_ceiling() {
        let calc = CrownMoldingCalculator;
        let params = BeginnerParameters {
            width: 4.0,
            length: 5.0,
            height: 2.44,  // Standard ceiling
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }
}