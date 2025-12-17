pub mod budget_forecast;
pub mod cost_breakdown;
pub mod equipment_cost;
pub mod labor_cost;
pub mod material_cost;
pub mod overhead;
pub mod quantity_takeoff;
pub mod value_engineering;

pub use budget_forecast::BudgetForecastCalculator;
pub use cost_breakdown::CostBreakdownCalculator;
pub use equipment_cost::EquipmentCostEstimator;
pub use labor_cost::LaborCostEstimator;
pub use material_cost::MaterialCostEstimator;
pub use overhead::OverheadCalculator;
pub use quantity_takeoff::QuantityTakeoffCalculator;
pub use value_engineering::ValueEngineeringCalculator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::contractor::test_utils::{minimal_parameters, parameters_with_dimensions};

    #[tokio::test]
    async fn test_quantity_takeoff() {
        let calc = QuantityTakeoffCalculator;
        let mut params = parameters_with_dimensions(vec![("length", 10.0), ("width", 5.0), ("height", 2.0)]);
        params.material = Some(MaterialProperties {
            waste_factor: Some(1.1),
            ..Default::default()
        });

        let result = calc.calculate(params).await.unwrap();
        assert_eq!(result.results.len(), 3);
        assert_eq!(result.results[2].value, 110.0); // 10*5*2 *1.1 = 110
    }
}