use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for critical path method
pub struct CriticalPathCalculator;

impl ParameterValidator for CriticalPathCalculator {
    fn calculator_id(&self) -> &str {
        "critical_path"
    }
}

#[async_trait]
impl ContractorCalculator for CriticalPathCalculator {
    fn id(&self) -> &str {
        "critical_path"
    }

    fn name(&self) -> &str {
        "Critical Path Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Scheduling
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("critical_path", "Critical Path")
            .category("scheduling")
            .description("Calculates project critical path duration")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "total_tasks".to_string(),
                path: "additional.total_tasks".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Total number of tasks".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: Some((5.0, 100.0)),
                validation_rules: Some(vec!["integer".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "avg_duration".to_string(),
                path: "additional.avg_duration".to_string(),
                data_type: ParameterType::Number,
                unit: "days".to_string(),
                description: "Average task duration".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(365.0),
                typical_range: Some((5.0, 30.0)),
                validation_rules: None,
                default_value: Some(10.0),
            })
            .parameter(ParameterMetadata {
                name: "parallel_factor".to_string(),
                path: "additional.parallel_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Factor for parallel tasks (0-1)".to_string(),
                required: false,
                min_value: Some(0.0),
                max_value: Some(1.0),
                typical_range: Some((0.2, 0.8)),
                validation_rules: None,
                default_value: Some(0.5),
            })
            .requires_certification()
            .complexity(ComplexityLevel::Advanced)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "total_tasks", Some(1.0), None)?;
        self.get_additional_param(params, "avg_duration", Some(1.0), Some(365.0))?;
        self.get_additional_param(params, "parallel_factor", Some(0.0), Some(1.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let total_tasks = self.get_additional_param(&params, "total_tasks", None, None)?;
        let avg_duration = self.get_additional_param(&params, "avg_duration", None, None)?;
        let parallel_factor = self.get_additional_param(&params, "parallel_factor", None, None).unwrap_or(0.5);

        let sequential_tasks = total_tasks * (1.0 - parallel_factor);
        let critical_duration = sequential_tasks * avg_duration;
        let total_duration = critical_duration * 1.1; // Add 10% buffer

        let mut results = vec![
            ContractingResultItem {
                label: "Critical Path Duration".to_string(),
                value: critical_duration,
                unit: "days".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.1} days", critical_duration)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Total Estimated Duration".to_string(),
                value: total_duration,
                unit: "days".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.1} days", total_duration)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration,
                risk_level: (1.0 - parallel_factor) * 100.0,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Identify and monitor critical path tasks".to_string()],
            compliance_notes: vec!["Compliant with PMP scheduling".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}