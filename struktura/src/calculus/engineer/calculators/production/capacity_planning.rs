use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::lean_manufacturing::*;
use super::helpers::*;

pub struct CapacityPlanningCalculator;

impl ParameterValidator for CapacityPlanningCalculator {
    fn calculator_id(&self) -> &str {
        "capacity_planning"
    }
}

#[async_trait]
impl EngineerCalculator for CapacityPlanningCalculator {
    fn id(&self) -> &str {
        "capacity_planning"
    }

    fn name(&self) -> &str {
        "Capacity Planning Analysis"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Production
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder(
            "capacity_planning",
            "Capacity Planning Analysis"
        )
        .category("production")
        .description("Determine required production capacity, number of machines, and utilization rates")
        .design_code("Lean Manufacturing")
        .parameter(ParameterMetadata {
            name: "Demand Units per Period".to_string(),
            path: "additional.demand_per_period".to_string(),
            data_type: ParameterType::Number,
            unit: "units/period".to_string(),
            description: "Forecasted demand per planning period".to_string(),
            required: true,
            default_value: Some(1000.0),
            min_value: Some(1.0),
            max_value: None,
            typical_range: Some((100.0, 5000.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Cycle Time".to_string(),
            path: "additional.cycle_time".to_string(),
            data_type: ParameterType::Number,
            unit: "minutes/unit".to_string(),
            description: "Average cycle time per unit".to_string(),
            required: true,
            default_value: Some(5.0),
            min_value: Some(0.1),
            max_value: Some(60.0),
            typical_range: Some((1.0, 20.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Available Time per Period".to_string(),
            path: "additional.available_time_per_period".to_string(),
            data_type: ParameterType::Number,
            unit: "minutes/period".to_string(),
            description: "Total available production time per period".to_string(),
            required: true,
            default_value: Some(24000.0), // e.g., 50 weeks * 5 days * 8 hours * 60 min
            min_value: Some(1000.0),
            max_value: None,
            typical_range: Some((10000.0, 50000.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Target Utilization".to_string(),
            path: "additional.target_utilization".to_string(),
            data_type: ParameterType::Number,
            unit: "%".to_string(),
            description: "Desired equipment utilization rate".to_string(),
            required: false,
            default_value: Some(85.0),
            min_value: Some(50.0),
            max_value: Some(95.0),
            typical_range: Some((75.0, 90.0)),
            validation_rules: None,
        })
        .complexity(ComplexityLevel::Intermediate)
        .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        self.get_additional_param(params, "demand_per_period", Some(1.0), None)?;
        self.get_additional_param(params, "cycle_time", Some(0.1), Some(60.0))?;
        self.get_additional_param(params, "available_time_per_period", Some(1000.0), None)?;
        if let Some(additional) = &params.additional {
            if let Some(util) = additional.get("target_utilization") {
                if *util < 50.0 || *util > 95.0 {
                    return Err(EngineeringError::InvalidParameter {
                        parameter: "target_utilization".to_string(),
                        value: util.to_string(),
                        reason: "Must be between 50% and 95%".to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let demand = self.get_additional_param(&params, "demand_per_period", None, None)?;
        let cycle_time = self.get_additional_param(&params, "cycle_time", None, None)?;
        let available_time = self.get_additional_param(&params, "available_time_per_period", None, None)?;
        let target_utilization = params.additional.as_ref().and_then(|a| a.get("target_utilization").copied()).unwrap_or(TARGET_LINE_EFFICIENCY);

        let required_time = demand * cycle_time;
        let required_capacity = required_time / (available_time * (target_utilization / 100.0));
        let num_machines = required_capacity.ceil();
        let actual_utilization = (required_time / (num_machines * available_time)) * 100.0;

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if actual_utilization > 90.0 {
            warnings.push(format!("High utilization ({:.1}%). Risk of overload and downtime.", actual_utilization));
            recommendations.push("Consider adding buffer capacity or overtime planning".to_string());
        } else if actual_utilization < 70.0 {
            recommendations.push(format!("Low utilization ({:.1}%). Optimize scheduling or reduce assets.", actual_utilization));
        }

        compliance_notes.push("Capacity planning per lean manufacturing to avoid over/under capacity".to_string());
        compliance_notes.push("Incorporate demand forecasting accuracy in planning".to_string());

        let results = vec![
            EngineeringResultItem::new("Required Machines", num_machines, "units")
                .critical()
                .with_format(format!("{:.0} machines", num_machines)),
            EngineeringResultItem::new("Required Capacity", required_capacity, "machine-periods")
                .with_format(format!("{:.2}", required_capacity)),
            EngineeringResultItem::new("Actual Utilization", actual_utilization, "%")
                .with_format(format!("{:.1}%", actual_utilization)),
            EngineeringResultItem::new("Required Time", required_time, "minutes"),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "capacity_planning".to_string(),
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
    async fn test_capacity_planning() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_per_period".to_string(), 1000.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("available_time_per_period".to_string(), 24000.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() == 4);
    }

    #[test]
    fn test_invalid_utilization() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_per_period".to_string(), 1000.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("available_time_per_period".to_string(), 24000.0);
        additional.insert("target_utilization".to_string(), 99.0); // Invalid
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}