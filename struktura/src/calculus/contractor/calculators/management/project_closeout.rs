use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for project closeout
pub struct ProjectCloseoutCalculator;

impl ParameterValidator for ProjectCloseoutCalculator {
    fn calculator_id(&self) -> &str {
        "project_closeout"
    }
}

#[async_trait]
impl ContractorCalculator for ProjectCloseoutCalculator {
    fn id(&self) -> &str {
        "project_closeout"
    }

    fn name(&self) -> &str {
        "Project Closeout Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Management
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("project_closeout", "Project Closeout")
            .category("management")
            .description("Assesses project closeout readiness")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "completion_percentage".to_string(),
                path: "additional.completion_percentage".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Project completion percentage".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(100.0),
                typical_range: None,
                validation_rules: None,
                default_value: Some(100.0),
            })
            .parameter(ParameterMetadata {
                name: "outstanding_issues".to_string(),
                path: "additional.outstanding_issues".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Number of outstanding issues".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((0.0, 10.0)),
                validation_rules: Some(vec!["non_negative".to_string()]),
                default_value: Some(0.0),
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "completion_percentage", Some(0.0), Some(100.0))?;
        self.get_additional_param(params, "outstanding_issues", Some(0.0), None)?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let completion = self.get_additional_param(&params, "completion_percentage", None, None)?;
        let issues = self.get_additional_param(&params, "outstanding_issues", None, None)?;

        let readiness_score = completion - (issues * 5.0);
        let status = if readiness_score >= 95.0 && issues == 0.0 { "Ready for Closeout" } else { "Not Ready" };

        let mut results = vec![
            ContractingResultItem {
                label: "Readiness Score".to_string(),
                value: readiness_score,
                unit: "%".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2}%", readiness_score)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Closeout Status".to_string(),
                value: 0.0, // Placeholder
                unit: "".to_string(),
                tolerance: None,
                formatted_value: Some(status.to_string()),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: 0.0,
                risk_level: issues as f64 * 10.0,
                compliance_score: readiness_score / 100.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Resolve all issues before closeout".to_string()],
            compliance_notes: vec!["Compliant with PMP closeout procedures".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}