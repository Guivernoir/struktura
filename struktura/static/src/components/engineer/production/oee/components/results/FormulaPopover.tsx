/**
 * FormulaPopover Component
 *
 * Displays literal math + substituted values for a metric.
 * Shows exactly how the value was calculated.
 */

import React, { useEffect, useRef } from "react";
import type { TrackedMetric } from "../../models";

export interface FormulaPopoverProps {
  metric: TrackedMetric;
  onClose: () => void;
}

const FormulaPopover: React.FC<FormulaPopoverProps> = ({ metric, onClose }) => {
  const popoverRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        popoverRef.current &&
        !popoverRef.current.contains(event.target as Node)
      ) {
        onClose();
      }
    };

    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        onClose();
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    document.addEventListener("keydown", handleEscape);

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleEscape);
    };
  }, [onClose]);

  const renderFormula = () => {
    const formulaKey = metric.formula_key;
    const params = metric.formula_params;

    // Format formula with parameters
    let formula = formulaKey;
    Object.entries(params).forEach(([key, value]) => {
      formula = formula.replace(
        `{${key}}`,
        String(typeof value === "number" ? value.toFixed(2) : value)
      );
    });

    return formula;
  };

  return (
    <>
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black bg-opacity-30 z-40"
        onClick={onClose}
      />

      {/* Popover */}
      <div
        ref={popoverRef}
        className="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 
                   bg-white dark:bg-charcoal-800 rounded-lg shadow-hard p-6 max-w-lg w-full z-50
                   animate-slide-up"
      >
        {/* Header */}
        <div className="flex items-start justify-between mb-4">
          <div>
            <h3 className="text-lg font-display font-semibold text-charcoal-900 dark:text-charcoal-100">
              {metric.name_key.replace("metrics.", "")} Formula
            </h3>
            <p className="text-sm text-charcoal-600 dark:text-charcoal-400 mt-1">
              How this value was calculated
            </p>
          </div>
          <button
            onClick={onClose}
            className="text-charcoal-400 hover:text-charcoal-600 dark:hover:text-charcoal-200 transition-colors"
          >
            <svg
              className="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        {/* Formula */}
        <div className="space-y-4">
          <div>
            <div className="text-xs font-semibold text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mb-2">
              Formula Key
            </div>
            <code className="block font-mono text-sm bg-sand-100 dark:bg-charcoal-900 p-3 rounded border border-charcoal-200 dark:border-charcoal-700">
              {metric.formula_key}
            </code>
          </div>

          <div>
            <div className="text-xs font-semibold text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mb-2">
              Parameters
            </div>
            <div className="space-y-1">
              {Object.entries(metric.formula_params).map(([key, value]) => (
                <div
                  key={key}
                  className="flex items-center justify-between text-sm bg-sand-50 dark:bg-charcoal-900 px-3 py-2 rounded"
                >
                  <span className="font-medium text-charcoal-700 dark:text-charcoal-300">
                    {key}
                  </span>
                  <code className="font-mono text-steel-600 dark:text-steel-400">
                    {typeof value === "number"
                      ? value.toFixed(4)
                      : String(value)}
                  </code>
                </div>
              ))}
            </div>
          </div>

          <div>
            <div className="text-xs font-semibold text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mb-2">
              Substituted Formula
            </div>
            <div className="formula-display">{renderFormula()}</div>
          </div>

          <div>
            <div className="text-xs font-semibold text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mb-2">
              Result
            </div>
            <div className="text-2xl font-display font-bold text-charcoal-900 dark:text-charcoal-100">
              {metric.unit_key === "units.percentage"
                ? `${(metric.value * 100).toFixed(2)}%`
                : `${metric.value.toFixed(4)} ${metric.unit_key.replace(
                    "units.",
                    ""
                  )}`}
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default FormulaPopover;
