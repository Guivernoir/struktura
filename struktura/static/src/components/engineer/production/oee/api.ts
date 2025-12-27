/**
 * OEE Calculator API Client
 * 
 * Handles communication with the Rust backend calculation engine.
 * All type definitions imported from models - no structure duplication.
 */

import type {
  OeeInput,
  ApiError,
  CalculateRequest,
  CalculateResponse,
  CalculateWithEconomicsRequest,
  CalculateFullRequest,
  CalculateFullResponse,
  SensitivityRequest,
  SensitivityResponse,
  LeverageRequest,
  LeverageResponse,
  TemporalScrapRequest,
  TemporalScrapResponse,
  SystemAggregateRequest,
  SystemAggregateResponse,
  SystemCompareMethodsRequest,
  SystemCompareMethodsResponse,
  EconomicParameters,
  AggregationMethod,
  MachineOeeData,
} from './models';

/**
 * Generic API response wrapper for type safety
 */
export type ApiResponse<T> = 
  | { success: true; data: T }
  | { success: false; error: ApiError };

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
   * Maps to: POST /calculate-full
   */
  async calculateFull(request: CalculateFullRequest): Promise<ApiResponse<CalculateFullResponse>> {
    return this.post<CalculateFullResponse>('/calculate-full', request);
  }

  /**
   * Basic Calculate
   * Maps to: POST /calculate
   */
  async calculate(input: OeeInput): Promise<ApiResponse<CalculateResponse>> {
    return this.post<CalculateResponse>('/calculate', { input } satisfies CalculateRequest);
  }

  /**
   * Calculate with Economics
   * Maps to: POST /calculate-with-economics
   */
  async calculateWithEconomics(
    input: OeeInput, 
    economicParameters: EconomicParameters
  ): Promise<ApiResponse<CalculateResponse>> {
    return this.post<CalculateResponse>('/calculate-with-economics', {
      input,
      economicParameters
    } satisfies CalculateWithEconomicsRequest);
  }

  /**
   * Analyze Leverage
   * Maps to: POST /leverage
   */
  async analyzeLeverage(input: OeeInput): Promise<ApiResponse<LeverageResponse>> {
    return this.post<LeverageResponse>('/leverage', { input } satisfies LeverageRequest);
  }

  /**
   * Analyze Sensitivity
   * Maps to: POST /sensitivity
   */
  async analyzeSensitivity(
    input: OeeInput, 
    variationPercent: number = 10.0
  ): Promise<ApiResponse<SensitivityResponse>> {
    return this.post<SensitivityResponse>('/sensitivity', {
      input,
      variationPercent
    } satisfies SensitivityRequest);
  }

  /**
   * Analyze Temporal Scrap
   * Maps to: POST /temporal-scrap
   */
  async analyzeTemporalScrap(request: TemporalScrapRequest): Promise<ApiResponse<TemporalScrapResponse>> {
    return this.post<TemporalScrapResponse>('/temporal-scrap', request);
  }

  /**
   * System Aggregate OEE
   * Maps to: POST /system/aggregate
   */
  async aggregateSystemOee(
    machines: MachineOeeData[], 
    aggregationMethod: AggregationMethod
  ): Promise<ApiResponse<SystemAggregateResponse>> {
    return this.post<SystemAggregateResponse>('/system/aggregate', {
      machines,
      aggregationMethod
    } satisfies SystemAggregateRequest);
  }

  /**
   * Compare System Aggregation Methods
   * Maps to: POST /system/compare-methods
   */
  async compareSystemMethods(machines: MachineOeeData[]): Promise<ApiResponse<SystemCompareMethodsResponse>> {
    return this.post<SystemCompareMethodsResponse>('/system/compare-methods', {
      machines
    } satisfies SystemCompareMethodsRequest);
  }

  /**
   * Parse error response from server
   */
  private async parseErrorResponse(response: Response): Promise<ApiError> {
    try {
      const data = await response.json();
      return {
        code: data.code || `HTTP_${response.status}`,
        messageKey: data.message_key || data.messageKey || 'api.error.unknown',
        params: data.params || {},
      };
    } catch {
      return {
        code: `HTTP_${response.status}`,
        messageKey: 'api.error.parse_failed',
        params: { statusText: response.statusText },
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
          messageKey: 'api.error.timeout',
          params: {},
        };
      }
      return {
        code: 'NETWORK_ERROR',
        messageKey: 'api.error.network',
        params: { message: error.message },
      };
    }
    return {
      code: 'UNKNOWN_ERROR',
      messageKey: 'api.error.unknown',
      params: {},
    };
  }
}

/**
 * Default API client instance
 */
export const oeeApi = new OeeApiClient();