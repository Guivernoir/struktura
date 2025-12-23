//! Time allocation assumptions
//! 
//! How the analyst has partitioned the analysis window into states.

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
}

impl TimeModel {
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
}
