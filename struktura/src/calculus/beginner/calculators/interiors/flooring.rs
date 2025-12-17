use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

/// Hardwood flooring costs (USD per m²)
const HARDWOOD_COST_PER_M2: f64 = 45.00;
const HARDWOOD_UNDERLAYMENT_PER_M2: f64 = 3.50;
const HARDWOOD_FINISH_PER_M2: f64 = 8.50;
const HARDWOOD_INSTALLATION_PER_M2: f64 = 28.00;

/// Laminate flooring costs (USD per m²)
const LAMINATE_COST_PER_M2: f64 = 18.50;
const LAMINATE_UNDERLAYMENT_PER_M2: f64 = 2.80;
const LAMINATE_INSTALLATION_PER_M2: f64 = 12.00;

/// Transition strips and baseboards
const TRANSITION_STRIP_COST_PER_M: f64 = 8.50;
const BASEBOARD_COST_PER_M: f64 = 4.20;

pub struct HardwoodFlooringCalculator;

#[async_trait]
impl BeginnerCalculator for HardwoodFlooringCalculator {
    fn id(&self) -> &str {
        "hardwood_flooring"
    }

    fn name(&self) -> &str {
        "Hardwood Flooring Calculator"
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
                min_value: Some(1.0),
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
                min_value: Some(1.0),
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
            description: "Calculate solid hardwood flooring materials including underlayment, finish, and installation costs.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 1.0, 15.0)?;
        self.validate_dimension("length", params.length, 1.0, 20.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let area = params.width * params.length;
        let perimeter = 2.0 * (params.width + params.length);
        
        // Strategic assessment
        if area < 10.0 {
            warnings.push("Small installations (<10m²) may have higher per-unit costs. Consider whether hardwood is cost-effective.".to_string());
        }
        if area > 50.0 {
            warnings.push("Large installations (>50m²) require careful moisture control and acclimation. Professional installation strongly recommended.".to_string());
        }
        
        // Material calculations with waste
        let area_with_waste = area * (1.0 + WASTE_FACTOR_FLOORING);
        
        let flooring_cost = area_with_waste * HARDWOOD_COST_PER_M2;
        let underlayment_cost = area_with_waste * HARDWOOD_UNDERLAYMENT_PER_M2;
        let finish_cost = area * HARDWOOD_FINISH_PER_M2;  // Finish doesn't need waste
        
        // Transitions (assume 2 doorways)
        let transition_length = params.width.min(1.5) * 2.0;
        let transition_cost = transition_length * TRANSITION_STRIP_COST_PER_M;
        
        // Baseboards (full perimeter)
        let baseboard_cost = perimeter * BASEBOARD_COST_PER_M;
        
        let total_material = flooring_cost + underlayment_cost + finish_cost + transition_cost + baseboard_cost;
        
        // Installation labor
        let installation_cost = area * HARDWOOD_INSTALLATION_PER_M2;
        let labor_hours = area * 0.35; // ~20 minutes per m²
        
        // Total project cost
        let total_cost = total_material + installation_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Floor Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Material Area (with 10% waste)".to_string(),
                value: area_with_waste,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Perimeter".to_string(),
                value: perimeter,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Hardwood Flooring Cost".to_string(),
                value: flooring_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Underlayment Cost".to_string(),
                value: underlayment_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Finish/Stain Cost".to_string(),
                value: finish_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Transition Strips Cost".to_string(),
                value: transition_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Baseboard Cost".to_string(),
                value: baseboard_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
                value: total_material,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Installation Labor Hours".to_string(),
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

impl ParameterValidator for HardwoodFlooringCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

pub struct LaminateFlooringCalculator;

#[async_trait]
impl BeginnerCalculator for LaminateFlooringCalculator {
    fn id(&self) -> &str {
        "laminate_flooring"
    }

    fn name(&self) -> &str {
        "Laminate Flooring Calculator"
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
                min_value: Some(1.0),
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
                min_value: Some(1.0),
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
            description: "Calculate laminate flooring materials including underlayment and installation. DIY-friendly option.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 1.0, 15.0)?;
        self.validate_dimension("length", params.length, 1.0, 20.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let area = params.width * params.length;
        let perimeter = 2.0 * (params.width + params.length);
        
        if area > 70.0 {
            warnings.push("Large laminate installations (>70m²) benefit from expansion gap planning across multiple rooms.".to_string());
        }
        
        // Laminate is DIY-friendly
        warnings.push("Laminate is suitable for DIY installation. Consider professional help for complex layouts.".to_string());
        
        let area_with_waste = area * (1.0 + WASTE_FACTOR_FLOORING);
        
        let flooring_cost = area_with_waste * LAMINATE_COST_PER_M2;
        let underlayment_cost = area_with_waste * LAMINATE_UNDERLAYMENT_PER_M2;
        
        let transition_length = params.width.min(1.5) * 2.0;
        let transition_cost = transition_length * TRANSITION_STRIP_COST_PER_M;
        
        let baseboard_cost = perimeter * BASEBOARD_COST_PER_M;
        
        let total_material = flooring_cost + underlayment_cost + transition_cost + baseboard_cost;
        
        // Professional installation (optional for laminate)
        let installation_cost = area * LAMINATE_INSTALLATION_PER_M2;
        let labor_hours = area * 0.20; // ~12 minutes per m²
        
        let total_with_labor = total_material + installation_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Floor Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Material Area (with 10% waste)".to_string(),
                value: area_with_waste,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Laminate Flooring Cost".to_string(),
                value: flooring_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Underlayment Cost".to_string(),
                value: underlayment_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Transition Strips Cost".to_string(),
                value: transition_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Baseboard Cost".to_string(),
                value: baseboard_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost (DIY)".to_string(),
                value: total_material,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Professional Installation Hours".to_string(),
                value: labor_hours,
                unit: "hours".to_string(),
            },
            BeginnerResultItem {
                label: "Professional Installation Cost".to_string(),
                value: installation_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total with Professional Install".to_string(),
                value: total_with_labor,
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

impl ParameterValidator for LaminateFlooringCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}