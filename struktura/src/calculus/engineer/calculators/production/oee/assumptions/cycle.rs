//! Cycle time assumptions
//! 
//! Ideal vs actual cycle times with override handling.

use super::*;

/// Cycle time model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleTimeModel {
    /// Theoretical minimum cycle time (design spec)
    pub ideal_cycle_time: InputValue<Duration>,
    /// Observed average cycle time (if available)
    pub average_cycle_time: Option<InputValue<Duration>>,
}

impl CycleTimeModel {
    /// Create with just ideal cycle time
    pub fn from_ideal(ideal: Duration) -> Self {
        Self {
            ideal_cycle_time: InputValue::Explicit(ideal),
            average_cycle_time: None,
        }
    }
    
    /// Create with both ideal and average
    pub fn with_average(ideal: Duration, average: Duration) -> Self {
        Self {
            ideal_cycle_time: InputValue::Explicit(ideal),
            average_cycle_time: Some(InputValue::Explicit(average)),
        }
    }
    
    /// Calculate average from production data if not provided
    pub fn infer_average(
        ideal: Duration,
        total_units: u32,
        running_time: Duration,
    ) -> Self {
        if total_units == 0 {
            return Self::from_ideal(ideal);
        }
        
        let avg = running_time / total_units;
        
        Self {
            ideal_cycle_time: InputValue::Explicit(ideal),
            average_cycle_time: Some(InputValue::Inferred(avg)),
        }
    }
    
    /// Get the effective cycle time for calculations
    /// Resolution: Explicit average > Inferred average > Ideal
    pub fn effective_cycle_time(&self) -> Duration {
        self.average_cycle_time
            .as_ref()
            .map(|a| *a.value())
            .unwrap_or_else(|| *self.ideal_cycle_time.value())
    }
    
    /// Check if average is explicitly overriding calculation
    pub fn has_explicit_override(&self) -> bool {
        self.average_cycle_time
            .as_ref()
            .map(|a| a.is_explicit())
            .unwrap_or(false)
    }
}
