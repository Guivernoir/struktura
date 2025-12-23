/**
 * useOee Hook
 * * Orchestrates OEE calculations using the full capabilities of the Rust backend.
 * Supports standard OEE, Economic Analysis, Sensitivity, and Leverage.
 */

import { useState, useCallback, useEffect, useRef } from 'react';
import type { OeeInput, OeeResult, EconomicParameters } from './models';
import { 
  oeeApi, 
  type ApiError, 
  type SensitivityAnalysis, 
  type TemporalScrapAnalysis, 
  type CalculateFullResponse,
  type LeverageResponse
} from './api';

/**
 * Expanded Calculation State
 */
export type OeeCalculationState =
  | { status: 'idle' }
  | { status: 'loading' }
  | { 
      status: 'success'; 
      result: OeeResult; 
      sensitivity?: SensitivityAnalysis;
      temporalScrap?: TemporalScrapAnalysis;
      leverage?: LeverageResponse;
    }
  | { status: 'error'; error: ApiError };

export interface UseOeeState {
  // Inputs
  input: OeeInput | null;
  economicParams: EconomicParameters | null;
  
  // Configuration
  config: {
    includeSensitivity: boolean;
    includeTemporalScrap: boolean;
    includeLeverage: boolean;
    sensitivityVariation: number;
  };

  // State
  calculation: OeeCalculationState;
  isDirty: boolean;
  lastCalculatedAt: Date | null;
}

export interface UseOeeActions {
  // Input Management
  setInput: (input: OeeInput) => void;
  updateInput: (partial: Partial<OeeInput>) => void;
  setEconomicParams: (params: EconomicParameters | null) => void;
  
  // Configuration
  toggleSensitivity: (enabled: boolean) => void;
  toggleTemporalScrap: (enabled: boolean) => void;
  toggleLeverage: (enabled: boolean) => void;
  
  // Execution
  calculate: () => Promise<void>;
  reset: () => void;
}

export interface UseOeeReturn extends UseOeeState, UseOeeActions {
  hasResult: boolean;
  hasError: boolean;
  isLoading: boolean;
}

export interface UseOeeOptions {
  autoCalculate?: boolean; // Use with caution
  initialInput?: OeeInput;
  initialEconomicParams?: EconomicParameters;
  initialConfig?: {
    includeSensitivity?: boolean;
    includeTemporalScrap?: boolean;
    includeLeverage?: boolean;
    sensitivityVariation?: number;
  };
  onSuccess?: (data: CalculateFullResponse) => void;
  onError?: (error: ApiError) => void;
}

const DEFAULT_OPTIONS: UseOeeOptions = {
  autoCalculate: false,
  initialConfig: {
    includeSensitivity: true,
    includeTemporalScrap: false,
    includeLeverage: false,
    sensitivityVariation: 10.0
  }
};

export function useOee(options: UseOeeOptions = {}): UseOeeReturn {
  const opts = { ...DEFAULT_OPTIONS, ...options };
  const initialConfig = { ...DEFAULT_OPTIONS.initialConfig, ...options.initialConfig };

  // --- State ---
  const [input, setInputState] = useState<OeeInput | null>(opts.initialInput || null);
  const [economicParams, setEconomicParamsState] = useState<EconomicParameters | null>(opts.initialEconomicParams || null);
  
  const [config, setConfig] = useState({
    includeSensitivity: initialConfig.includeSensitivity ?? true,
    includeTemporalScrap: initialConfig.includeTemporalScrap ?? false,
    includeLeverage: initialConfig.includeLeverage ?? false,
    sensitivityVariation: initialConfig.sensitivityVariation ?? 10.0,
  });

  const [calculation, setCalculation] = useState<OeeCalculationState>({ status: 'idle' });
  const [isDirty, setIsDirty] = useState(false);
  const [lastCalculatedAt, setLastCalculatedAt] = useState<Date | null>(null);
  
  const lastInputRef = useRef<OeeInput | null>(input);

  // --- Actions ---

  const setInput = useCallback((newInput: OeeInput) => {
    setInputState(newInput);
    setIsDirty(true);
    lastInputRef.current = newInput;
  }, []);

  const updateInput = useCallback((partial: Partial<OeeInput>) => {
    setInputState(prev => {
      if (!prev) return null;
      const updated = { ...prev, ...partial };
      lastInputRef.current = updated;
      return updated;
    });
    setIsDirty(true);
  }, []);

  const setEconomicParams = useCallback((params: EconomicParameters | null) => {
    setEconomicParamsState(params);
    setIsDirty(true);
  }, []);

  const toggleSensitivity = useCallback((enabled: boolean) => {
    setConfig(prev => ({ ...prev, includeSensitivity: enabled }));
    setIsDirty(true);
  }, []);

  const toggleTemporalScrap = useCallback((enabled: boolean) => {
    setConfig(prev => ({ ...prev, includeTemporalScrap: enabled }));
    setIsDirty(true);
  }, []);

  const toggleLeverage = useCallback((enabled: boolean) => {
    setConfig(prev => ({ ...prev, includeLeverage: enabled }));
    setIsDirty(true);
  }, []);

  const reset = useCallback(() => {
    setInputState(null);
    setEconomicParamsState(null);
    setCalculation({ status: 'idle' });
    setIsDirty(false);
  }, []);

  /**
   * Primary Calculation Function
   * Uses /calculate-full to fetch all requested data in one go
   */
  const calculate = useCallback(async () => {
    if (!input) return;
    
    setCalculation({ status: 'loading' });
    
    try {
      // Use the full analysis endpoint
      const response = await oeeApi.calculateFull({
        input,
        economic_parameters: economicParams || undefined,
        include_sensitivity: config.includeSensitivity,
        include_temporal_scrap: config.includeTemporalScrap,
        sensitivity_variation: config.sensitivityVariation
      });
      
      if (response.success) {
        let leverage: LeverageResponse | undefined;
        if (config.includeLeverage) {
          const levRes = await oeeApi.analyzeLeverage(input);
          if (levRes.success) {
            leverage = levRes.data;
          }
          // If error, ignore for now
        }

        setCalculation({ 
          status: 'success', 
          result: response.data.result,
          sensitivity: response.data.sensitivity_analysis,
          temporalScrap: response.data.temporal_scrap_analysis,
          leverage
        });
        setLastCalculatedAt(new Date());
        setIsDirty(false);
        opts.onSuccess?.(response.data);
      } else {
        setCalculation({ status: 'error', error: response.error });
        opts.onError?.(response.error);
      }
    } catch (error) {
      const apiError: ApiError = {
        code: 'UNEXPECTED_ERROR',
        message_key: 'api.error.unexpected',
        message: error instanceof Error ? error.message : 'Calculation failed',
      };
      setCalculation({ status: 'error', error: apiError });
      opts.onError?.(apiError);
    }
  }, [input, economicParams, config, opts]);

  // --- Effects ---

  // Auto-calculate effect
  useEffect(() => {
    if (opts.autoCalculate && input && isDirty) {
      const timeoutId = setTimeout(() => {
        calculate();
      }, 500); // Debounce auto-calculation
      return () => clearTimeout(timeoutId);
    }
  }, [input, economicParams, opts.autoCalculate, isDirty, calculate]);

  return {
    // State
    input,
    economicParams,
    config,
    calculation,
    isDirty,
    lastCalculatedAt,

    // Actions
    setInput,
    updateInput,
    setEconomicParams,
    toggleSensitivity,
    toggleTemporalScrap,
    toggleLeverage,
    calculate,
    reset,

    // Derived flags
    hasResult: calculation.status === 'success',
    hasError: calculation.status === 'error',
    isLoading: calculation.status === 'loading',
  };
}