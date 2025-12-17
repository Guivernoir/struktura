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
        .description("Calculate OEE based on availability, performance, and quality metrics per lean manufacturing principles")
        .design_code("Lean Manufacturing")
        .parameter(ParameterMetadata {
            name: "Availability".to_string(),
            path: "additional.availability".to_string(),
            data_type: ParameterType::Number,
            unit: "%".to_string(),
            description: "Percentage of time equipment is available for production".to_string(),
            required: true,
            default_value: Some(90.0),
            min_value: Some(0.0),
            max_value: Some(100.0),
            typical_range: Some((80.0, 95.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Performance".to_string(),
            path: "additional.performance".to_string(),
            data_type: ParameterType::Number,
            unit: "%".to_string(),
            description: "Percentage of maximum speed achieved".to_string(),
            required: true,
            default_value: Some(95.0),
            min_value: Some(0.0),
            max_value: Some(100.0),
            typical_range: Some((85.0, 98.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Quality".to_string(),
            path: "additional.quality".to_string(),
            data_type: ParameterType::Number,
            unit: "%".to_string(),
            description: "Percentage of good parts produced".to_string(),
            required: true,
            default_value: Some(98.0),
            min_value: Some(0.0),
            max_value: Some(100.0),
            typical_range: Some((95.0, 99.9)),
            validation_rules: None,
        })
        .complexity(ComplexityLevel::Basic)
        .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        self.get_additional_param(params, "availability", Some(0.0), Some(100.0))?;
        self.get_additional_param(params, "performance", Some(0.0), Some(100.0))?;
        self.get_additional_param(params, "quality", Some(0.0), Some(100.0))?;
        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let availability = self.get_additional_param(&params, "availability", None, None)?;
        let performance = self.get_additional_param(&params, "performance", None, None)?;
        let quality = self.get_additional_param(&params, "quality", None, None)?;

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
            EngineeringResultItem::new("Availability", availability, "%"),
            EngineeringResultItem::new("Performance", performance, "%"),
            EngineeringResultItem::new("Quality", quality, "%"),
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
        additional.insert("availability".to_string(), 90.0);
        additional.insert("performance".to_string(), 95.0);
        additional.insert("quality".to_string(), 98.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() == 4);
    }

    #[test]
    fn test_invalid_percentage() {
        let calc = OEECalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("availability".to_string(), 110.0); // Invalid
        additional.insert("performance".to_string(), 95.0);
        additional.insert("quality".to_string(), 98.0);
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}