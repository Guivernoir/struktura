pub mod cash_flow_analysis;
pub mod change_order;
pub mod progress_tracking;
pub mod project_closeout;
pub mod quality_control;
pub mod resource_allocation;
pub mod safety_planning;
pub mod subcontractor_evaluation;

pub use cash_flow_analysis::CashFlowAnalysisCalculator;
pub use change_order::ChangeOrderCalculator;
pub use progress_tracking::ProgressTrackingCalculator;
pub use project_closeout::ProjectCloseoutCalculator;
pub use quality_control::QualityControlCalculator;
pub use resource_allocation::ResourceAllocationCalculator;
pub use safety_planning::SafetyPlanningCalculator;
pub use subcontractor_evaluation::SubcontractorEvaluationCalculator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::contractor::test_utils::parameters_with_resources;

    #[tokio::test]
    async fn test_resource_allocation() {
        let calc = ResourceAllocationCalculator;
        let params = parameters_with_resources(80.0, 40.0);
        let mut params = params;
        params.additional = Some(HashMap::from([
            ("available_labor".to_string(), 100.0),
            ("available_equipment".to_string(), 50.0),
        ]));

        let result = calc.calculate(params).await.unwrap();
        assert_eq!(result.results.len(), 2);
        assert_eq!(result.results[0].value, 80.0); // 80/100 *100 =80%
    }
}