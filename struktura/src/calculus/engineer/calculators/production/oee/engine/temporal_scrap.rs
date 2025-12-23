//! Temporal scrap analysis
//! 
//! Distinguishes startup scrap from steady-state production scrap.
//! Requires temporal data (timestamps on scrap events).

use crate::calculus::engineer::calculators::production::oee::assumptions::AnalysisWindow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// A scrap event with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapEvent {
    /// When the scrap occurred
    pub timestamp: DateTime<Utc>,
    /// Number of units scrapped
    pub units: u32,
    /// Reason code (optional)
    pub reason: Option<String>,
    /// Additional context
    pub notes: Option<String>,
}

impl ScrapEvent {
    pub fn new(timestamp: DateTime<Utc>, units: u32) -> Self {
        Self {
            timestamp,
            units,
            reason: None,
            notes: None,
        }
    }
    
    pub fn with_reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_string());
        self
    }
    
    pub fn with_notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_string());
        self
    }
}

/// Collection of scrap events with temporal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalScrapData {
    pub events: Vec<ScrapEvent>,
    pub analysis_window: AnalysisWindow,
}

impl TemporalScrapData {
    pub fn new(analysis_window: AnalysisWindow) -> Self {
        Self {
            events: Vec::new(),
            analysis_window,
        }
    }
    
    pub fn add_event(&mut self, event: ScrapEvent) {
        self.events.push(event);
    }
    
    /// Total scrap units across all events
    pub fn total_scrap(&self) -> u32 {
        self.events.iter().map(|e| e.units).sum()
    }
}

/// Temporal scrap analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalScrapAnalysis {
    /// Total scrap units
    pub total_scrap: u32,
    /// Scrap during startup window
    pub startup_scrap: u32,
    /// Scrap during steady-state production
    pub steady_state_scrap: u32,
    /// Startup window duration used
    pub startup_window_duration: Duration,
    /// Startup scrap rate (% of total)
    pub startup_scrap_percentage: f64,
    /// Time equivalent of startup scrap
    pub startup_scrap_time_loss: Duration,
    /// Time equivalent of steady-state scrap
    pub steady_state_scrap_time_loss: Duration,
    /// Scrap events grouped by phase
    pub scrap_by_phase: ScrapByPhase,
}

/// Scrap categorized by production phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapByPhase {
    pub startup_events: Vec<ScrapEvent>,
    pub steady_state_events: Vec<ScrapEvent>,
}

/// Configuration for startup window detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupWindowConfig {
    /// Fixed duration from analysis start (e.g., first 30 minutes)
    pub fixed_duration: Option<Duration>,
    /// Or percentage of total time (e.g., first 10%)
    pub percentage_of_total: Option<f64>,
    /// Or dynamic detection based on scrap rate threshold
    pub dynamic_threshold: Option<f64>,
}

impl StartupWindowConfig {
    /// Use a fixed duration (e.g., 30 minutes)
    pub fn fixed(duration: Duration) -> Self {
        Self {
            fixed_duration: Some(duration),
            percentage_of_total: None,
            dynamic_threshold: None,
        }
    }
    
    /// Use a percentage of total production time
    pub fn percentage(pct: f64) -> Self {
        Self {
            fixed_duration: None,
            percentage_of_total: Some(pct),
            dynamic_threshold: None,
        }
    }
    
    /// Dynamic detection: startup ends when scrap rate drops below threshold
    pub fn dynamic(threshold: f64) -> Self {
        Self {
            fixed_duration: None,
            percentage_of_total: None,
            dynamic_threshold: Some(threshold),
        }
    }
    
    /// Default: first 30 minutes or 10% of production time, whichever is less
    pub fn default() -> Self {
        Self {
            fixed_duration: Some(Duration::from_secs(30 * 60)), // 30 minutes
            percentage_of_total: Some(0.10), // 10%
            dynamic_threshold: None,
        }
    }
}

/// Analyze temporal scrap data with startup window detection
pub fn analyze_temporal_scrap(
    scrap_data: &TemporalScrapData,
    ideal_cycle_time: Duration,
    config: &StartupWindowConfig,
) -> TemporalScrapAnalysis {
    let total_scrap = scrap_data.total_scrap();
    
    // Determine startup window end time
    let startup_end = determine_startup_end(scrap_data, config);
    let startup_window_duration = (startup_end - scrap_data.analysis_window.start)
        .to_std()
        .unwrap_or(Duration::ZERO);
    
    // Categorize scrap events
    let mut startup_events = Vec::new();
    let mut steady_state_events = Vec::new();
    let mut startup_scrap = 0u32;
    let mut steady_state_scrap = 0u32;
    
    for event in &scrap_data.events {
        if event.timestamp < startup_end {
            startup_scrap += event.units;
            startup_events.push(event.clone());
        } else {
            steady_state_scrap += event.units;
            steady_state_events.push(event.clone());
        }
    }
    
    // Calculate time losses
    let startup_scrap_time_loss = ideal_cycle_time * startup_scrap;
    let steady_state_scrap_time_loss = ideal_cycle_time * steady_state_scrap;
    
    // Calculate percentages
    let startup_scrap_percentage = if total_scrap > 0 {
        (startup_scrap as f64 / total_scrap as f64) * 100.0
    } else {
        0.0
    };
    
    TemporalScrapAnalysis {
        total_scrap,
        startup_scrap,
        steady_state_scrap,
        startup_window_duration,
        startup_scrap_percentage,
        startup_scrap_time_loss,
        steady_state_scrap_time_loss,
        scrap_by_phase: ScrapByPhase {
            startup_events,
            steady_state_events,
        },
    }
}

/// Determine when startup phase ends based on configuration
fn determine_startup_end(
    scrap_data: &TemporalScrapData,
    config: &StartupWindowConfig,
) -> DateTime<Utc> {
    let start = scrap_data.analysis_window.start;
    let end = scrap_data.analysis_window.end;
    let total_duration = scrap_data.analysis_window.duration();
    
    // If multiple methods specified, use the most conservative (shortest window)
    let mut candidates = Vec::new();
    
    // Fixed duration
    if let Some(fixed) = config.fixed_duration {
        candidates.push(start + chrono::Duration::from_std(fixed).unwrap_or_default());
    }
    
    // Percentage of total
    if let Some(pct) = config.percentage_of_total {
        let duration_secs = (total_duration.as_secs_f64() * pct) as i64;
        candidates.push(start + chrono::Duration::seconds(duration_secs));
    }
    
    // Dynamic detection
    if let Some(threshold) = config.dynamic_threshold {
        if let Some(dynamic_end) = detect_dynamic_startup_end(scrap_data, threshold) {
            candidates.push(dynamic_end);
        }
    }
    
    // Use the minimum (most conservative)
    candidates.into_iter()
        .min()
        .unwrap_or(end)
        .min(end) // Can't exceed analysis window
}

/// Detect startup end dynamically based on scrap rate threshold
fn detect_dynamic_startup_end(
    scrap_data: &TemporalScrapData,
    threshold: f64,
) -> Option<DateTime<Utc>> {
    if scrap_data.events.is_empty() {
        return None;
    }
    
    // Sort events by timestamp
    let mut sorted_events = scrap_data.events.clone();
    sorted_events.sort_by_key(|e| e.timestamp);
    
    // Use a rolling window to detect when scrap rate stabilizes
    let window_size = 10; // Look at 10 events at a time
    
    if sorted_events.len() < window_size {
        return None;
    }
    
    for i in window_size..sorted_events.len() {
        let window_events = &sorted_events[i - window_size..i];
        let window_scrap: u32 = window_events.iter().map(|e| e.units).sum();
        let avg_scrap = window_scrap as f64 / window_size as f64;
        
        if avg_scrap < threshold {
            return Some(window_events.first()?.timestamp);
        }
    }
    
    None
}

/// Analyze scrap rate over time (for visualization/trending)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapRateTrend {
    pub time_buckets: Vec<TimeBucket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBucket {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub scrap_count: u32,
    pub scrap_rate: f64,
}

/// Calculate scrap rate over time in buckets
pub fn calculate_scrap_trend(
    scrap_data: &TemporalScrapData,
    bucket_duration: Duration,
    total_production_by_bucket: &[u32], // Requires production data aligned with buckets
) -> ScrapRateTrend {
    let mut buckets = Vec::new();
    let start = scrap_data.analysis_window.start;
    let end = scrap_data.analysis_window.end;
    
    let mut current_time = start;
    let bucket_chrono = chrono::Duration::from_std(bucket_duration).unwrap_or_default();
    let mut bucket_idx = 0;
    
    while current_time < end && bucket_idx < total_production_by_bucket.len() {
        let bucket_end = (current_time + bucket_chrono).min(end);
        
        // Count scrap in this bucket
        let scrap_in_bucket: u32 = scrap_data.events
            .iter()
            .filter(|e| e.timestamp >= current_time && e.timestamp < bucket_end)
            .map(|e| e.units)
            .sum();
        
        let total_in_bucket = total_production_by_bucket[bucket_idx];
        let scrap_rate = if total_in_bucket > 0 {
            scrap_in_bucket as f64 / total_in_bucket as f64
        } else {
            0.0
        };
        
        buckets.push(TimeBucket {
            start_time: current_time,
            end_time: bucket_end,
            scrap_count: scrap_in_bucket,
            scrap_rate,
        });
        
        current_time = bucket_end;
        bucket_idx += 1;
    }
    
    ScrapRateTrend { time_buckets: buckets }
}

/// Quick analysis with default 30-minute startup window
pub fn quick_temporal_analysis(
    scrap_data: &TemporalScrapData,
    ideal_cycle_time: Duration,
) -> TemporalScrapAnalysis {
    let config = StartupWindowConfig::default();
    analyze_temporal_scrap(scrap_data, ideal_cycle_time, &config)
}