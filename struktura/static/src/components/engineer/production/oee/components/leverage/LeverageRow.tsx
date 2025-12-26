/**
 * LeverageRow Component
 *
 * Single leverage hypothesis display.
 * Shows theoretical impact with sensitivity indicator.
 */

import React from "react";
import SensitivityIndicator from "./SensitivityIndicator";

export interface LeverageImpact {
  category_key: string;
  oee_opportunity_points: number;
  throughput_gain_units: number;
  sensitivity_score: number;
}

export interface LeverageRowProps {
  impact: LeverageImpact;
  rank: number;
}

const LeverageRow: React.FC<LeverageRowProps> = ({ impact, rank }) => {
  return (
    <div className="border border-charcoal-200 dark:border-charcoal-700 rounded-lg p-4 hover:shadow-medium transition-shadow">
      <div className="flex items-start justify-between gap-4">
        {/* Left: Rank & Category */}
        <div className="flex items-start gap-3 flex-1 min-w-0">
          <div
            className={`flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold ${
              rank === 1
                ? "bg-yellow-100 dark:bg-yellow-900/20 text-yellow-700 dark:text-yellow-400"
                : rank === 2
                ? "bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-400"
                : rank === 3
                ? "bg-orange-100 dark:bg-orange-900/20 text-orange-700 dark:text-orange-400"
                : "bg-sand-100 dark:bg-charcoal-800 text-charcoal-600 dark:text-charcoal-400"
            }`}
          >
            {rank}
          </div>

          <div className="flex-1 min-w-0">
            <h4 className="font-semibold text-charcoal-900 dark:text-charcoal-100 truncate">
              {impact.category_key}
            </h4>
            <p className="text-sm text-charcoal-600 dark:text-charcoal-400 mt-1">
              If this category were eliminated
            </p>
          </div>
        </div>

        {/* Right: Metrics */}
        <div className="flex items-center gap-6">
          {/* OEE Opportunity */}
          <div className="text-right">
            <div className="text-2xl font-display font-bold text-steel-600 dark:text-steel-400">
              +{impact.oee_opportunity_points.toFixed(1)}%
            </div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400">
              OEE Points
            </div>
          </div>

          {/* Throughput Gain */}
          <div className="text-right">
            <div className="text-lg font-semibold text-charcoal-900 dark:text-charcoal-100">
              +{impact.throughput_gain_units}
            </div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400">
              Units
            </div>
          </div>

          {/* Sensitivity */}
          <SensitivityIndicator score={impact.sensitivity_score} />
        </div>
      </div>
    </div>
  );
};

export default LeverageRow;
