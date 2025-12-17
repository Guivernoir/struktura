use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for overhead costs
pub struct OverheadCalculator;

impl ParameterValidator for OverheadCalculator {
    fn calculator_id(&self) -> &str {
        "overhead"
    }
}

#[async_trait]
impl ContractorCalculator for OverheadCalculator {
    fn id(&self) -> &str {
        "overhead"
    }

    fn name(&self) -> &str {
        "Overhead Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Estimation
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("overhead", "Overhead Calculator")
            .category("estimation")
            .description("Calculates overhead costs as percentage of direct costs")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "direct_cost".to_string(),
                path: "additional.direct_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Direct costs".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "overhead_percentage".to_string(),
                path: "additional.overhead_percentage".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Overhead percentage".to_string(),
                required: true,
                min_value: Some(5.0),
                max_value: Some(50.0),
                typical_range: Some((10.0, 30.0)),
                validation_rules: None,
                default_value: Some(20.0),
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "direct_cost", Some(0.0), None)?;
        self.get_additional_param(params, "overhead_percentage", Some(5.0), Some(50.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let direct_cost = self.get_additional_param(&params, "direct_cost", None, None)?;
        let overhead_pct = self.get_additional_param(&params, "overhead_percentage", None, None)?;

        let overhead = direct_cost * (overhead_pct / 100.0);
        let total = direct_cost + overhead;

        let mut results = vec![
            ContractingResultItem {
                label: "Overhead Cost".to_string(),
                value: overhead,
                unit: "USD".to_string(),
                tolerance: Some(0.15),
                formatted_value: Some(format!("${:.2}", overhead)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Total with Overhead".to_string(),
                value: total,
                unit: "USD".to_string(),
                tolerance: Some(0.15),
                formatted_value: Some(format!("${:.2}", total)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: total,
                total_duration: 0.0,
                risk_level: 0.0,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Adjust percentage based on company averages".to_string()],
            compliance_notes: vec!["Compliant with PMP overhead guidelines".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}