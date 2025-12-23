//! Time allocation assumptions (with TEEP support)
//! 
//! How the analyst has partitioned the analysis window into states.
//! Now includes optional all_time for TEEP calculation.

use super::*;

/// Time allocation to a specific state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeAllocation {
    pub state: MachineState,
    pub duration: InputValue<Duration>,
    pub reason: Option<ReasonCode>,
    /// Optional notes/context
    pub notes: Option<String>,
}

impl TimeAllocation {
    pub fn new(state: MachineState, duration: InputValue<Duration>) -> Self {
        Self {
            state,
            duration,
            reason: None,
            notes: None,
        }
    }
    
    pub fn with_reason(mut self, reason: ReasonCode) -> Self {
        self.reason = Some(reason);
        self
    }
    
    pub fn with_notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_string());
        self
    }
}

/// Complete time allocation model for an analysis window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeModel {
    pub planned_production_time: InputValue<Duration>,
    pub allocations: Vec<TimeAllocation>,
    /// Optional: Total calendar time for TEEP calculation (e.g., 24/7 time)
    /// If provided, enables TEEP metric
    pub all_time: Option<InputValue<Duration>>,
}

impl TimeModel {
    /// Create with planned time only
    pub fn new(planned_production_time: InputValue<Duration>) -> Self {
        Self {
            planned_production_time,
            allocations: Vec::new(),
            all_time: None,
        }
    }
    
    /// Create with TEEP support (includes all_time)
    pub fn with_teep_support(
        planned_production_time: InputValue<Duration>,
        all_time: InputValue<Duration>,
    ) -> Self {
        Self {
            planned_production_time,
            allocations: Vec::new(),
            all_time: Some(all_time),
        }
    }
    
    /// Get total allocated time
    pub fn total_allocated(&self) -> Duration {
        self.allocations
            .iter()
            .map(|a| *a.duration.value())
            .sum()
    }
    
    /// Get unallocated time (if any)
    pub fn unallocated_time(&self) -> Duration {
        let planned = *self.planned_production_time.value();
        let allocated = self.total_allocated();
        
        if planned > allocated {
            planned - allocated
        } else {
            Duration::ZERO
        }
    }
    
    /// Get time spent in a specific state
    pub fn time_in_state(&self, state: &MachineState) -> Duration {
        self.allocations
            .iter()
            .filter(|a| &a.state == state)
            .map(|a| *a.duration.value())
            .sum()
    }
    
    /// Get running time (for availability calculation)
    pub fn running_time(&self) -> Duration {
        self.time_in_state(&MachineState::Running)
    }
    
    /// Get total downtime (all non-running states)
    pub fn total_downtime(&self) -> Duration {
        self.allocations
            .iter()
            .filter(|a| a.state != MachineState::Running)
            .map(|a| *a.duration.value())
            .sum()
    }
    
    /// Check if TEEP calculation is possible
    pub fn supports_teep(&self) -> bool {
        self.all_time.is_some()
    }
    
    /// Get all_time for TEEP calculation
    pub fn get_all_time(&self) -> Option<Duration> {
        self.all_time.as_ref().map(|t| *t.value())
    }
    
    /// Calculate loading factor (for TEEP)
    /// Loading Factor = Operating Time / All Time
    pub fn loading_factor(&self) -> Option<f64> {
        let all_time = self.get_all_time()?;
        let all_secs = all_time.as_secs_f64();
        
        if all_secs > 0.0 {
            Some(self.running_time().as_secs_f64() / all_secs)
        } else {
            None
        }
    }
}