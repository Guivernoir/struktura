/**
 * OEE Calculator Engine - Root Orchestration Component
 *
 * This component:
 * - Holds local OEE state
 * - Coordinates views
 * - Emits final results upward
 *
 * This component does NOT:
 * - Render detailed UI
 * - Contain formulas
 * - Know domain rules
 */

import React, { useState } from "react";
import { useOee } from "./useOee";
import type { OeeInput, EconomicParameters } from "./models";

// Views
import AssumptionsView from "./views/AssumptionsView";
import ResultsView from "./views/ResultsView";
import TraceabilityView from "./views/TraceabilityView";

// Styles
import "./styles/index.css";

export interface OeeEngineProps {
  /** Initial input data (optional) */
  initialInput?: OeeInput;
  /** Initial economic parameters (optional) */
  initialEconomicParams?: EconomicParameters;
  /** Callback when calculation completes */
  onCalculationComplete?: (result: any) => void;
  /** Callback on error */
  onError?: (error: any) => void;
}

type ViewMode = "assumptions" | "results" | "traceability";

export const OeeEngine: React.FC<OeeEngineProps> = ({
  initialInput,
  initialEconomicParams,
  onCalculationComplete,
  onError,
}) => {
  const [activeView, setActiveView] = useState<ViewMode>("assumptions");

  const oee = useOee({
    initialInput,
    initialEconomicParams,
    onSuccess: (data) => {
      setActiveView("results");
      onCalculationComplete?.(data);
    },
    onError: (error) => {
      onError?.(error);
    },
  });

  return (
    <div className="oee-engine min-h-screen bg-sand-50 dark:bg-charcoal-950">
      {/* Header */}
      <header className="bg-white dark:bg-charcoal-900 border-b border-charcoal-200 dark:border-charcoal-700 sticky top-0 z-20">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-display font-semibold text-charcoal-900 dark:text-charcoal-50">
                OEE Engineering Calculator
              </h1>
              <p className="text-sm text-charcoal-600 dark:text-charcoal-400 mt-1">
                Deterministic · Assumption-Driven · Analyst-First
              </p>
            </div>

            {/* Action Buttons */}
            <div className="flex items-center gap-3">
              {oee.isDirty && (
                <span className="text-sm text-steel-600 dark:text-steel-400">
                  Unsaved changes
                </span>
              )}

              <button
                onClick={oee.reset}
                disabled={!oee.input}
                className="px-4 py-2 text-sm font-medium text-charcoal-700 dark:text-charcoal-300 
                         hover:bg-charcoal-100 dark:hover:bg-charcoal-800 rounded-md 
                         disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                Reset
              </button>

              <button
                onClick={oee.calculate}
                disabled={!oee.input || oee.isLoading}
                className="px-6 py-2 text-sm font-medium text-white bg-steel-600 
                         hover:bg-steel-700 rounded-md disabled:opacity-50 
                         disabled:cursor-not-allowed transition-colors flex items-center gap-2"
              >
                {oee.isLoading ? (
                  <>
                    <div className="oee-spinner w-4 h-4" />
                    Calculating...
                  </>
                ) : (
                  "Calculate OEE"
                )}
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Navigation Tabs */}
      <nav className="bg-white dark:bg-charcoal-900 border-b border-charcoal-200 dark:border-charcoal-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex space-x-8">
            <button
              onClick={() => setActiveView("assumptions")}
              className={`
                py-4 px-1 border-b-2 font-medium text-sm transition-colors
                ${
                  activeView === "assumptions"
                    ? "border-steel-600 text-steel-600 dark:text-steel-400"
                    : "border-transparent text-charcoal-600 dark:text-charcoal-400 hover:text-charcoal-900 dark:hover:text-charcoal-200"
                }
              `}
            >
              Assumptions
            </button>

            <button
              onClick={() => setActiveView("results")}
              disabled={!oee.hasResult}
              className={`
                py-4 px-1 border-b-2 font-medium text-sm transition-colors
                disabled:opacity-50 disabled:cursor-not-allowed
                ${
                  activeView === "results"
                    ? "border-steel-600 text-steel-600 dark:text-steel-400"
                    : "border-transparent text-charcoal-600 dark:text-charcoal-400 hover:text-charcoal-900 dark:hover:text-charcoal-200"
                }
              `}
            >
              Results
              {oee.hasResult && (
                <span className="ml-2 inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-green-100 text-green-800">
                  Ready
                </span>
              )}
            </button>

            <button
              onClick={() => setActiveView("traceability")}
              disabled={!oee.hasResult}
              className={`
                py-4 px-1 border-b-2 font-medium text-sm transition-colors
                disabled:opacity-50 disabled:cursor-not-allowed
                ${
                  activeView === "traceability"
                    ? "border-steel-600 text-steel-600 dark:text-steel-400"
                    : "border-transparent text-charcoal-600 dark:text-charcoal-400 hover:text-charcoal-900 dark:hover:text-charcoal-200"
                }
              `}
            >
              Traceability
            </button>
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Error Display */}
        {oee.hasError && oee.calculation.status === "error" && (
          <div className="mb-6 validation-error">
            <div className="flex items-start">
              <div className="flex-shrink-0">
                <svg
                  className="h-5 w-5 text-red-600"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fillRule="evenodd"
                    d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                    clipRule="evenodd"
                  />
                </svg>
              </div>
              <div className="ml-3">
                <h3 className="text-sm font-medium text-red-800">
                  Calculation Error
                </h3>
                <div className="mt-2 text-sm text-red-700">
                  <p>
                    {oee.calculation.error.message ||
                      "An unexpected error occurred"}
                  </p>
                  <p className="mt-1 text-xs text-red-600">
                    Code: {oee.calculation.error.code}
                  </p>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* View Content */}
        <div className="animate-fade-in">
          {activeView === "assumptions" && (
            <AssumptionsView
              input={oee.input}
              economicParams={oee.economicParams}
              config={oee.config}
              onInputChange={oee.setInput}
              onEconomicParamsChange={oee.setEconomicParams}
              onToggleSensitivity={oee.toggleSensitivity}
              onToggleLeverage={oee.toggleLeverage}
            />
          )}

          {activeView === "results" &&
            oee.hasResult &&
            oee.calculation.status === "success" && (
              <ResultsView
                result={oee.calculation.result}
                sensitivity={oee.calculation.sensitivity}
                leverage={oee.calculation.leverage}
              />
            )}

          {activeView === "traceability" &&
            oee.hasResult &&
            oee.calculation.status === "success" && (
              <TraceabilityView
                result={oee.calculation.result}
                input={oee.input}
              />
            )}
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-white dark:bg-charcoal-900 border-t border-charcoal-200 dark:border-charcoal-700 mt-12">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="text-center text-sm text-charcoal-600 dark:text-charcoal-400">
            <p className="font-medium mb-2">
              ⚠️ This is a deterministic calculator, not a system of record
            </p>
            <p className="text-xs">
              Outputs are strictly mathematical consequences of your
              assumptions. Accuracy scales linearly with input accuracy.
            </p>
            {oee.lastCalculatedAt && (
              <p className="text-xs mt-2">
                Last calculated: {oee.lastCalculatedAt.toLocaleString()}
              </p>
            )}
          </div>
        </div>
      </footer>
    </div>
  );
};

export default OeeEngine;
