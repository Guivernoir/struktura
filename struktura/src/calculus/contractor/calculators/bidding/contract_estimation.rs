use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for estimating contract values
pub struct ContractEstimationCalculator;

impl ParameterValidator for ContractEstimationCalculator {
    fn calculator_id(&self) -> &str {
        "contract_estimation"
    }
}

#[async_trait]
impl ContractorCalculator for ContractEstimationCalculator {
    fn id(&self) -> &str {
        "contract_estimation"
    }

    fn name(&self) -> &str {
        "Contract Estimation Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Bidding
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("contract_estimation", "Contract Estimation")
            .category("bidding")
            .description("Estimates total contract value including contingencies")
            .regulation_code("IBC")
            .parameter(ParameterMetadata {
                name: "total_cost".to_string(),
                path: "additional.total_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Base total cost".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((1000.0, 10000000.0)),
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "contingency_percentage".to_string(),
                path: "additional.contingency_percentage".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Contingency allowance".to_string(),
                required: true,
                min_value: Some(5.0),
                max_value: Some(20.0),
                typical_range: Some((5.0, 15.0)),
                validation_rules: None,
                default_value: Some(10.0),
            })
            .requires_certification()
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "total_cost", Some(0.0), None)?;
        self.get_additional_param(params, "contingency_percentage", Some(5.0), Some(20.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let total_cost = self.get_additional_param(&params, "total_cost", None, None)?;
        let contingency_pct = self.get_additional_param(&params, "contingency_percentage", None, None)?;

        let contingency = total_cost * (contingency_pct / 100.0);
        let estimated_contract = total_cost + contingency;

        let mut results = vec![
            ContractingResultItem {
                label: "Base Cost".to_string(),
                value: total_cost,
                unit: "USD".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("${:.2}", total_cost)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Contingency".to_string(),
                value: contingency,
                unit: "USD".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("${:.2}", contingency)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Estimated Contract Value".to_string(),
                value: estimated_contract,
                unit: "USD".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("${:.2}", estimated_contract)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: estimated_contract,
                total_duration: 0.0,
                risk_level: contingency_pct,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Include escalation clauses in contract".to_string()],
            compliance_notes: vec!["Compliant with IBC estimation standards".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "IBC".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}