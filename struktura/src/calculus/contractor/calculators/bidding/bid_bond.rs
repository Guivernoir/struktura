use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{ContractorCalculator, ParameterValidator},
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calculator for bid bonds
pub struct BidBondCalculator;

impl ParameterValidator for BidBondCalculator {
    fn calculator_id(&self) -> &str {
        "bid_bond"
    }
}

#[async_trait]
impl ContractorCalculator for BidBondCalculator {
    fn id(&self) -> &str {
        "bid_bond"
    }

    fn name(&self) -> &str {
        "Bid Bond Calculator"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Bidding
    }

    fn metadata(&self) -> ContractingCalculatorMetadata {
        ContractingCalculatorMetadata::builder("bid_bond", "Bid Bond")
            .category("bidding")
            .description("Calculates required bid bond amount")
            .regulation_code("IBC")
            .parameter(ParameterMetadata {
                name: "bid_price".to_string(),
                path: "additional.bid_price".to_string(),
                data_type: ParameterType::Number,
                unit: "USD".to_string(),
                description: "Proposed bid price".to_string(),
                required: true,
                min_value: Some(0.0),
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["positive".to_string()]),
                default_value: None,
            })
            .parameter(ParameterMetadata {
                name: "bond_percentage".to_string(),
                path: "additional.bond_percentage".to_string(),
                data_type: ParameterType::Number,
                unit: "%".to_string(),
                description: "Bond percentage requirement".to_string(),
                required: true,
                min_value: Some(1.0),
                max_value: Some(10.0),
                typical_range: Some((2.0, 5.0)),
                validation_rules: None,
                default_value: Some(5.0),
            })
            .requires_certification()
            .complexity(ComplexityLevel::Basic)
            .build()
    }

    fn validate(&self, params: &ContractingParameters) -> ContractingResult<()> {
        self.get_additional_param(params, "bid_price", Some(0.0), None)?;
        self.get_additional_param(params, "bond_percentage", Some(1.0), Some(10.0))?;
        Ok(())
    }

    async fn calculate(&self, params: ContractingParameters) -> ContractingResult<ContractingCalculationResponse> {
        let bid_price = self.get_additional_param(&params, "bid_price", None, None)?;
        let bond_pct = self.get_additional_param(&params, "bond_percentage", None, None)?;

        let bond_amount = bid_price * (bond_pct / 100.0);

        let mut results = vec![
            ContractingResultItem {
                label: "Bid Bond Amount".to_string(),
                value: bond_amount,
                unit: "USD".to_string(),
                tolerance: Some(0.01),
                formatted_value: Some(format!("${:.2}", bond_amount)),
                is_critical: true,
            },
        ];

        Ok(ContractingCalculationResponse {
            calculation_type: self.id().to_string(),
            results,
            analysis: Some(ProjectAnalysisResult {
                total_cost: bond_amount,
                total_duration: 0.0,
                risk_level: bond_pct,
                compliance_score: 1.0,
            }),
            warnings: vec![],
            structured_warnings: None,
            recommendations: vec!["Ensure bond is obtained from approved surety".to_string()],
            compliance_notes: vec!["Compliant with IBC bonding requirements".to_string()],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: "1.0".to_string(),
                regulation_code_used: "IBC".to_string(),
                requires_certification_review: true,
            }),
        })
    }
}