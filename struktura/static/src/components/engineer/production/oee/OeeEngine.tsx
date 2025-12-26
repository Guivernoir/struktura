/**
 * OeeEngine Component
 *
 * Root orchestration component for the OEE calculator.
 * Coordinates views, manages state, and emits results.
 *
 * CRITICAL: This is orchestration only. No detailed UI. No formulas. No domain rules.
 * If this file grows teeth, something upstream failed.
 */

import React, { useState, useEffect } from "react";
import AssumptionsView from "./views/AssumptionsView";
import ResultsView from "./views/ResultsView";
import TraceabilityView from "./views/TraceabilityView";
import { useOee, type UseOeeOptions } from "./useOee";
import type { OeeInput, OeeResult, EconomicParameters } from "./models";
import "./styles/index.css";

export interface OeeEngineProps {
  /**
   * Initial input data (optional)
   */
  initialInput?: OeeInput;

  /**
   * Initial economic parameters (optional)
   */
  initialEconomicParams?: EconomicParameters;

  /**
   * Callback when calculation completes successfully
   */
  onResultsReady?: (result: OeeResult) => void;

  /**
   * Callback when input changes (for external persistence)
   */
  onInputChange?: (input: OeeInput) => void;

  /**
   * Enable/disable auto-calculation on input change
   */
  autoCalculate?: boolean;

  /**
   * Custom CSS class for the container
   */
  className?: string;
}

type ViewMode = "assumptions" | "results" | "traceability";

const OeeEngine: React.FC<OeeEngineProps> = ({
  initialInput,
  initialEconomicParams,
  onResultsReady,
  onInputChange,
  autoCalculate = false,
  className = "",
}) => {
  const [activeView, setActiveView] = useState<ViewMode>("assumptions");

  // Initialize OEE state with useOee hook
  const oeeOptions: UseOeeOptions = {
    initialInput,
    initialEconomicParams,
    autoCalculate,
    initialConfig: {
      includeSensitivity: true,
      includeTemporalScrap: false,
      includeLeverage: true,
      sensitivityVariation: 10.0,
    },
    onSuccess: (data) => {
      // Notify parent component
      if (onResultsReady) {
        onResultsReady(data.result);
      }
      // Auto-switch to results view on successful calculation
      setActiveView("results");
    },
    onError: (error) => {
      console.error("OEE Calculation Error:", error);
      // Could show error toast/notification here
    },
  };

  const {
    input,
    calculation,
    config,
    setInput,
    calculate,
    reset,
    toggleSensitivity,
    toggleLeverage,
    hasResult,
    isLoading,
  } = useOee(oeeOptions);

  // Notify parent of input changes
  useEffect(() => {
    if (input && onInputChange) {
      onInputChange(input);
    }
  }, [input, onInputChange]);

  // Handle input changes from AssumptionsView
  const handleInputChange = (newInput: OeeInput) => {
    setInput(newInput);
  };

  // Handle calculation trigger
  const handleCalculate = () => {
    if (input) {
      calculate();
    }
  };

  // Handle view switching
  const switchView = (view: ViewMode) => {
    // Prevent switching to results/traceability if no results yet
    if ((view === "results" || view === "traceability") && !hasResult) {
      return;
    }
    setActiveView(view);
  };

  // Get result data if available
  const result = calculation.status === "success" ? calculation.result : null;
  const leverage =
    calculation.status === "success" ? calculation.leverage : undefined;
  const sensitivity =
    calculation.status === "success" ? calculation.sensitivity : undefined;

  return (
    <div className={`oee-engine ${className}`}>
      {/* Header */}
      <div className="bg-gradient-to-br from-steel-50 to-sand-50 dark:from-charcoal-900 dark:to-charcoal-950 border-b border-charcoal-200 dark:border-charcoal-700 p-6">
        <div className="max-w-7xl mx-auto">
          <div className="flex items-center justify-between flex-wrap gap-4">
            <div>
              <h1 className="text-3xl font-display font-bold text-charcoal-900 dark:text-charcoal-50">
                OEE Engineering Calculator
              </h1>
              <p className="text-sm text-charcoal-600 dark:text-charcoal-400 mt-1">
                Deterministic. Assumption-Driven. Analyst-First.
              </p>
            </div>

            <div className="flex items-center gap-3">
              {/* Configuration Toggles */}
              <div className="flex items-center gap-2 px-3 py-2 bg-white dark:bg-charcoal-800 rounded-lg shadow-soft">
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={config.includeSensitivity}
                    onChange={(e) => toggleSensitivity(e.target.checked)}
                    className="rounded"
                  />
                  <span className="text-xs font-medium">Sensitivity</span>
                </label>
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={config.includeLeverage}
                    onChange={(e) => toggleLeverage(e.target.checked)}
                    className="rounded"
                  />
                  <span className="text-xs font-medium">Leverage</span>
                </label>
              </div>

              {/* Reset Button */}
              {input && (
                <button
                  onClick={reset}
                  className="px-4 py-2 bg-red-100 hover:bg-red-200 dark:bg-red-900/20 dark:hover:bg-red-900/30 
                           text-red-700 dark:text-red-400 rounded-lg font-medium text-sm transition-colors"
                >
                  Reset
                </button>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* View Navigation */}
      <div className="bg-white dark:bg-charcoal-900 border-b border-charcoal-200 dark:border-charcoal-700">
        <div className="max-w-7xl mx-auto px-6">
          <div className="flex gap-1">
            <button
              onClick={() => switchView("assumptions")}
              disabled={!input}
              className={`px-6 py-3 font-semibold text-sm transition-colors relative
                ${
                  activeView === "assumptions"
                    ? "text-steel-700 dark:text-steel-300"
                    : "text-charcoal-500 dark:text-charcoal-400 hover:text-charcoal-700 dark:hover:text-charcoal-200"
                }
                ${!input ? "opacity-50 cursor-not-allowed" : ""}
              `}
            >
              Assumptions
              {activeView === "assumptions" && (
                <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-steel-600" />
              )}
            </button>

            <button
              onClick={() => switchView("results")}
              disabled={!hasResult}
              className={`px-6 py-3 font-semibold text-sm transition-colors relative
                ${
                  activeView === "results"
                    ? "text-steel-700 dark:text-steel-300"
                    : "text-charcoal-500 dark:text-charcoal-400 hover:text-charcoal-700 dark:hover:text-charcoal-200"
                }
                ${!hasResult ? "opacity-50 cursor-not-allowed" : ""}
              `}
            >
              Results
              {activeView === "results" && (
                <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-steel-600" />
              )}
              {hasResult && (
                <span className="ml-2 inline-flex items-center justify-center w-2 h-2 bg-green-500 rounded-full" />
              )}
            </button>

            <button
              onClick={() => switchView("traceability")}
              disabled={!hasResult}
              className={`px-6 py-3 font-semibold text-sm transition-colors relative
                ${
                  activeView === "traceability"
                    ? "text-steel-700 dark:text-steel-300"
                    : "text-charcoal-500 dark:text-charcoal-400 hover:text-charcoal-700 dark:hover:text-charcoal-200"
                }
                ${!hasResult ? "opacity-50 cursor-not-allowed" : ""}
              `}
            >
              Traceability
              {activeView === "traceability" && (
                <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-steel-600" />
              )}
            </button>
          </div>
        </div>
      </div>

      {/* Main Content Area */}
      <div className="max-w-7xl mx-auto px-6 py-8">
        {/* Loading Overlay */}
        {isLoading && (
          <div className="fixed inset-0 bg-black bg-opacity-30 z-50 flex items-center justify-center">
            <div className="bg-white dark:bg-charcoal-800 rounded-lg shadow-hard p-8 max-w-md">
              <div className="flex items-center gap-4">
                <div className="oee-spinner" />
                <div>
                  <p className="font-semibold text-charcoal-900 dark:text-charcoal-100">
                    Calculating OEE...
                  </p>
                  <p className="text-sm text-charcoal-600 dark:text-charcoal-400 mt-1">
                    Running deterministic analysis
                  </p>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Error Display */}
        {calculation.status === "error" && (
          <div className="mb-6 bg-red-50 dark:bg-red-900/20 border-l-4 border-red-400 p-4 rounded-md">
            <div className="flex items-start gap-3">
              <svg
                className="w-5 h-5 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5"
                fill="currentColor"
                viewBox="0 0 20 20"
              >
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                  clipRule="evenodd"
                />
              </svg>
              <div>
                <p className="text-sm font-semibold text-red-800 dark:text-red-400">
                  Calculation Error
                </p>
                <p className="text-sm text-red-700 dark:text-red-400 mt-1">
                  {calculation.error.message || calculation.error.message_key}
                </p>
                {calculation.error.code && (
                  <p className="text-xs text-red-600 dark:text-red-500 mt-2 font-mono">
                    Error Code: {calculation.error.code}
                  </p>
                )}
              </div>
            </div>
          </div>
        )}

        {/* View Content */}
        {!input && (
          <div className="text-center py-16">
            <div className="text-6xl mb-4">📊</div>
            <h3 className="text-xl font-display font-semibold text-charcoal-900 dark:text-charcoal-100 mb-2">
              No Input Data
            </h3>
            <p className="text-charcoal-600 dark:text-charcoal-400 mb-6">
              Initialize with OEE input data to begin analysis
            </p>
            <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-6 max-w-2xl mx-auto text-left">
              <p className="text-sm text-blue-700 dark:text-blue-400 mb-3">
                <strong>This component requires:</strong>
              </p>
              <ul className="list-disc list-inside text-sm text-blue-700 dark:text-blue-400 space-y-1 ml-4">
                <li>Time model (planned time + allocations)</li>
                <li>Production counts (total, good, scrap, rework)</li>
                <li>Cycle time (ideal + optional average)</li>
                <li>Machine context (IDs)</li>
                <li>Analysis window (start/end timestamps)</li>
              </ul>
              <p className="text-xs text-blue-600 dark:text-blue-500 mt-4 italic">
                See <code>models/input.ts</code> for complete structure
              </p>
            </div>
          </div>
        )}

        {input && activeView === "assumptions" && (
          <AssumptionsView
            input={input}
            validation={result?.validation || null}
            onChange={handleInputChange}
            onCalculate={handleCalculate}
            isCalculating={isLoading}
          />
        )}

        {result && activeView === "results" && (
          <ResultsView
            result={result}
            leverage={leverage}
            sensitivity={sensitivity}
            periodStart={input?.window.start}
            periodEnd={input?.window.end}
          />
        )}

        {result && activeView === "traceability" && (
          <TraceabilityView ledger={result.ledger} />
        )}
      </div>

      {/* Footer */}
      <div className="bg-sand-50 dark:bg-charcoal-900 border-t border-charcoal-200 dark:border-charcoal-700 py-4">
        <div className="max-w-7xl mx-auto px-6">
          <div className="flex items-center justify-between text-xs text-charcoal-500 dark:text-charcoal-400">
            <p>OEE Calculator v1.0 | Deterministic Analysis Engine</p>
            <p>
              {hasResult && (
                <span className="font-mono">
                  Last calculated:{" "}
                  {new Date(
                    result!.ledger.analysis_timestamp
                  ).toLocaleTimeString()}
                </span>
              )}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default OeeEngine;
