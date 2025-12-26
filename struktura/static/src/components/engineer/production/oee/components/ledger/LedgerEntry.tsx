/**
 * LedgerEntry Component
 *
 * Single assumption with source, impact map, and downstream dependencies.
 * One assumption, one impact map.
 */

import React, { useState } from "react";
import TracebackLink from "./TracebackLink";
import type { AssumptionEntry } from "../../models";

export interface LedgerEntryProps {
  entry: AssumptionEntry;
}

const LedgerEntry: React.FC<LedgerEntryProps> = ({ entry }) => {
  const [isExpanded, setIsExpanded] = useState(false);

  const impactConfig = {
    Critical: "assumption-entry--critical",
    High: "assumption-entry--high",
    Medium: "assumption-entry--medium",
    Low: "",
    Info: "",
  };

  const sourceConfig = {
    explicit: {
      label: "Explicit",
      icon: "✓",
      color: "text-blue-600 dark:text-blue-400",
    },
    inferred: {
      label: "Inferred",
      icon: "⚙",
      color: "text-purple-600 dark:text-purple-400",
    },
    default: {
      label: "Default",
      icon: "○",
      color: "text-gray-600 dark:text-gray-400",
    },
  };

  const source = sourceConfig[entry.source as keyof typeof sourceConfig];

  const formatValue = (value: unknown): string => {
    if (typeof value === "number") {
      return value.toFixed(2);
    }
    if (typeof value === "object") {
      return JSON.stringify(value, null, 2);
    }
    return String(value);
  };

  return (
    <div className={`assumption-entry ${impactConfig[entry.impact]}`}>
      <div
        className="flex items-start justify-between cursor-pointer"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            <code className="text-sm font-mono font-semibold text-charcoal-900 dark:text-charcoal-100">
              {entry.assumption_key}
            </code>
            <span
              className={`text-xs px-2 py-0.5 rounded-full ${
                entry.impact === "Critical"
                  ? "bg-red-100 dark:bg-red-900/20 text-red-700 dark:text-red-400"
                  : entry.impact === "High"
                  ? "bg-orange-100 dark:bg-orange-900/20 text-orange-700 dark:text-orange-400"
                  : entry.impact === "Medium"
                  ? "bg-yellow-100 dark:bg-yellow-900/20 text-yellow-700 dark:text-yellow-400"
                  : "bg-blue-100 dark:bg-blue-900/20 text-blue-700 dark:text-blue-400"
              }`}
            >
              {entry.impact}
            </span>
          </div>
          <p className="text-sm text-charcoal-600 dark:text-charcoal-400">
            {entry.description_key}
          </p>
        </div>

        <div className="flex items-center gap-3 ml-4">
          <div
            className={`flex items-center gap-1 text-sm font-medium ${source.color}`}
          >
            <span>{source.icon}</span>
            <span>{source.label}</span>
          </div>
          <svg
            className={`w-5 h-5 text-charcoal-400 transition-transform ${
              isExpanded ? "rotate-180" : ""
            }`}
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </div>
      </div>

      {isExpanded && (
        <div className="mt-4 pt-4 border-t border-charcoal-200 dark:border-charcoal-700 space-y-3 animate-slide-down">
          {/* Value */}
          <div>
            <div className="text-xs font-semibold text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mb-1">
              Value
            </div>
            <code className="block text-sm font-mono bg-sand-100 dark:bg-charcoal-900 p-2 rounded border border-charcoal-200 dark:border-charcoal-700 overflow-x-auto">
              {formatValue(entry.value)}
            </code>
          </div>

          {/* Timestamp */}
          <div>
            <div className="text-xs font-semibold text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mb-1">
              Timestamp
            </div>
            <div className="text-sm text-charcoal-700 dark:text-charcoal-300">
              {new Date(entry.timestamp).toLocaleString()}
            </div>
          </div>

          {/* Related Assumptions */}
          {entry.related_assumptions.length > 0 && (
            <div>
              <div className="text-xs font-semibold text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mb-2">
                Affects {entry.related_assumptions.length} Other Assumption
                {entry.related_assumptions.length !== 1 ? "s" : ""}
              </div>
              <div className="flex flex-wrap gap-2">
                {entry.related_assumptions.map((related, idx) => (
                  <TracebackLink key={idx} assumptionKey={related} />
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default LedgerEntry;
