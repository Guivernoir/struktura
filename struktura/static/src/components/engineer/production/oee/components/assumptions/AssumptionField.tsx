/**
 * AssumptionField Component
 *
 * A single assumption input with metadata (source, impact, related assumptions).
 * This component must remain valid even if the calculation engine is removed entirely.
 */

import React from "react";
import AssumptionSourceBadge from "./AssumptionSourceBadge";
import type { InputValue, ImpactLevel } from "../../models";
import { InputValueHelpers } from "../../models";

export interface AssumptionFieldProps {
  label: string;
  value: InputValue<any>;
  unit?: string;
  helpText?: string;
  impact?: ImpactLevel;
  relatedAssumptions?: string[];
  disabled?: boolean;
  onChange?: (value: any) => void;
  type?: "number" | "text" | "duration";
  step?: number;
  min?: number;
  max?: number;
}

const AssumptionField: React.FC<AssumptionFieldProps> = ({
  label,
  value,
  unit,
  helpText,
  impact,
  relatedAssumptions = [],
  disabled = false,
  onChange,
  type = "number",
  step,
  min,
  max,
}) => {
  const actualValue = InputValueHelpers.getValue(value);
  const source = InputValueHelpers.sourceType(value);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!onChange) return;

    let newValue: any = e.target.value;
    if (type === "number" || type === "duration") {
      newValue = parseFloat(e.target.value) || 0;
    }
    onChange(newValue);
  };

  return (
    <div className="assumption-field">
      <div className="flex items-start justify-between mb-2">
        <div className="flex-1">
          <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300">
            {label}
            {unit && (
              <span className="text-charcoal-500 dark:text-charcoal-400 ml-1">
                ({unit})
              </span>
            )}
          </label>
          {helpText && (
            <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
              {helpText}
            </p>
          )}
        </div>
        <AssumptionSourceBadge source={source as any} />
      </div>

      <input
        type={type === "duration" ? "number" : type}
        value={actualValue}
        onChange={handleChange}
        disabled={disabled}
        step={step}
        min={min}
        max={max}
        className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                 rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100
                 disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none 
                 focus:ring-2 focus:ring-steel-500 transition-colors"
      />

      {/* Impact & Related Info */}
      {(impact || relatedAssumptions.length > 0) && (
        <div className="mt-2 flex items-center gap-2 flex-wrap">
          {impact && (
            <span
              className={`text-xs px-2 py-1 rounded-full ${
                impact === "Critical"
                  ? "bg-red-100 dark:bg-red-900/20 text-red-700 dark:text-red-400"
                  : impact === "High"
                  ? "bg-orange-100 dark:bg-orange-900/20 text-orange-700 dark:text-orange-400"
                  : impact === "Medium"
                  ? "bg-yellow-100 dark:bg-yellow-900/20 text-yellow-700 dark:text-yellow-400"
                  : "bg-blue-100 dark:bg-blue-900/20 text-blue-700 dark:text-blue-400"
              }`}
            >
              Impact: {impact}
            </span>
          )}
          {relatedAssumptions.length > 0 && (
            <span className="text-xs text-charcoal-500 dark:text-charcoal-400">
              Affects {relatedAssumptions.length} other assumption
              {relatedAssumptions.length !== 1 ? "s" : ""}
            </span>
          )}
        </div>
      )}
    </div>
  );
};

export default AssumptionField;
