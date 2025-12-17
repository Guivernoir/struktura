use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for safety planning
pub struct SafetyPlanningCalculator;

impl ParameterValidator for SafetyPlanningCalculator {
    fn calculator_id(&self) -> &str {
        "safety_planning"
    }
}

#[async_trait]
impl ContractorCalculator for SafetyPlanningCalculator {
    fn id(&self) -> &str {
        "safety_planning"
    }

    fn name(&self) -> &str {
        "Safety Planning Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Management
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("safety_planning", "Safety Planning")
            .category("management")
            .description("Assesses safety factors and requirements")
            .regulation_code("OSHA")
            .parameter(ParameterMetadata {
                name: "risk_reduction_factor".to_string(),
                path: "safety_factors.risk_reduction_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Risk reduction factor".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: Some(1.0),
                typical_range: Some((0.5, 0.95)),
                validation_rules: None,
                default_value: Some(0.9),
            })
            .parameter(ParameterMetadata {
                name: "importance_factor".to_string(),
                path: "safety_factors.importance_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Importance factor".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(2.0),
                typical_range: Some((1.0, 1.5)),
                validation_rules: None,
                default_value: Some(1.0),
            })
            .parameter(ParameterMetadata {
                name: "hazard_level".to_string(),
                path: "additional.hazard_level".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Hazard level (1-10)".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(10.0),
                typical_range: None,
                validation_rules: None,
                default_value: Some(5.0),
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        if params.safety_factors.is_none() {
            return Err(ContractingError::MissingParameter {
                parameter: "safety_factors".to_string(),
                calculator: self.id().to_string(),
            });
        }
        self.get_additional_param(params, "hazard_level", Some(1.0), Some(10.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let safety = params.safety_factors.as_ref().unwrap();
        let hazard = self.get_additional_param(&params, "hazard_level", None, None)?;

        let safety_index = (1.0 - safety.risk_reduction_factor) * safety.importance_factor * (hazard / 10.0);
        let safety_score = 1.0 - safety_index;

        let mut results = vec![
            ContractingResultItem {
                label: "Safety Index".to_string(),
                value: safety_index,
                unit: "".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2}", safety_index)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Safety Score".to_string(),
                value: safety_score * 100.0,
                unit: "%".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2}%", safety_score * 100.0)),
                is_critical: true,
            },
        ];

        let recommendations = if safety_index > 0.5 {
            vec!["Enhance safety measures".to_string()]
        } else {
            vec!["Current plan adequate".to_string()]
        };

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: 0.0,
                risk_level: safety_index * 100.0,
                compliance_score: safety_score,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations,
            compliance_notes: vec!["Compliant with OSHA safety planning".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "OSHA".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}