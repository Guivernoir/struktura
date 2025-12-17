use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for contingency planning
pub struct ContingencyPlanningCalculator;

impl ParameterValidator for ContingencyPlanningCalculator {
    fn calculator_id(&self) -> &str {
        "contingency_planning"
    }
}

#[async_trait]
impl ContractorCalculator for ContingencyPlanningCalculator {
    fn id(&self) -> &str {
        "contingency_planning"
    }

    fn name(&self) -> &str {
        "Contingency Planning Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Bidding
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("contingency_planning", "Contingency Planning")
            .category("bidding")
            .description("Determines contingency funds based on risk factors")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "total_cost".to_string(),
                path: "additional.total_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Base total cost".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "risk_factor".to_string(),
                path: "additional.risk_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Overall risk factor (0-1)".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(1.0),
                typical_range: None,
                validation_rules: None,
                default_value: Some(0.1),
            })
            .requires_certification()
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "total_cost", Some(0.0), None)?;
        self.get_additional_param(params, "risk_factor", Some(0.0), Some(1.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let total_cost = self.get_additional_param(&params, "total_cost", None, None)?;
        let risk_factor = self.get_additional_param(&params, "risk_factor", None, None)?;

        let contingency = total_cost * risk_factor * 1.5; // Adjusted by 1.5 for conservatism
        let total_with_contingency = total_cost + contingency;

        let mut results = vec![
            ContractingResultItem {
                label: "Contingency Fund".to_string(),
                value: contingency,
                unit: "USD".to_string(),
                tolerance: Some(0.2),
                formatted_value: Some(format!("${:.2}", contingency)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Total with Contingency".to_string(),
                value: total_with_contingency,
                unit: "USD".to_string(),
                tolerance: Some(0.2),
                formatted_value: Some(format!("${:.2}", total_with_contingency)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: total_with_contingency,
                total_duration: 0.0,
                risk_level: risk_factor * 100.0,
                compliance_score: 1.0 - risk_factor,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Allocate contingency based on identified risks".to_string()],
            compliance_notes: vec!["Compliant with PMP contingency planning".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}