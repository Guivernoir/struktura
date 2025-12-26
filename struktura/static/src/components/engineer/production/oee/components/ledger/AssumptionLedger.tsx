/**
 * AssumptionLedger Component
 *
 * Full ledger view showing complete assumption list with sources.
 * This is the trust anchor of the entire application.
 *
 * CRITICAL: If this folder is weak, the entire product is weak.
 */

import React, { useState } from "react";
import LedgerEntry from "./LedgerEntry";
import type { AssumptionLedger as AssumptionLedgerType } from "../../models";

export interface AssumptionLedgerProps {
  ledger: AssumptionLedgerType;
}

const AssumptionLedger: React.FC<AssumptionLedgerProps> = ({ ledger }) => {
  const [filterSource, setFilterSource] = useState<
    "all" | "explicit" | "inferred" | "default"
  >("all");
  const [filterImpact, setFilterImpact] = useState<
    "all" | "Critical" | "High" | "Medium" | "Low"
  >("all");
  const [searchQuery, setSearchQuery] = useState("");

  // Filter assumptions
  const filteredAssumptions = ledger.assumptions.filter((assumption) => {
    // Source filter
    if (filterSource !== "all" && assumption.source !== filterSource) {
      return false;
    }

    // Impact filter
    if (filterImpact !== "all" && assumption.impact !== filterImpact) {
      return false;
    }

    // Search filter
    if (searchQuery) {
      const searchLower = searchQuery.toLowerCase();
      return (
        assumption.assumption_key.toLowerCase().includes(searchLower) ||
        assumption.description_key.toLowerCase().includes(searchLower)
      );
    }

    return true;
  });

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h2 className="section-heading">Assumption Ledger</h2>
        <p className="helper-text">
          Complete receipts for every assumption in this analysis. This is your
          trust anchor.
        </p>
      </div>

      {/* Statistics */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
        <h3 className="section-subheading mb-4">Source Statistics</h3>
        <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
          <div className="text-center">
            <div className="text-3xl font-display font-bold text-charcoal-900 dark:text-charcoal-100">
              {ledger.source_statistics.total_count}
            </div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mt-1">
              Total
            </div>
          </div>
          <div className="text-center">
            <div className="text-3xl font-display font-bold text-blue-600">
              {ledger.source_statistics.explicit_count}
            </div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mt-1">
              Explicit
            </div>
            <div className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
              {ledger.source_statistics.explicit_percentage.toFixed(0)}%
            </div>
          </div>
          <div className="text-center">
            <div className="text-3xl font-display font-bold text-purple-600">
              {ledger.source_statistics.inferred_count}
            </div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mt-1">
              Inferred
            </div>
            <div className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
              {ledger.source_statistics.inferred_percentage.toFixed(0)}%
            </div>
          </div>
          <div className="text-center">
            <div className="text-3xl font-display font-bold text-gray-600">
              {ledger.source_statistics.default_count}
            </div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mt-1">
              Default
            </div>
            <div className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
              {ledger.source_statistics.default_percentage.toFixed(0)}%
            </div>
          </div>
          <div className="text-center">
            <div className="text-3xl font-display font-bold text-yellow-600">
              {ledger.warnings.length}
            </div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mt-1">
              Warnings
            </div>
          </div>
        </div>
      </div>

      {/* Filters */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-4">
        <div className="flex flex-col md:flex-row gap-4">
          {/* Search */}
          <div className="flex-1">
            <input
              type="text"
              placeholder="Search assumptions..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100
                       focus:outline-none focus:ring-2 focus:ring-steel-500"
            />
          </div>

          {/* Source Filter */}
          <select
            value={filterSource}
            onChange={(e) => setFilterSource(e.target.value as any)}
            className="px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                     rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100
                     focus:outline-none focus:ring-2 focus:ring-steel-500"
          >
            <option value="all">All Sources</option>
            <option value="explicit">Explicit</option>
            <option value="inferred">Inferred</option>
            <option value="default">Default</option>
          </select>

          {/* Impact Filter */}
          <select
            value={filterImpact}
            onChange={(e) => setFilterImpact(e.target.value as any)}
            className="px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                     rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100
                     focus:outline-none focus:ring-2 focus:ring-steel-500"
          >
            <option value="all">All Impacts</option>
            <option value="Critical">Critical</option>
            <option value="High">High</option>
            <option value="Medium">Medium</option>
            <option value="Low">Low</option>
          </select>
        </div>
      </div>

      {/* Warnings */}
      {ledger.warnings.length > 0 && (
        <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
          <h3 className="section-subheading mb-4">Warnings</h3>
          <div className="space-y-2">
            {ledger.warnings.map((warning, idx) => (
              <div key={idx} className="validation-warning">
                <p className="warning-text">{warning.message_key}</p>
                <p className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
                  Severity: {warning.severity}
                </p>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Assumptions List */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
        <h3 className="section-subheading mb-4">
          Assumptions ({filteredAssumptions.length} of{" "}
          {ledger.assumptions.length})
        </h3>
        <div className="space-y-3">
          {filteredAssumptions.length === 0 ? (
            <div className="text-center py-8 text-charcoal-600 dark:text-charcoal-400">
              <p>No assumptions match the current filters</p>
            </div>
          ) : (
            filteredAssumptions.map((assumption, idx) => (
              <LedgerEntry
                key={`${assumption.assumption_key}-${idx}`}
                entry={assumption}
              />
            ))
          )}
        </div>
      </div>
    </div>
  );
};

export default AssumptionLedger;
