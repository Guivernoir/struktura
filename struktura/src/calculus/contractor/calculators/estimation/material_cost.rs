use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Estimator for material costs
pub struct MaterialCostEstimator;

impl ParameterValidator for MaterialCostEstimator {
    fn calculator_id(&self) -> &str {
        "material_cost"
    }
}

#[async_trait]
impl ContractorCalculator for MaterialCostEstimator {
    fn id(&self) -> &str {
        "material_cost"
    }

    fn name(&self) -> &str {
        "Material Cost Estimator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Estimation
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("material_cost", "Material Cost Estimator")
            .category("estimation")
            .description("Estimates total material costs")
            .regulation_code("ASTM")
            .parameter(ParameterMetadata {
                name: "material_quantity".to_string(),
                path: "resources.material_quantity".to_string(),
                data_type: ParameterType::Number,
                unit: "units".to_string(),
                description: "Material quantity".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "unit_cost".to_string(),
                path: "material.unit_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD/unit".to_string(),
                description: "Unit cost".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "waste_factor".to_string(),
                path: "material.waste_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Waste factor".to_string(),
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
        self.validate_resources(&params.resources)?;
        self.validate_material(&params.material)?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let resources = params.resources.as_ref().unwrap();
        let material = params.material.as_ref().unwrap();
        let quantity = resources.material_quantity.unwrap_or(0.0);
        let unit_cost = material.unit_cost.unwrap_or(0.0);
        let waste_factor = material.waste_factor.unwrap_or(1.1);

        let adjusted_quantity = quantity * waste_factor;
        let total_material_cost = adjusted_quantity * unit_cost;

        let mut results = vec![
            ContractingResultItem {
                label: "Adjusted Quantity".to_string(),
                value: adjusted_quantity,
                unit: "units".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2} units", adjusted_quantity)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Total Material Cost".to_string(),
                value: total_material_cost,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", total_material_cost)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: total_material_cost,
                total_duration: 0.0,
                risk_level: 0.0,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Check current market prices".to_string()],
            compliance_notes: vec!["Compliant with ASTM material standards".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "ASTM".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}