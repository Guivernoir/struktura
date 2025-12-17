use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

pub struct MulchBedCalculator;

#[async_trait]
impl BeginnerCalculator for MulchBedCalculator {
    fn id(&self) -> &str {
        "mulch_bed"
    }

    fn name(&self) -> &str {
        "Mulch Bed Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Garden
    }

    fn metadata(&self) -> BeginnerCalculatorMetadata {
        let parameters = vec![
            ParameterMetadata {
                name: "width".to_string(),
                path: "width".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Bed width".to_string(),
                required: true,
                min_value: Some(0.5),
                max_value: Some(10.0),
                typical_range: Some((1.0, 5.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Bed length".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(20.0),
                typical_range: Some((2.0, 15.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Mulch depth (typically 5-10cm)".to_string(),
                required: true,
                min_value: Some(0.05),
                max_value: Some(0.15),
                typical_range: Some((0.05, 0.10)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate mulch volume, landscape fabric, and edging materials for garden beds and landscaping.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 0.5, 10.0)?;
        self.validate_dimension("length", params.length, 1.0, 20.0)?;
        self.validate_dimension("height", params.height, 0.05, 0.15)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();
        
        let area = params.width * params.length;
        let perimeter = 2.0 * (params.width + params.length);
        
        // Horticultural intelligence briefing
        if params.height < 0.05 {
            warnings.push("Mulch layers <5cm may not effectively suppress weeds or retain moisture. Consider 8-10cm depth.".to_string());
        }
        if params.height > 0.12 {
            warnings.push("Mulch layers >12cm can create anaerobic conditions and harm plant roots. Thin existing layer first.".to_string());
        }
        if area > 30.0 {
            warnings.push("Large mulch projects (>30m²) benefit from bulk delivery. Check local landscape supply yards for better pricing.".to_string());
        }
        
        // Mulch volume calculation
        let mulch_volume = area * params.height;
        let mulch_cost = mulch_volume * MULCH_COST_PER_M3;
        
        // Landscape fabric (prevents weed growth)
        let fabric_area = area * 1.10; // 10% overlap at seams
        let fabric_cost = fabric_area * LANDSCAPE_FABRIC_COST_PER_M2;
        
        // Edge border materials (contains mulch, prevents grass intrusion)
        let edging_cost = perimeter * EDGING_COST_PER_M;
        
        // Fabric stakes (secure fabric before mulching)
        let stakes_needed = (area / 1.0).ceil(); // 1 stake per m²
        let stakes_cost = stakes_needed * 0.75;
        
        let total_cost = mulch_cost + fabric_cost + edging_cost + stakes_cost;
        
        // Estimated weight (for delivery planning and vehicle capacity)
        let mulch_weight_kg = mulch_volume * 450.0; // Wood mulch ~450kg/m³
        let num_bags = (mulch_volume / 0.057).ceil(); // 2 cubic foot bags
        
        // Labor estimate
        let labor_hours = (area * 0.10) + 1.5; // ~6min per m² + setup
        
        warnings.push("Apply mulch when soil is moist and after weeding. Keep mulch 5-8cm away from plant stems to prevent rot.".to_string());
        
        let results = vec![
            BeginnerResultItem {
                label: "Garden Bed Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Bed Perimeter".to_string(),
                value: perimeter,
                unit: "m".to_string(),
            },
            BeginnerResultItem {
                label: "Mulch Volume Required".to_string(),
                value: mulch_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Estimated Mulch Weight".to_string(),
                value: mulch_weight_kg,
                unit: "kg".to_string(),
            },
            BeginnerResultItem {
                label: "Bags Needed (2 cu ft)".to_string(),
                value: num_bags,
                unit: "bags".to_string(),
            },
            BeginnerResultItem {
                label: "Landscape Fabric Area".to_string(),
                value: fabric_area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Fabric Stakes Needed".to_string(),
                value: stakes_needed,
                unit: "pieces".to_string(),
            },
            BeginnerResultItem {
                label: "Mulch Cost".to_string(),
                value: mulch_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Landscape Fabric Cost".to_string(),
                value: fabric_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Edging Materials".to_string(),
                value: edging_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Stakes & Hardware".to_string(),
                value: stakes_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Total Project Cost".to_string(),
                value: total_cost,
                unit: "USD".to_string(),
            },
            BeginnerResultItem {
                label: "Estimated Labor (DIY)".to_string(),
                value: labor_hours,
                unit: "hours".to_string(),
            },
        ];

        Ok(BeginnerCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            warnings,
        })
    }
}

impl ParameterValidator for MulchBedCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_standard_bed() {
        let calc = MulchBedCalculator;
        let params = BeginnerParameters {
            width: 2.0,
            length: 5.0,
            height: 0.08,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_thin_mulch_warning() {
        let calc = MulchBedCalculator;
        let params = BeginnerParameters {
            width: 2.0,
            length: 3.0,
            height: 0.03,  // Too thin
            additional: None,
        };
        
        let result = calc.calculate(params).await.unwrap();
        assert!(!result.warnings.is_empty());
    }
}