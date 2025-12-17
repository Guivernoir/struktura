use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for change orders
pub struct ChangeOrderCalculator;

impl ParameterValidator for ChangeOrderCalculator {
    fn calculator_id(&self) -> &str {
        "change_order"
    }
}

#[async_trait]
impl ContractorCalculator for ChangeOrderCalculator {
    fn id(&self) -> &str {
        "change_order"
    }

    fn name(&self) -> &str {
        "Change Order Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Management
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("change_order", "Change Order")
            .category("management")
            .description("Calculates impact of change orders")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "original_cost".to_string(),
                path: "additional.original_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Original contract cost".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "change_cost".to_string(),
                path: "additional.change_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Change order cost".to_string(),
                required: true,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: None,
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "change_duration".to_string(),
                path: "additional.change_duration".to_string(),
                data_type: ParameterType::Number,
                unit: "days".to_string(),
                description: "Change in duration".to_string(),
                required: false,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: None,
                default_value: Some(0.0),
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "original_cost", Some(0.0), None)?;
        self.get_additional_param(params, "change_cost", None, None)?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let original_cost = self.get_additional_param(&params, "original_cost", None, None)?;
        let change_cost = self.get_additional_param(&params, "change_cost", None, None)?;
        let change_duration = self.get_additional_param(&params, "change_duration", None, None).unwrap_or(0.0);

        let new_cost = original_cost + change_cost;
        let cost_impact = (change_cost / original_cost) * 100.0;

        let mut results = vec![
            ContractingResultItem {
                label: "New Total Cost".to_string(),
                value: new_cost,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", new_cost)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Cost Impact".to_string(),
                value: cost_impact,
                unit: "%".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2}%", cost_impact)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Duration Change".to_string(),
                value: change_duration,
                unit: "days".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.0} days", change_duration)),
                is_critical: false,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: new_cost,
                total_duration: change_duration,
                risk_level: cost_impact,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Document all changes".to_string()],
            compliance_notes: vec!["Compliant with PMP change management".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}