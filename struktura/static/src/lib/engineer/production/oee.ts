/**
 * OEE Calculator API Client
 * 
 * A bulletproof, type-safe client for the OEE calculation backend.
 * Because runtime errors are for people who don't plan ahead.
 * 
 * Architecture decisions:
 * - No axios dependency (fetch is fine, and one less supply chain attack vector)
 * - Explicit error types (because "something went wrong" is not actionable)
 * - Request/response validation (trust nothing, verify everything)
 * - Retry logic with exponential backoff (because networks lie)
 */

// ============================================================================
// Type Definitions - The Contract Layer
// ============================================================================

/** Input value provenance - critical for trust */
export type InputSource = 'explicit' | 'inferred' | 'default';

export interface InputValue<T> {
  Explicit?: T;
  Inferred?: T;
  Default?: T;
}

export interface AnalysisWindow {
  start: string; // ISO 8601
  end: string;   // ISO 8601
}

export interface MachineContext {
  machine_id: string;
  line_id?: string;
  product_id?: string;
  shift_id?: string;
}

export enum MachineState {
  Running = 'Running',
  Stopped = 'Stopped',
  Setup = 'Setup',
  Starved = 'Starved',
  Blocked = 'Blocked',
  Maintenance = 'Maintenance',
  Unknown = 'Unknown',
}

export interface TimeAllocation {
  state: MachineState;
  duration: InputValue<number>; // nanoseconds
  reason?: ReasonCode;
  notes?: string;
}

export interface ReasonCode {
  path: string[];
  is_failure: boolean;
}

export interface TimeModel {
  planned_production_time: InputValue<number>;
  allocations: TimeAllocation[];
  all_time?: InputValue<number>; // For TEEP
}

export interface ProductionSummary {
  total_units: InputValue<number>;
  good_units: InputValue<number>;
  scrap_units: InputValue<number>;
  reworked_units: InputValue<number>;
}

export interface CycleTimeModel {
  ideal_cycle_time: InputValue<number>;
  average_cycle_time?: InputValue<number>;
}

export interface DowntimeRecord {
  duration: InputValue<number>;
  reason: ReasonCode;
  timestamp?: string;
  notes?: string;
}

export interface DowntimeCollection {
  records: DowntimeRecord[];
}

export interface ThresholdConfiguration {
  micro_stoppage_threshold: number;
  small_stop_threshold: number;
  speed_loss_threshold: number;
  high_scrap_rate_threshold: number;
  low_utilization_threshold: number;
}

export interface OeeInput {
  window: AnalysisWindow;
  machine: MachineContext;
  time_model: TimeModel;
  production: ProductionSummary;
  cycle_time: CycleTimeModel;
  downtimes: DowntimeCollection;
  thresholds: ThresholdConfiguration;
}

// Response Types

export type Confidence = 'High' | 'Medium' | 'Low';

export interface TrackedMetric {
  name_key: string;
  value: number;
  unit_key: string;
  formula_key: string;
  formula_params: Record<string, number>;
  confidence: Confidence;
}

export interface CoreMetrics {
  availability: TrackedMetric;
  performance: TrackedMetric;
  quality: TrackedMetric;
  oee: TrackedMetric;
}

export interface ExtendedMetrics {
  teep?: TrackedMetric;
  utilization: TrackedMetric;
  mtbf?: TrackedMetric;
  mttr?: TrackedMetric;
  scrap_rate: TrackedMetric;
  rework_rate: TrackedMetric;
  net_operating_time: TrackedMetric;
}

export interface LossTreeNode {
  category_key: string;
  description_key: string;
  duration: number;
  percentage_of_planned: number;
  percentage_of_parent?: number;
  children: LossTreeNode[];
  source: 'Explicit' | 'Inferred' | 'Default';
}

export interface LossTree {
  root: LossTreeNode;
  planned_time: number;
}

export interface EconomicImpact {
  description_key: string;
  low_estimate: number;
  central_estimate: number;
  high_estimate: number;
  currency: string;
  assumptions: string[];
}

export interface EconomicAnalysis {
  throughput_loss: EconomicImpact;
  material_waste: EconomicImpact;
  rework_cost: EconomicImpact;
  opportunity_cost: EconomicImpact;
  total_impact: EconomicImpact;
}

export interface ValidationIssue {
  message_key: string;
  params: Record<string, unknown>;
  severity: 'Fatal' | 'Warning' | 'Info';
  field_path?: string;
  code: string;
}

export interface ValidationResult {
  is_valid: boolean;
  issues: ValidationIssue[];
}

export interface AssumptionEntry {
  assumption_key: string;
  description_key: string;
  value: unknown;
  source: string;
  timestamp: string;
  impact: 'Critical' | 'High' | 'Medium' | 'Low' | 'Info';
  related_assumptions: string[];
}

export interface AssumptionLedger {
  analysis_timestamp: string;
  assumptions: AssumptionEntry[];
  warnings: Array<{
    code: string;
    message_key: string;
    params: Record<string, unknown>;
    severity: 'High' | 'Medium' | 'Low';
    related_assumptions: string[];
  }>;
  thresholds: Array<{
    threshold_key: string;
    value: number;
    unit_key: string;
    rationale_key: string;
  }>;
  source_statistics: {
    explicit_count: number;
    inferred_count: number;
    default_count: number;
    total_count: number;
    explicit_percentage: number;
    inferred_percentage: number;
    default_percentage: number;
  };
  metadata: Record<string, string>;
}

export interface OeeResult {
  core_metrics: CoreMetrics;
  extended_metrics: ExtendedMetrics;
  loss_tree: LossTree;
  economic_analysis?: EconomicAnalysis;
  ledger: AssumptionLedger;
  validation: ValidationResult;
}

// Economic Parameters

export interface EconomicParameters {
  unit_price: [number, number, number]; // [low, central, high]
  marginal_contribution: [number, number, number];
  material_cost: [number, number, number];
  labor_cost_per_hour: [number, number, number];
  currency: string;
}

// Sensitivity Analysis

export type SensitivityImpact = 'Critical' | 'High' | 'Medium' | 'Low';

export interface MetricChanges {
  availability_delta: number;
  performance_delta: number;
  quality_delta: number;
}

export interface SensitivityResult {
  parameter_key: string;
  baseline_value: number;
  variation_percent: number;
  varied_value: number;
  baseline_oee: number;
  varied_oee: number;
  oee_delta: number;
  impact_level: SensitivityImpact;
  metric_changes: MetricChanges;
}

export interface SensitivityAnalysis {
  baseline_oee: number;
  results: SensitivityResult[];
  most_sensitive_parameter: string;
  least_sensitive_parameter: string;
}

// Leverage Analysis

export interface LeverageImpact {
  category_key: string;
  oee_opportunity_points: number;
  throughput_gain_units: number;
  sensitivity_score: number;
}

// Temporal Scrap Analysis

export interface ScrapEvent {
  timestamp: string;
  units: number;
  reason?: string;
  notes?: string;
}

export interface TemporalScrapData {
  events: ScrapEvent[];
  analysis_window: AnalysisWindow;
}

export interface StartupWindowConfig {
  fixed_duration?: number;
  percentage_of_total?: number;
  dynamic_threshold?: number;
}

export interface ScrapByPhase {
  startup_events: ScrapEvent[];
  steady_state_events: ScrapEvent[];
}

export interface TemporalScrapAnalysis {
  total_scrap: number;
  startup_scrap: number;
  steady_state_scrap: number;
  startup_window_duration: number;
  startup_scrap_percentage: number;
  startup_scrap_time_loss: number;
  steady_state_scrap_time_loss: number;
  scrap_by_phase: ScrapByPhase;
}

// Multi-Machine Analysis

export type AggregationMethod =
  | 'SimpleAverage'
  | 'ProductionWeighted'
  | 'TimeWeighted'
  | 'Minimum'
  | 'Multiplicative';

export interface MachineOeeData {
  machine_id: string;
  machine_name?: string;
  result: OeeResult;
  sequence_position?: number;
  is_bottleneck: boolean;
}

export interface SystemMetrics {
  avg_availability: number;
  avg_performance: number;
  avg_quality: number;
  total_planned_time: number;
  total_downtime: number;
  total_production: number;
  total_good_units: number;
  best_machine_id: string;
  worst_machine_id: string;
}

export interface BottleneckInfo {
  machine_id: string;
  oee: number;
  throughput_impact: number;
  recommended_action_key: string;
}

export interface BottleneckAnalysis {
  primary_bottlenecks: BottleneckInfo[];
  system_capacity_limit?: number;
  potential_throughput_gain: number;
}

export interface SystemOeeAnalysis {
  system_oee: number;
  aggregation_method: AggregationMethod;
  machines: MachineOeeData[];
  system_metrics: SystemMetrics;
  bottleneck_analysis: BottleneckAnalysis;
  confidence: Confidence;
}

// ============================================================================
// Error Handling - Because Hope Is Not A Strategy
// ============================================================================

export class OeeApiError extends Error {
  constructor(
    message: string,
    public code: string,
    public statusCode?: number,
    public details?: unknown
  ) {
    super(message);
    this.name = 'OeeApiError';
  }
}

export class OeeValidationError extends OeeApiError {
  constructor(
    message: string,
    public validation: ValidationResult
  ) {
    super(message, 'VALIDATION_FAILED', 400, validation);
    this.name = 'OeeValidationError';
  }
}

export class OeeNetworkError extends OeeApiError {
  constructor(message: string, public originalError: Error) {
    super(message, 'NETWORK_ERROR');
    this.name = 'OeeNetworkError';
  }
}

// ============================================================================
// API Client Configuration
// ============================================================================

export interface OeeApiConfig {
  baseUrl: string;
  timeout?: number;
  retryAttempts?: number;
  retryDelay?: number;
  headers?: Record<string, string>;
}

const DEFAULT_CONFIG: Required<Omit<OeeApiConfig, 'baseUrl'>> = {
  timeout: 30000, // 30 seconds - these calculations can be heavy
  retryAttempts: 3,
  retryDelay: 1000, // Start with 1 second
  headers: {
    'Content-Type': 'application/json',
  },
};

// ============================================================================
// The API Client - Where The Magic Happens
// ============================================================================

export class OeeApiClient {
  private config: Required<OeeApiConfig>;

  constructor(config: OeeApiConfig) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  /**
   * Generic request handler with retry logic
   * Because networks are unreliable and so are servers
   */
  private async request<T>(
    endpoint: string,
    options: RequestInit = {},
    attempt = 1
  ): Promise<T> {
    const url = `${this.config.baseUrl}${endpoint}`;
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.config.timeout);

    try {
      const response = await fetch(url, {
        ...options,
        headers: {
          ...this.config.headers,
          ...options.headers,
        },
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        const error = await response.json().catch(() => ({}));
        
        // Special handling for validation errors
        if (response.status === 400 && error.code === 'VALIDATION_FAILED') {
          throw new OeeValidationError(
            error.message_key || 'Validation failed',
            error.params as ValidationResult
          );
        }

        throw new OeeApiError(
          error.message_key || `Request failed with status ${response.status}`,
          error.code || 'REQUEST_FAILED',
          response.status,
          error
        );
      }

      return await response.json();
    } catch (error) {
      clearTimeout(timeoutId);

      // Network errors - retry with exponential backoff
      if (error instanceof TypeError || (error instanceof Error && error.name === 'AbortError')) {
        if (attempt < this.config.retryAttempts) {
          const delay = this.config.retryDelay * Math.pow(2, attempt - 1);
          await new Promise(resolve => setTimeout(resolve, delay));
          return this.request<T>(endpoint, options, attempt + 1);
        }
        throw new OeeNetworkError(
          'Network request failed after retries',
          error as Error
        );
      }

      throw error;
    }
  }

  // ============================================================================
  // Core OEE Calculations
  // ============================================================================

  /**
   * Calculate basic OEE
   * The bread and butter - A × P × Q
   */
  async calculateOee(input: OeeInput): Promise<OeeResult> {
    return this.request<OeeResult>('/api/oee/calculate', {
      method: 'POST',
      body: JSON.stringify({ input }),
    });
  }

  /**
   * Calculate OEE with economic analysis
   * For when you need to speak finance's language
   */
  async calculateOeeWithEconomics(
    input: OeeInput,
    economicParameters: EconomicParameters
  ): Promise<OeeResult> {
    return this.request<OeeResult>('/api/oee/calculate-with-economics', {
      method: 'POST',
      body: JSON.stringify({ input, economic_parameters: economicParameters }),
    });
  }

  /**
   * Calculate OEE with all optional analyses
   * The full tactical package
   */
  async calculateOeeFull(
    input: OeeInput,
    options?: {
      economicParameters?: EconomicParameters;
      includeSensitivity?: boolean;
      sensitivityVariation?: number;
      includeTemporalScrap?: boolean;
    }
  ): Promise<{
    result: OeeResult;
    sensitivity_analysis?: SensitivityAnalysis;
    temporal_scrap_analysis?: TemporalScrapAnalysis;
  }> {
    return this.request('/api/oee/calculate-full', {
      method: 'POST',
      body: JSON.stringify({
        input,
        economic_parameters: options?.economicParameters,
        include_sensitivity: options?.includeSensitivity,
        sensitivity_variation: options?.sensitivityVariation,
        include_temporal_scrap: options?.includeTemporalScrap,
      }),
    });
  }

  // ============================================================================
  // Analysis Endpoints
  // ============================================================================

  /**
   * Run sensitivity analysis
   * Find out which knobs actually move the needle
   */
  async analyzeSensitivity(
    input: OeeInput,
    variationPercent: number = 10
  ): Promise<SensitivityAnalysis> {
    return this.request('/api/oee/sensitivity', {
      method: 'POST',
      body: JSON.stringify({
        input,
        variation_percent: variationPercent,
      }),
    });
  }

  /**
   * Calculate leverage opportunities
   * Where to focus your improvement efforts
   */
  async analyzeLeverage(input: OeeInput): Promise<{
    leverage_impacts: LeverageImpact[];
    baseline_oee: number;
  }> {
    return this.request('/api/oee/leverage', {
      method: 'POST',
      body: JSON.stringify({ input }),
    });
  }

  /**
   * Analyze temporal scrap patterns
   * Startup vs steady-state - know the difference
   */
  async analyzeTemporalScrap(
    scrapData: TemporalScrapData,
    idealCycleTime: number,
    startupConfig?: StartupWindowConfig
  ): Promise<TemporalScrapAnalysis> {
    return this.request('/api/oee/temporal-scrap', {
      method: 'POST',
      body: JSON.stringify({
        scrap_data: scrapData,
        ideal_cycle_time: idealCycleTime,
        startup_config: startupConfig,
      }),
    });
  }

  // ============================================================================
  // Multi-Machine System Analysis
  // ============================================================================

  /**
   * Aggregate multiple machines into system-level OEE
   * Because you need to see the forest, not just the trees
   */
  async aggregateSystemOee(
    machines: MachineOeeData[],
    aggregationMethod: AggregationMethod
  ): Promise<SystemOeeAnalysis> {
    return this.request('/api/oee/system/aggregate', {
      method: 'POST',
      body: JSON.stringify({
        machines,
        aggregation_method: aggregationMethod,
      }),
    });
  }

  /**
   * Compare different aggregation methods
   * For when you need to explain why your number is what it is
   */
  async compareAggregationMethods(
    machines: MachineOeeData[]
  ): Promise<{
    comparisons: Record<string, {
      method: string;
      system_oee: number;
      use_case: string;
    }>;
    recommended_method: string;
  }> {
    return this.request('/api/oee/system/compare-methods', {
      method: 'POST',
      body: JSON.stringify({ machines }),
    });
  }
}

// ============================================================================
// Factory Function - The Civilized Way
// ============================================================================

/**
 * Create an OEE API client
 * 
 * @example
 * const client = createOeeApiClient({
 *   baseUrl: 'https://api.yourcompany.com',
 *   timeout: 45000, // For those really complex scenarios
 * });
 */
export function createOeeApiClient(config: OeeApiConfig): OeeApiClient {
  return new OeeApiClient(config);
}

// Default export for the lazy (but we won't judge)
export default OeeApiClient;