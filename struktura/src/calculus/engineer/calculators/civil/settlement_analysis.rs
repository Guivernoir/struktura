use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::soil_properties::*;

pub struct SettlementAnalysisCalculator;

impl ParameterValidator for SettlementAnalysisCalculator {
    fn calculator_id(&self) -> &str {
        "settlement_analysis"
    }
}

#[async_trait]
impl EngineerCalculator for SettlementAnalysisCalculator {
    fn id(&self) -> &str {
        "settlement_analysis"
    }

    fn name(&self) -> &str {
        "Consolidation Settlement Analysis"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Civil
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("settlement_analysis", "Consolidation Settlement Analysis")
            .category("civil")
            .description("Calculate primary consolidation settlement for clay layers using Terzaghi theory")
            .design_code("USACE EM 1110-1-1904")
            .parameter(ParameterMetadata {
                name: "Applied Stress".to_string(),
                path: "loads.dead_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kPa".to_string(),
                description: "Increase in vertical stress".to_string(),
                required: true,
                default_value: Some(100.0),
                min_value: Some(10.0),
                max_value: Some(1000.0),
                typical_range: Some((50.0, 200.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Layer Thickness".to_string(),
                path: "dimensions.thickness".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Compressible layer thickness".to_string(),
                required: true,
                default_value: Some(5.0),
                min_value: Some(0.5),
                max_value: Some(20.0),
                typical_range: Some((1.0, 10.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Compression Index".to_string(),
                path: "additional.cc".to_string(),
                data_type: ParameterType::Number,
                unit: "dimensionless".to_string(),
                description: "Compression index (Cc)".to_string(),
                required: true,
                default_value: Some(0.3),
                min_value: Some(0.1),
                max_value: Some(1.0),
                typical_range: Some((0.2, 0.5)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Initial Void Ratio".to_string(),
                path: "additional.e0".to_string(),
                data_type: ParameterType::Number,
                unit: "dimensionless".to_string(),
                description: "Initial void ratio".to_string(),
                required: true,
                default_value: Some(0.8),
                min_value: Some(0.5),
                max_value: Some(2.0),
                typical_range: Some((0.6, 1.2)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Preconsolidation Pressure".to_string(),
                path: "additional.pc".to_string(),
                data_type: ParameterType::Number,
                unit: "kPa".to_string(),
                description: "Preconsolidation pressure".to_string(),
                required: false,
                default_value: Some(100.0),
                min_value: Some(50.0),
                max_value: Some(500.0),
                typical_range: Some((80.0, 200.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Advanced)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        if let Some(loads) = &params.loads {
            self.get_additional_param(params, "cc", Some(0.1), Some(1.0))?;
            self.get_additional_param(params, "e0", Some(0.5), Some(2.0))?;
            self.validate_dimension("thickness", params.dimensions.get("thickness").copied(), 0.5, 20.0)?;

            if loads.dead_load < 50.0 {
                return Err(EngineeringError::DomainError {
                    field: "dead_load".to_string(),
                    message: "Low stress increase - minimal settlement".to_string(),
                });
            }
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let delta_sigma = params.loads.as_ref().map(|l| l.dead_load).unwrap_or(100.0);
        let h = params.dimensions.get("thickness").copied().unwrap_or(5.0);
        let cc = self.get_additional_param(&params, "cc", None, None)?;
        let e0 = self.get_additional_param(&params, "e0", None, None)?;
        let pc = params.additional.as_ref().and_then(|a| a.get("pc").copied()).unwrap_or(100.0);
        let sigma0 = 100.0; // Assume average effective stress

        let ocr = pc / sigma0;
        let settlement = if delta_sigma + sigma0 < pc {
            // Normally consolidated
            (cc * h / (1.0 + e0)) * ((sigma0 + delta_sigma) / sigma0).log10() * 1000.0 // mm
        } else {
            // Overconsolidated
            let cr = cc / 5.0; // Assume recompression index
            (cr * h / (1.0 + e0)) * ((sigma0 + delta_sigma) / sigma0).log10() * 1000.0
        };

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if settlement > 50.0 {
            warnings.push(format!("Significant settlement ({:.1} mm). Consider preloading.", settlement));
            recommendations.push("Perform time-rate analysis for consolidation".to_string());
        }

        if ocr > 2.0 {
            recommendations.push("Overconsolidated soil - verify with oedometer test".to_string());
        }

        compliance_notes.push("Primary consolidation per Terzaghi 1D theory".to_string());
        compliance_notes.push("Ignore secondary compression".to_string());

        let results = vec![
            EngineeringResultItem::new("Settlement", settlement, "mm")
                .critical()
                .with_format(format!("{:.1} mm", settlement)),
            EngineeringResultItem::new("OCR", ocr, "dimensionless")
                .with_format(format!("{:.2}", ocr)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "settlement_analysis".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "USACE EM 1110-1-1904".to_string(),
                requires_pe_review: true,
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
    async fn test_settlement() {
        let calc = SettlementAnalysisCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("thickness", 5.0),
        ]);
        params.loads = Some(LoadCase {
            dead_load: 100.0,
            ..Default::default()
        });
        let mut additional = HashMap::new();
        additional.insert("cc".to_string(), 0.3);
        additional.insert("e0".to_string(), 0.8);
        additional.insert("pc".to_string(), 100.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() >= 2);
    }

    #[test]
    fn test_low_stress() {
        let calc = SettlementAnalysisCalculator;
        
        let mut params = minimal_parameters();
        params.loads = Some(LoadCase {
            dead_load: 20.0, // Low
            ..Default::default()
        });

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}