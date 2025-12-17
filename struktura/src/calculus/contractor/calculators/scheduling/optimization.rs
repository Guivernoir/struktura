use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for schedule optimization
pub struct ScheduleOptimizationCalculator;

impl ParameterValidator for ScheduleOptimizationCalculator {
    fn calculator_id(&self) -> &str {
        "schedule_optimization"
    }
}

#[async_trait]
impl ContractorCalculator for ScheduleOptimizationCalculator {
    fn id(&self) -> &str {
        "schedule_optimization"
    }

    fn name(&self) -> &str {
        "Schedule Optimization Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Scheduling
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("schedule_optimization", "Schedule Optimization")
            .category("scheduling")
            .description("Optimizes project schedule")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "original_duration".to_string(),
                path: "additional.original_duration".to_string(),
                data_type: ParameterType::Number,
                unit: "days".to_string(),
                description: "Original duration".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: Some((30.0, 365.0)),
                validation_rules: None,
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "optimization_factor".to_string(),
                path: "additional.optimization_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Optimization factor (0-0.5)".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(0.5),
                typical_range: Some((0.1, 0.3)),
                validation_rules: None,
                default_value: Some(0.2),
            })
            .requires_certification()
            .complexity(ComplexityLevel::Advanced)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "original_duration", Some(1.0), None)?;
        self.get_additional_param(params, "optimization_factor", Some(0.0), Some(0.5))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let original = self.get_additional_param(&params, "original_duration", None, None)?;
        let factor = self.get_additional_param(&params, "optimization_factor", None, None)?;

        let optimized_duration = original * (1.0 - factor);
        let reduction = original - optimized_duration;

        let mut results = vec![
            ContractingResultItem {
                label: "Optimized Duration".to_string(),
                value: optimized_duration,
                unit: "days".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.1} days", optimized_duration)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Duration Reduction".to_string(),
                value: reduction,
                unit: "days".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.1} days", reduction)),
                is_critical: false,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: optimized_duration,
                risk_level: factor * 100.0,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Balance optimization with risk".to_string()],
            compliance_notes: vec!["Compliant with PMP optimization".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}