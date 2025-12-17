use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for delay analysis
pub struct DelayAnalysisCalculator;

impl ParameterValidator for DelayAnalysisCalculator {
    fn calculator_id(&self) -> &str {
        "delay_analysis"
    }
}

#[async_trait]
impl ContractorCalculator for DelayAnalysisCalculator {
    fn id(&self) -> &str {
        "delay_analysis"
    }

    fn name(&self) -> &str {
        "Delay Analysis Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Scheduling
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("delay_analysis", "Delay Analysis")
            .category("scheduling")
            .description("Analyzes schedule delays")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "planned_duration".to_string(),
                path: "additional.planned_duration".to_string(),
                data_type: ParameterType::Number,
                unit: "days".to_string(),
                description: "Planned duration".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: Some((30.0, 365.0)),
                validation_rules: None,
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "actual_duration".to_string(),
                path: "additional.actual_duration".to_string(),
                data_type: ParameterType::Number,
                unit: "days".to_string(),
                description: "Actual duration".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: Some((30.0, 365.0)),
                validation_rules: None,
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "delay_cause_factor".to_string(),
                path: "additional.delay_cause_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Factor for delay causes (0-1)".to_string(),
                required: false,
                min_value: Some(0.0),
                max_value: Some(1.0),
                typical_range: Some((0.1, 0.5)),
                validation_rules: None,
                default_value: Some(0.2),
            })
            .requires_certification()
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        let planned = self.get_additional_param(params, "planned_duration", Some(1.0), None)?;
        let actual = self.get_additional_param(params, "actual_duration", Some(1.0), None)?;
        if actual < planned {
            return Err(ContractingError::DomainError {
                field: "actual_duration".to_string(),
                message: "Actual should be >= planned for delay analysis".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let planned = self.get_additional_param(&params, "planned_duration", None, None)?;
        let actual = self.get_additional_param(&params, "actual_duration", None, None)?;
        let cause_factor = self.get_additional_param(&params, "delay_cause_factor", None, None).unwrap_or(0.2);

        let delay = actual - planned;
        let compensable_delay = delay * cause_factor;
        let non_compensable_delay = delay - compensable_delay;

        let mut results = vec![
            ContractingResultItem {
                label: "Total Delay".to_string(),
                value: delay,
                unit: "days".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.1} days", delay)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Compensable Delay".to_string(),
                value: compensable_delay,
                unit: "days".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.1} days", compensable_delay)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Non-Compensable Delay".to_string(),
                value: non_compensable_delay,
                unit: "days".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.1} days", non_compensable_delay)),
                is_critical: false,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: actual,
                risk_level: (delay / planned) * 100.0,
                compliance_score: planned / actual,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Analyze causes for compensable delays".to_string()],
            compliance_notes: vec!["Compliant with PMP delay analysis".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}