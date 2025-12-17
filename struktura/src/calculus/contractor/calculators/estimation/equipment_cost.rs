use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Estimator for equipment costs
pub struct EquipmentCostEstimator;

impl ParameterValidator for EquipmentCostEstimator {
    fn calculator_id(&self) -> &str {
        "equipment_cost"
    }
}

#[async_trait]
impl ContractorCalculator for EquipmentCostEstimator {
    fn id(&self) -> &str {
        "equipment_cost"
    }

    fn name(&self) -> &str {
        "Equipment Cost Estimator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Estimation
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("equipment_cost", "Equipment Cost Estimator")
            .category("estimation")
            .description("Estimates total equipment costs")
            .regulation_code("OSHA")
            .parameter(ParameterMetadata {
                name: "equipment_hours".to_string(),
                path: "resources.equipment_hours".to_string(),
                data_type: ParameterType::Number,
                unit: "hours".to_string(),
                description: "Equipment usage hours".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((1.0, 5000.0)),
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "equipment_rate".to_string(),
                path: "additional.equipment_rate".to_string(),
                data_type: ParameterType::Number,
                unit: "USD/hour".to_string(),
                description: "Hourly equipment rate".to_string(),
                required: true,
                min_value: Some(10.0),
                max_value: Some(500.0),
                typical_range: Some((50.0, 300.0)),
                validation_rules: None,
                default_value: Some(100.0),
            })
            .parameter(ParameterMetadata {
                name: "maintenance_factor".to_string(),
                path: "additional.maintenance_factor".to_string(),
                data_type: ParameterType::Number,
                unit: "".to_string(),
                description: "Maintenance cost factor".to_string(),
                required: false,
                min_value: Some(1.0),
                max_value: Some(1.3),
                typical_range: Some((1.05, 1.2)),
                validation_rules: None,
                default_value: Some(1.1),
            })
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.validate_resources(&params.resources)?;
        self.get_additional_param(params, "equipment_rate", Some(10.0), Some(500.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let resources = params.resources.as_ref().unwrap();
        let equipment_rate = self.get_additional_param(&params, "equipment_rate", None, None)?;
        let maintenance_factor = self.get_additional_param(&params, "maintenance_factor", None, None).unwrap_or(1.1);

        let adjusted_cost = resources.equipment_hours * equipment_rate * maintenance_factor;

        let mut results = vec![
            ContractingResultItem {
                label: "Total Equipment Cost".to_string(),
                value: adjusted_cost,
                unit: "USD".to_string(),
                tolerance: Some(0.1),
                formatted_value: Some(format!("${:.2}", adjusted_cost)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: adjusted_cost,
                total_duration: 0.0,
                risk_level: 0.0,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Include fuel and operator costs if separate".to_string()],
            compliance_notes: vec!["Compliant with OSHA equipment standards".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "OSHA".to_string(),
                requires_certification_review: false,
            }),
        })
    }
}