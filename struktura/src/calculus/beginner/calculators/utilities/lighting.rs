use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;

/// Lighting costs and specifications (USD)
const RECESSED_LIGHT_CAN_COST: f64 = 18.50;
const LED_BULB_COST: f64 = 8.50;
const WIRE_COST_PER_M: f64 = 1.80;  // 14/2 Romex
const JUNCTION_BOX_COST: f64 = 3.50;
const SWITCH_COST: f64 = 12.00;
const ELECTRICIAN_HOURLY: f64 = 85.00;

/// Track lighting costs
const TRACK_SECTION_COST_PER_M: f64 = 28.00;  // 4-foot section
const TRACK_HEAD_COST: f64 = 22.50;
const TRACK_CONNECTOR_COST: f64 = 8.50;

/// Lighting design standards
const RECESSED_SPACING_GENERAL: f64 = 1.8;     // 6 feet spacing
const RECESSED_SPACING_TASK: f64 = 1.2;        // 4 feet for task lighting
const LUMENS_PER_M2_LIVING: f64 = 100.0;       // Living spaces
const LUMENS_PER_M2_KITCHEN: f64 = 300.0;      // Task areas

pub struct RecessedLightingCalculator;

#[async_trait]
impl BeginnerCalculator for RecessedLightingCalculator {
    fn id(&self) -> &str {
        "recessed_lighting"
    }

    fn name(&self) -> &str {
        "Recessed Lighting Calculator"
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
                description: "Ceiling height (affects spacing and brightness)".to_string(),
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
            description: "Calculate recessed lighting layout, materials, and electrical costs for residential spaces.".to_string(),
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
        
        // Calculate optimal light count using spacing
        let lights_across_width = (params.width / RECESSED_SPACING_GENERAL).ceil().max(2.0);
        let lights_across_length = (params.length / RECESSED_SPACING_GENERAL).ceil().max(2.0);
        let num_lights = lights_across_width * lights_across_length;
        
        // Strategic assessment
        warnings.push("Recessed lighting requires ceiling cavity depth. Verify minimum 6 inches clearance above ceiling.".to_string());
        
        if params.height > 3.0 {
            warnings.push("High ceilings (>3m) may require higher-lumen bulbs or additional fixtures for adequate brightness.".to_string());
        }
        
        if num_lights > 12.0 {
            warnings.push("Large installations require proper circuit planning. May need multiple circuits to meet electrical code.".to_string());
        }
        
        // Electrical wire run estimation
        // Assume lights wired in series, distance to switch ~3m
        let wire_run_between_lights = ((params.width / lights_across_width) + (params.length / lights_across_length)) / 2.0;
        let total_wire_length = (num_lights * wire_run_between_lights) + 3.0; // +3m to switch
        
        // Material costs
        let cans_cost = num_lights * RECESSED_LIGHT_CAN_COST;
        let bulbs_cost = num_lights * LED_BULB_COST;
        let wire_cost = total_wire_length * WIRE_COST_PER_M;
        let electrical_cost = JUNCTION_BOX_COST + SWITCH_COST + (num_lights * 0.5); // Connectors
        
        let total_material = cans_cost + bulbs_cost + wire_cost + electrical_cost;
        
        // Labor (electrical work)
        let labor_hours = (num_lights * 0.75) + 2.0; // 45min per light + 2h planning/patching
        let labor_cost = labor_hours * ELECTRICIAN_HOURLY;
        
        let total_project = total_material + labor_cost;
        
        // Lighting calculations
        let recommended_lumens = area * LUMENS_PER_M2_LIVING;
        let lumens_per_bulb = recommended_lumens / num_lights;
        
        warnings.push("All electrical work must be performed by licensed electrician per local code requirements.".to_string());
        
        let results = vec![
            BeginnerResultItem {
                label: "Room Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Recommended Light Count".to_string(),
                value: num_lights,
                unit: "fixtures".to_string(),
            },
            BeginnerResultItem {
                label: "Layout Pattern".to_string(),
                value: lights_across_width,
                unit: format!("x{} grid", lights_across_length),
            },
            BeginnerResultItem {
                label: "Recommended Lumens per Bulb".to_string(),
                value: lumens_per_bulb,
                unit: "lumens".to_string(),
            },
            BeginnerResultItem {
                label: "Total Wire Required".to_string(),
                value: total_wire_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Recessed Cans Cost".to_string(),
                value: cans_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "LED Bulbs Cost".to_string(),
                value: bulbs_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Wire & Connectors".to_string(),
                value: wire_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Electrical Components".to_string(),
                value: electrical_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Material Cost".to_string(),
                value: total_material,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Electrician Hours".to_string(),
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

impl ParameterValidator for RecessedLightingCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

pub struct TrackLightingCalculator;

#[async_trait]
impl BeginnerCalculator for TrackLightingCalculator {
    fn id(&self) -> &str {
        "track_lighting"
    }

    fn name(&self) -> &str {
        "Track Lighting Calculator"
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
                description: "Room/area width".to_string(),
                required: true,
                min_value: Some(1.5),
                max_value: Some(8.0),
                typical_range: Some((2.0, 5.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Track run length".to_string(),
                required: true,
                min_value: Some(1.2),
                max_value: Some(10.0),
                typical_range: Some((2.4, 6.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Number of track heads desired".to_string(),
                required: true,
                min_value: Some(2.0),
                max_value: Some(12.0),
                typical_range: Some((3.0, 8.0)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate track lighting materials for adjustable accent and task lighting.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 1.5, 8.0)?;
        self.validate_dimension("length", params.length, 1.2, 10.0)?;
        self.validate_dimension("height", params.height, 2.0, 12.0)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let track_length = params.length;
        let num_heads = params.height.floor();
        
        warnings.push("Track lighting provides flexible, adjustable illumination. Ideal for galleries, kitchens, and display areas.".to_string());
        
        if num_heads > 8.0 {
            warnings.push("More than 8 track heads may overload single circuit. Verify electrical capacity.".to_string());
        }
        
        // Track sections (typically 1.2m or 2.4m lengths)
        let track_sections = (track_length / 1.2).ceil();
        let track_cost = track_length * TRACK_SECTION_COST_PER_M;
        
        // Track heads with LED bulbs
        let heads_cost = num_heads * TRACK_HEAD_COST;
        let bulbs_cost = num_heads * LED_BULB_COST;
        
        // Connectors and mounting
        let connector_cost = (track_sections - 1.0).max(0.0) * TRACK_CONNECTOR_COST;
        let mounting_cost = track_sections * 8.50; // Mounting clips
        
        // Power feed and switch
        let wire_length = 5.0; // Typical run to switch
        let electrical_cost = (wire_length * WIRE_COST_PER_M) + JUNCTION_BOX_COST + SWITCH_COST;
        
        let total_material = track_cost + heads_cost + bulbs_cost + connector_cost + mounting_cost + electrical_cost;
        
        // Installation
        let labor_hours = 2.5 + (track_length * 0.3); // Base + time per meter
        let labor_cost = labor_hours * ELECTRICIAN_HOURLY;
        
        let total_project = total_material + labor_cost;
        
        let results = vec![
            BeginnerResultItem {
                label: "Track Length".to_string(),
                value: track_length,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Number of Track Heads".to_string(),
                value: num_heads,
                unit: "heads".to_string(),
            },
            BeginnerResultItem {
                label: "Track Sections Required".to_string(),
                value: track_sections,
                unit: "sections".to_string(),
            },
            BeginnerResultItem {
                label: "Track System Cost".to_string(),
                value: track_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Track Heads Cost".to_string(),
                value: heads_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "LED Bulbs Cost".to_string(),
                value: bulbs_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Connectors & Mounting".to_string(),
                value: connector_cost + mounting_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Electrical Components".to_string(),
                value: electrical_cost,
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

impl ParameterValidator for TrackLightingCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}