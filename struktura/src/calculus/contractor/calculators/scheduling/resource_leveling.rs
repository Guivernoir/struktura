use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for resource leveling
pub struct ResourceLevelingCalculator;

impl ParameterValidator for ResourceLevelingCalculator {
    fn calculator_id(&self) -> &str {
        "resource_leveling"
    }
}

#[async_trait]
impl ContractorCalculator for ResourceLevelingCalculator {
    fn id(&self) -> &str {
        "resource_leveling"
    }

    fn name(&self) -> &str {
        "Resource Leveling Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Scheduling
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("resource_leveling", "Resource Leveling")
            .category("scheduling")
            .description("Levels resource usage across project")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "peak_demand".to_string(),
                path: "additional.peak_demand".to_string(),
                data_type: ParameterType::Number,
                unit: "units".to_string(),
                description: "Peak resource demand".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: Some((5.0, 50.0)),
                validation_rules: None,
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "available_resources".to_string(),
                path: "additional.available_resources".to_string(),
                data_type: ParameterType::Number,
                unit: "units".to_string(),
                description: "Available resources".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: Some((5.0, 50.0)),
                validation_rules: None,
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "project_duration".to_string(),
                path: "additional.project_duration".to_string(),
                data_type: ParameterType::Number,
                unit: "days".to_string(),
                description: "Original project duration".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: Some((30.0, 365.0)),
                validation_rules: None,
                default_value: Some(90.0),
            })
            .requires_certification()
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        let peak = self.get_additional_param(params, "peak_demand", Some(1.0), None)?;
        let avail = self.get_additional_param(params, "available_resources", Some(1.0), None)?;
        if peak > avail * 2.0 {
            return Err(ContractingError::DomainError {
                field: "peak_demand".to_string(),
                message: "Peak demand too high compared to available".to_string(),
            });
        }
        self.get_additional_param(params, "project_duration", Some(1.0), None)?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let peak = self.get_additional_param(&params, "peak_demand", None, None)?;
        let avail = self.get_additional_param(&params, "available_resources", None, None)?;
        let duration = self.get_additional_param(&params, "project_duration", None, None)?;

        let leveling_factor = peak / avail;
        let adjusted_duration = duration * leveling_factor;

        let mut results = vec![
            ContractingResultItem {
                label: "Leveling Factor".to_string(),
                value: leveling_factor,
                unit: "".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.2}", leveling_factor)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Adjusted Duration".to_string(),
                value: adjusted_duration,
                unit: "days".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("{:.1} days", adjusted_duration)),
                is_critical: true,
            },
        ];

        let warnings = if leveling_factor > 1.5 {
            vec!["Significant schedule extension due to leveling".to_string()]
        } else {
            vec![]
        };

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: adjusted_duration,
                risk_level: (leveling_factor - 1.0) * 100.0,
                compliance_score: 1.0 / leveling_factor,
            }),
            warnings,
            structured_warnings: None,
            recommendations: vec!["Add resources if possible to reduce duration".to_string()],
            compliance_notes: vec!["Compliant with PMP resource management".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}