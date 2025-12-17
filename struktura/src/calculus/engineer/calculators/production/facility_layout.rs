use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;

pub struct FacilityLayoutCalculator;

impl ParameterValidator for FacilityLayoutCalculator {
    fn calculator_id(&self) -> &str {
        "facility_layout"
    }
}

#[async_trait]
impl EngineerCalculator for FacilityLayoutCalculator {
    fn id(&self) -> &str {
        "facility_layout"
    }

    fn name(&self) -> &str {
        "Facility Layout Optimization"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Production
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder(
            "facility_layout",
            "Facility Layout Optimization"
        )
        .category("production")
        .description("Evaluate facility layout efficiency using material flow distance and relationship metrics")
        .design_code("Lean Manufacturing")
        .design_code("Industrial Engineering Standards")
        .parameter(ParameterMetadata {
            name: "Total Flow Distance".to_string(),
            path: "additional.total_flow_distance".to_string(),
            data_type: ParameterType::Number,
            unit: "m".to_string(),
            description: "Sum of material flow distances between departments".to_string(),
            required: true,
            default_value: Some(1000.0),
            min_value: Some(0.0),
            max_value: None,
            typical_range: Some((500.0, 5000.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Number of Departments".to_string(),
            path: "additional.num_departments".to_string(),
            data_type: ParameterType::Integer,
            unit: "departments".to_string(),
            description: "Number of departments or work centers".to_string(),
            required: true,
            default_value: Some(10.0),
            min_value: Some(2.0),
            max_value: Some(50.0),
            typical_range: Some((5.0, 20.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Facility Area".to_string(),
            path: "additional.facility_area".to_string(),
            data_type: ParameterType::Number,
            unit: "m²".to_string(),
            description: "Total facility floor area".to_string(),
            required: true,
            default_value: Some(2000.0),
            min_value: Some(100.0),
            max_value: None,
            typical_range: Some((500.0, 10000.0)),
            validation_rules: None,
        })
        .parameter(ParameterMetadata {
            name: "Target Efficiency".to_string(),
            path: "additional.target_efficiency".to_string(),
            data_type: ParameterType::Number,
            unit: "%".to_string(),
            description: "Target layout efficiency".to_string(),
            required: false,
            default_value: Some(80.0),
            min_value: Some(50.0),
            max_value: Some(95.0),
            typical_range: Some((70.0, 90.0)),
            validation_rules: None,
        })
        .complexity(ComplexityLevel::Advanced)
        .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        self.get_additional_param(params, "total_flow_distance", Some(0.0), None)?;
        self.get_additional_param(params, "num_departments", Some(2.0), Some(50.0))?;
        self.get_additional_param(params, "facility_area", Some(100.0), None)?;
        if let Some(additional) = &params.additional {
            if let Some(eff) = additional.get("target_efficiency") {
                if *eff < 50.0 || *eff > 95.0 {
                    return Err(EngineeringError::InvalidParameter {
                        parameter: "target_efficiency".to_string(),
                        value: eff.to_string(),
                        reason: "Must be between 50% and 95%".to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let total_flow_distance = self.get_additional_param(&params, "total_flow_distance", None, None)?;
        let num_departments = self.get_additional_param(&params, "num_departments", None, None)?;
        let facility_area = self.get_additional_param(&params, "facility_area", None, None)?;
        let target_efficiency = params.additional.as_ref().and_then(|a| a.get("target_efficiency").copied()).unwrap_or(80.0);

        // Simplified layout efficiency metric (lower flow distance per area is better)
        let avg_distance_per_dept = total_flow_distance / num_departments;
        let space_utilization = (num_departments * 100.0) / facility_area; // Arbitrary assumption: 100m² per dept
        let layout_efficiency = 100.0 - (total_flow_distance / facility_area * 10.0); // Heuristic formula

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut compliance_notes = Vec::new();

        if layout_efficiency < target_efficiency {
            warnings.push(format!("Layout efficiency ({:.1}%) below target ({:.1}%). Reoptimization recommended.", layout_efficiency, target_efficiency));
            recommendations.push("Use systematic layout planning (SLP) to minimize flow distances".to_string());
        }

        if space_utilization > 0.1 { // Arbitrary threshold
            warnings.push("High space utilization. Risk of congestion.".to_string());
            recommendations.push("Consider cellular manufacturing or flexible layouts".to_string());
        }

        compliance_notes.push("Facility layout per lean principles to minimize transportation waste".to_string());
        compliance_notes.push("Incorporate safety and ergonomics in final design".to_string());

        let results = vec![
            EngineeringResultItem::new("Layout Efficiency", layout_efficiency, "%")
                .critical()
                .with_format(format!("{:.1}%", layout_efficiency)),
            EngineeringResultItem::new("Average Flow Distance per Department", avg_distance_per_dept, "m")
                .with_format(format!("{:.1} m", avg_distance_per_dept)),
            EngineeringResultItem::new("Space Utilization", space_utilization * 100.0, "%")
                .with_format(format!("{:.1}%", space_utilization * 100.0)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "facility_layout".to_string(),
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
    async fn test_facility_layout() {
        let calc = FacilityLayoutCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("total_flow_distance".to_string(), 1000.0);
        additional.insert("num_departments".to_string(), 10.0);
        additional.insert("facility_area".to_string(), 2000.0);
        additional.insert("target_efficiency".to_string(), 80.0);
        params.additional = Some(additional);

        let result = calc.calculate(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.results.len() == 3);
    }

    #[test]
    fn test_invalid_efficiency() {
        let calc = FacilityLayoutCalculator;
        
        let mut params = minimal_parameters();
        let mut additional = HashMap::new();
        additional.insert("total_flow_distance".to_string(), 1000.0);
        additional.insert("num_departments".to_string(), 10.0);
        additional.insert("facility_area".to_string(), 2000.0);
        additional.insert("target_efficiency".to_string(), 45.0); // Invalid
        params.additional = Some(additional);

        let result = calc.validate(&params);
        assert!(result.is_err());
    }
}