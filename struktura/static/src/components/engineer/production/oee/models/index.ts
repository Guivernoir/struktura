/**
 * OEE Calculator - Production Loss & OEE Engineering Framework
 * Complete TypeScript Type System
 * 
 * A deterministic, assumption-driven calculator for production analysis.
 * This is a System of Reasoning, not a System of Record.
 */

// ============================================================================
// Re-exports from subsystems
// ============================================================================

export * from './assumptions';
export * from './domain';
export * from './validation';
export * from './ledger';
export * from './engine';

// ============================================================================
// Core Input/Output Types
// ============================================================================

/**
 * Complete OEE analysis input
 */
export interface OeeInput {
  window: AnalysisWindow;
  machine: MachineContext;
  timeModel: TimeModel;
  production: ProductionSummary;
  cycleTime: CycleTimeModel;
  downtimes: DowntimeCollection;
  thresholds: ThresholdConfiguration;
}

/**
 * Complete OEE analysis result
 */
export interface OeeResult {
  /** Core OEE metrics (Availability, Performance, Quality, OEE) */
  coreMetrics: CoreMetrics;
  /** Extended metrics (TEEP, Utilization, MTBF, etc.) */
  extendedMetrics: ExtendedMetrics;
  /** Loss tree decomposition */
  lossTree: LossTree;
  /** Economic analysis (if parameters provided) */
  economicAnalysis?: EconomicAnalysis;
  /** Complete assumption ledger */
  ledger: AssumptionLedger;
  /** Validation result */
  validation: ValidationResult;
}

// ============================================================================
// API Request/Response Types
// ============================================================================

/**
 * Request body for basic OEE calculation
 */
export interface CalculateRequest {
  input: OeeInput;
}

/**
 * Response body for OEE calculation
 */
export interface CalculateResponse {
  result: OeeResult;
}

/**
 * Request body for OEE calculation with economics
 */
export interface CalculateWithEconomicsRequest {
  input: OeeInput;
  economicParameters: EconomicParameters;
}

/**
 * Request body for full analysis (OEE + all optional analyses)
 */
export interface CalculateFullRequest {
  input: OeeInput;
  economicParameters?: EconomicParameters;
  /** Include sensitivity analysis (default: true) */
  includeSensitivity?: boolean;
  /** Sensitivity variation percentage (default: 10.0) */
  sensitivityVariation?: number;
  /** Include temporal scrap analysis if data available (default: true) */
  includeTemporalScrap?: boolean;
}

/**
 * Response body for full analysis
 */
export interface CalculateFullResponse {
  result: OeeResult;
  sensitivityAnalysis?: SensitivityAnalysis;
  temporalScrapAnalysis?: TemporalScrapAnalysis;
}

/**
 * Request body for sensitivity analysis
 */
export interface SensitivityRequest {
  input: OeeInput;
  /** Variation percentage (default: 10.0 for ±10%) */
  variationPercent?: number;
}

/**
 * Response body for sensitivity analysis
 */
export interface SensitivityResponse {
  analysis: SensitivityAnalysis;
}

/**
 * Request body for leverage analysis
 */
export interface LeverageRequest {
  input: OeeInput;
}

/**
 * Response body for leverage analysis
 */
export interface LeverageResponse {
  leverageImpacts: LeverageImpact[];
  baselineOee: number;
}

/**
 * Request body for temporal scrap analysis
 */
export interface TemporalScrapRequest {
  scrapData: TemporalScrapData;
  /** Ideal cycle time in seconds */
  idealCycleTime: number;
  startupConfig?: StartupWindowConfig;
}

/**
 * Response body for temporal scrap analysis
 */
export interface TemporalScrapResponse {
  analysis: TemporalScrapAnalysis;
}

/**
 * Request body for system aggregation
 */
export interface SystemAggregateRequest {
  machines: MachineOeeData[];
  aggregationMethod: AggregationMethod;
}

/**
 * Response body for system aggregation
 */
export interface SystemAggregateResponse {
  analysis: SystemOeeAnalysis;
}

/**
 * Request body for comparing aggregation methods
 */
export interface SystemCompareMethodsRequest {
  machines: MachineOeeData[];
}

/**
 * Response body for method comparison
 */
export interface SystemCompareMethodsResponse {
  comparisons: Record<string, SystemMethodComparison>;
  recommendedMethod: string;
}

/**
 * System method comparison details
 */
export interface SystemMethodComparison {
  method: string;
  systemOee: number;
  useCase: string;
}

/**
 * API error response (translation-ready)
 */
export interface ApiError {
  /** Error code for programmatic handling */
  code: string;
  /** Error message key for translation */
  messageKey: string;
  /** Parameters for translation */
  params: Record<string, any>;
}

// ============================================================================
// Type imports from subsystems (for re-export context)
// ============================================================================

import type {
  AnalysisWindow,
  MachineContext,
  TimeModel,
  ProductionSummary,
  CycleTimeModel,
  DowntimeCollection,
  ThresholdConfiguration,
} from './assumptions';

import type {
  CoreMetrics,
  ExtendedMetrics,
  LossTree,
  EconomicAnalysis,
  EconomicParameters,
} from './domain';

import type {
  ValidationResult,
} from './validation';

import type {
  AssumptionLedger,
} from './ledger';

import type {
  SensitivityAnalysis,
  LeverageImpact,
  TemporalScrapAnalysis,
  TemporalScrapData,
  StartupWindowConfig,
  SystemOeeAnalysis,
  MachineOeeData,
  AggregationMethod,
} from './engine';