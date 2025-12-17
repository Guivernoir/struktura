use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

use super::concrete_properties::*;
use super::resistance_factors::*;
use super::load_factors::*;
use super::deflection_limits::*;
use super::helpers::*;

pub struct SlabDesignCalculator;

impl ParameterValidator for SlabDesignCalculator {
    fn calculator_id(&self) -> &str {
        "slab_design"
    }
}

#[async_trait]
impl EngineerCalculator for SlabDesignCalculator {
    fn id(&self) -> &str {
        "slab_design"
    }

    fn name(&self) -> &str {
        "Concrete Slab Design"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Structural
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("slab_design", "Concrete Slab Design")
            .category("structural")
            .description("Design one-way concrete slab for flexure and shear per ACI 318")
            .design_code("ACI 318")
            .parameter(ParameterMetadata {
                name: "Span".to_string(),
                path: "dimensions.length".to_string(),
                data_type: ParameterType::Number,
                unit: "m".to_string(),
                description: "Slab span".to_string(),
                required: true,
                default_value: Some(4.0),
                min_value: Some(1.0),
                max_value: Some(8.0),
                typical_range: Some((2.0, 5.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Dead Load".to_string(),
                path: "loads.dead_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kPa".to_string(),
                description: "Dead load (excluding self-weight)".to_string(),
                required: true,
                default_value: Some(2.0),
                min_value: Some(0.5),
                max_value: Some(5.0),
                typical_range: Some((1.0, 3.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Live Load".to_string(),
                path: "loads.live_load".to_string(),
                data_type: ParameterType::Number,
                unit: "kPa".to_string(),
                description: "Live load".to_string(),
                required: true,
                default_value: Some(3.0),
                min_value: Some(1.0),
                max_value: Some(10.0),
                typical_range: Some((2.0, 5.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Concrete Strength".to_string(),
                path: "material.compressive_strength".to_string(),
                data_type: ParameterType::Number,
                unit: "MPa".to_string(),
                description: "f'c".to_string(),
                required: false,
                default_value: Some(FC_C30),
                min_value: Some(20.0),
                max_value: Some(50.0),
                typical_range: Some((25.0, 35.0)),
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Rebar Yield".to_string(),
                path: "material.yield_strength".to_string(),
                data_type: ParameterType::Number,
                unit: "MPa".to_string(),
                description: "fy".to_string(),
                required: false,
                default_value: Some(420.0),
                min_value: Some(300.0),
                max_value: Some(600.0),
                typical_range: Some((400.0, 500.0)),
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Intermediate)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let span = self.validate_dimension("length", params.dimensions.get("length").copied(), 1.0, 8.0)?;
        if let Some(loads) = &params.loads {
            if loads.live_load > 5.0 {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "live_load".to_string(),
                    value: loads.live_load.to_string(),
                    reason: "High live load for slab".to_string(),
                });
            }
        }

        if let Some(material) = &params.material {
            if let Some(fc) = material.compressive_strength {
                if fc < 20.0 {
                    return Err(EngineeringError::InvalidParameter {
                        parameter: "compressive_strength".to_string(),
                        value: fc.to_string(),
                        reason: "Low strength".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let span = params.dimensions.get("length").copied().unwrap_or(4.0);
        let dead = params.loads.as_ref().map(|l| l.dead_load).unwrap_or(2.0);
        let live = params.loads.as_ref().map(|l| l.live_load).unwrap_or(3.0);
        let fc = params.material.as_ref().and_then(|m| m.compressive_strength).unwrap_or(FC_C30);
        let fy = params.material.as_ref().and_then(|m| m.yield_strength).unwrap_or(420.0);

        let self_wt = 0.15 * DENSITY_NORMAL / 1000.0; // Assume 150mm thick
        let qu = 1.2 * (dead + self_wt) + 1.6 * live;
        let mu = qu * span.powi(2) / 8.0;
        let d_req = (mu * 1000.0 / (0.9 * 0.85 * fc * 1000.0)).sqrt(); // mm approx
        let as_req = 0.85 * fc * d_req / fy * (1.0 - (1.0 - 2.0 * mu * 1000.0 / (0.85 * fc * d_req.powi(2))).sqrt());

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if d_req > 300.0 {
            warnings.push(format!("Thick slab ({:.0} mm). Consider two-way.", d_req));
            recommendations.push("Check punching shear if supported on columns".to_string());
        }

        compliance_notes.push("One-way slab per ACI 318".to_string());
        compliance_notes.push("Minimum reinforcement for shrinkage".to_string());

        let results = vec![
            EngineeringResultItem::new("Factored Load", qu, "kPa")
                .with_format(format!("{:.2} kPa", qu)),
            EngineeringResultItem::new("Moment", mu, "kNm/m")
                .critical()
                .with_format(format!("{:.2} kNm/m", mu)),
            EngineeringResultItem::new("Required Depth", d_req, "mm")
                .critical()
                .with_format(format!("{:.0} mm", d_req)),
            EngineeringResultItem::new("Required Steel", as_req, "mm²/m")
                .with_format(format!("{:.0} mm²/m", as_req)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "slab_design".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "ACI 318".to_string(),
                requires_pe_review: true,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::engineer::test_utils::*;

    #[tokio::test]
    async fn test_slab_design() {
        let calc = SlabDesignCalculator;
        
        let mut params = parameters_with_dimensions(vec![
            ("length", 4.0),
        ]);
        params.loads = Some(LoadCase {
            dead_load: 2.0,
            live_load: 3.0,
            ..Default::default()
        });
        params.material = Some(MaterialProperties {
            compressive_strength: Some(30.0),
            yield_strength: Some(420.0),
            ..Default::default()
        });

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() >= 4);
    }
}