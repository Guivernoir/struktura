/**
 * MetricCard Component
 *
 * Single metric display with unit, confidence context, and formula drill-down.
 * Includes direct formula inspection capability.
 */

import React, { useState } from "react";
import FormulaPopover from "./FormulaPopover";
import type { TrackedMetric } from "../../models";
import { formatPercentage } from "../../utils";

export interface MetricCardProps {
  metric: TrackedMetric;
  color?: "blue" | "purple" | "green" | "yellow" | "red";
  icon?: string;
  showFormula?: boolean;
}

const MetricCard: React.FC<MetricCardProps> = ({
  metric,
  color = "blue",
  icon,
  showFormula = true,
}) => {
  const [isFormulaOpen, setIsFormulaOpen] = useState(false);

  const colorClasses = {
    blue: "border-blue-400 text-blue-600",
    purple: "border-purple-400 text-purple-600",
    green: "border-green-400 text-green-600",
    yellow: "border-yellow-400 text-yellow-600",
    red: "border-red-400 text-red-600",
  };

  return (
    <div className="oee-metric-card relative">
      {/* Color Indicator */}
      <div
        className={`absolute left-0 top-0 bottom-0 w-1 ${colorClasses[
          color
        ].replace("text-", "bg-")}`}
      />

      <div className="pl-4">
        {/* Header */}
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-2">
            {icon && <span className="text-2xl">{icon}</span>}
            <span className="metric-label">
              {metric.name_key.replace("metrics.", "")}
            </span>
          </div>
          <div
            className={`confidence-badge confidence-badge--${metric.confidence.toLowerCase()}`}
          >
            {metric.confidence}
          </div>
        </div>

        {/* Value */}
        <div className={`metric-value ${colorClasses[color]}`}>
          {metric.unit_key === "units.percentage"
            ? formatPercentage(metric.value)
            : `${metric.value.toFixed(2)} ${metric.unit_key.replace(
                "units.",
                ""
              )}`}
        </div>

        {/* Formula Button */}
        {showFormula && (
          <button
            onClick={() => setIsFormulaOpen(true)}
            className="mt-3 text-xs text-steel-600 dark:text-steel-400 hover:underline flex items-center gap-1"
          >
            <svg
              className="w-3 h-3"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            View formula
          </button>
        )}
      </div>

      {/* Formula Popover */}
      {isFormulaOpen && (
        <FormulaPopover
          metric={metric}
          onClose={() => setIsFormulaOpen(false)}
        />
      )}
    </div>
  );
};

export default MetricCard;
