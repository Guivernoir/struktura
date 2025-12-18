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
        .description("Determine required production capacity, number of machines, and utilization rates from raw field data, including quality considerations")
        .design_code("Lean Manufacturing")
        .parameter(ParameterMetadata {
            name: "Forecasted Demand Units".to_string(),
            path: "additional.demand_units".to_string(),
            data_type: ParameterType::Number,
            unit: "units".to_string(),
            description: "Forecasted demand units for the planning period".to_string(),
            required: true,
            default_value: Some(1000.0),
            min_value: Some(1.0),
            max_value: None,
            typical_range: Some((100.0, 5000.0)),
            validation_rules: Some(vec!["Must be positive".to_string()]),
        })
        .parameter(ParameterMetadata {
            name: "Planning Period Days".to_string(),
            path: "additional.period_days".to_string(),
            data_type: ParameterType::Number,
            unit: "days".to_string(),
            description: "Number of working days in the planning period".to_string(),
            required: true,
            default_value: Some(20.0),
            min_value: Some(1.0),
            max_value: None,
            typical_range: Some((5.0, 30.0)),
            validation_rules: Some(vec!["Must be positive integer (use float if needed)".to_string()]),
        })
        .parameter(ParameterMetadata {
            name: "Shifts Per Day".to_string(),
            path: "additional.shifts_per_day".to_string(),
            data_type: ParameterType::Number,
            unit: "shifts/day".to_string(),
            description: "Number of shifts per working day".to_string(),
            required: true,
            default_value: Some(1.0),
            min_value: Some(1.0),
            max_value: Some(3.0),
            typical_range: Some((1.0, 3.0)),
            validation_rules: Some(vec!["Typically 1-3".to_string()]),
        })
        .parameter(ParameterMetadata {
            name: "Hours Per Shift".to_string(),
            path: "additional.hours_per_shift".to_string(),
            data_type: ParameterType::Number,
            unit: "hours/shift".to_string(),
            description: "Duration of each shift in hours".to_string(),
            required: true,
            default_value: Some(8.0),
            min_value: Some(1.0),
            max_value: Some(12.0),
            typical_range: Some((6.0, 12.0)),
            validation_rules: Some(vec!["Must be positive".to_string()]),
        })
        .parameter(ParameterMetadata {
            name: "Cycle Time".to_string(),
            path: "additional.cycle_time".to_string(),
            data_type: ParameterType::Number,
            unit: "minutes/cycle".to_string(),
            description: "Time to complete one production cycle".to_string(),
            required: true,
            default_value: Some(5.0),
            min_value: Some(0.1),
            max_value: Some(60.0),
            typical_range: Some((1.0, 20.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Output Per Cycle".to_string(),
            path: "additional.output_per_cycle".to_string(),
            data_type: ParameterType::Number,
            unit: "units/cycle".to_string(),
            description: "Amount of pieces produced per cycle".to_string(),
            required: true,
            default_value: Some(1.0),
            min_value: Some(0.1),
            max_value: None,
            typical_range: Some((1.0, 10.0)),
            validation_rules: Some(vec!["Must be positive".to_string()]),
        })
        .parameter(ParameterMetadata {
            name: "Quality Yield".to_string(),
            path: "additional.quality_yield".to_string(),
            data_type: ParameterType::Number,
            unit: "%".to_string(),
            description: "Expected quality yield rate (percentage of good units)".to_string(),
            required: false,
            default_value: Some(100.0),
            min_value: Some(1.0),
            max_value: Some(100.0),
            typical_range: Some((90.0, 99.0)),
            validation_rules: Some(vec!["Between 1% and 100%".to_string()]),
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
        let demand = self.get_additional_param(params, "demand_units", Some(1.0), None)?;
        let period_days = self.get_additional_param(params, "period_days", Some(1.0), None)?;
        let shifts_per_day = self.get_additional_param(params, "shifts_per_day", Some(1.0), Some(3.0))?;
        let hours_per_shift = self.get_additional_param(params, "hours_per_shift", Some(1.0), Some(12.0))?;
        let cycle_time = self.get_additional_param(params, "cycle_time", Some(0.1), Some(60.0))?;
        let output_per_cycle = self.get_additional_param(params, "output_per_cycle", Some(0.1), None)?;
        let quality_yield = params.additional.as_ref().and_then(|a| a.get("quality_yield").copied()).unwrap_or(100.0);
        let target_utilization = params.additional.as_ref().and_then(|a| a.get("target_utilization").copied()).unwrap_or(TARGET_LINE_EFFICIENCY);

        if quality_yield < 1.0 || quality_yield > 100.0 {
            return Err(EngineeringError::InvalidParameter {
                parameter: "quality_yield".to_string(),
                value: quality_yield.to_string(),
                reason: "Must be between 1% and 100%".to_string(),
            });
        }

        if target_utilization < 50.0 || target_utilization > 95.0 {
            return Err(EngineeringError::InvalidParameter {
                parameter: "target_utilization".to_string(),
                value: target_utilization.to_string(),
                reason: "Must be between 50% and 95%".to_string(),
            });
        }

        let available_time_per_machine = period_days * shifts_per_day * hours_per_shift * 60.0;
        if available_time_per_machine <= 0.0 {
            return Err(EngineeringError::DomainError {
                field: "available_time_per_machine".to_string(),
                message: "Must be positive (check period days, shifts, and hours)".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let demand = self.get_additional_param(&params, "demand_units", None, None)?;
        let period_days = self.get_additional_param(&params, "period_days", None, None)?;
        let shifts_per_day = self.get_additional_param(&params, "shifts_per_day", None, None)?;
        let hours_per_shift = self.get_additional_param(&params, "hours_per_shift", None, None)?;
        let cycle_time = self.get_additional_param(&params, "cycle_time", None, None)?;
        let output_per_cycle = self.get_additional_param(&params, "output_per_cycle", None, None)?;
        let quality_yield = params.additional.as_ref().and_then(|a| a.get("quality_yield").copied()).unwrap_or(100.0);
        let target_utilization = params.additional.as_ref().and_then(|a| a.get("target_utilization").copied()).unwrap_or(TARGET_LINE_EFFICIENCY);

        let available_time_per_machine = period_days * shifts_per_day * hours_per_shift * 60.0;

        let effective_cycle_time = cycle_time / output_per_cycle;
        let required_production = demand / (quality_yield / 100.0);
        let required_time = required_production * effective_cycle_time;
        let required_capacity = required_time / (available_time_per_machine * (target_utilization / 100.0));
        let num_machines = required_capacity.ceil();
        let actual_utilization = (required_time / (num_machines * available_time_per_machine)) * 100.0;

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if actual_utilization > 90.0 {
            warnings.push(format!("High utilization ({:.1}%). Risk of overload and downtime.", actual_utilization));
            recommendations.push("Consider adding buffer capacity or overtime planning".to_string());
        } else if actual_utilization < 70.0 {
            recommendations.push(format!("Low utilization ({:.1}%). Optimize scheduling or reduce assets.", actual_utilization));
        }

        if quality_yield < 95.0 {
            warnings.push(format!("Low quality yield ({:.1}%). Consider process improvements to reduce scrap.", quality_yield));
            recommendations.push("Implement quality control measures or Six Sigma analysis".to_string());
        }

        compliance_notes.push("Capacity planning per lean manufacturing to avoid over/under capacity".to_string());
        compliance_notes.push("Incorporate demand forecasting accuracy and quality metrics in planning".to_string());

        let results = vec![
            EngineeringResultItem::new("Required Machines", num_machines, "units")
                .critical()
                .with_format(format!("{:.0} machines", num_machines)),
            EngineeringResultItem::new("Required Capacity", required_capacity, "machine-periods")
                .with_format(format!("{:.2}", required_capacity)),
            EngineeringResultItem::new("Actual Utilization", actual_utilization, "%")
                .with_format(format!("{:.1}%", actual_utilization)),
            EngineeringResultItem::new("Required Time", required_time, "minutes"),
            EngineeringResultItem::new("Available Time per Machine", available_time_per_machine, "minutes/period"),
            EngineeringResultItem::new("Effective Cycle Time", effective_cycle_time, "minutes/unit")
                .with_format(format!("{:.2} min/unit", effective_cycle_time)),
            EngineeringResultItem::new("Required Production Attempts", required_production, "units")
                .with_format(format!("{:.0} units", required_production)),
            EngineeringResultItem::new("Quality Yield", quality_yield, "%")
                .with_format(format!("{:.1}%", quality_yield)),
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
    async fn test_capacity_planning_calculation() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.results.len(), 8);
    }

    #[test]
    fn test_valid_validation() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_demand() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 0.0); // Invalid
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_period_days() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 0.0); // Invalid
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_shifts_per_day_low() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 0.0); // Invalid
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_shifts_per_day_high() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 4.0); // Invalid
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_hours_per_shift_low() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 0.0); // Invalid
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_hours_per_shift_high() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 13.0); // Invalid
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_cycle_time_low() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 0.0); // Invalid
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_cycle_time_high() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 61.0); // Invalid
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_output_per_cycle() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 0.0); // Invalid
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_quality_yield_low() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 0.0); // Invalid
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_quality_yield_high() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 101.0); // Invalid
        additional.insert("target_utilization".to_string(), 85.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_target_utilization_low() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 40.0); // Invalid
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_target_utilization_high() {
        let calc = CapacityPlanningCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("demand_units".to_string(), 1000.0);
        additional.insert("period_days".to_string(), 20.0);
        additional.insert("shifts_per_day".to_string(), 1.0);
        additional.insert("hours_per_shift".to_string(), 8.0);
        additional.insert("cycle_time".to_string(), 5.0);
        additional.insert("output_per_cycle".to_string(), 1.0);
        additional.insert("quality_yield".to_string(), 95.0);
        additional.insert("target_utilization".to_string(), 96.0); // Invalid
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}