use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for assessing project risks
pub struct RiskAssessmentCalculator;

impl ParameterValidator for RiskAssessmentCalculator {
    fn calculator_id(&self) -> &str {
        "risk_assessment"
    }
}

#[async_trait]
impl ContractorCalculator for RiskAssessmentCalculator {
    fn id(&self) -> &str {
        "risk_assessment"
    }

    fn name(&self) -> &str {
        "Risk Assessment Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Bidding
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("risk_assessment", "Risk Assessment")
            .category("bidding")
            .description("Evaluates project risk levels based on various factors")
            .regulation_code("OSHA")
            .parameter(ParameterMetadata {
                name: "risk_reduction_factor".to_string(),
                path: "safety_factors.risk_reduction_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Factor for risk reduction measures".to_string(),
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
                description: "Project importance factor".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(2.0),
                typical_range: Some((1.0, 1.5)),
                validation_rules: None,
                default_value: Some(1.0),
            })
            .parameter(ParameterMetadata {
                name: "project_complexity".to_string(),
                path: "additional.project_complexity".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Complexity score (1-10)".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(10.0),
                typical_range: None,
                validation_rules: None,
                default_value: Some(5.0),
            })
            .requires_certification()
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        if params.safety_factors.is_none() {
            return Err(ContractingError::MissingParameter {
                parameter: "safety_factors".to_string(),
                calculator: self.id().to_string(),
            });
        }
        self.get_additional_param(params, "project_complexity", Some(1.0), Some(10.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let safety = params.safety_factors.as_ref().unwrap();
        let complexity = self.get_additional_param(&params, "project_complexity", None, None)?;

        let base_risk = complexity / 10.0;
        let adjusted_risk = base_risk * safety.importance_factor * (1.0 - safety.risk_reduction_factor);
        let risk_level = adjusted_risk * 100.0; // As percentage

        let mut results = vec![
            ContractingResultItem {
                label: "Base Risk".to_string(),
                value: base_risk,
                unit: "".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.2}%", base_risk * 100.0)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Adjusted Risk Level".to_string(),
                value: risk_level,
                unit: "%".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.2}%", risk_level)),
                is_critical: true,
            },
        ];

        let warnings = if risk_level > 50.0 {
            vec!["High risk level detected".to_string()]
        } else {
            vec![]
        };

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: 0.0,
                risk_level,
                compliance_score: 1.0 - adjusted_risk,
            }),
            warnings,
            structured_warnings: None,
            recommendations: vec!["Implement additional risk mitigation if level > 30%".to_string()],
            compliance_notes: vec!["Compliant with OSHA risk assessment".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "OSHA".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}