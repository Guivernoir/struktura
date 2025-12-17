use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::helpers::*;

pub struct InventoryOptimizationCalculator;

impl ParameterValidator for InventoryOptimizationCalculator {
    fn calculator_id(&self) -> &str {
        "inventory_optimization"
    }
}

#[async_trait]
impl EngineerCalculator for InventoryOptimizationCalculator {
    fn id(&self) -> &str {
        "inventory_optimization"
    }

    fn name(&self) -> &str {
        "Inventory Optimization (EOQ, ROP, Safety Stock)"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Production
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder(
            "inventory_optimization",
            "Inventory Optimization (EOQ, ROP, Safety Stock)"
        )
        .category("production")
        .description("Calculate Economic Order Quantity (EOQ), Reorder Point (ROP), and Safety Stock for inventory management")
        .design_code("Lean Manufacturing")
        .parameter(ParameterMetadata {
            name: "Annual Demand".to_string(),
            path: "additional.annual_demand".to_string(),
            data_type: ParameterType::Number,
            unit: "units/year".to_string(),
            description: "Expected annual demand".to_string(),
            required: true,
            default_value: Some(10000.0),
            min_value: Some(1.0),
            max_value: None,
            typical_range: Some((1000.0, 50000.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Ordering Cost".to_string(),
            path: "additional.ordering_cost".to_string(),
            data_type: ParameterType::Number,
            unit: "USD/order".to_string(),
            description: "Cost per order placement".to_string(),
            required: true,
            default_value: Some(50.0),
            min_value: Some(0.1),
            max_value: None,
            typical_range: Some((10.0, 200.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Holding Cost per Unit".to_string(),
            path: "additional.holding_cost_per_unit".to_string(),
            data_type: ParameterType::Number,
            unit: "USD/unit/year".to_string(),
            description: "Annual holding cost per unit".to_string(),
            required: true,
            default_value: Some(2.0),
            min_value: Some(0.1),
            max_value: None,
            typical_range: Some((1.0, 10.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Daily Demand".to_string(),
            path: "additional.daily_demand".to_string(),
            data_type: ParameterType::Number,
            unit: "units/day".to_string(),
            description: "Average daily demand".to_string(),
            required: true,
            default_value: Some(40.0),
            min_value: Some(0.1),
            max_value: None,
            typical_range: Some((10.0, 200.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Lead Time Days".to_string(),
            path: "additional.lead_time_days".to_string(),
            data_type: ParameterType::Number,
            unit: "days".to_string(),
            description: "Supplier lead time".to_string(),
            required: true,
            default_value: Some(7.0),
            min_value: Some(1.0),
            max_value: Some(365.0),
            typical_range: Some((3.0, 30.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Safety Stock".to_string(),
            path: "additional.safety_stock".to_string(),
            data_type: ParameterType::Number,
            unit: "units".to_string(),
            description: "Buffer stock for variability".to_string(),
            required: false,
            default_value: Some(100.0),
            min_value: Some(0.0),
            max_value: None,
            typical_range: Some((50.0, 500.0)),
            validation_rules: None,
        })
        .complexity(ComplexityLevel::Basic)
        .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        self.get_additional_param(params, "annual_demand", Some(1.0), None)?;
        self.get_additional_param(params, "ordering_cost", Some(0.1), None)?;
        self.get_additional_param(params, "holding_cost_per_unit", Some(0.1), None)?;
        self.get_additional_param(params, "daily_demand", Some(0.1), None)?;
        self.get_additional_param(params, "lead_time_days", Some(1.0), Some(365.0))?;
        if let Some(additional) = &params.additional {
            if let Some(safety_stock) = additional.get("safety_stock") {
                if *safety_stock < 0.0 {
                    return Err(EngineeringError::InvalidParameter {
                        parameter: "safety_stock".to_string(),
                        value: safety_stock.to_string(),
                        reason: "Must be >= 0".to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let annual_demand = self.get_additional_param(&params, "annual_demand", None, None)?;
        let ordering_cost = self.get_additional_param(&params, "ordering_cost", None, None)?;
        let holding_cost_per_unit = self.get_additional_param(&params, "holding_cost_per_unit", None, None)?;
        let daily_demand = self.get_additional_param(&params, "daily_demand", None, None)?;
        let lead_time_days = self.get_additional_param(&params, "lead_time_days", None, None)?;
        let safety_stock = params.additional.as_ref().and_then(|a| a.get("safety_stock").copied()).unwrap_or(0.0);

        let eoq_value = eoq(annual_demand, ordering_cost, holding_cost_per_unit);
        let rop_value = reorder_point(daily_demand, lead_time_days, safety_stock);

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if eoq_value > annual_demand * 0.1 {
            warnings.push(format!("EOQ ({:.0} units) is high relative to annual demand. Verify costs.", eoq_value));
        }

        if safety_stock == 0.0 {
            recommendations.push("Consider adding safety stock for demand variability".to_string());
        }

        compliance_notes.push("Inventory optimization per lean principles to minimize waste".to_string());
        compliance_notes.push("Regular review of demand forecasts recommended".to_string());

        let results = vec![
            EngineeringResultItem::new("Economic Order Quantity (EOQ)", eoq_value, "units")
                .critical()
                .with_format(format!("{:.0} units", eoq_value)),
            EngineeringResultItem::new("Reorder Point (ROP)", rop_value, "units")
                .critical()
                .with_format(format!("{:.0} units", rop_value)),
            EngineeringResultItem::new("Safety Stock", safety_stock, "units"),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "inventory_optimization".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "Lean Manufacturing".to_string(),
                requires_pe_review: false,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::engineer::test_utils::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_inventory_optimization() {
        let calc = InventoryOptimizationCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("annual_demand".to_string(), 10000.0);
        additional.insert("ordering_cost".to_string(), 50.0);
        additional.insert("holding_cost_per_unit".to_string(), 2.0);
        additional.insert("daily_demand".to_string(), 40.0);
        additional.insert("lead_time_days".to_string(), 7.0);
        additional.insert("safety_stock".to_string(), 100.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() == 3);
    }

    #[test]
    fn test_negative_safety_stock() {
        let calc = InventoryOptimizationCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("annual_demand".to_string(), 10000.0);
        additional.insert("ordering_cost".to_string(), 50.0);
        additional.insert("holding_cost_per_unit".to_string(), 2.0);
        additional.insert("daily_demand".to_string(), 40.0);
        additional.insert("lead_time_days".to_string(), 7.0);
        additional.insert("safety_stock".to_string(), -10.0); // Invalid
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}