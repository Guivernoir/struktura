use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for cost breakdown
pub struct CostBreakdownCalculator;

impl ParameterValidator for CostBreakdownCalculator {
    fn calculator_id(&self) -> &str {
        "cost_breakdown"
    }
}

#[async_trait]
impl ContractorCalculator for CostBreakdownCalculator {
    fn id(&self) -> &str {
        "cost_breakdown"
    }

    fn name(&self) -> &str {
        "Cost Breakdown Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Estimation
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("cost_breakdown", "Cost Breakdown")
            .category("estimation")
            .description("Breaks down total costs into categories")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "material_cost".to_string(),
                path: "additional.material_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Material costs".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "labor_cost".to_string(),
                path: "additional.labor_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Labor costs".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "equipment_cost".to_string(),
                path: "additional.equipment_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Equipment costs".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "overhead".to_string(),
                path: "resources.overhead".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Overhead costs".to_string(),
                required: false,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: Some(0.0),
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "material_cost", Some(0.0), None)?;
        self.get_additional_param(params, "labor_cost", Some(0.0), None)?;
        self.get_additional_param(params, "equipment_cost", Some(0.0), None)?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let material_cost = self.get_additional_param(&params, "material_cost", None, None)?;
        let labor_cost = self.get_additional_param(&params, "labor_cost", None, None)?;
        let equipment_cost = self.get_additional_param(&params, "equipment_cost", None, None)?;
        let overhead = params.resources.as_ref().and_then(|r| r.overhead).unwrap_or(0.0);
        let total = material_cost + labor_cost + equipment_cost + overhead;

        let mut results = vec![
            ContractingResultItem {
                label: "Material Cost".to_string(),
                value: material_cost,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", material_cost)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Labor Cost".to_string(),
                value: labor_cost,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", labor_cost)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Equipment Cost".to_string(),
                value: equipment_cost,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", equipment_cost)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Overhead".to_string(),
                value: overhead,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", overhead)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Total Cost".to_string(),
                value: total,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", total)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: total,
                total_duration: 0.0,
                risk_level: 0.0,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Review cost allocations".to_string()],
            compliance_notes: vec!["Compliant with PMP breakdown".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}