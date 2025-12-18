use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::lean_manufacturing::*;
use super::helpers::*;

pub struct ProductionLineBalancingCalculator;

impl ParameterValidator for ProductionLineBalancingCalculator {
    fn calculator_id(&self) -> &str {
        "production_line_balancing"
    }
}

#[async_trait]
impl EngineerCalculator for ProductionLineBalancingCalculator {
    fn id(&self) -> &str {
        "production_line_balancing"
    }

    fn name(&self) -> &str {
        "Production Line Balancing Analysis"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Production
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder(
            "production_line_balancing",
            "Production Line Balancing Analysis"
        )
        .category("production")
        .description("Analyze and optimize assembly line efficiency using takt time, cycle time, and workstation balance metrics per lean manufacturing principles")
        .design_code("Lean Manufacturing")
        .design_code("Industrial Engineering Standards")
        .parameter(ParameterMetadata {
            name: "Number of Workstations".to_string(),
            path: "additional.num_workstations".to_string(),
            data_type: ParameterType::Integer,
            unit: "stations".to_string(),
            description: "Current or proposed number of workstations in the line".to_string(),
            required: false,
            default_value: Some(5.0),
            min_value: Some(2.0),
            max_value: Some(50.0),
            typical_range: Some((3.0, 15.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Desired Daily Output".to_string(),
            path: "additional.desired_output_per_day".to_string(),
            data_type: ParameterType::Number,
            unit: "units/day".to_string(),
            description: "Target production quantity per day".to_string(),
            required: true,
            default_value: Some(480.0),
            min_value: Some(1.0),
            max_value: Some(10000.0),
            typical_range: Some((100.0, 2000.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Available Time Per Day".to_string(),
            path: "additional.available_time_per_day".to_string(),
            data_type: ParameterType::Number,
            unit: "minutes".to_string(),
            description: "Net available production time per day (excludes breaks, maintenance)".to_string(),
            required: true,
            default_value: Some(480.0),
            min_value: Some(60.0),
            max_value: Some(1440.0),
            typical_range: Some((420.0, 540.0)),
            validation_rules: Some(vec!["Typically 7-9 hours (420-540 min) for 8-hour shift".to_string()]),
        })
        .parameter(ParameterMetadata {
            name: "Total Task Time".to_string(),
            path: "additional.total_task_time".to_string(),
            data_type: ParameterType::Number,
            unit: "minutes".to_string(),
            description: "Sum of all task times per unit (cycle time if one station)".to_string(),
            required: true,
            default_value: Some(12.0),
            min_value: Some(0.1),
            max_value: Some(1000.0),
            typical_range: Some((5.0, 60.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Longest Task Time".to_string(),
            path: "additional.longest_task_time".to_string(),
            data_type: ParameterType::Number,
            unit: "minutes".to_string(),
            description: "Duration of bottleneck task (determines minimum cycle time)".to_string(),
            required: false,
            default_value: Some(3.5),
            min_value: Some(0.1),
            max_value: Some(100.0),
            typical_range: Some((2.0, 10.0)),
            validation_rules: Some(vec!["Must be ≤ total_task_time".to_string()]),
        })
        .complexity(ComplexityLevel::Intermediate)
        .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        // Validate required parameters
        let desired_output = self.get_additional_param(
            params, "desired_output_per_day", Some(1.0), Some(10000.0))?;
        
        let available_time = self.get_additional_param(
            params, "available_time_per_day", Some(60.0), Some(1440.0))?;
        
        let total_task_time = self.get_additional_param(
            params, "total_task_time", Some(0.1), Some(1000.0))?;

        // Validate optional parameters
        if let Some(additional) = &params.additional {
            if let Some(longest_task) = additional.get("longest_task_time") {
                if longest_task > &total_task_time {
                    return Err(EngineeringError::InvalidParameter {
                        parameter: "longest_task_time".to_string(),
                        value: longest_task.to_string(),
                        reason: "Cannot exceed total_task_time".to_string(),
                    });
                }
            }

            if let Some(num_stations) = additional.get("num_workstations") {
                if *num_stations < 2.0 || *num_stations > 50.0 {
                    return Err(EngineeringError::InvalidParameter {
                        parameter: "num_workstations".to_string(),
                        value: num_stations.to_string(),
                        reason: "Must be between 2 and 50".to_string(),
                    });
                }
            }
        }

        // Validate takt time is achievable
        let takt_time_calc = available_time / desired_output;
        if takt_time_calc < total_task_time {
            return Err(EngineeringError::DomainError {
                field: "takt_time".to_string(),
                message: format!(
                    "Demand cannot be met: takt time ({:.2} min) < total task time ({:.2} min). \
                     Reduce demand or increase available time.",
                    takt_time_calc, total_task_time
                ),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) 
        -> EngineeringResult<EngineeringCalculationResponse> {
        
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        // Extract parameters
        let num_workstations = params.additional.as_ref()
            .and_then(|a| a.get("num_workstations"))
            .copied()
            .unwrap_or(5.0) as usize;

        let desired_output = params.additional.as_ref()
            .and_then(|a| a.get("desired_output_per_day"))
            .copied()
            .unwrap_or(480.0);

        let available_time_per_day = params.additional.as_ref()
            .and_then(|a| a.get("available_time_per_day"))
            .copied()
            .unwrap_or(480.0);

        let total_task_time = params.additional.as_ref()
            .and_then(|a| a.get("total_task_time"))
            .copied()
            .unwrap_or(12.0);

        let longest_task_time = params.additional.as_ref()
            .and_then(|a| a.get("longest_task_time"))
            .copied()
            .unwrap_or(3.5);

        // Takt time (available time / desired output)
        let takt_time_val = takt_time(available_time_per_day, desired_output);

        // Theoretical minimum number of workstations
        let theoretical_min_stations = min_workstations(total_task_time, takt_time_val);

        // Cycle time (determines production rate)
        let cycle_time_val = takt_time_val.max(longest_task_time);

        // Actual production rate
        let actual_output = available_time_per_day / cycle_time_val;

        // Line efficiency
        let line_efficiency_val = line_efficiency(total_task_time, num_workstations, cycle_time_val);

        // Balance delay (idle time percentage)
        let balance_delay_val = balance_delay(line_efficiency_val);

        // Smoothness index (estimated - would need station times for exact calculation)
        let avg_station_time = total_task_time / num_workstations as f64;
        let smoothness_index_val = ((cycle_time_val - avg_station_time).powi(2)).sqrt() * 100.0 / cycle_time_val;

        // Idle time per cycle
        let total_idle_time = (num_workstations as f64 * cycle_time_val) - total_task_time;

        // Daily idle time cost (assume $30/hour labor rate)
        let labor_rate_per_minute = 30.0 / 60.0;
        let daily_idle_cost = total_idle_time * (available_time_per_day / cycle_time_val) * labor_rate_per_minute;

        // Warnings and recommendations
        if line_efficiency_val < MINIMUM_ACCEPTABLE_EFFICIENCY {
            warnings.push(format!(
                "CRITICAL: Low line efficiency ({:.1}%). Significant idle time detected. \
                 Line is operating below acceptable threshold.",
                line_efficiency_val
            ));
            recommendations.push("Rebalance workstations or combine tasks to improve efficiency".to_string());
            recommendations.push("Consider time-motion study to identify improvement opportunities".to_string());
        } else if line_efficiency_val < TARGET_LINE_EFFICIENCY {
            warnings.push(format!(
                "Line efficiency ({:.1}%) below target ({:.1}%). Improvement possible.",
                line_efficiency_val, TARGET_LINE_EFFICIENCY
            ));
        }

        if num_workstations > theoretical_min_stations * 2 {
            warnings.push(format!(
                "Excessive workstations: {} actual vs {} theoretical minimum. \
                 Consider consolidating operations.",
                num_workstations, theoretical_min_stations
            ));
        }

        if cycle_time_val > takt_time_val * 1.1 {
            warnings.push(format!(
                "DEMAND NOT MET: Cycle time ({:.2} min) exceeds takt time ({:.2} min) by >10%. \
                 Cannot maintain required production rate.",
                cycle_time_val, takt_time_val
            ));
            recommendations.push("Increase workstations or reduce task times to meet demand".to_string());
            recommendations.push("Consider parallel workstations for bottleneck tasks".to_string());
        }

        if smoothness_index_val > 20.0 {
            warnings.push(format!(
                "Poor line balance (smoothness index: {:.1}). Tasks are unevenly distributed.",
                smoothness_index_val
            ));
            recommendations.push("Redistribute tasks among workstations for better balance".to_string());
        }

        if balance_delay_val > 30.0 {
            recommendations.push(format!(
                "High balance delay ({:.1}%). Consider implementing parallel workstations \
                 for bottleneck tasks or cross-training operators.",
                balance_delay_val
            ));
        }

        if daily_idle_cost > 200.0 {
            recommendations.push(format!(
                "Significant idle time cost (${:.2}/day). Line optimization could save \
                 approximately ${:.2}/year.",
                daily_idle_cost, daily_idle_cost * 250.0 // 250 working days
            ));
        }

        compliance_notes.push("Follow lean manufacturing principles for continuous improvement".to_string());
        compliance_notes.push("Implement visual management (andon boards) for cycle time tracking".to_string());
        compliance_notes.push("Consider operator training to reduce task times and variability".to_string());
        compliance_notes.push("Regular time studies recommended to validate and update task times".to_string());
        compliance_notes.push("Implement standardized work procedures for consistency".to_string());

        let results = vec![
            EngineeringResultItem::new("Takt Time", takt_time_val, "min/unit")
                .critical()
                .with_format(format!("{:.2} min/unit", takt_time_val)),
            EngineeringResultItem::new("Actual Cycle Time", cycle_time_val, "min/unit")
                .critical()
                .with_format(format!("{:.2} min/unit", cycle_time_val)),
            EngineeringResultItem::new("Theoretical Min Workstations", theoretical_min_stations as f64, "stations"),
            EngineeringResultItem::new("Actual Workstations", num_workstations as f64, "stations"),
            EngineeringResultItem::new("Actual Daily Output", actual_output, "units/day")
                .with_tolerance(0.05)
                .critical(),
            EngineeringResultItem::new("Line Efficiency", line_efficiency_val, "%")
                .critical()
                .with_format(format!("{:.1}%", line_efficiency_val)),
            EngineeringResultItem::new("Balance Delay", balance_delay_val, "%")
                .critical()
                .with_format(format!("{:.1}%", balance_delay_val)),
            EngineeringResultItem::new("Smoothness Index", smoothness_index_val, "dimensionless")
                .with_format(format!("{:.1}", smoothness_index_val)),
            EngineeringResultItem::new("Total Idle Time per Cycle", total_idle_time, "min"),
            EngineeringResultItem::new("Daily Idle Time Cost", daily_idle_cost, "USD")
                .with_tolerance(0.10)
                .with_format(format!("${:.2}", daily_idle_cost)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "production_line_balancing".to_string(),
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
