//! Test module for OEE calculator
//! 
//! Comprehensive test suite covering:
//! - Core OEE calculations
//! - Sensitivity analysis
//! - Temporal scrap analysis
//! - Multi-machine aggregation
//! - TEEP calculations
//! - Validation logic
//! - API endpoints
//! - Integration tests

pub mod api;
pub mod integration;
pub mod invalid_inputs;
pub mod loss_tree;
pub mod multi_machine;
pub mod oee_math;
pub mod sensitivity;
pub mod temporal_scrap;
pub mod teep;

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use std::time::Duration;

use crate::calculus::engineer::calculators::production::oee::{
    assumptions::{
        counts::ProductionSummary,
        cycle::CycleTimeModel,
        downtime::{DowntimeCollection, DowntimeRecord},
        time::{TimeAllocation, TimeModel},
        AnalysisWindow, InputValue, MachineContext, MachineState, ReasonCode,
    },
    OeeInput,
};

/// Test fixture builder for common test scenarios
pub struct TestFixture {
    window: AnalysisWindow,
    machine: MachineContext,
    time_model: TimeModel,
    production: ProductionSummary,
    cycle_time: CycleTimeModel,
    downtimes: DowntimeCollection,
}

impl TestFixture {
    /// Create a basic valid fixture with typical values
    pub fn basic() -> Self {
        let start = Utc::now();
        let end = start + ChronoDuration::hours(8);
        
        let window = AnalysisWindow { start, end };
        
        let machine = MachineContext {
            machine_id: "TEST-001".to_string(),
            line_id: Some("LINE-A".to_string()),
            product_id: Some("PROD-X".to_string()),
            shift_id: None,
        };
        
        let planned_time = Duration::from_secs(8 * 3600); // 8 hours
        let running_time = Duration::from_secs(7 * 3600); // 7 hours running
        let downtime = Duration::from_secs(1 * 3600); // 1 hour downtime
        
        let mut time_model = TimeModel {
            planned_production_time: InputValue::Explicit(planned_time),
            allocations: Vec::new(),
            all_time: None,
        };
        
        // Add time allocations
        time_model.allocations.push(TimeAllocation::new(
            MachineState::Running,
            InputValue::Explicit(running_time),
        ));
        time_model.allocations.push(TimeAllocation::new(
            MachineState::Stopped,
            InputValue::Explicit(downtime),
        ));
        
        let production = ProductionSummary {
            total_units: InputValue::Explicit(1000),
            good_units: InputValue::Explicit(950),
            scrap_units: InputValue::Explicit(50),
            reworked_units: InputValue::Explicit(0),
        };
        
        let cycle_time = CycleTimeModel::from_ideal(Duration::from_secs(25));
        
        let downtimes = DowntimeCollection::new();
        
        Self {
            window,
            machine,
            time_model,
            production,
            cycle_time,
            downtimes,
        }
    }
    
    /// Build into OeeInput
    pub fn build(self) -> OeeInput {
        OeeInput {
            window: self.window,
            machine: self.machine,
            time_model: self.time_model,
            production: self.production,
            cycle_time: self.cycle_time,
            downtimes: self.downtimes,
            thresholds: crate::calculus::engineer::calculators::production::oee::assumptions::thresholds::ThresholdConfiguration::defaults(),
        }
    }
    
    /// Modify production counts
    pub fn with_production(mut self, total: u32, good: u32, scrap: u32, rework: u32) -> Self {
        self.production = ProductionSummary {
            total_units: InputValue::Explicit(total),
            good_units: InputValue::Explicit(good),
            scrap_units: InputValue::Explicit(scrap),
            reworked_units: InputValue::Explicit(rework),
        };
        self
    }
    
    /// Modify cycle time
    pub fn with_cycle_time(mut self, ideal_secs: u64, average_secs: Option<u64>) -> Self {
        self.cycle_time = if let Some(avg) = average_secs {
            CycleTimeModel::with_average(
                Duration::from_secs(ideal_secs),
                Duration::from_secs(avg),
            )
        } else {
            CycleTimeModel::from_ideal(Duration::from_secs(ideal_secs))
        };
        self
    }
    
    /// Add downtime record
    pub fn with_downtime(mut self, duration_secs: u64, is_failure: bool) -> Self {
        let mut reason = ReasonCode::from_single("Test Downtime");
        reason.is_failure = is_failure;
        
        let record = DowntimeRecord::new(
            Duration::from_secs(duration_secs),
            reason,
        );
        self.downtimes.add(record);
        self
    }
    
    /// Set planned production time (for TEEP scenarios)
    pub fn with_planned_time(mut self, hours: u64) -> Self {
        self.time_model.planned_production_time = InputValue::Explicit(
            Duration::from_secs(hours * 3600)
        );
        self
    }
    
    /// Enable TEEP calculation
    pub fn with_teep(mut self, all_time_hours: u64) -> Self {
        self.time_model.all_time = Some(InputValue::Explicit(
            Duration::from_secs(all_time_hours * 3600)
        ));
        self
    }
    
    /// Set time allocations to explicit values
    pub fn with_time_allocations(mut self, running_hours: u64, downtime_hours: u64) -> Self {
        self.time_model.allocations.clear();
        
        self.time_model.allocations.push(TimeAllocation::new(
            MachineState::Running,
            InputValue::Explicit(Duration::from_secs(running_hours * 3600)),
        ));
        
        if downtime_hours > 0 {
            self.time_model.allocations.push(TimeAllocation::new(
                MachineState::Stopped,
                InputValue::Explicit(Duration::from_secs(downtime_hours * 3600)),
            ));
        }
        
        self
    }
}

/// Helper to create invalid fixture (for negative tests)
pub fn invalid_fixture_time_overflow() -> OeeInput {
    TestFixture::basic()
        .with_time_allocations(10, 5) // 15 hours total, but only 8 planned
        .build()
}

/// Helper to create invalid fixture (production count mismatch)
pub fn invalid_fixture_count_mismatch() -> OeeInput {
    TestFixture::basic()
        .with_production(1000, 950, 100, 0) // good + scrap + rework > total
        .build()
}

/// Assertion helpers for floating point comparisons
pub fn assert_approx_eq(a: f64, b: f64, tolerance: f64, msg: &str) {
    let diff = (a - b).abs();
    assert!(
        diff < tolerance,
        "{}: expected {}, got {} (diff: {})",
        msg, b, a, diff
    );
}

/// Assert OEE is within expected range
pub fn assert_oee_in_range(oee: f64, min: f64, max: f64) {
    assert!(
        oee >= min && oee <= max,
        "OEE {} not in range [{}, {}]",
        oee, min, max
    );
}

/// Assert metric is valid percentage (0-1)
pub fn assert_valid_percentage(value: f64, name: &str) {
    assert!(
        value >= 0.0 && value <= 1.0,
        "{} should be in [0, 1], got {}",
        name, value
    );
}