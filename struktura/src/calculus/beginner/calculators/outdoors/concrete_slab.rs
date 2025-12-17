use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

pub struct ConcreteSlabCalculator;

#[async_trait]
impl BeginnerCalculator for ConcreteSlabCalculator {
    fn id(&self) -> &str {
        "concrete_slab"
    }

    fn name(&self) -> &str {
        "Concrete Slab Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Outdoors
    }

    fn metadata(&self) -> BeginnerCalculatorMetadata {
        let parameters = vec![
            ParameterMetadata {
                name: "width".to_string(),
                path: "width".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Slab width".to_string(),
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
                description: "Slab length".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(20.0),
                typical_range: Some((2.0, 20.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Slab thickness (typically 10-15cm)".to_string(),
                required: true,
                min_value: Some(0.05),
                max_value: Some(0.5),
                typical_range: Some((0.08, 0.30)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate concrete, gravel base, and reinforcement materials for slabs, patios, or foundations.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        if params.width <= 0.0 || params.length <= 0.0 || params.height <= 0.0 {
            return Err(BeginnerError::DomainError {
                field: "dimensions".to_string(),
                message: "All dimensions must be positive".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        let area = params.width * params.length;
        
        if params.height < 0.08 {
            warnings.push("Slab thickness <8cm may not be structurally adequate for most applications.".to_string());
        }
        if params.height > 0.30 {
            warnings.push("Thick slabs (>30cm) typically require specialized engineering.".to_string());
        }
        if area > 50.0 {
            warnings.push("Large slabs (>50m²) require expansion joints and may need professional consultation.".to_string());
        }
        
        let concrete_volume = params.width * params.length * params.height;
        let concrete_volume_with_waste = concrete_volume * CONCRETE_WASTE_FACTOR;
        let concrete_cost = concrete_volume_with_waste * CONCRETE_COST_PER_M3;
        
        let gravel_volume = area * GRAVEL_BASE_THICKNESS;
        let gravel_cost = gravel_volume * GRAVEL_COST_PER_M3;
        
        let rebar_weight = concrete_volume * REBAR_DENSITY_KG_PER_M3;
        let rebar_cost = rebar_weight * REBAR_COST_PER_KG;
        
        let wire_mesh_cost = if params.height < 0.15 { area * 4.50 } else { 0.0 };
        
        let total_material_cost = concrete_cost + gravel_cost + rebar_cost + wire_mesh_cost;
        
        let labor_hours = area * 0.75;
        let labor_cost = labor_hours * GENERAL_LABOR_RATE;
        
        let total_project_cost = total_material_cost + labor_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Slab Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Concrete Volume (with 8% waste)".to_string(),
                value: concrete_volume_with_waste,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Gravel Base Volume".to_string(),
                value: gravel_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Rebar Weight Required".to_string(),
                value: rebar_weight,
                unit: "kg".to_string(),
            },
            BeginnerResultItem {
                label: "Concrete Cost".to_string(),
                value: concrete_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Gravel Base Cost".to_string(),
                value: gravel_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Reinforcement Cost".to_string(),
                value: rebar_cost + wire_mesh_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
                value: total_material_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Estimated Labor Cost".to_string(),
                value: labor_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Project Cost".to_string(),
                value: total_project_cost,
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

impl ParameterValidator for ConcreteSlabCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}