use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for milestone tracking
pub struct MilestoneTrackingCalculator;

impl ParameterValidator for MilestoneTrackingCalculator {
    fn calculator_id(&self) -> &str {
        "milestone_tracking"
    }
}

#[async_trait]
impl ContractorCalculator for MilestoneTrackingCalculator {
    fn id(&self) -> &str {
        "milestone_tracking"
    }

    fn name(&self) -> &str {
        "Milestone Tracking Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Scheduling
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("milestone_tracking", "Milestone Tracking")
            .category("scheduling")
            .description("Tracks milestone completion")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "total_milestones".to_string(),
                path: "additional.total_milestones".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Total milestones".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: Some((3.0, 20.0)),
                validation_rules: Some(vec!["integer".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "completed_milestones".to_string(),
                path: "additional.completed_milestones".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Completed milestones".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["integer".to_string()]),
                default_value: None,
            })
            .requires_certification()
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        let total = self.get_additional_param(params, "total_milestones", Some(1.0), None)?;
        let completed = self.get_additional_param(params, "completed_milestones", Some(0.0), Some(total))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let total = self.get_additional_param(&params, "total_milestones", None, None)?;
        let completed = self.get_additional_param(&params, "completed_milestones", None, None)?;

        let progress = (completed / total) * 100.0;
        let remaining = total - completed;

        let mut results = vec![
            ContractingResultItem {
                label: "Milestone Progress".to_string(),
                value: progress,
                unit: "%".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2}%", progress)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Remaining Milestones".to_string(),
                value: remaining,
                unit: "".to_string(),
                tolerance: None,
                formatted_value: Some(format!("{:.0}", remaining)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: 0.0,
                risk_level: (remaining / total) * 100.0,
                compliance_score: progress / 100.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Track milestones regularly".to_string()],
            compliance_notes: vec!["Compliant with PMP milestone management".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}