use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for determining optimal bid price
pub struct BidPricingCalculator;

impl ParameterValidator for BidPricingCalculator {
    fn calculator_id(&self) -> &str {
        "bid_pricing"
    }
}

#[async_trait]
impl ContractorCalculator for BidPricingCalculator {
    fn id(&self) -> &str {
        "bid_pricing"
    }

    fn name(&self) -> &str {
        "Bid Pricing Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Bidding
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("bid_pricing", "Bid Pricing")
            .category("bidding")
            .description("Calculates optimal bid price based on direct costs, overhead, and markup")
            .regulation_code("PMP")
            .parameter(ParameterMetadata {
                name: "labor_hours".to_string(),
                path: "resources.labor_hours".to_string(),
                data_type: ParameterType::Number,
                unit: "hours".to_string(),
                description: "Total labor hours required".to_string(),
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
                description: "Total equipment usage hours".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((1.0, 5000.0)),
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "material_quantity".to_string(),
                path: "resources.material_quantity".to_string(),
                data_type: ParameterType::Number,
                unit: "units".to_string(),
                description: "Quantity of material required".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((1.0, 100000.0)),
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "unit_cost".to_string(),
                path: "material.unit_cost".to_string(),
                data_type: ParameterType::Number,
                unit: "USD/unit".to_string(),
                description: "Cost per unit of material".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: Some((0.1, 1000.0)),
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
                name: "markup_percentage".to_string(),
                path: "additional.markup_percentage".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Markup percentage on total costs".to_string(),
                required: true,
                min_value: Some(5.0),
                max_value: Some(50.0),
                typical_range: Some((10.0, 30.0)),
                validation_rules: None,
                default_value: Some(20.0),
            })
            .requires_certification()
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.validate_resources(&params.resources)?;
        self.validate_material(&params.material)?;
        self.get_additional_param(params, "labor_rate", Some(10.0), Some(200.0))?;
        self.get_additional_param(params, "equipment_rate", Some(10.0), Some(500.0))?;
        self.get_additional_param(params, "markup_percentage", Some(5.0), Some(50.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let resources = params.resources.as_ref().unwrap();
        let material = params.material.as_ref().unwrap();
        let labor_rate = self.get_additional_param(&params, "labor_rate", None, None)?;
        let equipment_rate = self.get_additional_param(&params, "equipment_rate", None, None)?;
        let markup_percentage = self.get_additional_param(&params, "markup_percentage", None, None)?;

        let labor_cost = resources.labor_hours * labor_rate;
        let equipment_cost = resources.equipment_hours * equipment_rate;
        let material_cost = resources.material_quantity.unwrap_or(0.0) * material.unit_cost.unwrap_or(0.0);
        let total_direct_cost = labor_cost + equipment_cost + material_cost;
        let overhead = resources.overhead.unwrap_or(0.0);
        let sub_cost = resources.subcontractor_cost.unwrap_or(0.0);
        let total_cost = total_direct_cost + overhead + sub_cost;
        let markup = total_cost * (markup_percentage / 100.0);
        let bid_price = total_cost + markup;

        let mut results = vec![
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
                label: "Material Cost".to_string(),
                value: material_cost,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", material_cost)),
                is_critical: false,
            },
            ContractingResultItem {
                label: "Total Cost".to_string(),
                value: total_cost,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", total_cost)),
                is_critical: true,
            },
            ContractingResultItem {
                label: "Bid Price".to_string(),
                value: bid_price,
                unit: "USD".to_string(),
                tolerance: Some(0.05),
                formatted_value: Some(format!("${:.2}", bid_price)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost,
                total_duration: 0.0, // Not applicable
                risk_level: 0.0,    // Not calculated here
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Review market conditions before finalizing bid".to_string()],
            compliance_notes: vec!["Compliant with PMP guidelines".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "PMP".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}