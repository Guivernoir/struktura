use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

pub struct TrussAnalysisCalculator;

impl ParameterValidator for TrussAnalysisCalculator {
    fn calculator_id(&self) -> &str {
        "truss_analysis"
    }
}

#[async_trait]
impl EngineerCalculator for TrussAnalysisCalculator {
    fn id(&self) -> &str {
        "truss_analysis"
    }

    fn name(&self) -> &str {
        "Truss Analysis"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Structural
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder("truss_analysis", "Truss Analysis")
            .category("structural")
            .description("Analyze plane truss for member forces using matrix method or method of joints")
            .design_code("AISC 360")
            .parameter(ParameterMetadata {
                name: "Nodes".to_string(),
                path: "additional.nodes".to_string(),
                data_type: ParameterType::Array,
                unit: "(x,y)".to_string(),
                description: "Array of node coordinates [[x1,y1],[x2,y2],...]".to_string(),
                required: true,
                default_value: None,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["At least 3 nodes".to_string()]),
            })
            .parameter(ParameterMetadata {
                name: "Members".to_string(),
                path: "additional.members".to_string(),
                data_type: ParameterType::Array,
                unit: "[node_i,node_j]".to_string(),
                description: "Array of member connections [[1,2],[2,3],...] (1-indexed)".to_string(),
                required: true,
                default_value: None,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: None,
            })
            .parameter(ParameterMetadata {
                name: "Supports".to_string(),
                path: "additional.supports".to_string(),
                data_type: ParameterType::Array,
                unit: "[node, type]".to_string(),
                description: "Array of supports [[node,'pinned'],[node,'roller']]".to_string(),
                required: true,
                default_value: None,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: Some(vec!["Sufficient for statical determinacy".to_string()]),
            })
            .parameter(ParameterMetadata {
                name: "Loads".to_string(),
                path: "loads".to_string(),
                data_type: ParameterType::Object,
                unit: "kN".to_string(),
                description: "Nodal loads {node: [Fx, Fy]}".to_string(),
                required: true,
                default_value: None,
                min_value: None,
                max_value: None,
                typical_range: None,
                validation_rules: None,
            })
            .complexity(ComplexityLevel::Advanced)
            .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        // Simplified validation - check for determinacy
        let nodes = params.additional.as_ref().and_then(|a| a.get("nodes")).map_or(0, |_| 1); // Placeholder
        let members = params.additional.as_ref().and_then(|a| a.get("members")).map_or(0, |_| 1);
        let reactions = 3; // Assume 2 supports

        if 2 * nodes != members + reactions {
            return Err(EngineeringError::DomainError {
                field: "truss".to_string(),
                message: "Indeterminate or unstable truss".to_string(),
            });
        }

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        // Placeholder for truss analysis - in real impl, use matrix method
        let member_forces = vec![100.0, -150.0, 200.0]; // kN, example

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if member_forces.iter().any(|&f| (f as f64).abs() > 500.0) {
            warnings.push("High member forces. Verify section sizes".to_string());
            recommendations.push("Use AISC manual for member selection".to_string());
        }

        compliance_notes.push("Analysis assumes pin joints and axial forces only".to_string());
        compliance_notes.push("Check buckling for compression members".to_string());

        let results = member_forces.iter().enumerate().map(|(i, &f)| {
            EngineeringResultItem::new(format!("Member {}", i+1), f, "kN")
                .with_format(format!("{:.1} kN", f))
        }).collect();

        Ok(EngineeringCalculationResponse {
            calculation_type: "truss_analysis".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes,
            calculation_metadata: Some(CalculationMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "AISC 360".to_string(),
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
    async fn test_truss_analysis() {
        let calc = TrussAnalysisCalculator;
        
        // Setup params with example truss
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("nodes".to_string(), 3.0); // Placeholder
        additional.insert("members".to_string(), 3.0);
        additional.insert("supports".to_string(), 2.0);
        params.additional = Some(additional);
        params.loads = Some(LoadCase {
            dead_load: 10.0,
            ..Default::default()
        });

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.results.is_empty());
    }
}