use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for subcontractor evaluation
pub struct SubcontractorEvaluationCalculator;

impl ParameterValidator for SubcontractorEvaluationCalculator {
    fn calculator_id(&self) -> &str {
        "subcontractor_evaluation"
    }
}

#[async_trait]
impl ContractorCalculator for SubcontractorEvaluationCalculator {
    fn id(&self) -> &str {
        "subcontractor_evaluation"
    }

    fn name(&self) -> &str {
        "Subcontractor Evaluation Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Management
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("subcontractor_evaluation", "Subcontractor Evaluation")
            .category("management")
            .description("Evaluates subcontractor performance")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "performance_score".to_string(),
                path: "additional.performance_score".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Performance score (1-10)".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(10.0),
                typical_range: None,
                validation_rules: None,
                default_value: Some(8.0),
            })
            .parameter(ParameterMetadata {
                name: "reliability_score".to_string(),
                path: "additional.reliability_score".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Reliability score (1-10)".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(10.0),
                typical_range: None,
                validation_rules: None,
                default_value: Some(8.0),
            })
            .parameter(ParameterMetadata {
                name: "cost_score".to_string(),
                path: "additional.cost_score".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Cost management score (1-10)".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(10.0),
                typical_range: None,
                validation_rules: None,
                default_value: Some(8.0),
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "performance_score", Some(1.0), Some(10.0))?;
        self.get_additional_param(params, "reliability_score", Some(1.0), Some(10.0))?;
        self.get_additional_param(params, "cost_score", Some(1.0), Some(10.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let perf = self.get_additional_param(&params, "performance_score", None, None)?;
        let reli = self.get_additional_param(&params, "reliability_score", None, None)?;
        let cost = self.get_additional_param(&params, "cost_score", None, None)?;

        let overall_score = (perf + reli + cost) / 3.0;

        let mut results = vec![
            ContractingResultItem {
                label: "Overall Score".to_string(),
                value: overall_score,
                unit: "/10".to_string(),
                tolerance: Some(0.5),
                formatted_value: Some(format!("{:.1}/10", overall_score)),
                is_critical: true,
            },
        ];

        let recommendations = if overall_score < 7.0 {
            vec!["Consider alternative subcontractor".to_string()]
        } else {
            vec!["Subcontractor meets standards".to_string()]
        };

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: 0.0,
                risk_level: (10.0 - overall_score) * 10.0,
                compliance_score: overall_score / 10.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations,
            compliance_notes: vec!["Compliant with PMP procurement management".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}