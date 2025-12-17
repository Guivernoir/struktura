use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for time-cost tradeoff
pub struct TimeCostTradeoffCalculator;

impl ParameterValidator for TimeCostTradeoffCalculator {
    fn calculator_id(&self) -> &str {
        "time_cost_tradeoff"
    }
}

#[async_trait]
impl ContractorCalculator for TimeCostTradeoffCalculator {
    fn id(&self) -> &str {
        "time_cost_tradeoff"
    }

    fn name(&self) -> &str {
        "Time-Cost Tradeoff Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Scheduling
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("time_cost_tradeoff", "Time-Cost Tradeoff")
            .category("scheduling")
            .description("Analyzes time-cost tradeoffs")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "normal_duration".to_string(),
                path: "additional.normal_duration".to_string(),
                data_type: ParameterType::Number,
                unit: "days".to_string(),
                description: "Normal duration".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: Some((30.0, 365.0)),
                validation_rules: None,
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "normal_cost".to_string(),
                path: "additional.normal_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Normal cost".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "crash_duration".to_string(),
                path: "additional.crash_duration".to_string(),
                data_type: ParameterType::Number,
                unit: "days".to_string(),
                description: "Crash duration".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: None,
                validation_rules: None,
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "crash_cost".to_string(),
                path: "additional.crash_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Crash cost".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .requires_certification()
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        let normal_dur = self.get_additional_param(params, "normal_duration", Some(1.0), None)?;
        let crash_dur = self.get_additional_param(params, "crash_duration", Some(1.0), None)?;
        let normal_cost = self.get_additional_param(params, "normal_cost", Some(0.0), None)?;
        let crash_cost = self.get_additional_param(params, "crash_cost", Some(0.0), None)?;
        if crash_dur >= normal_dur {
            return Err(ContractingError::DomainError {
                field: "crash_duration".to_string(),
                message: "Crash duration must be less than normal".to_string(),
            });
        }
        if crash_cost <= normal_cost {
            return Err(ContractingError::DomainError {
                field: "crash_cost".to_string(),
                message: "Crash cost must be greater than normal".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let normal_dur = self.get_additional_param(&params, "normal_duration", None, None)?;
        let crash_dur = self.get_additional_param(&params, "crash_duration", None, None)?;
        let normal_cost = self.get_additional_param(&params, "normal_cost", None, None)?;
        let crash_cost = self.get_additional_param(&params, "crash_cost", None, None)?;

        let time_saved = normal_dur - crash_dur;
        let added_cost = crash_cost - normal_cost;
        let cost_per_day = added_cost / time_saved;

        let mut results = vec![
            ContractingResultItem {
                label: "Time Saved".to_string(),
                value: time_saved,
                unit: "days".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.1} days", time_saved)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Added Cost".to_string(),
                value: added_cost,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", added_cost)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Cost per Day Saved".to_string(),
                value: cost_per_day,
                unit: "USD/day".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}/day", cost_per_day)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: crash_cost,
                total_duration: crash_dur,
                risk_level: (added_cost / normal_cost) * 100.0,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Evaluate if time savings justify cost".to_string()],
            compliance_notes: vec!["Compliant with PMP crashing techniques".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}