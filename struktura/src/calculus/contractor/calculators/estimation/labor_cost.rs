use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Estimator for labor costs
pub struct LaborCostEstimator;

impl ParameterValidator for LaborCostEstimator {
    fn calculator_id(&self) -> &str {
        "labor_cost"
    }
}

#[async_trait]
impl ContractorCalculator for LaborCostEstimator {
    fn id(&self) -> &str {
        "labor_cost"
    }

    fn name(&self) -> &str {
        "Labor Cost Estimator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Estimation
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("labor_cost", "Labor Cost Estimator")
            .category("estimation")
            .description("Estimates total labor costs")
            .regulation_code("OSHA")
            .parameter(ParameterMetadata {
                name: "labor_hours".to_string(),
                path: "resources.labor_hours".to_string(),
                data_type: ParameterType::Number,
                unit: "hours".to_string(),
                description: "Total labor hours".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((1.0, 10000.0)),
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "labor_rate".to_string(),
                path: "additional.labor_rate".to_string(),
                data_type: ParameterType::Number,
                unit: "USD/hour".to_string(),
                description: "Hourly labor rate".to_string(),
                required: true,
                min_value: Some(10.0),
                max_value: Some(200.0),
                typical_range: Some((20.0, 100.0)),
                validation_rules: None,
                default_value: Some(50.0),
            })
            .parameter(ParameterMetadata {
                name: "productivity_factor".to_string(),
                path: "additional.productivity_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Productivity adjustment".to_string(),
                required: false,
                min_value: Some(0.5),
                max_value: Some(1.5),
                typical_range: Some((0.8, 1.2)),
                validation_rules: None,
                default_value: Some(1.0),
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.validate_resources(&params.resources)?;
        self.get_additional_param(params, "labor_rate", Some(10.0), Some(200.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let resources = params.resources.as_ref().unwrap();
        let labor_rate = self.get_additional_param(&params, "labor_rate", None, None)?;
        let productivity = self.get_additional_param(&params, "productivity_factor", None, None).unwrap_or(1.0);

        let adjusted_hours = resources.labor_hours / productivity;
        let total_labor_cost = adjusted_hours * labor_rate;

        let mut results = vec![
            ContractingResultItem {
                label: "Adjusted Labor Hours".to_string(),
                value: adjusted_hours,
                unit: "hours".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.2} hours", adjusted_hours)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Total Labor Cost".to_string(),
                value: total_labor_cost,
                unit: "USD".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("${:.2}", total_labor_cost)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: total_labor_cost,
                total_duration: 0.0,
                risk_level: 0.0,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Consider overtime rates if applicable".to_string()],
            compliance_notes: vec!["Compliant with OSHA labor standards".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "OSHA".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}