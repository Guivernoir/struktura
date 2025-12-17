use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Generator for Gantt chart parameters
pub struct GanttChartGenerator;

impl ParameterValidator for GanttChartGenerator {
    fn calculator_id(&self) -> &str {
        "gantt_chart"
    }
}

#[async_trait]
impl ContractorCalculator for GanttChartGenerator {
    fn id(&self) -> &str {
        "gantt_chart"
    }

    fn name(&self) -> &str {
        "Gantt Chart Generator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Scheduling
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("gantt_chart", "Gantt Chart Generator")
            .category("scheduling")
            .description("Generates parameters for Gantt chart")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "start_date".to_string(),
                path: "additional.start_date".to_string(),
                data_type: ParameterType::Number,
                unit: "unix timestamp".to_string(),
                description: "Project start date as unix timestamp".to_string(),
                required: true,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: None,
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "duration".to_string(),
                path: "additional.duration".to_string(),
                data_type: ParameterType::Number,
                unit: "days".to_string(),
                description: "Project duration".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: Some((30.0, 365.0)),
                validation_rules: None,
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "milestones".to_string(),
                path: "additional.milestones".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Number of milestones".to_string(),
                required: false,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((2.0, 10.0)),
                validation_rules: Some(vec!["integer".to_string()]),
                default_value: Some(5.0),
            })
            .requires_certification()
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "start_date", None, None)?;
        self.get_additional_param(params, "duration", Some(1.0), None)?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let start = self.get_additional_param(&params, "start_date", None, None)?;
        let duration = self.get_additional_param(&params, "duration", None, None)?;
        let milestones = self.get_additional_param(&params, "milestones", None, None).unwrap_or(5.0);

        let end = start + duration * 86400.0; // seconds in day
        let milestone_interval = duration / milestones;

        let mut results = vec![
            ContractingResultItem {
                label: "End Date".to_string(),
                value: end,
                unit: "unix timestamp".to_string(),
                tolerance: None,
                formatted_value: Some(format!("{:.0}", end)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Milestone Interval".to_string(),
                value: milestone_interval,
                unit: "days".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.1} days", milestone_interval)),
                is_critical: false,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: duration,
                risk_level: 0.0,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Use for visual scheduling".to_string()],
            compliance_notes: vec!["Compliant with PMP visualization".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}