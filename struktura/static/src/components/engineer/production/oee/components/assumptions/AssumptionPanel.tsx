/**
 * AssumptionPanel Component
 *
 * Container for a domain of assumptions (Time, Counts, Cycle, Downtime)
 * Provides grouping logic and contextual warnings.
 *
 * CRITICAL: This component MUST NOT contain any calculation logic.
 */

import React from "react";
import AssumptionWarnings from "./AssumptionWarnings";
import type { ValidationHint } from "../../utils";

export interface AssumptionPanelProps {
  title: string;
  description?: string;
  icon?: string;
  children: React.ReactNode;
  warnings?: ValidationHint[];
  expanded?: boolean;
  onToggle?: () => void;
}

const AssumptionPanel: React.FC<AssumptionPanelProps> = ({
  title,
  description,
  icon,
  children,
  warnings = [],
  expanded = true,
  onToggle,
}) => {
  const criticalWarnings = warnings.filter((w) => w.severity === "critical");
  const otherWarnings = warnings.filter((w) => w.severity !== "critical");

  return (
    <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft overflow-hidden">
      {/* Header */}
      <div
        className="px-6 py-4 border-b border-charcoal-200 dark:border-charcoal-700 cursor-pointer hover:bg-sand-50 dark:hover:bg-charcoal-800 transition-colors"
        onClick={onToggle}
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            {icon && <span className="text-2xl">{icon}</span>}
            <div>
              <h3 className="section-subheading mb-0">{title}</h3>
              {description && (
                <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
                  {description}
                </p>
              )}
            </div>
          </div>
          <div className="flex items-center gap-3">
            {/* Warning Indicator */}
            {criticalWarnings.length > 0 && (
              <div className="flex items-center gap-1 px-2 py-1 bg-red-100 dark:bg-red-900/20 rounded-full">
                <span className="text-red-600 dark:text-red-400 text-xs font-semibold">
                  {criticalWarnings.length} Critical
                </span>
              </div>
            )}
            {otherWarnings.length > 0 && (
              <div className="flex items-center gap-1 px-2 py-1 bg-yellow-100 dark:bg-yellow-900/20 rounded-full">
                <span className="text-yellow-600 dark:text-yellow-400 text-xs font-semibold">
                  {otherWarnings.length} Warning
                  {otherWarnings.length !== 1 ? "s" : ""}
                </span>
              </div>
            )}
            {/* Expand/Collapse Icon */}
            {onToggle && (
              <svg
                className={`w-5 h-5 text-charcoal-600 dark:text-charcoal-400 transition-transform ${
                  expanded ? "rotate-180" : ""
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
            )}
          </div>
        </div>
      </div>

      {/* Content */}
      {expanded && (
        <div className="p-6 animate-slide-down">
          {/* Warnings */}
          {warnings.length > 0 && (
            <div className="mb-4">
              <AssumptionWarnings warnings={warnings} />
            </div>
          )}

          {/* Assumptions */}
          <div className="space-y-4">{children}</div>
        </div>
      )}
    </div>
  );
};

export default AssumptionPanel;
