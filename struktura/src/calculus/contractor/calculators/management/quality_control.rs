use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for quality control metrics
pub struct QualityControlCalculator;

impl ParameterValidator for QualityControlCalculator {
    fn calculator_id(&self) -> &str {
        "quality_control"
    }
}

#[async_trait]
impl ContractorCalculator for QualityControlCalculator {
    fn id(&self) -> &str {
        "quality_control"
    }

    fn name(&self) -> &str {
        "Quality Control Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Management
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("quality_control", "Quality Control")
            .category("management")
            .description("Calculates quality metrics and defect rates")
            .regulation_code("ISO")
            .parameter(ParameterMetadata {
                name: "total_items".to_string(),
                path: "additional.total_items".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Total items produced/inspected".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: None,
                typical_range: Some((10.0, 100000.0)),
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "defective_items".to_string(),
                path: "additional.defective_items".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Number of defective items".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["non_negative".to_string()]),
                default_value: None,
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        let total = self.get_additional_param(params, "total_items", Some(1.0), None)?;
        let defective = self.get_additional_param(params, "defective_items", Some(0.0), Some(total))?;
        if defective > total {
            return Err(ContractingError::InvalidParameter {
                parameter: "defective_items".to_string(),
                value: defective.to_string(),
                reason: "Cannot exceed total items".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let total = self.get_additional_param(&params, "total_items", None, None)?;
        let defective = self.get_additional_param(&params, "defective_items", None, None)?;

        let defect_rate = (defective / total) * 100.0;
        let quality_score = 100.0 - defect_rate;

        let mut results = vec![
            ContractingResultItem {
                label: "Defect Rate".to_string(),
                value: defect_rate,
                unit: "%".to_string(),
                tolerance: Some(0.01),
                formatted_value: Some(format!("{:.2}%", defect_rate)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Quality Score".to_string(),
                value: quality_score,
                unit: "%".to_string(),
                tolerance: Some(0.01),
                formatted_value: Some(format!("{:.2}%", quality_score)),
                is_critical: true,
            },
        ];

        let warnings = if defect_rate > 5.0 {
            vec!["High defect rate detected".to_string()]
        } else {
            vec![]
        };

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: 0.0,
                risk_level: defect_rate,
                compliance_score: quality_score / 100.0,
            }),
            warnings,
            structured_warnings: None,
            recommendations: vec!["Implement quality checks if rate > 2%".to_string()],
            compliance_notes: vec!["Compliant with ISO quality standards".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "ISO".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}