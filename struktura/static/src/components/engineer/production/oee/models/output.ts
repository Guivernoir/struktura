/**
 * Complete OEE analysis output structure
 * * This is what the backend returns after calculation.
 * Strictly typed to match the Rust Serde serialization.
 */

import { 
  Confidence, 
  ValueSource, 
  ImpactLevel, 
  LedgerWarningSeverity, 
  ValidationSeverity 
} from './enums';

// --- Primitives ---

/** Duration in seconds (Rust: std::time::Duration) */
export type Duration = number;

/** ISO 8601 Date String (Rust: chrono::DateTime<Utc>) */
export type DateTimeString = string;

/** Range tuple [low, central, high] (Rust: (f64, f64, f64)) */
export type EstimateRange = [number, number, number];

// --- Metrics ---

/**
 * A calculated metric with full traceability
 * Matches struct TrackedMetric in mod.rs
 */
export interface TrackedMetric {
  name_key: string;
  value: number;
  unit_key: string;
  formula_key: string;
  formula_params: Record<string, number>;
  confidence: Confidence;
}

/**
 * Core OEE metrics bundle
 * Matches struct CoreMetrics in metrics.rs
 */
export interface CoreMetrics {
  availability: TrackedMetric;
  performance: TrackedMetric;
  quality: TrackedMetric;
  oee: TrackedMetric;
}

/**
 * Extended metrics beyond core OEE
 * Matches struct ExtendedMetrics in extended.rs
 */
export interface ExtendedMetrics {
  /** Total Effective Equipment Performance (Option in Rust) */
  teep: TrackedMetric | null;
  /** Operating Time / Planned Time */
  utilization: TrackedMetric;
  /** Mean Time Between Failures (Option in Rust) */
  mtbf: TrackedMetric | null;
  /** Mean Time To Repair (Option in Rust) */
  mttr: TrackedMetric | null;
  /** Scrap units / Total units */
  scrap_rate: TrackedMetric;
  /** Rework units / Total units */
  rework_rate: TrackedMetric;
  /** Operating time in seconds */
  net_operating_time: TrackedMetric;
}

// --- Loss Tree ---

/**
 * A node in the loss tree
 * Matches struct LossTreeNode in loss_tree.rs
 */
export interface LossTreeNode {
  category_key: string;
  description_key: string;
  duration: Duration;
  percentage_of_planned: number;
  percentage_of_parent: number | null; // Option<f64>
  children: LossTreeNode[];
  source: ValueSource;
}

/**
 * Complete loss tree structure
 * Matches struct LossTree in loss_tree.rs
 */
export interface LossTree {
  root: LossTreeNode;
  planned_time: Duration;
}

// --- Economics ---

/**
 * Economic impact with uncertainty bounds
 * Matches struct EconomicImpact in economics.rs
 */
export interface EconomicImpact {
  description_key: string;
  low_estimate: number;
  central_estimate: number;
  high_estimate: number;
  currency: string;
  assumptions: string[];
}

/**
 * Complete economic analysis
 * Matches struct EconomicAnalysis in economics.rs
 */
export interface EconomicAnalysis {
  throughput_loss: EconomicImpact;
  material_waste: EconomicImpact;
  rework_cost: EconomicImpact;
  opportunity_cost: EconomicImpact;
  total_impact: EconomicImpact;
}

// --- Ledger & Traceability ---

/**
 * A single tracked assumption
 * Matches struct AssumptionEntry in assumption_tracking module
 */
export interface AssumptionEntry {
  assumption_key: string;
  description_key: string;
  value: unknown; // serde_json::Value
  source: string; // "explicit" | "inferred" | "default"
  timestamp: DateTimeString;
  impact: ImpactLevel;
  related_assumptions: string[];
}

/**
 * Warning recorded during analysis (Business Logic)
 * Matches struct LedgerWarning in assumption_tracking module
 */
export interface LedgerWarning {
  code: string;
  message_key: string;
  params: unknown;
  severity: LedgerWarningSeverity;
  related_assumptions: string[];
}

/**
 * Threshold configuration record
 * Matches struct ThresholdRecord in assumption_tracking module
 */
export interface ThresholdRecord {
  threshold_key: string;
  value: number;
  unit_key: string;
  rationale_key: string;
}

/**
 * Statistics about input sources
 * Matches struct SourceStatistics in assumption_tracking module
 */
export interface SourceStatistics {
  explicit_count: number;
  inferred_count: number;
  default_count: number;
  total_count: number;
  explicit_percentage: number;
  inferred_percentage: number;
  default_percentage: number;
}

/**
 * Complete assumption ledger
 * Matches struct AssumptionLedger in assumption_tracking module
 */
export interface AssumptionLedger {
  analysis_timestamp: DateTimeString;
  assumptions: AssumptionEntry[];
  warnings: LedgerWarning[];
  thresholds: ThresholdRecord[];
  source_statistics: SourceStatistics;
  metadata: Record<string, string>;
}

// --- Validation ---

/**
 * A validation issue
 * Matches struct ValidationIssue in validation module
 */
export interface ValidationIssue {
  message_key: string;
  params: unknown;
  severity: ValidationSeverity;
  field_path: string | null;
  code: string;
}

/**
 * Validation result
 * Matches struct ValidationResult in validation module
 */
export interface ValidationResult {
  is_valid: boolean;
  issues: ValidationIssue[];
}

// --- Top Level ---

/**
 * Complete OEE analysis result
 * The main response object from the calculation engine
 */
export interface OeeResult {
  core_metrics: CoreMetrics;
  extended_metrics: ExtendedMetrics;
  loss_tree: LossTree;
  /** Economic analysis is optional based on input parameters */
  economic_analysis: EconomicAnalysis | null; 
  ledger: AssumptionLedger;
  validation: ValidationResult;
}

// --- Helpers ---

export const formatPercentage = (metric: TrackedMetric, decimals: number = 1): string => {
  return `${(metric.value * 100).toFixed(decimals)}%`;
};

export const formatDuration = (seconds: number): string => {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  
  if (hours > 0) return `${hours}h ${minutes}m ${secs}s`;
  if (minutes > 0) return `${minutes}m ${secs}s`;
  return `${secs}s`;
};

export const formatCurrency = (value: number, currency: string, decimals: number = 2): string => {
  return `${currency} ${value.toFixed(decimals)}`;
};

export const getOeeColorClass = (oeeValue: number): string => {
  if (oeeValue >= 0.85) return 'text-green-600'; // World class
  if (oeeValue >= 0.60) return 'text-yellow-600'; // Acceptable
  return 'text-red-600'; // Needs improvement
};

export const getConfidenceColorClass = (confidence: Confidence): string => {
  switch (confidence) {
    case Confidence.High: return 'bg-green-100 text-green-800';
    case Confidence.Medium: return 'bg-yellow-100 text-yellow-800';
    case Confidence.Low: return 'bg-red-100 text-red-800';
  }
};