/**
 * AssumptionWarnings Component
 *
 * Non-blocking warnings tied to specific assumptions.
 * These explain concerns but never prevent computation.
 */

import React from "react";
import type { ValidationHint } from "../../utils";

export interface AssumptionWarningsProps {
  warnings: ValidationHint[];
  compact?: boolean;
}

const AssumptionWarnings: React.FC<AssumptionWarningsProps> = ({
  warnings,
  compact = false,
}) => {
  if (warnings.length === 0) return null;

  const criticalWarnings = warnings.filter((w) => w.severity === "critical");
  const regularWarnings = warnings.filter((w) => w.severity === "warning");
  const infoHints = warnings.filter((w) => w.severity === "info");

  return (
    <div className="space-y-2">
      {/* Critical Warnings */}
      {criticalWarnings.map((warning, idx) => (
        <div
          key={`critical-${idx}`}
          className="validation-error animate-slide-down"
        >
          <div className="flex items-start gap-2">
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
            <div className="flex-1">
              <p className="error-text font-semibold">{warning.message}</p>
              {!compact && warning.suggestion && (
                <p className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
                  💡 {warning.suggestion}
                </p>
              )}
              {!compact && warning.explanation && (
                <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1 italic">
                  {warning.explanation}
                </p>
              )}
            </div>
          </div>
        </div>
      ))}

      {/* Regular Warnings */}
      {regularWarnings.map((warning, idx) => (
        <div
          key={`warning-${idx}`}
          className="validation-warning animate-slide-down"
        >
          <div className="flex items-start gap-2">
            <svg
              className="w-5 h-5 text-yellow-600 dark:text-yellow-400 flex-shrink-0 mt-0.5"
              fill="currentColor"
              viewBox="0 0 20 20"
            >
              <path
                fillRule="evenodd"
                d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                clipRule="evenodd"
              />
            </svg>
            <div className="flex-1">
              <p className="warning-text">{warning.message}</p>
              {!compact && warning.suggestion && (
                <p className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
                  💡 {warning.suggestion}
                </p>
              )}
            </div>
          </div>
        </div>
      ))}

      {/* Info Hints */}
      {!compact &&
        infoHints.map((hint, idx) => (
          <div
            key={`info-${idx}`}
            className="validation-info animate-slide-down"
          >
            <div className="flex items-start gap-2">
              <svg
                className="w-5 h-5 text-blue-600 dark:text-blue-400 flex-shrink-0 mt-0.5"
                fill="currentColor"
                viewBox="0 0 20 20"
              >
                <path
                  fillRule="evenodd"
                  d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                  clipRule="evenodd"
                />
              </svg>
              <div className="flex-1">
                <p className="text-sm text-blue-700 dark:text-blue-400">
                  {hint.message}
                </p>
              </div>
            </div>
          </div>
        ))}
    </div>
  );
};

export default AssumptionWarnings;
