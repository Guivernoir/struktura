//! Configurable thresholds for categorization
//! 
//! These define boundaries between categories (e.g., micro-stoppage vs downtime).
//! All user-configurable, all tracked in ledger.

use super::*;

/// Threshold definitions for loss categorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdConfiguration {
    /// Minimum duration to count as downtime (vs micro-stoppage)
    pub micro_stoppage_threshold: Duration,
    
    /// Maximum duration for "small stop" categorization
    pub small_stop_threshold: Duration,
    
    /// Speed loss detection threshold (% below ideal)
    pub speed_loss_threshold: f64,
    
    /// High scrap rate warning threshold (%)
    pub high_scrap_rate_threshold: f64,
    
    /// Low utilization warning threshold (%)
    pub low_utilization_threshold: f64,
}

impl ThresholdConfiguration {
    /// Conservative defaults per industry standards
    pub fn defaults() -> Self {
        Self {
            micro_stoppage_threshold: Duration::from_secs(30),     // 30 seconds
            small_stop_threshold: Duration::from_secs(5 * 60),      // 5 minutes
            speed_loss_threshold: 0.05,                             // 5% below ideal
            high_scrap_rate_threshold: 0.20,                        // 20%
            low_utilization_threshold: 0.30,                        // 30%
        }
    }
    
    /// Strict thresholds (more aggressive categorization)
    pub fn strict() -> Self {
        Self {
            micro_stoppage_threshold: Duration::from_secs(15),
            small_stop_threshold: Duration::from_secs(3 * 60),
            speed_loss_threshold: 0.02,
            high_scrap_rate_threshold: 0.10,
            low_utilization_threshold: 0.50,
        }
    }
    
    /// Lenient thresholds (less noise)
    pub fn lenient() -> Self {
        Self {
            micro_stoppage_threshold: Duration::from_secs(60),
            small_stop_threshold: Duration::from_secs(10 * 60),
            speed_loss_threshold: 0.10,
            high_scrap_rate_threshold: 0.30,
            low_utilization_threshold: 0.20,
        }
    }
}

/// Threshold application result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdClassification {
    pub category_key: String,
    pub threshold_used: String,
    pub threshold_value: f64,
    pub actual_value: f64,
}
