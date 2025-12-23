/**
 * Assumption layer types
 * 
 * This is the "curation layer" - where we accept analyst-provided
 * summaries and track their provenance religiously.
 * 
 * Every value knows if it's Explicit, Inferred, or Default.
 */

import { MachineState } from './enums';

/**
 * How a value was obtained - the foundation of trust
 * 
 * TypeScript representation of Rust's InputValue<T> enum
 */
export type InputValue<T> = 
  | { type: 'Explicit'; value: T }
  | { type: 'Inferred'; value: T }
  | { type: 'Default'; value: T };

/**
 * Helper functions for InputValue
 */
export const InputValueHelpers = {
  /**
   * Create an explicit input value
   */
  explicit: <T>(value: T): InputValue<T> => ({
    type: 'Explicit',
    value,
  }),

  /**
   * Create an inferred input value
   */
  inferred: <T>(value: T): InputValue<T> => ({
    type: 'Inferred',
    value,
  }),

  /**
   * Create a default input value
   */
  default: <T>(value: T): InputValue<T> => ({
    type: 'Default',
    value,
  }),

  /**
   * Get the actual value regardless of source
   */
  getValue: <T>(input: InputValue<T>): T => input.value,

  /**
   * Check if this is an explicit value
   */
  isExplicit: <T>(input: InputValue<T>): boolean => input.type === 'Explicit',

  /**
   * Check if this is inferred
   */
  isInferred: <T>(input: InputValue<T>): boolean => input.type === 'Inferred',

  /**
   * Check if this is a default
   */
  isDefault: <T>(input: InputValue<T>): boolean => input.type === 'Default',

  /**
   * Get the source type as string (for ledger)
   */
  sourceType: <T>(input: InputValue<T>): string => {
    switch (input.type) {
      case 'Explicit': return 'explicit';
      case 'Inferred': return 'inferred';
      case 'Default': return 'default';
    }
  },

  /**
   * Map the value to a different type
   */
  map: <T, U>(input: InputValue<T>, fn: (value: T) => U): InputValue<U> => {
    const newValue = fn(input.value);
    switch (input.type) {
      case 'Explicit':
        return { type: 'Explicit', value: newValue };
      case 'Inferred':
        return { type: 'Inferred', value: newValue };
      case 'Default':
        return { type: 'Default', value: newValue };
    }
  },
};

/**
 * Analysis time window
 */
export interface AnalysisWindow {
  start: string; // ISO 8601 datetime string
  end: string;   // ISO 8601 datetime string
}

/**
 * Machine context for the analysis
 */
export interface MachineContext {
  machine_id: string;
  line_id?: string;
  product_id?: string;
  shift_id?: string;
}

/**
 * Hierarchical reason code
 */
export interface ReasonCode {
  /** Hierarchical path, e.g., ["Mechanical", "Bearing Failure"] */
  path: string[];
  is_failure: boolean;
}

/**
 * Production count summary
 */
export interface ProductionSummary {
  total_units: InputValue<number>;
  good_units: InputValue<number>;
  scrap_units: InputValue<number>;
  reworked_units: InputValue<number>;
}

/**
 * Cycle time model
 */
export interface CycleTimeModel {
  /** Theoretical minimum cycle time (design spec) in seconds */
  ideal_cycle_time: InputValue<number>;
  /** Observed average cycle time (if available) in seconds */
  average_cycle_time?: InputValue<number>;
}

/**
 * Individual downtime event
 */
export interface DowntimeRecord {
  duration: InputValue<number>; // Duration in seconds
  reason: ReasonCode;
  /** When it occurred (optional) */
  timestamp?: string; // ISO 8601 datetime string
  /** Additional context */
  notes?: string;
}

/**
 * Collection of downtime records
 */
export interface DowntimeCollection {
  records: DowntimeRecord[];
}

/**
 * Threshold definitions for loss categorization
 */
export interface ThresholdConfiguration {
  /** Minimum duration to count as downtime (vs micro-stoppage) in seconds */
  micro_stoppage_threshold: number;
  /** Maximum duration for "small stop" categorization in seconds */
  small_stop_threshold: number;
  /** Speed loss detection threshold (% below ideal) */
  speed_loss_threshold: number;
  /** High scrap rate warning threshold (%) */
  high_scrap_rate_threshold: number;
  /** Low utilization warning threshold (%) */
  low_utilization_threshold: number;
}

/**
 * Time allocation to a specific state
 */
export interface TimeAllocation {
  state: MachineState;
  duration: InputValue<number>; // Duration in seconds
  reason?: ReasonCode;
  /** Optional notes/context */
  notes?: string;
}

/**
 * Complete time allocation model for an analysis window
 */
export interface TimeModel {
  planned_production_time: InputValue<number>; // Duration in seconds
  allocations: TimeAllocation[];
  /** Optional: Total calendar time for TEEP calculation (e.g., 24/7 time) in seconds */
  all_time?: InputValue<number>;
}

/**
 * Default threshold configuration per industry standards
 */
export const DEFAULT_THRESHOLDS: ThresholdConfiguration = {
  micro_stoppage_threshold: 30,        // 30 seconds
  small_stop_threshold: 5 * 60,        // 5 minutes
  speed_loss_threshold: 0.05,          // 5% below ideal
  high_scrap_rate_threshold: 0.20,     // 20%
  low_utilization_threshold: 0.30,     // 30%
};

/**
 * Strict thresholds (more aggressive categorization)
 */
export const STRICT_THRESHOLDS: ThresholdConfiguration = {
  micro_stoppage_threshold: 15,        // 15 seconds
  small_stop_threshold: 3 * 60,        // 3 minutes
  speed_loss_threshold: 0.02,          // 2% below ideal
  high_scrap_rate_threshold: 0.10,     // 10%
  low_utilization_threshold: 0.50,     // 50%
};

/**
 * Lenient thresholds (less noise)
 */
export const LENIENT_THRESHOLDS: ThresholdConfiguration = {
  micro_stoppage_threshold: 60,        // 60 seconds
  small_stop_threshold: 10 * 60,       // 10 minutes
  speed_loss_threshold: 0.10,          // 10% below ideal
  high_scrap_rate_threshold: 0.30,     // 30%
  low_utilization_threshold: 0.20,     // 20%
};