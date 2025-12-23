//! Downtime record assumptions
//! 
//! Detailed breakdown of stopped time by reason.

use super::*;

/// Individual downtime event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DowntimeRecord {
    pub duration: InputValue<Duration>,
    pub reason: ReasonCode,
    /// When it occurred (optional for now)
    pub timestamp: Option<DateTime<Utc>>,
    /// Additional context
    pub notes: Option<String>,
}

impl DowntimeRecord {
    pub fn new(duration: Duration, reason: ReasonCode) -> Self {
        Self {
            duration: InputValue::Explicit(duration),
            reason,
            timestamp: None,
            notes: None,
        }
    }
    
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
    
    pub fn with_notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_string());
        self
    }
}

/// Collection of downtime records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DowntimeCollection {
    pub records: Vec<DowntimeRecord>,
}

impl DowntimeCollection {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }
    
    pub fn add(&mut self, record: DowntimeRecord) {
        self.records.push(record);
    }
    
    /// Total downtime across all records
    pub fn total_duration(&self) -> Duration {
        self.records
            .iter()
            .map(|r| *r.duration.value())
            .sum()
    }
    
    /// Group by root reason category
    pub fn group_by_root_reason(&self) -> std::collections::HashMap<String, Duration> {
        let mut map = std::collections::HashMap::new();
        
        for record in &self.records {
            if let Some(root) = record.reason.root() {
                let entry = map.entry(root.clone()).or_insert(Duration::ZERO);
                *entry += *record.duration.value();
            }
        }
        
        map
    }

    /// Group by failures
    pub fn group_by_failures(&self) -> std::collections::HashMap<bool, Duration> {
        let mut map = std::collections::HashMap::new();

        for record in &self.records {
            if let failure = record.reason.is_failure {
                let entry = map.entry(failure.clone()).or_insert(Duration::ZERO);
                *entry += *record.duration.value();
            }
        }

        map
    }
    
    /// Count records with reason codes
    pub fn count_with_reasons(&self) -> usize {
        self.records
            .iter()
            .filter(|r| !r.reason.path.is_empty())
            .count()
    }

    /// Count records with failure reason codes
    pub fn count_with_failure_reasons(&self) -> usize {
        self.records
            .iter()
            .filter(|r| r.reason.is_failure)
            .count()
    }
    
    /// Get longest single downtime
    pub fn longest_event(&self) -> Option<&DowntimeRecord> {
        self.records
            .iter()
            .max_by_key(|r| r.duration.value())
    }
}

impl Default for DowntimeCollection {
    fn default() -> Self {
        Self::new()
    }
}
