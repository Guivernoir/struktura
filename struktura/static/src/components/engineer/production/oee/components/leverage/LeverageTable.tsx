/**
 * LeverageTable Component
 *
 * Ranked theoretical impacts for loss categories.
 * Shows "if X were reduced" potential, never prescriptive "should" language.
 */

import React, { useState } from "react";
import LeverageRow from "./LeverageRow";
import type { LeverageResponse } from "../../api";

export interface LeverageTableProps {
  leverage: LeverageResponse;
  sortBy?: "impact" | "sensitivity" | "throughput";
}

const LeverageTable: React.FC<LeverageTableProps> = ({
  leverage,
  sortBy: initialSortBy = "impact",
}) => {
  const [sortBy, setSortBy] = useState(initialSortBy);
  const [sortDirection, setSortDirection] = useState<"asc" | "desc">("desc");

  const sortedImpacts = [...leverage.leverage_impacts].sort((a, b) => {
    let comparison = 0;

    switch (sortBy) {
      case "impact":
        comparison = a.oee_opportunity_points - b.oee_opportunity_points;
        break;
      case "sensitivity":
        comparison = a.sensitivity_score - b.sensitivity_score;
        break;
      case "throughput":
        comparison = a.throughput_gain_units - b.throughput_gain_units;
        break;
    }

    return sortDirection === "desc" ? -comparison : comparison;
  });

  const handleSort = (column: typeof sortBy) => {
    if (sortBy === column) {
      setSortDirection((prev) => (prev === "desc" ? "asc" : "desc"));
    } else {
      setSortBy(column);
      setSortDirection("desc");
    }
  };

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="bg-blue-50 dark:bg-blue-900/20 border-l-4 border-blue-400 p-4 rounded-md">
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
          <div>
            <p className="text-sm font-semibold text-blue-800 dark:text-blue-400">
              Theoretical Impact Analysis
            </p>
            <p className="text-xs text-blue-700 dark:text-blue-400 mt-1">
              These show potential if categories were eliminated. This is "if X
              were reduced", never "should".
            </p>
          </div>
        </div>
      </div>

      {/* Baseline */}
      <div className="bg-sand-50 dark:bg-charcoal-800 rounded-lg p-4">
        <div className="flex items-center justify-between">
          <span className="text-sm font-medium text-charcoal-700 dark:text-charcoal-300">
            Baseline OEE
          </span>
          <span className="text-2xl font-display font-bold text-charcoal-900 dark:text-charcoal-100">
            {(leverage.baseline_oee * 100).toFixed(1)}%
          </span>
        </div>
      </div>

      {/* Sort Controls */}
      <div className="flex items-center gap-2 flex-wrap">
        <span className="text-sm text-charcoal-600 dark:text-charcoal-400">
          Sort by:
        </span>
        <button
          onClick={() => handleSort("impact")}
          className={`text-xs px-3 py-1.5 rounded transition-colors ${
            sortBy === "impact"
              ? "bg-steel-600 text-white"
              : "bg-sand-100 dark:bg-charcoal-700 text-charcoal-700 dark:text-charcoal-300 hover:bg-sand-200 dark:hover:bg-charcoal-600"
          }`}
        >
          OEE Impact{" "}
          {sortBy === "impact" && (sortDirection === "desc" ? "↓" : "↑")}
        </button>
        <button
          onClick={() => handleSort("throughput")}
          className={`text-xs px-3 py-1.5 rounded transition-colors ${
            sortBy === "throughput"
              ? "bg-steel-600 text-white"
              : "bg-sand-100 dark:bg-charcoal-700 text-charcoal-700 dark:text-charcoal-300 hover:bg-sand-200 dark:hover:bg-charcoal-600"
          }`}
        >
          Throughput{" "}
          {sortBy === "throughput" && (sortDirection === "desc" ? "↓" : "↑")}
        </button>
        <button
          onClick={() => handleSort("sensitivity")}
          className={`text-xs px-3 py-1.5 rounded transition-colors ${
            sortBy === "sensitivity"
              ? "bg-steel-600 text-white"
              : "bg-sand-100 dark:bg-charcoal-700 text-charcoal-700 dark:text-charcoal-300 hover:bg-sand-200 dark:hover:bg-charcoal-600"
          }`}
        >
          Sensitivity{" "}
          {sortBy === "sensitivity" && (sortDirection === "desc" ? "↓" : "↑")}
        </button>
      </div>

      {/* Impacts */}
      <div className="space-y-3">
        {sortedImpacts.length === 0 ? (
          <div className="text-center py-8 text-charcoal-600 dark:text-charcoal-400">
            <p>No leverage impacts available</p>
          </div>
        ) : (
          sortedImpacts.map((impact, idx) => (
            <LeverageRow
              key={`${impact.category_key}-${idx}`}
              impact={impact}
              rank={idx + 1}
            />
          ))
        )}
      </div>
    </div>
  );
};

export default LeverageTable;
