use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::process_capability_indices::*;
use super::helpers::*;

pub struct ProcessCapabilityCalculator;

impl ParameterValidator for ProcessCapabilityCalculator {
    fn calculator_id(&self) -> &str {
        "process_capability"
    }
}

#[async_trait]
impl EngineerCalculator for ProcessCapabilityCalculator {
    fn id(&self) -> &str {
        "process_capability"
    }

    fn name(&self) -> &str {
        "Process Capability Analysis (Cp, Cpk)"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Production
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder(
            "process_capability",
            "Process Capability Analysis (Cp, Cpk)"
        )
        .category("production")
        .description("Calculate Cp and Cpk indices for process capability assessment")
        .design_code("Six Sigma")
        .design_code("ISO 22514")
        .parameter(ParameterMetadata {
            name: "Process Mean".to_string(),
            path: "additional.mean".to_string(),
            data_type: ParameterType::Number,
            unit: "".to_string(),
            description: "Average process value".to_string(),
            required: true,
            default_value: Some(10.0),
            min_value: None,
            max_value: None,
            typical_range: None,
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Standard Deviation".to_string(),
            path: "additional.std_dev".to_string(),
            data_type: ParameterType::Number,
            unit: "".to_string(),
            description: "Process standard deviation".to_string(),
            required: true,
            default_value: Some(1.0),
            min_value: Some(0.001),
            max_value: None,
            typical_range: Some((0.1, 5.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Lower Specification Limit".to_string(),
            path: "additional.lower_spec".to_string(),
            data_type: ParameterType::Number,
            unit: "".to_string(),
            description: "Lower specification limit (LSL)".to_string(),
            required: true,
            default_value: Some(5.0),
            min_value: None,
            max_value: None,
            typical_range: None,
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Upper Specification Limit".to_string(),
            path: "additional.upper_spec".to_string(),
            data_type: ParameterType::Number,
            unit: "".to_string(),
            description: "Upper specification limit (USL)".to_string(),
            required: true,
            default_value: Some(15.0),
            min_value: None,
            max_value: None,
            typical_range: None,
            validation_rules: None,
        })
        .complexity(ComplexityLevel::Basic)
        .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let mean = self.get_additional_param(params, "mean", None, None)?;
        let std_dev = self.get_additional_param(params, "std_dev", Some(0.001), None)?;
        let lower_spec = self.get_additional_param(params, "lower_spec", None, None)?;
        let upper_spec = self.get_additional_param(params, "upper_spec", None, None)?;

        if lower_spec >= upper_spec {
            return Err(EngineeringError::InvalidParameter {
                parameter: "specification_limits".to_string(),
                value: format!("LSL: {}, USL: {}", lower_spec, upper_spec),
                reason: "Lower spec must be < upper spec".to_string(),
            });
        }

        if mean < lower_spec || mean > upper_spec {
            return Err(EngineeringError::DomainError {
                field: "mean".to_string(),
                message: "Process mean outside specification limits".to_string(),
            });
        }

        if std_dev == 0.0 {
            return Err(EngineeringError::InvalidParameter {
                parameter: "std_dev".to_string(),
                value: "0.0".to_string(),
                reason: "Cannot be zero".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let mean = self.get_additional_param(&params, "mean", None, None)?;
        let std_dev = self.get_additional_param(&params, "std_dev", None, None)?;
        let lower_spec = self.get_additional_param(&params, "lower_spec", None, None)?;
        let upper_spec = self.get_additional_param(&params, "upper_spec", None, None)?;

        let cpk_value = cpk(mean, std_dev, lower_spec, upper_spec);
        let cp_value = (upper_spec - lower_spec) / (6.0 * std_dev);
        let ppm = if cpk_value >= 1.33 { SIX_SIGMA_PPM } else if cpk_value >= 1.0 { FIVE_SIGMA_PPM } else { THREE_SIGMA_PPM };

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if cpk_value < CPK_MINIMUM {
            warnings.push(format!("Cpk ({:.2}) below minimum ({:.2}). Process not capable.", cpk_value, CPK_MINIMUM));
            recommendations.push("Reduce process variation or adjust specifications".to_string());
        } else if cpk_value < CPK_ADEQUATE {
            warnings.push(format!("Cpk ({:.2}) below adequate level ({:.2}). Improvement recommended.", cpk_value, CPK_ADEQUATE));
            recommendations.push("Implement process controls to center mean and reduce std dev".to_string());
        }

        compliance_notes.push("Process capability per Six Sigma methodology".to_string());
        compliance_notes.push("Based on normal distribution assumption".to_string());

        let results = vec![
            EngineeringResultItem::new("Cpk", cpk_value, "dimensionless")
                .critical()
                .with_format(format!("{:.2}", cpk_value)),
            EngineeringResultItem::new("Cp", cp_value, "dimensionless")
                .with_format(format!("{:.2}", cp_value)),
            EngineeringResultItem::new("Defects per Million (PPM)", ppm, "ppm")
                .with_format(format!("{:.1} ppm", ppm)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "process_capability".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "Six Sigma".to_string(),
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
    async fn test_process_capability() {
        let calc = ProcessCapabilityCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("mean".to_string(), 10.0);
        additional.insert("std_dev".to_string(), 1.0);
        additional.insert("lower_spec".to_string(), 5.0);
        additional.insert("upper_spec".to_string(), 15.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() == 3);
    }

    #[test]
    fn test_invalid_specs() {
        let calc = ProcessCapabilityCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("mean".to_string(), 10.0);
        additional.insert("std_dev".to_string(), 1.0);
        additional.insert("lower_spec".to_string(), 15.0); // Invalid
        additional.insert("upper_spec".to_string(), 5.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}