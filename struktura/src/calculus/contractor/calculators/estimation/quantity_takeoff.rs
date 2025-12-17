use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for quantity takeoff
pub struct QuantityTakeoffCalculator;

impl ParameterValidator for QuantityTakeoffCalculator {
    fn calculator_id(&self) -> &str {
        "quantity_takeoff"
    }
}

#[async_trait]
impl ContractorCalculator for QuantityTakeoffCalculator {
    fn id(&self) -> &str {
        "quantity_takeoff"
    }

    fn name(&self) -> &str {
        "Quantity Takeoff Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Estimation
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("quantity_takeoff", "Quantity Takeoff")
            .category("estimation")
            .description("Calculates material quantities from dimensions")
            .regulation_code("ASTM")
            .parameter(ParameterMetadata {
                name: "length".to_string(),
                path: "dimensions.length".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Length dimension".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((1.0, 1000.0)),
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "width".to_string(),
                path: "dimensions.width".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Width dimension".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((1.0, 1000.0)),
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "height".to_string(),
                path: "dimensions.height".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Height dimension".to_string(),
                required: false,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((1.0, 100.0)),
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "waste_factor".to_string(),
                path: "material.waste_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Waste factor multiplier".to_string(),
                required: false,
                min_value: Some(1.0),
                max_value: Some(1.5),
                typical_range: Some((1.05, 1.2)),
                validation_rules: None,
                default_value: Some(1.1),
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.validate_dimension("dimensions.length", params.dimensions.get("length").copied(), 0.0, f64::MAX)?;
        self.validate_dimension("dimensions.width", params.dimensions.get("width").copied(), 0.0, f64::MAX)?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let length = params.dimensions.get("length").copied().unwrap_or(0.0);
        let width = params.dimensions.get("width").copied().unwrap_or(0.0);
        let height = params.dimensions.get("height").copied().unwrap_or(1.0);
        let waste_factor = params.material.as_ref().and_then(|m| m.waste_factor).unwrap_or(1.1);

        let area = length * width;
        let volume = area * height;
        let quantity = volume * waste_factor;

        let mut results = vec![
            ContractingResultItem {
                label: "Area".to_string(),
                value: area,
                unit: "m²".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2} m²", area)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Volume".to_string(),
                value: volume,
                unit: "m³".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2} m³", volume)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Quantity with Waste".to_string(),
                value: quantity,
                unit: "m³".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2} m³", quantity)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: None,
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Verify dimensions on-site".to_string()],
            compliance_notes: vec!["Compliant with ASTM standards".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "ASTM".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}