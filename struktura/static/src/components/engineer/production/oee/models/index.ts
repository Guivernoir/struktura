/**
 * OEE Calculator Models
 * 
 * TypeScript type definitions that mirror the Rust backend structures.
 * 
 * Usage:
 *   import { OeeInput, OeeResult, InputValueHelpers } from './models';
 */

// Enums
export {
  MachineState,
  Confidence,
  ValueSource,
  ImpactLevel,
  LedgerWarningSeverity,
  ValidationSeverity,
  machineStateTranslationKey,
} from './enums';

// Assumptions and Input Types
export type {
  InputValue,
  AnalysisWindow,
  MachineContext,
  ReasonCode,
  ProductionSummary,
  CycleTimeModel,
  DowntimeRecord,
  DowntimeCollection,
  ThresholdConfiguration,
  TimeAllocation,
  TimeModel,
} from './assumptions';

export {
  InputValueHelpers,
  DEFAULT_THRESHOLDS,
  STRICT_THRESHOLDS,
  LENIENT_THRESHOLDS,
} from './assumptions';

// Input Structure
export type {
  OeeInput,
  EconomicParameters,
} from './input';

export {
  getInputValue,
  explicit,
} from './input';

// Output Structure
export type {
  EstimateRange,
  TrackedMetric,
  CoreMetrics,
  ExtendedMetrics,
  LossTreeNode,
  LossTree,
  EconomicImpact,
  EconomicAnalysis,
  AssumptionEntry,
  LedgerWarning,
  ThresholdRecord,
  SourceStatistics,
  AssumptionLedger,
  ValidationIssue,
  ValidationResult,
  OeeResult,
} from './output';

export {
  formatPercentage,
  formatDuration,
  formatCurrency,
  getOeeColorClass,
  getConfidenceColorClass,
} from './output';