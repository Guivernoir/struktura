use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

pub struct WorkSamplingCalculator;

impl ParameterValidator for WorkSamplingCalculator {
    fn calculator_id(&self) -> &str {
        "work_sampling"
    }
}

#[async_trait]
impl EngineerCalculator for WorkSamplingCalculator {
    fn id(&self) -> &str {
        "work_sampling"
    }

    fn name(&self) -> &str {
        "Work Sampling Analysis"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Production
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder(
            "work_sampling",
            "Work Sampling Analysis"
        )
        .category("production")
        .description("Estimate time allocation to activities using work sampling technique")
        .design_code("Industrial Engineering Standards")
        .parameter(ParameterMetadata {
            name: "Total Observations".to_string(),
            path: "additional.total_observations".to_string(),
            data_type: ParameterType::Integer,
            unit: "counts".to_string(),
            description: "Total number of random observations".to_string(),
            required: true,
            default_value: Some(500.0),
            min_value: Some(100.0),
            max_value: Some(10000.0),
            typical_range: Some((200.0, 1000.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Productive Observations".to_string(),
            path: "additional.productive_observations".to_string(),
            data_type: ParameterType::Integer,
            unit: "counts".to_string(),
            description: "Number of observations where productive work was occurring".to_string(),
            required: true,
            default_value: Some(400.0),
            min_value: Some(0.0),
            max_value: None,
            typical_range: Some((100.0, 900.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Confidence Level".to_string(),
            path: "additional.confidence_level".to_string(),
            data_type: ParameterType::Number,
            unit: "%".to_string(),
            description: "Desired confidence level for estimates".to_string(),
            required: false,
            default_value: Some(95.0),
            min_value: Some(90.0),
            max_value: Some(99.0),
            typical_range: Some((95.0, 99.0)),
            validation_rules: None,
        })
        .complexity(ComplexityLevel::Intermediate)
        .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let total_obs = self.get_additional_param(params, "total_observations", Some(100.0), Some(10000.0))?;
        let prod_obs = self.get_additional_param(params, "productive_observations", Some(0.0), Some(total_obs))?;

        if prod_obs > total_obs {
            return Err(EngineeringError::InvalidParameter {
                parameter: "productive_observations".to_string(),
                value: prod_obs.to_string(),
                reason: "Cannot exceed total observations".to_string(),
            });
        }

        if let Some(additional) = &params.additional {
            if let Some(conf) = additional.get("confidence_level") {
                if *conf < 90.0 || *conf > 99.0 {
                    return Err(EngineeringError::InvalidParameter {
                        parameter: "confidence_level".to_string(),
                        value: conf.to_string(),
                        reason: "Must be between 90% and 99%".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let total_obs = self.get_additional_param(&params, "total_observations", None, None)?;
        let prod_obs = self.get_additional_param(&params, "productive_observations", None, None)?;
        let confidence_level = params.additional.as_ref().and_then(|a| a.get("confidence_level").copied()).unwrap_or(95.0);

        let productive_percentage = (prod_obs / total_obs) * 100.0;
        let non_productive_percentage = 100.0 - productive_percentage;

        // Approximate confidence interval (using normal approximation)
        let z_score = if confidence_level == 95.0 { 1.96 } else if confidence_level == 99.0 { 2.58 } else { 1.645 }; // 90%
        let std_error = ((productive_percentage / 100.0 * (1.0 - productive_percentage / 100.0)) / total_obs).sqrt();
        let margin_error = z_score * std_error * 100.0;

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if total_obs < 200.0 {
            warnings.push(format!("Low observations ({:.0}). Confidence interval may be wide.", total_obs));
            recommendations.push("Increase number of observations for better accuracy".to_string());
        }

        if productive_percentage < 70.0 {
            warnings.push(format!("Low productivity ({:.1}%). Investigate non-value activities.", productive_percentage));
            recommendations.push("Conduct time-motion study on non-productive activities".to_string());
        }

        compliance_notes.push("Work sampling per industrial engineering practices".to_string());
        compliance_notes.push("Assumes random sampling and stable process".to_string());

        let results = vec![
            EngineeringResultItem::new("Productive Time", productive_percentage, "%")
                .critical()
                .with_format(format!("{:.1}%", productive_percentage)),
            EngineeringResultItem::new("Non-Productive Time", non_productive_percentage, "%")
                .with_format(format!("{:.1}%", non_productive_percentage)),
            EngineeringResultItem::new("Confidence Interval Margin", margin_error, "%")
                .with_format(format!("±{:.1}%", margin_error)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "work_sampling".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "Industrial Engineering Standards".to_string(),
                requires_pe_review: false,
            }),
        })
    }
}
