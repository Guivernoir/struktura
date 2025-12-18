use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::lean_manufacturing::*;
use super::helpers::*;

pub struct OEECalculator;

impl ParameterValidator for OEECalculator {
    fn calculator_id(&self) -> &str {
        "oee_calculation"
    }
}

#[async_trait]
impl EngineerCalculator for OEECalculator {
    fn id(&self) -> &str {
        "oee_calculation"
    }

    fn name(&self) -> &str {
        "Overall Equipment Effectiveness (OEE) Calculation"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Production
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder(
            "oee_calculation",
            "Overall Equipment Effectiveness (OEE) Calculation"
        )
        .category("production")
        .description("Calculate OEE from raw field data based on lean manufacturing principles")
        .design_code("Lean Manufacturing")
        .parameter(ParameterMetadata {
            name: "Total Shift Time".to_string(),
            path: "additional.total_shift_time".to_string(),
            data_type: ParameterType::Number,
            unit: "min".to_string(),
            description: "Total scheduled shift time in minutes".to_string(),
            required: true,
            default_value: Some(480.0),
            min_value: Some(0.1),
            max_value: None,
            typical_range: Some((240.0, 1440.0)),
            validation_rules: Some(vec!["Must be positive".to_string()]),
        })
        .parameter(ParameterMetadata {
            name: "Planned Downtime".to_string(),
            path: "additional.planned_downtime".to_string(),
            data_type: ParameterType::Number,
            unit: "min".to_string(),
            description: "Planned downtime (e.g., breaks, scheduled maintenance) in minutes".to_string(),
            required: false,
            default_value: Some(0.0),
            min_value: Some(0.0),
            max_value: None,
            typical_range: Some((0.0, 120.0)),
            validation_rules: Some(vec!["Must be non-negative and less than total shift time".to_string()]),
        })
        .parameter(ParameterMetadata {
            name: "Unplanned Downtime".to_string(),
            path: "additional.unplanned_downtime".to_string(),
            data_type: ParameterType::Number,
            unit: "min".to_string(),
            description: "Unplanned downtime (e.g., breakdowns) in minutes".to_string(),
            required: true,
            default_value: Some(0.0),
            min_value: Some(0.0),
            max_value: None,
            typical_range: Some((0.0, 240.0)),
            validation_rules: Some(vec!["Must be non-negative and allow positive operating time".to_string()]),
        })
        .parameter(ParameterMetadata {
            name: "Ideal Cycle Time".to_string(),
            path: "additional.ideal_cycle_time".to_string(),
            data_type: ParameterType::Number,
            unit: "sec/piece".to_string(),
            description: "Ideal time to produce one piece in seconds".to_string(),
            required: true,
            default_value: Some(60.0),
            min_value: Some(0.001),
            max_value: None,
            typical_range: Some((1.0, 300.0)),
            validation_rules: Some(vec!["Must be positive".to_string()]),
        })
        .parameter(ParameterMetadata {
            name: "Total Pieces".to_string(),
            path: "additional.total_pieces".to_string(),
            data_type: ParameterType::Number,
            unit: "pieces".to_string(),
            description: "Total pieces produced (good + defective)".to_string(),
            required: true,
            default_value: Some(0.0),
            min_value: Some(0.0),
            max_value: None,
            typical_range: Some((0.0, 10000.0)),
            validation_rules: Some(vec!["Must be non-negative integer (use float for fractional if needed)".to_string()]),
        })
        .parameter(ParameterMetadata {
            name: "Good Pieces".to_string(),
            path: "additional.good_pieces".to_string(),
            data_type: ParameterType::Number,
            unit: "pieces".to_string(),
            description: "Good quality pieces produced".to_string(),
            required: true,
            default_value: Some(0.0),
            min_value: Some(0.0),
            max_value: None,
            typical_range: Some((0.0, 10000.0)),
            validation_rules: Some(vec!["Must be non-negative and <= total pieces".to_string()]),
        })
        .complexity(ComplexityLevel::Basic)
        .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let total_shift_time = self.get_additional_param(params, "total_shift_time", Some(0.1), None)?;
        let planned_downtime = self.get_additional_param(params, "planned_downtime", Some(0.0), None).unwrap_or(0.0);
        let unplanned_downtime = self.get_additional_param(params, "unplanned_downtime", Some(0.0), None)?;
        let ideal_cycle_time = self.get_additional_param(params, "ideal_cycle_time", Some(0.001), None)?;
        let total_pieces = self.get_additional_param(params, "total_pieces", Some(0.0), None)?;
        let good_pieces = self.get_additional_param(params, "good_pieces", Some(0.0), None)?;

        if planned_downtime > total_shift_time {
            return Err(EngineeringError::InvalidParameter {
                parameter: "planned_downtime".to_string(),
                value: planned_downtime.to_string(),
                reason: "Cannot exceed total shift time".to_string(),
            });
        }

        let planned_production_time = total_shift_time - planned_downtime;
        if unplanned_downtime > planned_production_time {
            return Err(EngineeringError::InvalidParameter {
                parameter: "unplanned_downtime".to_string(),
                value: unplanned_downtime.to_string(),
                reason: "Cannot exceed planned production time (total shift - planned downtime)".to_string(),
            });
        }

        let operating_time = planned_production_time - unplanned_downtime;
        if operating_time <= 0.0 {
            return Err(EngineeringError::DomainError {
                field: "operating_time".to_string(),
                message: "Must be positive (adjust downtimes)".to_string(),
            });
        }

        if good_pieces > total_pieces {
            return Err(EngineeringError::InvalidParameter {
                parameter: "good_pieces".to_string(),
                value: good_pieces.to_string(),
                reason: "Cannot exceed total pieces".to_string(),
            });
        }

        if ideal_cycle_time <= 0.0 {
            return Err(EngineeringError::InvalidParameter {
                parameter: "ideal_cycle_time".to_string(),
                value: ideal_cycle_time.to_string(),
                reason: "Must be positive".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let total_shift_time = self.get_additional_param(&params, "total_shift_time", None, None)?;
        let planned_downtime = self.get_additional_param(&params, "planned_downtime", None, None).unwrap_or(0.0);
        let unplanned_downtime = self.get_additional_param(&params, "unplanned_downtime", None, None)?;
        let ideal_cycle_time = self.get_additional_param(&params, "ideal_cycle_time", None, None)?;
        let total_pieces = self.get_additional_param(&params, "total_pieces", None, None)?;
        let good_pieces = self.get_additional_param(&params, "good_pieces", None, None)?;

        let planned_production_time = total_shift_time - planned_downtime;
        let operating_time = planned_production_time - unplanned_downtime;

        let availability = (operating_time / planned_production_time) * 100.0;

        let operating_time_sec = operating_time * 60.0;
        let ideal_pieces = if ideal_cycle_time > 0.0 { operating_time_sec / ideal_cycle_time } else { 0.0 };
        let performance = if ideal_pieces > 0.0 { (total_pieces / ideal_pieces) * 100.0 } else { 0.0 };

        let quality = if total_pieces > 0.0 { (good_pieces / total_pieces) * 100.0 } else { 0.0 };

        let oee_value = oee(availability, performance, quality);

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if oee_value < OEE_ACCEPTABLE {
            warnings.push(format!("OEE ({:.1}%) below acceptable level ({:.1}%). Significant improvement needed.", oee_value, OEE_ACCEPTABLE));
            recommendations.push("Implement TPM (Total Productive Maintenance) program".to_string());
        } else if oee_value < OEE_GOOD {
            warnings.push(format!("OEE ({:.1}%) below good benchmark ({:.1}%). Room for improvement.", oee_value, OEE_GOOD));
            recommendations.push("Analyze downtime logs and implement quick changeover techniques".to_string());
        } else if oee_value < OEE_WORLD_CLASS {
            recommendations.push(format!("OEE ({:.1}%) approaching world-class ({:.1}%). Focus on minor losses.", oee_value, OEE_WORLD_CLASS));
        }

        compliance_notes.push("OEE calculation per lean manufacturing standards".to_string());
        compliance_notes.push("Regular monitoring and kaizen events recommended".to_string());

        let results = vec![
            EngineeringResultItem::new("OEE", oee_value, "%")
                .critical()
                .with_format(format!("{:.1}%", oee_value)),
            EngineeringResultItem::new("Availability", availability, "%")
                .with_format(format!("{:.1}%", availability)),
            EngineeringResultItem::new("Performance", performance, "%")
                .with_format(format!("{:.1}%", performance)),
            EngineeringResultItem::new("Quality", quality, "%")
                .with_format(format!("{:.1}%", quality)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "oee_calculation".to_string(),
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
    async fn test_oee_calculation() {
        let calc = OEECalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("total_shift_time".to_string(), 480.0); // 8 hours
        additional.insert("planned_downtime".to_string(), 30.0);
        additional.insert("unplanned_downtime".to_string(), 45.0);
        additional.insert("ideal_cycle_time".to_string(), 60.0); // 1 min per piece
        additional.insert("total_pieces".to_string(), 400.0);
        additional.insert("good_pieces".to_string(), 380.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.results.len(), 4);
    }

    #[test]
    fn test_invalid_downtime() {
        let calc = OEECalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("total_shift_time".to_string(), 480.0);
        additional.insert("planned_downtime".to_string(), 30.0);
        additional.insert("unplanned_downtime".to_string(), 500.0); // Invalid: exceeds planned production time
        additional.insert("ideal_cycle_time".to_string(), 60.0);
        additional.insert("total_pieces".to_string(), 400.0);
        additional.insert("good_pieces".to_string(), 380.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_pieces() {
        let calc = OEECalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("total_shift_time".to_string(), 480.0);
        additional.insert("planned_downtime".to_string(), 30.0);
        additional.insert("unplanned_downtime".to_string(), 45.0);
        additional.insert("ideal_cycle_time".to_string(), 60.0);
        additional.insert("total_pieces".to_string(), 400.0);
        additional.insert("good_pieces".to_string(), 450.0); // Invalid: exceeds total pieces
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}