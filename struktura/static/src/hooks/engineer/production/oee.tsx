/**
 * OEE Calculator React Hooks
 * 
 * Strategic implementation using TanStack Query because:
 * 1. Automatic retry logic (networks lie)
 * 2. Request deduplication (users click twice)
 * 3. Background refetching (data goes stale)
 * 4. Optimistic updates (users are impatient)
 * 5. Cache invalidation (the hardest problem in CS, now solved)
 * 
 * These hooks are our tactical advantage in the UI battlefield.
 */

import { 
  useMutation, 
  useQueryClient,
  UseMutationOptions,
} from '@tanstack/react-query';
import { useCallback, useMemo } from 'react';
import {
  OeeApiClient,
  OeeInput,
  OeeResult,
  EconomicParameters,
  SensitivityAnalysis,
  LeverageImpact,
  TemporalScrapData,
  TemporalScrapAnalysis,
  StartupWindowConfig,
  MachineOeeData,
  AggregationMethod,
  SystemOeeAnalysis,
  OeeApiError,
  OeeValidationError,
} from 'lib/engineer/production/oee.ts';

// ============================================================================
// Query Keys - The Foundation of Proper Caching
// ============================================================================

/**
 * Query key factory
 * Centralized because changing query keys in multiple places is how bugs happen
 */
export const oeeQueryKeys = {
  all: ['oee'] as const,
  
  calculations: () => [...oeeQueryKeys.all, 'calculations'] as const,
  calculation: (inputHash: string) => [...oeeQueryKeys.calculations(), inputHash] as const,
  
  sensitivity: (inputHash: string, variation: number) => 
    [...oeeQueryKeys.all, 'sensitivity', inputHash, variation] as const,
    
  leverage: (inputHash: string) => 
    [...oeeQueryKeys.all, 'leverage', inputHash] as const,
    
  temporalScrap: (dataHash: string) => 
    [...oeeQueryKeys.all, 'temporal-scrap', dataHash] as const,
    
  systemAnalysis: (machineIds: string[], method: AggregationMethod) =>
    [...oeeQueryKeys.all, 'system', machineIds.sort().join(','), method] as const,
};

/**
 * Simple hash function for cache keys
 * Not cryptographic, just needs to be consistent
 */
function hashObject(obj: unknown): string {
  return btoa(JSON.stringify(obj)).slice(0, 16);
}

// ============================================================================
// Context & Provider - Dependency Injection, The Civilized Way
// ============================================================================

import { createContext, useContext, ReactNode } from 'react';

interface OeeApiContextValue {
  client: OeeApiClient;
}

const OeeApiContext = createContext<OeeApiContextValue | null>(null);

export interface OeeApiProviderProps {
  client: OeeApiClient;
  children: ReactNode;
}

/**
 * Provider for OEE API client
 * Wrap your app with this, or suffer prop drilling
 */
export function OeeApiProvider({ client, children }: OeeApiProviderProps) {
  const value = useMemo(() => ({ client }), [client]);
  return <OeeApiContext.Provider value={value}>{children}</OeeApiContext.Provider>;
}

/**
 * Get the OEE API client from context
 * Throws if used outside provider (fail fast, fail loud)
 */
function useOeeApiClient(): OeeApiClient {
  const context = useContext(OeeApiContext);
  if (!context) {
    throw new Error(
      'useOeeApiClient must be used within OeeApiProvider. ' +
      'Did you forget to wrap your app?'
    );
  }
  return context.client;
}

// ============================================================================
// Core Calculation Hooks
// ============================================================================

/**
 * Calculate OEE
 * 
 * @example
 * const { mutate, data, isLoading, error } = useCalculateOee();
 * mutate(oeeInput);
 */
export function useCalculateOee(
  options?: UseMutationOptions<OeeResult, OeeApiError, OeeInput>
) {
  const client = useOeeApiClient();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: OeeInput) => client.calculateOee(input),
    onSuccess: (data, variables) => {
      // Cache the result for potential reuse
      const key = oeeQueryKeys.calculation(hashObject(variables));
      queryClient.setQueryData(key, data);
    },
    ...options,
  });
}

/**
 * Calculate OEE with economic analysis
 * For when the CFO wants to see the money
 */
export function useCalculateOeeWithEconomics(
  options?: UseMutationOptions<
    OeeResult,
    OeeApiError,
    { input: OeeInput; economicParameters: EconomicParameters }
  >
) {
  const client = useOeeApiClient();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ input, economicParameters }) =>
      client.calculateOeeWithEconomics(input, economicParameters),
    onSuccess: (data, variables) => {
      const key = oeeQueryKeys.calculation(hashObject(variables));
      queryClient.setQueryData(key, data);
    },
    ...options,
  });
}

/**
 * Calculate OEE with all optional analyses
 * The full intelligence briefing
 */
export function useCalculateOeeFull(
  options?: UseMutationOptions<
    {
      result: OeeResult;
      sensitivity_analysis?: SensitivityAnalysis;
      temporal_scrap_analysis?: TemporalScrapAnalysis;
    },
    OeeApiError,
    {
      input: OeeInput;
      economicParameters?: EconomicParameters;
      includeSensitivity?: boolean;
      sensitivityVariation?: number;
      includeTemporalScrap?: boolean;
    }
  >
) {
  const client = useOeeApiClient();

  return useMutation({
    mutationFn: (params) => client.calculateOeeFull(params.input, params),
    ...options,
  });
}

// ============================================================================
// Analysis Hooks
// ============================================================================

/**
 * Analyze sensitivity
 * Find out which parameters actually matter
 */
export function useAnalyzeSensitivity(
  options?: UseMutationOptions<
    SensitivityAnalysis,
    OeeApiError,
    { input: OeeInput; variationPercent?: number }
  >
) {
  const client = useOeeApiClient();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ input, variationPercent = 10 }) =>
      client.analyzeSensitivity(input, variationPercent),
    onSuccess: (data, variables) => {
      const key = oeeQueryKeys.sensitivity(
        hashObject(variables.input),
        variables.variationPercent || 10
      );
      queryClient.setQueryData(key, data);
    },
    ...options,
  });
}

/**
 * Analyze leverage opportunities
 * Where to focus your improvement budget
 */
export function useAnalyzeLeverage(
  options?: UseMutationOptions<
    { leverage_impacts: LeverageImpact[]; baseline_oee: number },
    OeeApiError,
    OeeInput
  >
) {
  const client = useOeeApiClient();

  return useMutation({
    mutationFn: (input) => client.analyzeLeverage(input),
    ...options,
  });
}

/**
 * Analyze temporal scrap patterns
 * Startup vs steady-state - the eternal question
 */
export function useAnalyzeTemporalScrap(
  options?: UseMutationOptions<
    TemporalScrapAnalysis,
    OeeApiError,
    {
      scrapData: TemporalScrapData;
      idealCycleTime: number;
      startupConfig?: StartupWindowConfig;
    }
  >
) {
  const client = useOeeApiClient();

  return useMutation({
    mutationFn: ({ scrapData, idealCycleTime, startupConfig }) =>
      client.analyzeTemporalScrap(scrapData, idealCycleTime, startupConfig),
    ...options,
  });
}

// ============================================================================
// Multi-Machine System Hooks
// ============================================================================

/**
 * Aggregate system OEE
 * See the big picture
 */
export function useAggregateSystemOee(
  options?: UseMutationOptions<
    SystemOeeAnalysis,
    OeeApiError,
    { machines: MachineOeeData[]; aggregationMethod: AggregationMethod }
  >
) {
  const client = useOeeApiClient();

  return useMutation({
    mutationFn: ({ machines, aggregationMethod }) =>
      client.aggregateSystemOee(machines, aggregationMethod),
    ...options,
  });
}

/**
 * Compare aggregation methods
 * For when you need to justify your methodology
 */
export function useCompareAggregationMethods(
  options?: UseMutationOptions<
    {
      comparisons: Record<string, {
        method: string;
        system_oee: number;
        use_case: string;
      }>;
      recommended_method: string;
    },
    OeeApiError,
    MachineOeeData[]
  >
) {
  const client = useOeeApiClient();

  return useMutation({
    mutationFn: (machines) => client.compareAggregationMethods(machines),
    ...options,
  });
}

// ============================================================================
// Composite Hooks - Because DRY Is A Virtue
// ============================================================================

/**
 * Complete OEE analysis workflow
 * Calculate + Sensitivity + Leverage in one coordinated strike
 * 
 * @example
 * const { calculate, data, isLoading } = useCompleteOeeAnalysis();
 * 
 * await calculate({
 *   input: myOeeInput,
 *   runSensitivity: true,
 *   runLeverage: true,
 * });
 */
export function useCompleteOeeAnalysis() {
  const calculateOee = useCalculateOee();
  const analyzeSensitivity = useAnalyzeSensitivity();
  const analyzeLeverage = useAnalyzeLeverage();

  const calculate = useCallback(
    async (params: {
      input: OeeInput;
      economicParameters?: EconomicParameters;
      runSensitivity?: boolean;
      sensitivityVariation?: number;
      runLeverage?: boolean;
    }) => {
      // Step 1: Core calculation
      const oeeResult = params.economicParameters
        ? await calculateOee.mutateAsync(params.input)
        : await calculateOee.mutateAsync(params.input);

      // Step 2: Optional sensitivity analysis
      const sensitivity = params.runSensitivity
        ? await analyzeSensitivity.mutateAsync({
            input: params.input,
            variationPercent: params.sensitivityVariation,
          })
        : undefined;

      // Step 3: Optional leverage analysis
      const leverage = params.runLeverage
        ? await analyzeLeverage.mutateAsync(params.input)
        : undefined;

      return {
        oeeResult,
        sensitivity,
        leverage,
      };
    },
    [calculateOee, analyzeSensitivity, analyzeLeverage]
  );

  return {
    calculate,
    isLoading:
      calculateOee.isPending ||
      analyzeSensitivity.isPending ||
      analyzeLeverage.isPending,
    error: calculateOee.error || analyzeSensitivity.error || analyzeLeverage.error,
    data: {
      oeeResult: calculateOee.data,
      sensitivity: analyzeSensitivity.data,
      leverage: analyzeLeverage.data,
    },
  };
}

// ============================================================================
// Utility Hooks - The Supporting Cast
// ============================================================================

/**
 * Handle OEE validation errors with grace
 * Because users make mistakes and so do we
 */
export function useOeeValidationHandler() {
  return useCallback((error: unknown) => {
    if (error instanceof OeeValidationError) {
      // Extract validation issues for UI display
      return {
        isValidationError: true,
        issues: error.validation.issues,
        fatalErrors: error.validation.issues.filter((i: { severity: string }) => i.severity === 'Fatal'),
        warnings: error.validation.issues.filter((i: { severity: string }) => i.severity === 'Warning'),
        info: error.validation.issues.filter((i: { severity: string }) => i.severity === 'Info'),
      };
    }
    return {
      isValidationError: false,
      issues: [],
      fatalErrors: [],
      warnings: [],
      info: [],
    };
  }, []);
}

/**
 * Format OEE metrics for display
 * Because raw decimals are for machines, not humans
 */
export function useFormatOeeMetrics() {
  return useCallback((value: number, decimals = 1): string => {
    return `${(value * 100).toFixed(decimals)}%`;
  }, []);
}

/**
 * Check if OEE result meets target
 * Simple utility, but saves typing
 */
export function useCheckOeeTarget(target: number) {
  return useCallback(
    (oee: number): { meets: boolean; delta: number } => {
      const meets = oee >= target;
      const delta = oee - target;
      return { meets, delta };
    },
    [target]
  );
}

// ============================================================================
// Query Invalidation Helpers
// ============================================================================

/**
 * Invalidate all OEE calculations
 * For when you know the data has changed
 */
export function useInvalidateOeeCache() {
  const queryClient = useQueryClient();

  return useCallback(() => {
    queryClient.invalidateQueries({ queryKey: oeeQueryKeys.all });
  }, [queryClient]);
}

/**
 * Clear specific calculation from cache
 * Surgical precision when needed
 */
export function useClearOeeCalculation() {
  const queryClient = useQueryClient();

  return useCallback(
    (input: OeeInput) => {
      const key = oeeQueryKeys.calculation(hashObject(input));
      queryClient.removeQueries({ queryKey: key });
    },
    [queryClient]
  );
}

// ============================================================================
// Optimistic Update Helpers
// ============================================================================

/**
 * Hook for optimistic OEE updates
 * Make the UI feel fast even when the backend is slow
 * 
 * Use sparingly - optimistic updates with complex calculations
 * can backfire if your estimates are wrong
 */
export function useOptimisticOeeUpdate() {
  const queryClient = useQueryClient();

  return useCallback(
    (input: OeeInput, estimatedResult: OeeResult) => {
      const key = oeeQueryKeys.calculation(hashObject(input));
      queryClient.setQueryData(key, estimatedResult);
    },
    [queryClient]
  );
}

// ============================================================================
// Export All The Things
// ============================================================================

export type {
  OeeInput,
  OeeResult,
  EconomicParameters,
  SensitivityAnalysis,
  LeverageImpact,
  TemporalScrapData,
  TemporalScrapAnalysis,
  StartupWindowConfig,
  MachineOeeData,
  AggregationMethod,
  SystemOeeAnalysis,
  OeeApiError,
  OeeValidationError,
} from 'lib/engineer/production/oee';
