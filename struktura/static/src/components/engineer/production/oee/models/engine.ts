/**
 * OEE Calculation Engine - Data Structures
 * 
 * Where Input becomes Result. Pure functions, no side effects,
 * complete traceability. The orchestrator speaks in data.
 */

/**
 * Leverage impact analysis result
 * 
 * Theoretical impact of eliminating loss categories.
 */
export interface LeverageImpact {
  /** Loss category (translation key) */
  categoryKey: string;
  /** OEE points gained if eliminated (e.g., 4.2 = +4.2%) */
  oeeOpportunityPoints: number;
  /** Additional throughput possible */
  throughputGainUnits: number;
  /** How sensitive this is to input assumptions */
  sensitivityScore: number;
}

/**
 * Classification of sensitivity impact
 */
export type SensitivityImpact = 'critical' | 'high' | 'medium' | 'low';

/**
 * Detailed changes in all metrics
 */
export interface MetricChanges {
  availabilityDelta: number;
  performanceDelta: number;
  qualityDelta: number;
}

/**
 * Sensitivity analysis result for a single parameter
 */
export interface SensitivityResult {
  /** Parameter varied (translation key) */
  parameterKey: string;
  /** Baseline value (original) */
  baselineValue: number;
  /** Variation tested (±%) */
  variationPercent: number;
  /** New value after variation */
  variedValue: number;
  /** Baseline OEE (%) */
  baselineOee: number;
  /** New OEE after variation (%) */
  variedOee: number;
  /** Impact on OEE (absolute percentage points) */
  oeeDelta: number;
  /** Impact classification */
  impactLevel: SensitivityImpact;
  /** Detailed metric changes */
  metricChanges: MetricChanges;
}

/**
 * Complete sensitivity analysis report
 */
export interface SensitivityAnalysis {
  baselineOee: number;
  results: SensitivityResult[];
  mostSensitiveParameter: string;
  leastSensitiveParameter: string;
}

/**
 * Method for aggregating multiple machines into system OEE
 */
export type AggregationMethod = 
  | 'simple_average'
  | 'production_weighted'
  | 'time_weighted'
  | 'minimum'
  | 'multiplicative';

/**
 * A single machine's OEE result with context
 */
export interface MachineOeeData {
  machineId: string;
  machineName?: string;
  result: any; // OeeResult type from main module
  /** Machine's role in the line (for bottleneck analysis) */
  sequencePosition?: number;
  /** Is this machine a bottleneck? */
  isBottleneck: boolean;
}

/**
 * System-level aggregated metrics
 */
export interface SystemMetrics {
  /** Weighted average availability */
  avgAvailability: number;
  /** Weighted average performance */
  avgPerformance: number;
  /** Weighted average quality */
  avgQuality: number;
  /** Total planned time across all machines (seconds) */
  totalPlannedTime: number;
  /** Total downtime across all machines (seconds) */
  totalDowntime: number;
  /** Total production across all machines */
  totalProduction: number;
  /** Total good units across all machines */
  totalGoodUnits: number;
  /** Best performing machine */
  bestMachineId: string;
  /** Worst performing machine */
  worstMachineId: string;
}

/**
 * Bottleneck identification info
 */
export interface BottleneckInfo {
  machineId: string;
  oee: number;
  /** % impact on system throughput */
  throughputImpact: number;
  recommendedActionKey: string;
}

/**
 * Bottleneck identification and impact
 */
export interface BottleneckAnalysis {
  /** Primary bottleneck machine(s) */
  primaryBottlenecks: BottleneckInfo[];
  /** System capacity constraint (units/hour) */
  systemCapacityLimit?: number;
  /** Estimated throughput gain if bottleneck eliminated */
  potentialThroughputGain: number;
}

/**
 * System-level OEE aggregation result
 */
export interface SystemOeeAnalysis {
  /** Overall system OEE (aggregated) */
  systemOee: number;
  /** Aggregation method used */
  aggregationMethod: AggregationMethod;
  /** Individual machine results */
  machines: MachineOeeData[];
  /** System-level metrics */
  systemMetrics: SystemMetrics;
  /** Bottleneck analysis */
  bottleneckAnalysis: BottleneckAnalysis;
  /** Confidence in system-level results */
  confidence: string; // Confidence type
}

/**
 * A scrap event with timestamp
 */
export interface ScrapEvent {
  /** When the scrap occurred (ISO 8601) */
  timestamp: string;
  /** Number of units scrapped */
  units: number;
  /** Reason code (optional) */
  reason?: string;
  /** Additional context */
  notes?: string;
}

/**
 * Collection of scrap events with temporal information
 */
export interface TemporalScrapData {
  events: ScrapEvent[];
  analysisWindow: {
    start: string; // ISO 8601
    end: string;   // ISO 8601
  };
}

/**
 * Scrap categorized by production phase
 */
export interface ScrapByPhase {
  startupEvents: ScrapEvent[];
  steadyStateEvents: ScrapEvent[];
}

/**
 * Temporal scrap analysis result
 */
export interface TemporalScrapAnalysis {
  /** Total scrap units */
  totalScrap: number;
  /** Scrap during startup window */
  startupScrap: number;
  /** Scrap during steady-state production */
  steadyStateScrap: number;
  /** Startup window duration used (seconds) */
  startupWindowDuration: number;
  /** Startup scrap rate (% of total) */
  startupScrapPercentage: number;
  /** Time equivalent of startup scrap (seconds) */
  startupScrapTimeLoss: number;
  /** Time equivalent of steady-state scrap (seconds) */
  steadyStateScrapTimeLoss: number;
  /** Scrap events grouped by phase */
  scrapByPhase: ScrapByPhase;
}

/**
 * Configuration for startup window detection
 */
export interface StartupWindowConfig {
  /** Fixed duration from analysis start (seconds) */
  fixedDuration?: number;
  /** Or percentage of total time (e.g., 0.10 for 10%) */
  percentageOfTotal?: number;
  /** Or dynamic detection based on scrap rate threshold */
  dynamicThreshold?: number;
}

/**
 * Time bucket for scrap rate trending
 */
export interface TimeBucket {
  startTime: string; // ISO 8601
  endTime: string;   // ISO 8601
  scrapCount: number;
  scrapRate: number;
}

/**
 * Scrap rate trend over time
 */
export interface ScrapRateTrend {
  timeBuckets: TimeBucket[];
}