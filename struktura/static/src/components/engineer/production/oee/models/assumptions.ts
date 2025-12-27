/**
 * OEE Assumptions Layer - TypeScript Edition
 * 
 * The "curation layer" - where analyst-provided summaries meet type safety.
 * Every value knows its provenance. Trust, but verify. And document.
 */

/**
 * How a value was obtained - the foundation of trust
 * 
 * Because "I think it was 42" and "the sensor said 42" are rather different propositions.
 */
export type InputValue<T> = 
  | { type: 'explicit'; value: T }
  | { type: 'inferred'; value: T }
  | { type: 'default'; value: T };

/**
 * Machine operational states
 */
export enum MachineState {
  Running = 'running',
  Stopped = 'stopped',
  Setup = 'setup',
  Starved = 'starved',
  Blocked = 'blocked',
  Maintenance = 'maintenance',
  Unknown = 'unknown'
}

/**
 * Hierarchical reason code
 * 
 * e.g., ["Mechanical", "Bearing Failure"] - because "stuff broke" lacks tactical precision
 */
export interface ReasonCode {
  /** Hierarchical path from general to specific */
  path: string[];
  /** Whether this represents an equipment failure */
  isFailure: boolean;
}

/**
 * Analysis time window
 */
export interface AnalysisWindow {
  start: string; // ISO 8601 timestamp
  end: string;   // ISO 8601 timestamp
}

/**
 * Machine context for the analysis
 */
export interface MachineContext {
  machineId: string;
  lineId?: string;
  productId?: string;
  shiftId?: string;
}

/**
 * Production count summary
 * 
 * The core production numbers with source tracking.
 */
export interface ProductionSummary {
  totalUnits: InputValue<number>;
  goodUnits: InputValue<number>;
  scrapUnits: InputValue<number>;
  reworkedUnits: InputValue<number>;
}

/**
 * Cycle time model
 * 
 * Ideal vs actual cycle times with override handling.
 */
export interface CycleTimeModel {
  /** Theoretical minimum cycle time (design spec) - in seconds */
  idealCycleTime: InputValue<number>;
  /** Observed average cycle time (if available) - in seconds */
  averageCycleTime?: InputValue<number>;
}

/**
 * Individual downtime event
 * 
 * One machine stop, documented with surgical precision.
 */
export interface DowntimeRecord {
  /** Duration in seconds */
  duration: InputValue<number>;
  reason: ReasonCode;
  /** When it occurred (ISO 8601) */
  timestamp?: string;
  /** Additional context - for when "bearing failure" needs elaboration */
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
 * 
 * The boundaries between "minor inconvenience" and "call the engineers immediately."
 */
export interface ThresholdConfiguration {
  /** Minimum duration to count as downtime (vs micro-stoppage) - in seconds */
  microStoppageThreshold: number;
  /** Maximum duration for "small stop" categorization - in seconds */
  smallStopThreshold: number;
  /** Speed loss detection threshold (ratio below ideal, e.g., 0.05 = 5%) */
  speedLossThreshold: number;
  /** High scrap rate warning threshold (ratio, e.g., 0.20 = 20%) */
  highScrapRateThreshold: number;
  /** Low utilization warning threshold (ratio, e.g., 0.30 = 30%) */
  lowUtilizationThreshold: number;
}

/**
 * Threshold application result
 */
export interface ThresholdClassification {
  categoryKey: string;
  thresholdUsed: string;
  thresholdValue: number;
  actualValue: number;
}

/**
 * Time allocation to a specific state
 */
export interface TimeAllocation {
  state: MachineState;
  /** Duration in seconds */
  duration: InputValue<number>;
  reason?: ReasonCode;
  /** Optional notes/context */
  notes?: string;
}

/**
 * Complete time allocation model for an analysis window
 * 
 * Now with optional TEEP support - for when OEE just isn't comprehensive enough.
 */
export interface TimeModel {
  /** Planned production time in seconds */
  plannedProductionTime: InputValue<number>;
  allocations: TimeAllocation[];
  /** Optional: Total calendar time for TEEP calculation (e.g., 24/7 time) in seconds */
  allTime?: InputValue<number>;
}

/**
 * Helper functions for working with InputValue
 */
export const InputValueHelpers = {
  /** Extract the value, regardless of provenance */
  getValue: <T>(input: InputValue<T>): T => input.value,
  
  /** Check if value is explicitly provided */
  isExplicit: <T>(input: InputValue<T>): boolean => input.type === 'explicit',
  
  /** Check if value is inferred */
  isInferred: <T>(input: InputValue<T>): boolean => input.type === 'inferred',
  
  /** Check if value is a default */
  isDefault: <T>(input: InputValue<T>): boolean => input.type === 'default',
  
  /** Create explicit value */
  explicit: <T>(value: T): InputValue<T> => ({ type: 'explicit', value }),
  
  /** Create inferred value */
  inferred: <T>(value: T): InputValue<T> => ({ type: 'inferred', value }),
  
  /** Create default value */
  default: <T>(value: T): InputValue<T> => ({ type: 'default', value }),
};

/**
 * Default threshold configurations
 */
export const DefaultThresholds = {
  /** Conservative defaults per industry standards */
  defaults: (): ThresholdConfiguration => ({
    microStoppageThreshold: 30,        // 30 seconds
    smallStopThreshold: 5 * 60,        // 5 minutes
    speedLossThreshold: 0.05,          // 5% below ideal
    highScrapRateThreshold: 0.20,      // 20%
    lowUtilizationThreshold: 0.30,     // 30%
  }),
  
  /** Strict thresholds (more aggressive categorization) */
  strict: (): ThresholdConfiguration => ({
    microStoppageThreshold: 15,
    smallStopThreshold: 3 * 60,
    speedLossThreshold: 0.02,
    highScrapRateThreshold: 0.10,
    lowUtilizationThreshold: 0.50,
  }),
  
  /** Lenient thresholds (less noise, more peace) */
  lenient: (): ThresholdConfiguration => ({
    microStoppageThreshold: 60,
    smallStopThreshold: 10 * 60,
    speedLossThreshold: 0.10,
    highScrapRateThreshold: 0.30,
    lowUtilizationThreshold: 0.20,
  }),
};