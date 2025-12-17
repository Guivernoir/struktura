use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for value engineering
pub struct ValueEngineeringCalculator;

impl ParameterValidator for ValueEngineeringCalculator {
    fn calculator_id(&self) -> &str {
        "value_engineering"
    }
}

#[async_trait]
impl ContractorCalculator for ValueEngineeringCalculator {
    fn id(&self) -> &str {
        "value_engineering"
    }

    fn name(&self) -> &str {
        "Value Engineering Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Estimation
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("value_engineering", "Value Engineering")
            .category("estimation")
            .description("Evaluates cost savings from alternative approaches")
            .regulation_code("ASTM")
            .parameter(ParameterMetadata {
                name: "original_cost".to_string(),
                path: "additional.original_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Original estimated cost".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "alternative_cost".to_string(),
                path: "additional.alternative_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Alternative approach cost".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "performance_factor".to_string(),
                path: "additional.performance_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Performance ratio of alternative".to_string(),
                required: false,
                min_value: Some(0.5),
                max_value: Some(2.0),
                typical_range: Some((0.8, 1.2)),
                validation_rules: None,
                default_value: Some(1.0),
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        let original = self.get_additional_param(params, "original_cost", Some(0.0), None)?;
        let alternative = self.get_additional_param(params, "alternative_cost", Some(0.0), None)?;
        if alternative > original {
            return Err(ContractingError::DomainError {
                field: "alternative_cost".to_string(),
                message: "Alternative should be less than original for savings".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let original_cost = self.get_additional_param(&params, "original_cost", None, None)?;
        let alternative_cost = self.get_additional_param(&params, "alternative_cost", None, None)?;
        let performance = self.get_additional_param(&params, "performance_factor", None, None).unwrap_or(1.0);

        let savings = original_cost - alternative_cost;
        let value_index = savings / alternative_cost * performance;

        let mut results = vec![
            ContractingResultItem {
                label: "Cost Savings".to_string(),
                value: savings,
                unit: "USD".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("${:.2}", savings)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Value Index".to_string(),
                value: value_index,
                unit: "".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.2}", value_index)),
                is_critical: true,
            },
        ];

        let recommendations = if value_index > 0.1 {
            vec!["Alternative provides good value".to_string()]
        } else {
            vec!["Reevaluate alternative".to_string()]
        };

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: alternative_cost,
                total_duration: 0.0,
                risk_level: 1.0 - performance,
                compliance_score: performance,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations,
            compliance_notes: vec!["Compliant with ASTM value engineering".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "ASTM".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}