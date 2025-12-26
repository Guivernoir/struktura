/**
 * EconomicRangeBar Component
 *
 * Visual representation of economic uncertainty bounds.
 * Shows low, central, and high estimates with range visualization.
 */

import React from "react";
import { formatCurrency } from "../../utils";

export interface EconomicRangeBarProps {
  low: number;
  central: number;
  high: number;
  currency: string;
  isTotal?: boolean;
}

const EconomicRangeBar: React.FC<EconomicRangeBarProps> = ({
  low,
  central,
  high,
  currency,
  isTotal = false,
}) => {
  const range = high - low;
  const centralPosition = range > 0 ? ((central - low) / range) * 100 : 50;

  return (
    <div className="space-y-3">
      {/* Range Bar */}
      <div className="relative h-12 bg-gradient-to-r from-green-100 via-yellow-100 to-red-100 dark:from-green-900/20 dark:via-yellow-900/20 dark:to-red-900/20 rounded-lg overflow-hidden">
        {/* Central Value Indicator */}
        <div
          className="absolute top-0 bottom-0 w-1 bg-charcoal-900 dark:bg-charcoal-100"
          style={{ left: `${centralPosition}%` }}
        />

        {/* Labels */}
        <div className="absolute inset-0 flex items-center justify-between px-3">
          <span className="text-xs font-semibold text-green-800 dark:text-green-400">
            Low
          </span>
          <span className="text-xs font-semibold text-charcoal-900 dark:text-charcoal-100">
            Central
          </span>
          <span className="text-xs font-semibold text-red-800 dark:text-red-400">
            High
          </span>
        </div>
      </div>

      {/* Values */}
      <div className="flex items-center justify-between">
        <div className="text-center flex-1">
          <div
            className={`${
              isTotal ? "text-xl" : "text-base"
            } font-semibold text-green-700 dark:text-green-400`}
          >
            {formatCurrency(low, currency, 0)}
          </div>
          <div className="text-xs text-charcoal-500 dark:text-charcoal-400">
            Best Case
          </div>
        </div>

        <div className="text-center flex-1">
          <div
            className={`${
              isTotal ? "text-3xl" : "text-2xl"
            } font-display font-bold text-charcoal-900 dark:text-charcoal-100`}
          >
            {formatCurrency(central, currency, 0)}
          </div>
          <div className="text-xs text-charcoal-500 dark:text-charcoal-400">
            Expected
          </div>
        </div>

        <div className="text-center flex-1">
          <div
            className={`${
              isTotal ? "text-xl" : "text-base"
            } font-semibold text-red-700 dark:text-red-400`}
          >
            {formatCurrency(high, currency, 0)}
          </div>
          <div className="text-xs text-charcoal-500 dark:text-charcoal-400">
            Worst Case
          </div>
        </div>
      </div>

      {/* Uncertainty Range */}
      <div className="text-center">
        <div className="text-xs text-charcoal-600 dark:text-charcoal-400">
          Uncertainty Range:{" "}
          <span className="font-semibold">
            {formatCurrency(range, currency, 0)}
          </span>{" "}
          (±{((range / central) * 50).toFixed(0)}% of central)
        </div>
      </div>
    </div>
  );
};

export default EconomicRangeBar;
