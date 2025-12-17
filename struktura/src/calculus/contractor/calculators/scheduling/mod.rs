pub mod critical_path;
pub mod delay_analysis;
pub mod gantt;
pub mod milestone_tracking;
pub mod optimization;
pub mod resource_leveling;
pub mod time_cost;

pub use critical_path::CriticalPathCalculator;
pub use delay_analysis::DelayAnalysisCalculator;
pub use gantt::GanttChartGenerator;
pub use milestone_tracking::MilestoneTrackingCalculator;
pub use optimization::ScheduleOptimizationCalculator;
pub use resource_leveling::ResourceLevelingCalculator;
pub use time_cost::TimeCostTradeoffCalculator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::contractor::test_utils::minimal_parameters;

    #[tokio::test]
    async fn test_critical_path() {
        let calc = CriticalPathCalculator;
        let mut params = minimal_parameters();
        params.additional = Some(HashMap::from([
            ("total_tasks".to_string(), 20.0),
            ("avg_duration".to_string(), 5.0),
            ("parallel_factor".to_string(), 0.4),
        ]));

        let result = calc.calculate(params).await.unwrap();
        assert_eq!(result.results.len(), 2);
        assert_eq!(result.results[0].value, 60.0); // 20*(1-0.4)*5 =60
    }
}