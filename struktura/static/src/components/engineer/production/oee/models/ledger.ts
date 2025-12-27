/**
 * OEE Assumption Ledger - Data Structures
 * 
 * The "trust builder" - every assumption, every source,
 * every threshold, every warning, all in one auditable structure.
 * 
 * Always accessible from results. No secrets here.
 */

/**
 * Impact level of an assumption on results
 */
export type ImpactLevel = 'critical' | 'high' | 'medium' | 'low' | 'info';

/**
 * Warning severity levels
 */
export type WarningSeverity = 'high' | 'medium' | 'low';

/**
 * A single tracked assumption
 */
export interface AssumptionEntry {
  /** Assumption identifier (translation key) */
  assumptionKey: string;
  /** Human-readable description (translation key) */
  descriptionKey: string;
  /** The actual value */
  value: any;
  /** How it was obtained: "explicit", "inferred", "default" */
  source: string;
  /** When it was recorded (ISO 8601) */
  timestamp: string;
  /** Impact level on results */
  impact: ImpactLevel;
  /** Related assumptions (dependencies) */
  relatedAssumptions: string[];
}

/**
 * Warning recorded during analysis
 */
export interface LedgerWarning {
  /** Warning code */
  code: string;
  /** Warning message (translation key) */
  messageKey: string;
  /** Parameters for translation */
  params: Record<string, any>;
  /** Severity */
  severity: WarningSeverity;
  /** Related assumptions */
  relatedAssumptions: string[];
}

/**
 * Threshold configuration record
 */
export interface ThresholdRecord {
  /** Threshold name (translation key) */
  thresholdKey: string;
  /** Value used */
  value: number;
  /** Unit (translation key) */
  unitKey: string;
  /** Why this threshold was used */
  rationaleKey: string;
}

/**
 * Statistics about input sources
 */
export interface SourceStatistics {
  explicitCount: number;
  inferredCount: number;
  defaultCount: number;
  totalCount: number;
  explicitPercentage: number;
  inferredPercentage: number;
  defaultPercentage: number;
}

/**
 * Complete assumption ledger
 */
export interface AssumptionLedger {
  /** When this analysis was performed (ISO 8601) */
  analysisTimestamp: string;
  /** All tracked assumptions */
  assumptions: AssumptionEntry[];
  /** All warnings raised */
  warnings: LedgerWarning[];
  /** All thresholds used */
  thresholds: ThresholdRecord[];
  /** Input source statistics */
  sourceStatistics: SourceStatistics;
  /** Metadata */
  metadata: Record<string, string>;
}