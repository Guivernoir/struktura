use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for resource allocation
pub struct ResourceAllocationCalculator;

impl ParameterValidator for ResourceAllocationCalculator {
    fn calculator_id(&self) -> &str {
        "resource_allocation"
    }
}

#[async_trait]
impl ContractorCalculator for ResourceAllocationCalculator {
    fn id(&self) -> &str {
        "resource_allocation"
    }

    fn name(&self) -> &str {
        "Resource Allocation Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Management
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("resource_allocation", "Resource Allocation")
            .category("management")
            .description("Allocates resources based on requirements")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "labor_hours".to_string(),
                path: "resources.labor_hours".to_string(),
                data_type: ParameterType::Number,
                unit: "hours".to_string(),
                description: "Required labor hours".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((1.0, 10000.0)),
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "equipment_hours".to_string(),
                path: "resources.equipment_hours".to_string(),
                data_type: ParameterType::Number,
                unit: "hours".to_string(),
                description: "Required equipment hours".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((1.0, 5000.0)),
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "available_labor".to_string(),
                path: "additional.available_labor".to_string(),
                data_type: ParameterType::Number,
                unit: "hours".to_string(),
                description: "Available labor hours".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "available_equipment".to_string(),
                path: "additional.available_equipment".to_string(),
                data_type: ParameterType::Number,
                unit: "hours".to_string(),
                description: "Available equipment hours".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.validate_resources(&params.resources)?;
        let avail_labor = self.get_additional_param(params, "available_labor", Some(0.0), None)?;
        let avail_equip = self.get_additional_param(params, "available_equipment", Some(0.0), None)?;
        let resources = params.resources.as_ref().unwrap();
        if resources.labor_hours > avail_labor {
            return Err(ContractingError::DomainError {
                field: "labor_hours".to_string(),
                message: "Required labor exceeds available".to_string(),
            });
        }
        if resources.equipment_hours > avail_equip {
            return Err(ContractingError::DomainError {
                field: "equipment_hours".to_string(),
                message: "Required equipment exceeds available".to_string(),
            });
        }
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let resources = params.resources.as_ref().unwrap();
        let avail_labor = self.get_additional_param(&params, "available_labor", None, None)?;
        let avail_equip = self.get_additional_param(&params, "available_equipment", None, None)?;

        let labor_util = (resources.labor_hours / avail_labor) * 100.0;
        let equip_util = (resources.equipment_hours / avail_equip) * 100.0;

        let mut results = vec![
            ContractingResultItem {
                label: "Labor Utilization".to_string(),
                value: labor_util,
                unit: "%".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2}%", labor_util)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Equipment Utilization".to_string(),
                value: equip_util,
                unit: "%".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("{:.2}%", equip_util)),
                is_critical: true,
            },
        ];

        let warnings = if labor_util > 90.0 || equip_util > 90.0 {
            vec!["High utilization - risk of overallocation".to_string()]
        } else {
            vec![]
        };

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: 0.0,
                total_duration: 0.0,
                risk_level: (labor_util + equip_util) / 2.0,
                compliance_score: 1.0,
            }),
            warnings,
            structured_warnings: None,
            recommendations: vec!["Monitor allocation weekly".to_string()],
            compliance_notes: vec!["Compliant with PMP resource management".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}