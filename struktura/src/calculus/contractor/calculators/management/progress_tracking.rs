use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for project progress
pub struct ProgressTrackingCalculator;

impl ParameterValidator for ProgressTrackingCalculator {
    fn calculator_id(&self) -> &str {
        "progress_tracking"
    }
}

#[async_trait]
impl ContractorCalculator for ProgressTrackingCalculator {
    fn id(&self) -> &str {
        "progress_tracking"
    }

    fn name(&self) -> &str {
        "Progress Tracking Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Management
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("progress_tracking", "Progress Tracking")
            .category("management")
            .description("Tracks project progress and variance")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "planned_progress".to_string(),
                path: "additional.planned_progress".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Planned progress percentage".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(100.0),
                typical_range: None,
                validation_rules: None,
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "actual_progress".to_string(),
                path: "additional.actual_progress".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Actual progress percentage".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(100.0),
                typical_range: None,
                validation_rules: None,
                default_value: None,
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "planned_progress", Some(0.0), Some(100.0))?;
        self.get_additional_param(params, "actual_progress", Some(0.0), Some(100.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let planned = self.get_additional_param(&params, "planned_progress", None, None)?;
        let actual = self.get_additional_param(&params, "actual_progress", None, None)?;

        let variance = actual - planned;
        let status = if variance > 0.0 { "Ahead" } else if variance < 0.0 { "Behind" } else { "On Track" };

        let mut results = vec![
            ContractingResultItem {
                label: "Progress Variance".to_string(),
                value: variance,
                unit: "%".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2}%", variance)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Project Status".to_string(),
                value: 0.0, // Placeholder, since it's string
                unit: "".to_string(),
                tolerance: None,
                formatted_value: Some(status.to_string()),
                is_critical: true,
            },
        ];

        let warnings = if variance < -10.0 {
            vec!["Significant delay detected".to_string()]
        } else {
            vec![]
        };

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: variance.abs(),
                risk_level: if variance < 0.0 { -variance } else { 0.0 },
                compliance_score: actual / 100.0,
            }),
            warnings,
            structured_warnings: None,
            recommendations: vec!["Adjust resources if behind schedule".to_string()],
            compliance_notes: vec!["Compliant with PMP progress tracking".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}