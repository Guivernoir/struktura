use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for profit margins
pub struct ProfitMarginCalculator;

impl ParameterValidator for ProfitMarginCalculator {
    fn calculator_id(&self) -> &str {
        "profit_margin"
    }
}

#[async_trait]
impl ContractorCalculator for ProfitMarginCalculator {
    fn id(&self) -> &str {
        "profit_margin"
    }

    fn name(&self) -> &str {
        "Profit Margin Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Bidding
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("profit_margin", "Profit Margin")
            .category("bidding")
            .description("Calculates expected profit margin")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "total_cost".to_string(),
                path: "additional.total_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Total project cost".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "bid_price".to_string(),
                path: "additional.bid_price".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Proposed bid price".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .requires_certification()
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        let total_cost = self.get_additional_param(params, "total_cost", Some(0.0), None)?;
        let bid_price = self.get_additional_param(params, "bid_price", Some(0.0), None)?;
        if bid_price < total_cost {
            return Err(ContractingError::DomainError {
                field: "bid_price".to_string(),
                message: "Bid price must be greater than total cost".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let total_cost = self.get_additional_param(&params, "total_cost", None, None)?;
        let bid_price = self.get_additional_param(&params, "bid_price", None, None)?;

        let profit = bid_price - total_cost;
        let margin = (profit / total_cost) * 100.0;

        let mut results = vec![
            ContractingResultItem {
                label: "Profit".to_string(),
                value: profit,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", profit)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Profit Margin".to_string(),
                value: margin,
                unit: "%".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2}%", margin)),
                is_critical: true,
            },
        ];

        let warnings = if margin < 10.0 {
            vec!["Low profit margin".to_string()]
        } else {
            vec![]
        };

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost,
                total_duration: 0.0,
                risk_level: 100.0 - margin,
                compliance_score: 1.0,
            }),
            warnings,
            structured_warnings: None,
            recommendations: vec!["Aim for margins above 15% for sustainability".to_string()],
            compliance_notes: vec!["Compliant with PMP profit guidelines".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}