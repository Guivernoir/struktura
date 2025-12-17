use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for budget forecasting
pub struct BudgetForecastCalculator;

impl ParameterValidator for BudgetForecastCalculator {
    fn calculator_id(&self) -> &str {
        "budget_forecast"
    }
}

#[async_trait]
impl ContractorCalculator for BudgetForecastCalculator {
    fn id(&self) -> &str {
        "budget_forecast"
    }

    fn name(&self) -> &str {
        "Budget Forecast Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Estimation
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("budget_forecast", "Budget Forecast")
            .category("estimation")
            .description("Forecasts budget requirements over time")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "total_cost".to_string(),
                path: "additional.total_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Total projected cost".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "duration_months".to_string(),
                path: "additional.duration_months".to_string(),
                data_type: ParameterType::Number,
                unit: "months".to_string(),
                description: "Project duration in months".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(120.0),
                typical_range: Some((3.0, 24.0)),
                validation_rules: None,
                default_value: Some(12.0),
            })
            .parameter(ParameterMetadata {
                name: "inflation_rate".to_string(),
                path: "additional.inflation_rate".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Annual inflation rate".to_string(),
                required: false,
                min_value: Some(0.0),
                max_value: Some(10.0),
                typical_range: Some((2.0, 5.0)),
                validation_rules: None,
                default_value: Some(3.0),
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "total_cost", Some(0.0), None)?;
        self.get_additional_param(params, "duration_months", Some(1.0), Some(120.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let total_cost = self.get_additional_param(&params, "total_cost", None, None)?;
        let duration = self.get_additional_param(&params, "duration_months", None, None)?;
        let inflation = self.get_additional_param(&params, "inflation_rate", None, None).unwrap_or(3.0) / 100.0;

        let monthly_cost = total_cost / duration;
        let adjusted_monthly = monthly_cost * (1.0 + inflation / 12.0).powf(duration);
        let forecast_total = adjusted_monthly * duration;

        let mut results = vec![
            ContractingResultItem {
                label: "Monthly Cost".to_string(),
                value: monthly_cost,
                unit: "USD/month".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("${:.2}/month", monthly_cost)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Forecast Total".to_string(),
                value: forecast_total,
                unit: "USD".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("${:.2}", forecast_total)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: forecast_total,
                total_duration: duration,
                risk_level: inflation * 100.0,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Monitor inflation trends".to_string()],
            compliance_notes: vec!["Compliant with PMP forecasting".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}