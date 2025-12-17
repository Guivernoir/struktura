pub mod bid_bond;
pub mod bid_pricing;
pub mod contingency_planning;
pub mod contract_estimation;
pub mod profit_margin;
pub mod risk_assessment;

pub use bid_bond::BidBondCalculator;
pub use bid_pricing::BidPricingCalculator;
pub use contingency_planning::ContingencyPlanningCalculator;
pub use contract_estimation::ContractEstimationCalculator;
pub use profit_margin::ProfitMarginCalculator;
pub use risk_assessment::RiskAssessmentCalculator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::contractor::test_utils::parameters_with_resources;

    #[tokio::test]
    async fn test_bid_pricing() {
        let calc = BidPricingCalculator;
        let mut params = parameters_with_resources(100.0, 50.0);
        params.resources.as_mut().unwrap().material_quantity = Some(200.0);
        params.material = Some(MaterialProperties {
            unit_cost: Some(10.0),
            ..Default::default()
        });
        params.additional = Some(HashMap::from([
            ("labor_rate".to_string(), 50.0),
            ("equipment_rate".to_string(), 100.0),
            ("markup_percentage".to_string(), 20.0),
        ]));

        let result = calc.calculate(params).await.unwrap();
        assert_eq!(result.results.len(), 5);
        assert_eq!(result.results[4].value, 12000.0); // Example expected bid_price
    }
}