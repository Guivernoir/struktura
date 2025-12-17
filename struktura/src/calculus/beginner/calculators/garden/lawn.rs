use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::{BeginnerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use super::constants::*;

pub struct LawnSeedCalculator;

#[async_trait]
impl BeginnerCalculator for LawnSeedCalculator {
    fn id(&self) -> &str {
        "lawn_seed"
    }

    fn name(&self) -> &str {
        "Lawn Seed Calculator"
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
                description: "Lawn width".to_string(),
                required: true,
                min_value: Some(5.0),
                max_value: Some(50.0),
                typical_range: Some((10.0, 30.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Lawn length".to_string(),
                required: true,
                min_value: Some(5.0),
                max_value: Some(50.0),
                typical_range: Some((10.0, 30.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Topsoil amendment depth (if needed, 0 if none)".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(0.1),
                typical_range: Some((0.0, 0.05)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate seed quantity, fertilizer, and amendments for new lawns.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 5.0, 50.0)?;
        self.validate_dimension("length", params.length, 5.0, 50.0)?;
        self.validate_dimension("height", params.height, 0.0, 0.1)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();

        let area = params.width * params.length;

        // Seed rate 0.05 kg/m² for new lawn
        let seed_kg = area * 0.05;
        let seed_cost = seed_kg * 10.0; // $10/kg assume

        // Starter fertilizer 0.01 kg/m²
        let fertilizer_kg = area * 0.01;
        let fertilizer_cost = fertilizer_kg * 5.0;

        // Topsoil if height >0
        let topsoil_volume = area * params.height;
        let topsoil_cost = topsoil_volume * TOPSOIL_COST_PER_M3;

        // Germination 7-21 days
        let germination_days = 14.0;

        let total_cost = seed_cost + fertilizer_cost + topsoil_cost;

        let results = vec![
            BeginnerResultItem {
                label: "Lawn Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Seed Quantity".to_string(),
                value: seed_kg,
                unit: "kg".to_string(),
            },
            BeginnerResultItem {
                label: "Starter Fertilizer".to_string(),
                value: fertilizer_kg,
                unit: "kg".to_string(),
            },
            BeginnerResultItem {
                label: "Topsoil Amendment".to_string(),
                value: topsoil_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Est. Germination Time".to_string(),
                value: germination_days,
                unit: "days".to_string(),
            },
            BeginnerResultItem {
                label: "Total Cost".to_string(),
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

impl ParameterValidator for LawnSeedCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

pub struct SodCalculator;

#[async_trait]
impl BeginnerCalculator for SodCalculator {
    fn id(&self) -> &str {
        "sod"
    }

    fn name(&self) -> &str {
        "Sod Calculator"
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
                description: "Lawn width".to_string(),
                required: true,
                min_value: Some(5.0),
                max_value: Some(50.0),
                typical_range: Some((10.0, 30.0)),
            },
            ParameterMetadata {
                name: "length".to_string(),
                path: "length".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Lawn length".to_string(),
                required: true,
                min_value: Some(5.0),
                max_value: Some(50.0),
                typical_range: Some((10.0, 30.0)),
            },
            ParameterMetadata {
                name: "height".to_string(),
                path: "height".to_string(),
                data_type: "number".to_string(),
                unit: "m".to_string(),
                description: "Soil preparation depth (typically 0.05m)".to_string(),
                required: true,
                min_value: Some(0.02),
                max_value: Some(0.1),
                typical_range: Some((0.03, 0.05)),
            },
        ];

        BeginnerCalculatorMetadata {
            id: self.id().to_string(),
            name: self.name().to_string(),
            category: self.category().as_str().to_string(),
            description: "Calculate sod pallets, soil prep, and watering for instant lawns.".to_string(),
            parameters,
            required_parameters: vec!["width".to_string(), "length".to_string(), "height".to_string()],
            optional_parameters: vec![],
        }
    }

    fn validate(&self, params: &BeginnerParameters) -> BeginnerResult<()> {
        self.validate_dimension("width", params.width, 5.0, 50.0)?;
        self.validate_dimension("length", params.length, 5.0, 50.0)?;
        self.validate_dimension("height", params.height, 0.02, 0.1)?;
        Ok(())
    }

    async fn calculate(&self, params: BeginnerParameters) -> BeginnerResult<BeginnerCalculationResponse> {
        let mut warnings = Vec::new();

        let area = params.width * params.length;

        // Pallets: 50m² per pallet
        let pallets = (area / 50.0).ceil();
        let sod_cost = pallets * 200.0; // $200/pallet assume

        // Soil prep volume
        let soil_volume = area * params.height;
        let soil_cost = soil_volume * TOPSOIL_COST_PER_M3;

        // Starter fertilizer same as seed
        let fertilizer_kg = area * 0.01;
        let fertilizer_cost = fertilizer_kg * 5.0;

        // Watering first 2 weeks: 2.5cm/week
        let weekly_water_m3 = area * 0.025;

        let total_cost = sod_cost + soil_cost + fertilizer_cost;

        let results = vec![
            BeginnerResultItem {
                label: "Lawn Area".to_string(),
                value: area,
                unit: "m²".to_string(),
            },
            BeginnerResultItem {
                label: "Sod Pallets".to_string(),
                value: pallets,
                unit: "pallets".to_string(),
            },
            BeginnerResultItem {
                label: "Soil Prep Volume".to_string(),
                value: soil_volume,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Starter Fertilizer".to_string(),
                value: fertilizer_kg,
                unit: "kg".to_string(),
            },
            BeginnerResultItem {
                label: "Weekly Water (first 2 wks)".to_string(),
                value: weekly_water_m3,
                unit: "m³".to_string(),
            },
            BeginnerResultItem {
                label: "Total Cost".to_string(),
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

impl ParameterValidator for SodCalculator {
    fn calculator_id(&self) -> &str {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lawn_seed() {
        let calc = LawnSeedCalculator;
        let params = BeginnerParameters {
            width: 10.0,
            length: 20.0,
            height: 0.05,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sod() {
        let calc = SodCalculator;
        let params = BeginnerParameters {
            width: 10.0,
            length: 20.0,
            height: 0.05,
            additional: None,
        };
        
        let result = calc.calculate(params).await;
        assert!(result.is_ok());
    }
}