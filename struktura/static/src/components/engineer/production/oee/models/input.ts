/**
 * OEE Analysis Input Models
 * * These types mirror the Rust backend structures found in the `assumptions` module.
 * Strictly typed to ensure correct serialization/deserialization with the calculation engine.
 */

import { MachineState } from './enums';

// --- Core Primitives ---

/**
 * Duration in seconds
 * Rust: std::time::Duration
 */
export type Duration = number;

/**
 * ISO 8601 Date String
 * Rust: chrono::DateTime<Utc>
 */
export type DateTimeString = string;

/**
 * Wrapper for values that tracks provenance (Source of Truth).
 * Matches Rust enum InputValue<T> in mod.rs.
 * * Serialization assumed: External tagging (Default Serde)
 * e.g., { "Explicit": 100 }
 */
export type InputValue<T> = 
  | { Explicit: T }
  | { Inferred: T }
  | { Default: T };

/**
 * Helper to extract the raw value from an InputValue wrapper
 */
export const getInputValue = <T>(input: InputValue<T>): T => {
  if ('Explicit' in input) return input.Explicit;
  if ('Inferred' in input) return input.Inferred;
  if ('Default' in input) return input.Default;
  throw new Error('Invalid InputValue format');
};

/**
 * Helper to wrap a value as Explicit
 */
export const explicit = <T>(value: T): InputValue<T> => ({ Explicit: value });

// --- Domain Models ---

/**
 * Analysis time window
 * Matches struct AnalysisWindow in mod.rs
 */
export interface AnalysisWindow {
  start: DateTimeString;
  end: DateTimeString;
}

/**
 * Machine context for the analysis
 * Matches struct MachineContext in mod.rs
 */
export interface MachineContext {
  machine_id: string;
  line_id: string | null;
  product_id: string | null;
  shift_id: string | null;
}

/**
 * Hierarchical reason code
 * Matches struct ReasonCode in mod.rs
 */
export interface ReasonCode {
  /** Hierarchical path, e.g., ["Mechanical", "Bearing Failure"] */
  path: string[];
  /** Whether this reason counts as a failure (vs. operational stop) */
  is_failure: boolean;
}

// --- Time & Allocations ---

/**
 * Time allocation to a specific state
 * Matches struct TimeAllocation in time.rs
 */
export interface TimeAllocation {
  state: MachineState;
  duration: InputValue<Duration>;
  reason: ReasonCode | null;
  notes: string | null;
}

/**
 * Complete time allocation model
 * Matches struct TimeModel in time.rs
 */
export interface TimeModel {
  planned_production_time: InputValue<Duration>;
  allocations: TimeAllocation[];
  /** * Optional: Total calendar time for TEEP calculation.
   * If provided, enables TEEP metric.
   */
  all_time: InputValue<Duration> | null;
}

// --- Production Counts ---

/**
 * Production count summary
 * Matches struct ProductionSummary in counts.rs
 */
export interface ProductionSummary {
  total_units: InputValue<number>; // u32 in Rust
  good_units: InputValue<number>;  // u32 in Rust
  scrap_units: InputValue<number>; // u32 in Rust
  reworked_units: InputValue<number>; // u32 in Rust
}

// --- Cycle Time ---

/**
 * Cycle time model
 * Matches struct CycleTimeModel in cycle.rs
 */
export interface CycleTimeModel {
  /** Theoretical minimum cycle time (design spec) */
  ideal_cycle_time: InputValue<Duration>;
  /** Observed average cycle time (if available) */
  average_cycle_time: InputValue<Duration> | null;
}

// --- Downtime Records ---

/**
 * Individual downtime event
 * Matches struct DowntimeRecord in downtime.rs
 */
export interface DowntimeRecord {
  duration: InputValue<Duration>;
  reason: ReasonCode;
  timestamp: DateTimeString | null;
  notes: string | null;
}

/**
 * Collection of downtime records
 * Matches struct DowntimeCollection in downtime.rs
 */
export interface DowntimeCollection {
  records: DowntimeRecord[];
}

// --- Thresholds ---

/**
 * Configurable thresholds for categorization
 * Matches struct ThresholdConfiguration in thresholds.rs
 */
export interface ThresholdConfiguration {
  /** Minimum duration to count as downtime (vs micro-stoppage) */
  micro_stoppage_threshold: Duration;
  
  /** Maximum duration for "small stop" categorization */
  small_stop_threshold: Duration;
  
  /** Speed loss detection threshold (% below ideal, e.g., 0.05) */
  speed_loss_threshold: number;
  
  /** High scrap rate warning threshold (e.g., 0.20) */
  high_scrap_rate_threshold: number;
  
  /** Low utilization warning threshold (e.g., 0.30) */
  low_utilization_threshold: number;
}

// --- Top Level Input ---

/**
 * Complete OEE analysis input
 * Matches the struct expected by the /calculate endpoint in api.rs
 */
export interface OeeInput {
  window: AnalysisWindow;
  machine: MachineContext;
  time_model: TimeModel;
  production: ProductionSummary;
  cycle_time: CycleTimeModel;
  downtimes: DowntimeCollection;
  thresholds: ThresholdConfiguration;
}

// --- Economics (Maintained from previous, as Rust source not provided but API uses it) ---

/**
 * Economic parameters with uncertainty bounds
 * Matches domain::economics::EconomicParameters
 */
export interface EconomicParameters {
  unit_price: [number, number, number]; // (low, central, high)
  marginal_contribution: [number, number, number];
  material_cost: [number, number, number];
  labor_cost_per_hour: [number, number, number];
  currency: string;
}