use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for cash flow
pub struct CashFlowAnalysisCalculator;

impl ParameterValidator for CashFlowAnalysisCalculator {
    fn calculator_id(&self) -> &str {
        "cash_flow_analysis"
    }
}

#[async_trait]
impl ContractorCalculator for CashFlowAnalysisCalculator {
    fn id(&self) -> &str {
        "cash_flow_analysis"
    }

    fn name(&self) -> &str {
        "Cash Flow Analysis Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Management
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("cash_flow_analysis", "Cash Flow Analysis")
            .category("management")
            .description("Analyzes project cash flow")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "inflows".to_string(),
                path: "additional.inflows".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Total cash inflows".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "outflows".to_string(),
                path: "additional.outflows".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Total cash outflows".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "inflows", Some(0.0), None)?;
        self.get_additional_param(params, "outflows", Some(0.0), None)?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let inflows = self.get_additional_param(&params, "inflows", None, None)?;
        let outflows = self.get_additional_param(&params, "outflows", None, None)?;

        let net_flow = inflows - outflows;
        let flow_ratio = if outflows > 0.0 { inflows / outflows } else { 0.0 };

        let mut results = vec![
            ContractingResultItem {
                label: "Net Cash Flow".to_string(),
                value: net_flow,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", net_flow)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Flow Ratio".to_string(),
                value: flow_ratio,
                unit: "".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2}", flow_ratio)),
                is_critical: true,
            },
        ];

        let warnings = if net_flow < 0.0 {
            vec!["Negative cash flow".to_string()]
        } else {
            vec![]
        };

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: outflows,
                total_duration: 0.0,
                risk_level: if net_flow < 0.0 { -net_flow / inflows * 100.0 } else { 0.0 },
                compliance_score: 1.0,
            }),
            warnings,
            structured_warnings: None,
            recommendations: vec!["Monitor cash flow monthly".to_string()],
            compliance_notes: vec!["Compliant with PMP financial management".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}