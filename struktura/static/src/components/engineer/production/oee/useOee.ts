/**
 * useOee Hook
 * 
 * Orchestrates OEE calculations using the full capabilities of the Rust backend.
 * All domain types imported from models - only UI state types defined here.
 */

import { useState, useCallback, useEffect, useRef } from 'react';
import type {
  OeeInput,
  OeeResult,
  EconomicParameters,
  ApiError,
  SensitivityAnalysis,
  TemporalScrapAnalysis,
  CalculateFullResponse,
  LeverageResponse,
} from './models';
import { oeeApi } from './api';

/**
 * UI-specific calculation state
 * (Not a domain type - pure UI state management)
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

/**
 * Hook state interface
 */
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

/**
 * Hook actions interface
 */
export interface UseOeeActions {
  // Input Management
  setInput: (input: OeeInput) => void;
  updateInput: (partial: Partial<OeeInput>) => void;
  setEconomicParams: (params: EconomicParameters | null) => void;
  
  // Configuration
  toggleSensitivity: (enabled: boolean) => void;
  toggleTemporalScrap: (enabled: boolean) => void;
  toggleLeverage: (enabled: boolean) => void;
  setSensitivityVariation: (percent: number) => void;
  
  // Execution
  calculate: () => Promise<void>;
  reset: () => void;
}

/**
 * Complete hook return type
 */
export interface UseOeeReturn extends UseOeeState, UseOeeActions {
  hasResult: boolean;
  hasError: boolean;
  isLoading: boolean;
}

/**
 * Hook configuration options
 */
export interface UseOeeOptions {
  /** Auto-calculate on input change (use with caution) */
  autoCalculate?: boolean;
  /** Initial input data */
  initialInput?: OeeInput;
  /** Initial economic parameters */
  initialEconomicParams?: EconomicParameters;
  /** Initial configuration */
  initialConfig?: {
    includeSensitivity?: boolean;
    includeTemporalScrap?: boolean;
    includeLeverage?: boolean;
    sensitivityVariation?: number;
  };
  /** Success callback */
  onSuccess?: (data: CalculateFullResponse) => void;
  /** Error callback */
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

/**
 * OEE Calculator Hook
 * 
 * Manages OEE calculation state and orchestrates API calls.
 * Supports full analysis including sensitivity, leverage, and temporal scrap.
 */
export function useOee(options: UseOeeOptions = {}): UseOeeReturn {
  const opts = { ...DEFAULT_OPTIONS, ...options };
  const initialConfig = { ...DEFAULT_OPTIONS.initialConfig, ...options.initialConfig };

  // --- State ---
  const [input, setInputState] = useState<OeeInput | null>(opts.initialInput || null);
  const [economicParams, setEconomicParamsState] = useState<EconomicParameters | null>(
    opts.initialEconomicParams || null
  );
  
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

  const setSensitivityVariation = useCallback((percent: number) => {
    setConfig(prev => ({ ...prev, sensitivityVariation: percent }));
    setIsDirty(true);
  }, []);

  const reset = useCallback(() => {
    setInputState(null);
    setEconomicParamsState(null);
    setCalculation({ status: 'idle' });
    setIsDirty(false);
    setLastCalculatedAt(null);
  }, []);

  /**
   * Primary Calculation Function
   * 
   * Uses /calculate-full for comprehensive analysis,
   * then makes additional calls for leverage if requested.
   */
  const calculate = useCallback(async () => {
    if (!input) return;
    
    setCalculation({ status: 'loading' });
    
    try {
      // Primary calculation with sensitivity and temporal scrap
      const response = await oeeApi.calculateFull({
        input,
        economicParameters: economicParams || undefined,
        includeSensitivity: config.includeSensitivity,
        includeTemporalScrap: config.includeTemporalScrap,
        sensitivityVariation: config.sensitivityVariation
      });
      
      if (!response.success) {
        setCalculation({ status: 'error', error: response.error });
        opts.onError?.(response.error);
        return;
      }

      // Additional leverage analysis if requested
      let leverage: LeverageResponse | undefined;
      if (config.includeLeverage) {
        const levResponse = await oeeApi.analyzeLeverage(input);
        if (levResponse.success) {
          leverage = levResponse.data;
        }
        // Ignore leverage errors - don't fail the whole calculation
      }

      setCalculation({ 
        status: 'success', 
        result: response.data.result,
        sensitivity: response.data.sensitivityAnalysis,
        temporalScrap: response.data.temporalScrapAnalysis,
        leverage
      });
      
      setLastCalculatedAt(new Date());
      setIsDirty(false);
      opts.onSuccess?.(response.data);
      
    } catch (error) {
      const apiError: ApiError = {
        code: 'UNEXPECTED_ERROR',
        messageKey: 'api.error.unexpected',
        params: { message: error instanceof Error ? error.message : 'Unknown error' },
      };
      setCalculation({ status: 'error', error: apiError });
      opts.onError?.(apiError);
    }
  }, [input, economicParams, config, opts]);

  // --- Effects ---

  /**
   * Auto-calculate effect (debounced)
   */
  useEffect(() => {
    if (opts.autoCalculate && input && isDirty) {
      const timeoutId = setTimeout(() => {
        calculate();
      }, 500);
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
    setSensitivityVariation,
    calculate,
    reset,

    // Derived flags
    hasResult: calculation.status === 'success',
    hasError: calculation.status === 'error',
    isLoading: calculation.status === 'loading',
  };
}