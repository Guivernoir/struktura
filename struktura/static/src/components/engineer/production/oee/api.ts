/**
 * OEE Calculator API Client
 * * Handles communication with the Rust backend calculation engine.
 * Fully supports Core OEE, Economic Analysis, Sensitivity, Leverage, and Multi-machine aggregation.
 */

import type { OeeInput, OeeResult, EconomicParameters } from './models';

// --- Type Definitions matching Backend Structs ---

/**
 * Backend API Error structure
 */
export interface ApiError {
  code: string;
  message_key: string;
  params?: Record<string, unknown>;
  // Legacy support for older UI components expecting 'message'
  message?: string; 
}

/**
 * API response wrapper
 */
export type ApiResponse<T> = 
  | { success: true; data: T }
  | { success: false; error: ApiError };

/**
 * Sensitivity Analysis Result
 */
export interface SensitivityAnalysis {
    factors: Record<string, { impact: number; sensitivity: number }>;
}

/**
 * Leverage Impact Result
 */
export interface LeverageImpact {
    category_key: string;
    oee_opportunity_points: number;
    throughput_gain_units: number;
    sensitivity_score: number;
}

/**
 * Temporal Scrap Analysis Result
 */
export interface TemporalScrapAnalysis {
    patterns: Record<string, any>;
    recommendations: string[];
}

/**
 * System OEE Analysis Result
 */
export interface SystemOeeAnalysis {
    system_oee: number;
    bottleneck_machine?: string;
    machines: Record<string, OeeResult>;
}

/**
 * Machine OEE Data for System Analysis
 */
export interface MachineOeeData {
    id: string;
    name: string;
    oee_data: OeeResult;
    production_volume?: number;
    planned_production_time?: number;
}

export type AggregationMethod = 'SimpleAverage' | 'ProductionWeighted' | 'TimeWeighted' | 'Minimum' | 'Multiplicative';

/**
 * Request Payloads
 */
export interface CalculateFullRequest {
    input: OeeInput;
    economic_parameters?: EconomicParameters;
    include_sensitivity?: boolean;
    sensitivity_variation?: number;
    include_temporal_scrap?: boolean;
}

export interface CalculateFullResponse {
    result: OeeResult;
    sensitivity_analysis?: SensitivityAnalysis;
    temporal_scrap_analysis?: TemporalScrapAnalysis;
}

export interface LeverageResponse {
    leverage_impacts: LeverageImpact[];
    baseline_oee: number;
}

export interface SystemCompareMethodsResponse {
    comparisons: Record<string, { method: string; system_oee: number; use_case: string }>;
    recommended_method: string;
}

/**
 * Configuration for the API client
 */
export interface ApiConfig {
  baseUrl: string;
  timeout?: number;
  headers?: Record<string, string>;
}

const DEFAULT_CONFIG: ApiConfig = {
  baseUrl: '/api/v1/calculus/engineer/oee',
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json',
  },
};

/**
 * OEE API Client
 */
export class OeeApiClient {
  private config: ApiConfig;

  constructor(config: Partial<ApiConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  /**
   * Helper to fetch with timeout and error handling
   */
  private async post<T>(endpoint: string, body: unknown): Promise<ApiResponse<T>> {
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), this.config.timeout);

      const response = await fetch(`${this.config.baseUrl}${endpoint}`, {
        method: 'POST',
        headers: this.config.headers,
        body: JSON.stringify(body),
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        const errorData = await this.parseErrorResponse(response);
        return { success: false, error: errorData };
      }

      const data: T = await response.json();
      return { success: true, data };
    } catch (error) {
      return { success: false, error: this.handleError(error) };
    }
  }

  /**
   * Calculate Full OEE Analysis
   * Maps to: /calculate-full
   */
  async calculateFull(request: CalculateFullRequest): Promise<ApiResponse<CalculateFullResponse>> {
    return this.post<CalculateFullResponse>('/calculate-full', request);
  }

  /**
   * Basic Calculate (Legacy support)
   * Maps to: /calculate
   */
  async calculate(input: OeeInput): Promise<ApiResponse<{ result: OeeResult }>> {
    return this.post<{ result: OeeResult }>('/calculate', { input });
  }

  /**
   * Calculate with Economics only
   * Maps to: /calculate-with-economics
   */
  async calculateWithEconomics(
    input: OeeInput, 
    economic_parameters: EconomicParameters
  ): Promise<ApiResponse<{ result: OeeResult }>> {
    return this.post<{ result: OeeResult }>('/calculate-with-economics', { input, economic_parameters });
  }

  /**
   * Analyze Leverage
   * Maps to: /leverage
   */
  async analyzeLeverage(input: OeeInput): Promise<ApiResponse<LeverageResponse>> {
    return this.post<LeverageResponse>('/leverage', { input });
  }

  /**
   * Analyze Sensitivity
   * Maps to: /sensitivity
   */
  async analyzeSensitivity(input: OeeInput, variation_percent: number = 10.0): Promise<ApiResponse<{ analysis: SensitivityAnalysis }>> {
    return this.post<{ analysis: SensitivityAnalysis }>('/sensitivity', { input, variation_percent });
  }

  /**
   * System Aggregate OEE
   * Maps to: /system/aggregate
   */
  async aggregateSystemOee(
    machines: MachineOeeData[], 
    aggregation_method: AggregationMethod
  ): Promise<ApiResponse<{ analysis: SystemOeeAnalysis }>> {
    return this.post<{ analysis: SystemOeeAnalysis }>('/system/aggregate', { machines, aggregation_method });
  }

  /**
   * Compare System Aggregation Methods
   * Maps to: /system/compare-methods
   */
  async compareSystemMethods(machines: MachineOeeData[]): Promise<ApiResponse<SystemCompareMethodsResponse>> {
    return this.post<SystemCompareMethodsResponse>('/system/compare-methods', { machines });
  }

  /**
   * Parse error response from server
   */
  private async parseErrorResponse(response: Response): Promise<ApiError> {
    try {
      const data = await response.json();
      // Ensure we always provide a fallback 'message' for legacy UI components
      return {
        code: data.code || `HTTP_${response.status}`,
        message_key: data.message_key || 'api.error.unknown',
        params: data.params || {},
        message: data.message || data.message_key || response.statusText,
      };
    } catch {
      return {
        code: `HTTP_${response.status}`,
        message_key: 'api.error.parse_failed',
        message: response.statusText,
      };
    }
  }

  /**
   * Handle various error types
   */
  private handleError(error: unknown): ApiError {
    if (error instanceof Error) {
      if (error.name === 'AbortError') {
        return {
          code: 'TIMEOUT',
          message_key: 'api.error.timeout',
          message: 'Request timed out',
        };
      }
      return {
        code: 'NETWORK_ERROR',
        message_key: 'api.error.network',
        message: error.message,
      };
    }
    return {
      code: 'UNKNOWN_ERROR',
      message_key: 'api.error.unknown',
      message: 'An unknown error occurred',
    };
  }
}

export const oeeApi = new OeeApiClient();