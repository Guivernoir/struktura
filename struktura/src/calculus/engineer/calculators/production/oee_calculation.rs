use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{EngineerCalculator, ParameterValidator},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::helpers::*;
use super::lean_manufacturing::*;

// ============================================================================
// OEE CALCULATOR - WORLD-CLASS FACTORY FLOOR EDITION (TYPE-SAFE)
// ============================================================================
//
// USAGE: How to structure the API request
//
// This calculator uses the new extended_parameters system with proper typing:
//
// ```json
// {
//   "calculation_type": "oee_calculation",
//   "parameters": {
//     "calculation_date": "2025-12-21T10:30:00Z",
//     "dimensions": {},
//     "extended_parameters": {
//       "shift_start_time": {
//         "type": "DateTime",
//         "value": "2025-12-18T06:00:00Z"
//       },
//       "shift_end_time": {
//         "type": "DateTime",
//         "value": "2025-12-18T14:00:00Z"
//       },
//       "planned_downtime_minutes": {
//         "type": "Number",
//         "value": 30.0
//       },
//       "downtime_events": {
//         "type": "Array",
//         "value": [...]
//       },
//       "production_runs": {
//         "type": "Array",
//         "value": [...]
//       },
//       "quality_events": {
//         "type": "Array",
//         "value": [...]
//       }
//     }
//   }
// }
// ```
//
// Well, that was quite the strategic decision, wasn't it? Moving from 
// untyped JSON to a proper type system. Your compiler will thank you.
//
// ============================================================================

// ============================================================================
// DATA STRUCTURES - What supervisors actually record
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftData {
    pub start_time: String,
    pub end_time: String,
    pub planned_downtime: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowntimeCategory {
    EquipmentFailure,      // Loss 1: Breakdowns, equipment stops
    SetupAndAdjustment,    // Loss 2: Changeovers, tool changes
    IdlingAndMinorStops,   // Loss 3: Small stops, blockages, sensor issues
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DowntimeEvent {
    pub start_time: String,      // ISO 8601 timestamp
    pub end_time: String,        // ISO 8601 timestamp
    pub category: DowntimeCategory,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment_id: Option<String>,
}

impl DowntimeEvent {
    /// Calculate duration in minutes
    pub fn duration_minutes(&self) -> EngineeringResult<f64> {
        let start = DateTime::parse_from_rfc3339(&self.start_time)
            .map_err(|e| EngineeringError::InvalidParameter {
                parameter: "start_time".to_string(),
                value: self.start_time.clone(),
                reason: format!("Invalid ISO 8601 timestamp: {}", e),
            })?;
        
        let end = DateTime::parse_from_rfc3339(&self.end_time)
            .map_err(|e| EngineeringError::InvalidParameter {
                parameter: "end_time".to_string(),
                value: self.end_time.clone(),
                reason: format!("Invalid ISO 8601 timestamp: {}", e),
            })?;
        
        let duration = end.signed_duration_since(start);
        Ok(duration.num_seconds() as f64 / 60.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityLossType {
    ProcessDefect,    // Loss 5: Quality defects during production
    StartupLoss,      // Loss 6: Scrap during startup/warmup
    Rework,          // Pieces requiring rework
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityEvent {
    pub timestamp: String,
    pub loss_type: QualityLossType,
    pub quantity: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionRun {
    pub start_time: String,
    pub end_time: String,
    pub pieces_produced: f64,
    pub ideal_cycle_time: f64,  // seconds per piece
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_cycle_time: Option<f64>,  // If known
}

impl ProductionRun {
    pub fn duration_minutes(&self) -> EngineeringResult<f64> {
        let start = DateTime::parse_from_rfc3339(&self.start_time)
            .map_err(|e| EngineeringError::InvalidParameter {
                parameter: "start_time".to_string(),
                value: self.start_time.clone(),
                reason: format!("Invalid timestamp: {}", e),
            })?;
        
        let end = DateTime::parse_from_rfc3339(&self.end_time)
            .map_err(|e| EngineeringError::InvalidParameter {
                parameter: "end_time".to_string(),
                value: self.end_time.clone(),
                reason: format!("Invalid timestamp: {}", e),
            })?;
        
        Ok(end.signed_duration_since(start).num_seconds() as f64 / 60.0)
    }
}

// ============================================================================
// SIX BIG LOSSES BREAKDOWN
// ============================================================================

#[derive(Debug, Serialize)]
pub struct SixBigLosses {
    pub loss_1_equipment_failure: LossDetails,
    pub loss_2_setup_and_adjustment: LossDetails,
    pub loss_3_idling_and_minor_stops: LossDetails,
    pub loss_4_reduced_speed: LossDetails,
    pub loss_5_process_defects: LossDetails,
    pub loss_6_startup_losses: LossDetails,
}

#[derive(Debug, Serialize, Clone)]
pub struct LossDetails {
    pub time_lost_minutes: f64,
    pub percentage_of_planned: f64,
    pub event_count: usize,
    pub improvement_opportunity: String,
}

#[derive(Debug, Serialize)]
pub struct SMEDAnalysis {
    pub total_setup_events: usize,
    pub total_setup_time_minutes: f64,
    pub average_setup_time_minutes: f64,
    pub longest_setup_minutes: f64,
    pub target_setup_time: f64,  // Based on SMED principles (<10 min)
    pub potential_time_savings: f64,
}

// ============================================================================
// OEE CALCULATOR - Type-Safe World-Class Edition
// ============================================================================

pub struct OEECalculator;

impl ParameterValidator for OEECalculator {
    fn calculator_id(&self) -> &str {
        "oee_calculation"
    }
}

#[async_trait]
impl EngineerCalculator for OEECalculator {
    fn id(&self) -> &str {
        "oee_calculation"
    }

    fn name(&self) -> &str {
        "Overall Equipment Effectiveness (OEE) - Six Big Losses Analysis"
    }

    fn category(&self) -> CalculatorCategory {
        CalculatorCategory::Production
    }

    fn metadata(&self) -> EngineeringCalculatorMetadata {
        EngineeringCalculatorMetadata::builder(
            "oee_calculation",
            "Overall Equipment Effectiveness (OEE) - Six Big Losses Analysis"
        )
        .category("production")
        .description("Calculate OEE from raw event data with Six Big Losses breakdown and SMED analysis. Uses type-safe parameter system for bulletproof data handling.")
        .design_code("Lean Manufacturing")
        .parameter(ParameterMetadata {
            name: "Shift Start Time".to_string(),
            path: "extended_parameters.shift_start_time".to_string(),
            data_type: ParameterType::DateTime,
            unit: "ISO8601".to_string(),
            description: "Shift start timestamp in ISO 8601 format (e.g., 2025-12-18T06:00:00Z)".to_string(),
            required: true,
            default_value: None,
            min_value: None,
            max_value: None,
            typical_range: None,
            validation_rules: Some(vec![
                "Must be valid ISO 8601 timestamp".to_string(),
                "Must be before shift_end_time".to_string(),
            ]),
        })
        .parameter(ParameterMetadata {
            name: "Shift End Time".to_string(),
            path: "extended_parameters.shift_end_time".to_string(),
            data_type: ParameterType::DateTime,
            unit: "ISO8601".to_string(),
            description: "Shift end timestamp in ISO 8601 format".to_string(),
            required: true,
            default_value: None,
            min_value: None,
            max_value: None,
            typical_range: None,
            validation_rules: Some(vec![
                "Must be valid ISO 8601 timestamp".to_string(),
                "Must be after shift_start_time".to_string(),
            ]),
        })
        .parameter(ParameterMetadata {
            name: "Planned Downtime".to_string(),
            path: "extended_parameters.planned_downtime_minutes".to_string(),
            data_type: ParameterType::Number,
            unit: "minutes".to_string(),
            description: "Scheduled downtime (breaks, maintenance) in minutes".to_string(),
            required: true,
            default_value: Some(0.0),
            min_value: Some(0.0),
            max_value: None,
            typical_range: Some((0.0, 60.0)),
            validation_rules: Some(vec![
                "Must be non-negative".to_string(),
                "Cannot exceed shift duration".to_string(),
            ]),
        })
        .parameter(ParameterMetadata {
            name: "Downtime Events".to_string(),
            path: "extended_parameters.downtime_events".to_string(),
            data_type: ParameterType::Array,
            unit: "events".to_string(),
            description: "Array of unplanned downtime events with timestamps, categories, and reasons".to_string(),
            required: false,
            default_value: None,
            min_value: None,
            max_value: None,
            typical_range: None,
            validation_rules: Some(vec![
                "Each event must have: start_time, end_time, category, reason".to_string(),
                "Category must be: equipment_failure, setup_and_adjustment, or idling_and_minor_stops".to_string(),
                "All events must fall within shift boundaries".to_string(),
            ]),
        })
        .parameter(ParameterMetadata {
            name: "Production Runs".to_string(),
            path: "extended_parameters.production_runs".to_string(),
            data_type: ParameterType::Array,
            unit: "runs".to_string(),
            description: "Array of production runs with pieces produced and cycle times".to_string(),
            required: true,
            default_value: None,
            min_value: None,
            max_value: None,
            typical_range: None,
            validation_rules: Some(vec![
                "Each run must have: start_time, end_time, pieces_produced, ideal_cycle_time".to_string(),
                "pieces_produced must be non-negative".to_string(),
                "ideal_cycle_time must be positive (seconds/piece)".to_string(),
            ]),
        })
        .parameter(ParameterMetadata {
            name: "Quality Events".to_string(),
            path: "extended_parameters.quality_events".to_string(),
            data_type: ParameterType::Array,
            unit: "events".to_string(),
            description: "Array of quality loss events (defects, startup losses, rework)".to_string(),
            required: false,
            default_value: None,
            min_value: None,
            max_value: None,
            typical_range: None,
            validation_rules: Some(vec![
                "Each event must have: timestamp, loss_type, quantity, reason".to_string(),
                "loss_type must be: process_defect, startup_loss, or rework".to_string(),
                "quantity must be non-negative".to_string(),
            ]),
        })
        .complexity(ComplexityLevel::Intermediate)
        .build()
    }

    fn validate(&self, params: &EngineeringParameters) -> EngineeringResult<()> {
        let extended = params.extended_parameters.as_ref()
            .ok_or_else(|| EngineeringError::MissingParameter {
                parameter: "extended_parameters".to_string(),
                calculator: "oee_calculation".to_string(),
            })?;

        // Validate required timestamp fields exist
        self.require_parameter(extended, "shift_start_time")?;
        self.require_parameter(extended, "shift_end_time")?;
        self.require_parameter(extended, "planned_downtime_minutes")?;
        self.require_parameter(extended, "production_runs")?;

        Ok(())
    }

    async fn calculate(&self, params: EngineeringParameters) -> EngineeringResult<EngineeringCalculationResponse> {
        let extended = params.extended_parameters.as_ref()
            .ok_or_else(|| EngineeringError::MissingParameter {
                parameter: "extended_parameters".to_string(),
                calculator: "oee_calculation".to_string(),
            })?;

        // Extract shift timing - type-safe, no JSON parsing
        let shift_start = self.extract_datetime(extended, "shift_start_time")?;
        let shift_end = self.extract_datetime(extended, "shift_end_time")?;
        let planned_downtime = self.extract_number(extended, "planned_downtime_minutes")?;

        // Calculate shift duration
        let total_shift_time = self.calculate_shift_duration(&shift_start, &shift_end)?;
        let planned_production_time = total_shift_time - planned_downtime;

        if planned_production_time <= 0.0 {
            return Err(EngineeringError::DomainError {
                field: "planned_production_time".to_string(),
                message: "Planned downtime cannot exceed shift time. Well, that was quite the strategic decision, wasn't it?".to_string(),
            });
        }

        // Extract event arrays - properly typed
        let downtime_events = self.extract_downtime_events(extended)?;
        let production_runs = self.extract_production_runs(extended)?;
        let quality_events = self.extract_quality_events(extended)?;

        // Validate all events are within shift boundaries
        let shift_data = ShiftData {
            start_time: shift_start.clone(),
            end_time: shift_end.clone(),
            planned_downtime,
        };
        self.validate_event_timestamps(&downtime_events, &shift_data)?;

        // Calculate the Six Big Losses
        let (availability, loss_1, loss_2, loss_3) = self.calculate_availability_losses(
            planned_production_time,
            &downtime_events,
        );

        let operating_time = planned_production_time - (loss_1.time_lost_minutes + loss_2.time_lost_minutes + loss_3.time_lost_minutes);

        let (performance, loss_4) = self.calculate_performance_losses(
            operating_time,
            &production_runs,
        );

        // Calculate total pieces from production runs
        let total_pieces: f64 = production_runs.iter()
            .map(|r| r.pieces_produced)
            .sum();

        let (quality, loss_5, loss_6) = self.calculate_quality_losses(
            total_pieces,
            &quality_events,
        );

        let oee_value = oee(availability, performance, quality);

        // SMED Analysis
        let smed_analysis = self.analyze_smed(&downtime_events);

        let six_big_losses = SixBigLosses {
            loss_1_equipment_failure: loss_1.clone(),
            loss_2_setup_and_adjustment: loss_2,
            loss_3_idling_and_minor_stops: loss_3,
            loss_4_reduced_speed: loss_4,
            loss_5_process_defects: loss_5,
            loss_6_startup_losses: loss_6,
        };

        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();

        // Strategic intelligence, not just metrics
        self.generate_tactical_recommendations(
            &six_big_losses,
            &smed_analysis,
            oee_value,
            &mut warnings,
            &mut recommendations,
        );

        let results = vec![
            EngineeringResultItem::new("OEE", oee_value, "%")
                .critical()
                .with_format(format!("{:.1}%", oee_value)),
            EngineeringResultItem::new("Availability", availability, "%")
                .with_format(format!("{:.1}%", availability)),
            EngineeringResultItem::new("Performance", performance, "%")
                .with_format(format!("{:.1}%", performance)),
            EngineeringResultItem::new("Quality", quality, "%")
                .with_format(format!("{:.1}%", quality)),
            EngineeringResultItem::new("Total Shift Time", total_shift_time, "min")
                .with_format(format!("{:.0} min", total_shift_time)),
            EngineeringResultItem::new("Planned Production Time", planned_production_time, "min")
                .with_format(format!("{:.0} min", planned_production_time)),
            EngineeringResultItem::new("Operating Time", operating_time, "min")
                .with_format(format!("{:.0} min", operating_time)),
        ];

        Ok(EngineeringCalculationResponse {
            calculation_type: "oee_calculation".to_string(),
            results,
            analysis: None,
            warnings,
            structured_warnings: None,
            recommendations,
            compliance_notes: vec![
                "OEE calculation per lean manufacturing Six Big Losses framework".to_string(),
                "SMED analysis included for setup time reduction opportunities".to_string(),
                "Type-safe parameter system ensures data integrity - no JSON gymnastics required".to_string(),
            ],
            calculation_metadata: Some(CalculationMetadata {
                timestamp: params.calculation_date.unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
                calculator_version: env!("CARGO_PKG_VERSION").to_string(),
                design_code_used: "Lean Manufacturing".to_string(),
                requires_pe_review: false,
            }),
        })
    }
}

impl OEECalculator {
    // ========================================================================
    // TYPE-SAFE PARAMETER EXTRACTION - No more JSON archaeology
    // ========================================================================
    
    fn require_parameter(&self, params: &HashMap<String, ParameterValue>, key: &str) -> EngineeringResult<()> {
        params.get(key).ok_or_else(|| EngineeringError::MissingParameter {
            parameter: key.to_string(),
            calculator: "oee_calculation".to_string(),
        })?;
        Ok(())
    }

    fn extract_datetime(&self, params: &HashMap<String, ParameterValue>, key: &str) -> EngineeringResult<String> {
        let value = params.get(key)
            .ok_or_else(|| EngineeringError::MissingParameter {
                parameter: key.to_string(),
                calculator: "oee_calculation".to_string(),
            })?;

        value.as_string()
            .map(|s| s.to_string())
            .ok_or_else(|| EngineeringError::InvalidParameter {
                parameter: key.to_string(),
                value: format!("{:?}", value),
                reason: "Expected DateTime string".to_string(),
            })
    }

    fn extract_number(&self, params: &HashMap<String, ParameterValue>, key: &str) -> EngineeringResult<f64> {
        let value = params.get(key)
            .ok_or_else(|| EngineeringError::MissingParameter {
                parameter: key.to_string(),
                calculator: "oee_calculation".to_string(),
            })?;

        value.as_number()
            .ok_or_else(|| EngineeringError::InvalidParameter {
                parameter: key.to_string(),
                value: format!("{:?}", value),
                reason: "Expected Number".to_string(),
            })
    }

    fn extract_downtime_events(&self, params: &HashMap<String, ParameterValue>) -> EngineeringResult<Vec<DowntimeEvent>> {
        let array = params.get("downtime_events")
            .ok_or_else(|| EngineeringError::MissingParameter {
                parameter: "downtime_events".to_string(),
                calculator: "oee_calculation".to_string(),
            })?
            .as_array()
            .ok_or_else(|| EngineeringError::InvalidParameter {
                parameter: "downtime_events".to_string(),
                value: "not an array".to_string(),
                reason: "Expected Array of downtime events".to_string(),
            })?;

        let mut events = Vec::new();
        for (i, item) in array.iter().enumerate() {
            let event: DowntimeEvent = serde_json::from_value(item.clone())
                .map_err(|e| EngineeringError::InvalidParameter {
                    parameter: format!("downtime_events[{}]", i),
                    value: format!("{:?}", item),
                    reason: format!("Invalid DowntimeEvent structure: {}", e),
                })?;
            events.push(event);
        }

        Ok(events)
    }

    fn extract_production_runs(&self, params: &HashMap<String, ParameterValue>) -> EngineeringResult<Vec<ProductionRun>> {
        let array = params.get("production_runs")
            .ok_or_else(|| EngineeringError::MissingParameter {
                parameter: "production_runs".to_string(),
                calculator: "oee_calculation".to_string(),
            })?
            .as_array()
            .ok_or_else(|| EngineeringError::InvalidParameter {
                parameter: "production_runs".to_string(),
                value: "not an array".to_string(),
                reason: "Expected Array of production runs".to_string(),
            })?;

        let mut runs = Vec::new();
        for (i, item) in array.iter().enumerate() {
            let run: ProductionRun = serde_json::from_value(item.clone())
                .map_err(|e| EngineeringError::InvalidParameter {
                    parameter: format!("production_runs[{}]", i),
                    value: format!("{:?}", item),
                    reason: format!("Invalid ProductionRun structure: {}", e),
                })?;
            runs.push(run);
        }

        Ok(runs)
    }

    fn extract_quality_events(&self, params: &HashMap<String, ParameterValue>) -> EngineeringResult<Vec<QualityEvent>> {
        let array = params.get("quality_events")
            .ok_or_else(|| EngineeringError::MissingParameter {
                parameter: "quality_events".to_string(),
                calculator: "oee_calculation".to_string(),
            })?
            .as_array()
            .ok_or_else(|| EngineeringError::InvalidParameter {
                parameter: "quality_events".to_string(),
                value: "not an array".to_string(),
                reason: "Expected Array of quality events".to_string(),
            })?;

        let mut events = Vec::new();
        for (i, item) in array.iter().enumerate() {
            let event: QualityEvent = serde_json::from_value(item.clone())
                .map_err(|e| EngineeringError::InvalidParameter {
                    parameter: format!("quality_events[{}]", i),
                    value: format!("{:?}", item),
                    reason: format!("Invalid QualityEvent structure: {}", e),
                })?;
            events.push(event);
        }

        Ok(events)
    }

    // ========================================================================
    // CALCULATION LOGIC - The actual engineering happens here
    // ========================================================================

    fn calculate_shift_duration(&self, start: &str, end: &str) -> EngineeringResult<f64> {
        let start_time = DateTime::parse_from_rfc3339(start)
            .map_err(|e| EngineeringError::InvalidParameter {
                parameter: "shift_start_time".to_string(),
                value: start.to_string(),
                reason: format!("Invalid ISO 8601 timestamp: {}", e),
            })?;

        let end_time = DateTime::parse_from_rfc3339(end)
            .map_err(|e| EngineeringError::InvalidParameter {
                parameter: "shift_end_time".to_string(),
                value: end.to_string(),
                reason: format!("Invalid ISO 8601 timestamp: {}", e),
            })?;

        if end_time <= start_time {
            return Err(EngineeringError::InvalidParameter {
                parameter: "shift_times".to_string(),
                value: format!("start: {}, end: {}", start, end),
                reason: "End time must be after start time. Temporal causality still applies in manufacturing.".to_string(),
            });
        }

        let duration = end_time.signed_duration_since(start_time);
        Ok(duration.num_seconds() as f64 / 60.0)
    }

    fn validate_event_timestamps(
        &self,
        events: &[DowntimeEvent],
        shift_data: &ShiftData,
    ) -> EngineeringResult<()> {
        let shift_start = DateTime::parse_from_rfc3339(&shift_data.start_time)
            .map_err(|e| EngineeringError::InvalidParameter {
                parameter: "shift_start_time".to_string(),
                value: shift_data.start_time.clone(),
                reason: format!("Invalid timestamp: {}", e),
            })?;

        let shift_end = DateTime::parse_from_rfc3339(&shift_data.end_time)
            .map_err(|e| EngineeringError::InvalidParameter {
                parameter: "shift_end_time".to_string(),
                value: shift_data.end_time.clone(),
                reason: format!("Invalid timestamp: {}", e),
            })?;

        for event in events {
            let event_start = DateTime::parse_from_rfc3339(&event.start_time)
                .map_err(|e| EngineeringError::InvalidParameter {
                    parameter: "event.start_time".to_string(),
                    value: event.start_time.clone(),
                    reason: format!("Invalid timestamp: {}", e),
                })?;

            let event_end = DateTime::parse_from_rfc3339(&event.end_time)
                .map_err(|e| EngineeringError::InvalidParameter {
                    parameter: "event.end_time".to_string(),
                    value: event.end_time.clone(),
                    reason: format!("Invalid timestamp: {}", e),
                })?;

            if event_start < shift_start || event_end > shift_end {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "downtime_event".to_string(),
                    value: format!("{} - {}", event.start_time, event.end_time),
                    reason: format!(
                        "Event outside shift boundaries ({} - {}). Perhaps check your time zones?",
                        shift_data.start_time, shift_data.end_time
                    ),
                });
            }

            if event_end <= event_start {
                return Err(EngineeringError::InvalidParameter {
                    parameter: "downtime_event".to_string(),
                    value: format!("{} - {}", event.start_time, event.end_time),
                    reason: "Event end time must be after start time. Physics insists.".to_string(),
                });
            }
        }

        Ok(())
    }

    fn calculate_availability_losses(
        &self,
        planned_production_time: f64,
        downtime_events: &[DowntimeEvent],
    ) -> (f64, LossDetails, LossDetails, LossDetails) {
        let mut loss_1_time = 0.0;
        let mut loss_1_count = 0;
        let mut loss_2_time = 0.0;
        let mut loss_2_count = 0;
        let mut loss_3_time = 0.0;
        let mut loss_3_count = 0;

        for event in downtime_events {
            if let Ok(duration) = event.duration_minutes() {
                match event.category {
                    DowntimeCategory::EquipmentFailure => {
                        loss_1_time += duration;
                        loss_1_count += 1;
                    }
                    DowntimeCategory::SetupAndAdjustment => {
                        loss_2_time += duration;
                        loss_2_count += 1;
                    }
                    DowntimeCategory::IdlingAndMinorStops => {
                        loss_3_time += duration;
                        loss_3_count += 1;
                    }
                }
            }
        }

        let total_downtime = loss_1_time + loss_2_time + loss_3_time;
        let operating_time = planned_production_time - total_downtime;
        let availability = (operating_time / planned_production_time) * 100.0;

        let loss_1 = LossDetails {
            time_lost_minutes: loss_1_time,
            percentage_of_planned: (loss_1_time / planned_production_time) * 100.0,
            event_count: loss_1_count,
            improvement_opportunity: "Implement TPM (Total Productive Maintenance)".to_string(),
        };

        let loss_2 = LossDetails {
            time_lost_minutes: loss_2_time,
            percentage_of_planned: (loss_2_time / planned_production_time) * 100.0,
            event_count: loss_2_count,
            improvement_opportunity: "Apply SMED - target <10 min changeovers".to_string(),
        };

        let loss_3 = LossDetails {
            time_lost_minutes: loss_3_time,
            percentage_of_planned: (loss_3_time / planned_production_time) * 100.0,
            event_count: loss_3_count,
            improvement_opportunity: "Root cause analysis for minor stops".to_string(),
        };

        (availability, loss_1, loss_2, loss_3)
    }

    fn calculate_performance_losses(
        &self,
        operating_time_minutes: f64,
        production_runs: &[ProductionRun],
    ) -> (f64, LossDetails) {
        let mut total_pieces = 0.0;
        let mut ideal_time_seconds = 0.0;

        for run in production_runs {
            total_pieces += run.pieces_produced;
            ideal_time_seconds += run.pieces_produced * run.ideal_cycle_time;
        }

        let ideal_pieces = if total_pieces > 0.0 {
            operating_time_minutes * 60.0 / (ideal_time_seconds / total_pieces)
        } else {
            0.0
        };

        let performance = if ideal_pieces > 0.0 {
            (total_pieces / ideal_pieces) * 100.0
        } else {
            0.0
        };

        let time_lost = operating_time_minutes * (1.0 - performance / 100.0);

        let loss_4 = LossDetails {
            time_lost_minutes: time_lost,
            percentage_of_planned: (time_lost / operating_time_minutes.max(1.0)) * 100.0,
            event_count: production_runs.len(),
            improvement_opportunity: "Speed optimization study required".to_string(),
        };

        (performance, loss_4)
    }

    fn calculate_quality_losses(
        &self,
        total_pieces: f64,
        quality_events: &[QualityEvent],
    ) -> (f64, LossDetails, LossDetails) {
        let mut process_defects = 0.0;
        let mut startup_losses = 0.0;

        for event in quality_events {
            match event.loss_type {
                QualityLossType::ProcessDefect | QualityLossType::Rework => {
                    process_defects += event.quantity;
                }
                QualityLossType::StartupLoss => {
                    startup_losses += event.quantity;
                }
            }
        }

        let good_pieces = total_pieces - (process_defects + startup_losses);
        let quality = if total_pieces > 0.0 {
            (good_pieces / total_pieces) * 100.0
        } else {
            100.0  // No production means no defects, technically
        };

        let loss_5 = LossDetails {
            time_lost_minutes: 0.0, // Quality losses measured in pieces, not time
            percentage_of_planned: if total_pieces > 0.0 { (process_defects / total_pieces) * 100.0 } else { 0.0 },
            event_count: quality_events.iter()
                .filter(|e| matches!(e.loss_type, QualityLossType::ProcessDefect | QualityLossType::Rework))
                .count(),
            improvement_opportunity: "Poka-yoke and process control improvements".to_string(),
        };

        let loss_6 = LossDetails {
            time_lost_minutes: 0.0,
            percentage_of_planned: if total_pieces > 0.0 { (startup_losses / total_pieces) * 100.0 } else { 0.0 },
            event_count: quality_events.iter()
                .filter(|e| matches!(e.loss_type, QualityLossType::StartupLoss))
                .count(),
            improvement_opportunity: "Standardize startup procedures".to_string(),
        };

        (quality, loss_5, loss_6)
    }

    fn analyze_smed(&self, downtime_events: &[DowntimeEvent]) -> SMEDAnalysis {
        let setup_events: Vec<_> = downtime_events.iter()
            .filter(|e| matches!(e.category, DowntimeCategory::SetupAndAdjustment))
            .collect();

        let total_setup_time: f64 = setup_events.iter()
            .filter_map(|e| e.duration_minutes().ok())
            .sum();

        let longest_setup = setup_events.iter()
            .filter_map(|e| e.duration_minutes().ok())
            .fold(0.0, f64::max);

        let average_setup = if !setup_events.is_empty() {
            total_setup_time / setup_events.len() as f64
        } else {
            0.0
        };

        let target_setup_time = 10.0; // SMED target
        let potential_savings = if average_setup > target_setup_time {
            (average_setup - target_setup_time) * setup_events.len() as f64
        } else {
            0.0
        };

        SMEDAnalysis {
            total_setup_events: setup_events.len(),
            total_setup_time_minutes: total_setup_time,
            average_setup_time_minutes: average_setup,
            longest_setup_minutes: longest_setup,
            target_setup_time,
            potential_time_savings: potential_savings,
        }
    }

    fn generate_tactical_recommendations(
        &self,
        losses: &SixBigLosses,
        smed: &SMEDAnalysis,
        oee: f64,
        warnings: &mut Vec<String>,
        recommendations: &mut Vec<String>,
    ) {
        // Find the biggest loss (chess strategy: attack the weakest position)
        let mut loss_ranking = vec![
            ("Equipment Failure", losses.loss_1_equipment_failure.percentage_of_planned),
            ("Setup/Adjustments", losses.loss_2_setup_and_adjustment.percentage_of_planned),
            ("Minor Stops", losses.loss_3_idling_and_minor_stops.percentage_of_planned),
            ("Reduced Speed", losses.loss_4_reduced_speed.percentage_of_planned),
            ("Process Defects", losses.loss_5_process_defects.percentage_of_planned),
            ("Startup Losses", losses.loss_6_startup_losses.percentage_of_planned),
        ];
        loss_ranking.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        warnings.push(format!(
            "Primary loss category: {} ({:.1}% of planned time)",
            loss_ranking[0].0, loss_ranking[0].1
        ));

        // SMED opportunities
        if smed.average_setup_time_minutes > 10.0 {
            recommendations.push(format!(
                "SMED Opportunity: Reduce average setup from {:.1} to 10 minutes → save {:.1} min/shift",
                smed.average_setup_time_minutes,
                smed.potential_time_savings
            ));
        }

        // Strategic recommendations based on OEE level
        if oee < OEE_ACCEPTABLE {
            recommendations.push("CRITICAL: OEE below acceptable. Recommend management review and resource allocation.".to_string());
        } else if oee < OEE_GOOD {
            recommendations.push("Focus on top 2 loss categories for maximum impact.".to_string());
        } else if oee < OEE_WORLD_CLASS {
            recommendations.push("Approaching world-class. Focus on cultural embedding of continuous improvement.".to_string());
        } else {
            recommendations.push("World-class OEE achieved. Now the hard part: maintaining it. Well done.".to_string());
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS - Type-safe request construction
// ============================================================================

/// Helper to construct properly formatted OEE calculation request
/// Now with 100% less JSON wrestling
pub fn build_oee_request(
    shift_start: String,
    shift_end: String,
    planned_downtime: f64,
    downtime_events: Vec<DowntimeEvent>,
    production_runs: Vec<ProductionRun>,
    quality_events: Vec<QualityEvent>,
) -> EngineeringResult<EngineeringCalculationRequest> {
    let mut extended_parameters = HashMap::new();
    
    // Shift timing
    extended_parameters.insert(
        "shift_start_time".to_string(),
        ParameterValue::DateTime(shift_start)
    );
    extended_parameters.insert(
        "shift_end_time".to_string(),
        ParameterValue::DateTime(shift_end)
    );
    extended_parameters.insert(
        "planned_downtime_minutes".to_string(),
        ParameterValue::Number(planned_downtime)
    );
    
    // Event arrays - properly typed, no JSON involved
    extended_parameters.insert(
        "downtime_events".to_string(),
        ParameterValue::Array(
            downtime_events.iter()
                .map(|e| serde_json::to_value(e).unwrap())
                .collect()
        )
    );
    
    extended_parameters.insert(
        "production_runs".to_string(),
        ParameterValue::Array(
            production_runs.iter()
                .map(|r| serde_json::to_value(r).unwrap())
                .collect()
        )
    );
    
    extended_parameters.insert(
        "quality_events".to_string(),
        ParameterValue::Array(
            quality_events.iter()
                .map(|e| serde_json::to_value(e).unwrap())
                .collect()
        )
    );

    Ok(EngineeringCalculationRequest {
        calculation_type: "oee_calculation".to_string(),
        parameters: EngineeringParameters {
            dimensions: HashMap::new(),
            material: None,
            loads: None,
            safety_factors: None,
            design_code: None,
            exposure_class: None,
            temperature: None,
            humidity: None,
            calculation_date: Some(chrono::Utc::now().to_rfc3339()),
            extended_parameters: Some(extended_parameters),
            additional: None,
            structured_data: None,
            project_metadata: None,
        },
        output_format: Some(OutputFormat::Detailed),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::engineer::test_utils::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_oee_with_type_safe_parameters() {
        let calc = OEECalculator;
        
        let mut params = minimal_parameters();
        
        // Construct downtime events
        let downtime_events = vec![
            DowntimeEvent {
                start_time: "2025-12-18T08:15:00Z".to_string(),
                end_time: "2025-12-18T08:47:00Z".to_string(),
                category: DowntimeCategory::EquipmentFailure,
                reason: "Conveyor motor bearing seized".to_string(),
                equipment_id: Some("CONV-02".to_string()),
            },
            DowntimeEvent {
                start_time: "2025-12-18T10:00:00Z".to_string(),
                end_time: "2025-12-18T10:23:00Z".to_string(),
                category: DowntimeCategory::SetupAndAdjustment,
                reason: "Product changeover A→B".to_string(),
                equipment_id: None,
            },
        ];
        
        let production_runs = vec![
            ProductionRun {
                start_time: "2025-12-18T06:30:00Z".to_string(),
                end_time: "2025-12-18T08:15:00Z".to_string(),
                pieces_produced: 180.0,
                ideal_cycle_time: 60.0,
                actual_cycle_time: Some(62.0),
            },
        ];
        
        let quality_events = vec![
            QualityEvent {
                timestamp: "2025-12-18T07:30:00Z".to_string(),
                loss_type: QualityLossType::ProcessDefect,
                quantity: 5.0,
                reason: "Dimension out of tolerance".to_string(),
            },
        ];
        
        // Use the new extended_parameters field - type-safe
        let mut extended_parameters = HashMap::new();
        extended_parameters.insert("shift_start_time".to_string(), 
            ParameterValue::DateTime("2025-12-18T06:00:00Z".to_string()));
        extended_parameters.insert("shift_end_time".to_string(), 
            ParameterValue::DateTime("2025-12-18T14:00:00Z".to_string()));
        extended_parameters.insert("planned_downtime_minutes".to_string(), 
            ParameterValue::Number(30.0));
        extended_parameters.insert("downtime_events".to_string(),
            ParameterValue::Array(downtime_events.iter().map(|e| serde_json::to_value(e).unwrap()).collect()));
        extended_parameters.insert("production_runs".to_string(),
            ParameterValue::Array(production_runs.iter().map(|r| serde_json::to_value(r).unwrap()).collect()));
        extended_parameters.insert("quality_events".to_string(),
            ParameterValue::Array(quality_events.iter().map(|e| serde_json::to_value(e).unwrap()).collect()));
        
        params.extended_parameters = Some(extended_parameters);

        let result = calc.calculate(params).await;
        assert!(result.is_ok(), "Calculation should succeed");
        
        let response = result.unwrap();
        assert_eq!(response.calculation_type, "oee_calculation");
        assert!(!response.results.is_empty());
        
        // Verify OEE is calculated
        let oee_result = response.results.iter()
            .find(|r| r.label == "OEE")
            .expect("OEE result should exist");
        assert!(oee_result.value > 0.0 && oee_result.value <= 100.0);
    }
    
    #[test]
    fn test_parameter_value_extraction() {
        let datetime = ParameterValue::DateTime("2025-12-18T06:00:00Z".to_string());
        assert_eq!(datetime.as_string(), Some("2025-12-18T06:00:00Z"));
        
        let num = ParameterValue::Number(30.0);
        assert_eq!(num.as_number(), Some(30.0));
    }
}