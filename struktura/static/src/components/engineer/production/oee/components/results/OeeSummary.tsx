/**
 * OeeSummary Component
 *
 * Displays core A/P/Q/OEE metrics in a period-scoped summary.
 * This component must render correctly given a frozen OeeResult.
 * No backreferences to input state.
 */

import React from "react";
import MetricCard from "./MetricCard";
import type { CoreMetrics } from "../../models";

export interface OeeSummaryProps {
  metrics: CoreMetrics;
  periodStart?: string;
  periodEnd?: string;
}

const OeeSummary: React.FC<OeeSummaryProps> = ({
  metrics,
  periodStart,
  periodEnd,
}) => {
  const formatPeriod = () => {
    if (!periodStart || !periodEnd) return null;

    const start = new Date(periodStart);
    const end = new Date(periodEnd);

    return (
      <div className="text-center mb-6">
        <p className="text-sm text-charcoal-600 dark:text-charcoal-400">
          Analysis Period
        </p>
        <p className="text-base font-medium text-charcoal-900 dark:text-charcoal-100">
          {start.toLocaleDateString()} {start.toLocaleTimeString()}
          {" → "}
          {end.toLocaleDateString()} {end.toLocaleTimeString()}
        </p>
      </div>
    );
  };

  return (
    <div className="space-y-6">
      {formatPeriod()}

      {/* OEE Hero */}
      <div className="text-center py-8">
        <div className="text-sm font-medium text-charcoal-600 dark:text-charcoal-400 uppercase tracking-wide mb-2">
          Overall Equipment Effectiveness
        </div>
        <div
          className={`text-7xl font-display font-bold mb-4 ${
            metrics.oee.value >= 0.85
              ? "text-green-600"
              : metrics.oee.value >= 0.6
              ? "text-yellow-600"
              : "text-red-600"
          }`}
        >
          {(metrics.oee.value * 100).toFixed(1)}%
        </div>
        <div className="inline-flex items-center gap-2 px-4 py-2 bg-white dark:bg-charcoal-800 rounded-full shadow-soft">
          <div
            className={`w-3 h-3 rounded-full ${
              metrics.oee.value >= 0.85
                ? "bg-green-500"
                : metrics.oee.value >= 0.6
                ? "bg-yellow-500"
                : "bg-red-500"
            }`}
          />
          <span className="text-sm font-medium text-charcoal-700 dark:text-charcoal-300">
            {metrics.oee.value >= 0.85
              ? "World Class"
              : metrics.oee.value >= 0.6
              ? "Acceptable"
              : "Needs Improvement"}
          </span>
        </div>
      </div>

      {/* Component Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <MetricCard metric={metrics.availability} color="blue" icon="📊" />
        <MetricCard metric={metrics.performance} color="purple" icon="⚡" />
        <MetricCard metric={metrics.quality} color="green" icon="✓" />
      </div>

      {/* Formula Breakdown */}
      <div className="bg-sand-50 dark:bg-charcoal-800 rounded-lg p-6">
        <h4 className="text-sm font-semibold text-charcoal-700 dark:text-charcoal-300 mb-3">
          Calculation Breakdown
        </h4>
        <div className="formula-display">
          OEE = Availability × Performance × Quality
          <br />
          OEE = {(metrics.availability.value * 100).toFixed(1)}% ×{" "}
          {(metrics.performance.value * 100).toFixed(1)}% ×{" "}
          {(metrics.quality.value * 100).toFixed(1)}%
          <br />
          OEE = {(metrics.oee.value * 100).toFixed(1)}%
        </div>
      </div>
    </div>
  );
};

export default OeeSummary;
