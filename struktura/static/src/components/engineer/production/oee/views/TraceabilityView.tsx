/**
 * TraceabilityView Component
 *
 * Complete assumption-to-result mapping with ledger.
 * Shows how every input flows through to every output.
 * This is the trust mechanism that makes the tool credible.
 */

import React, { useState } from "react";
import AssumptionLedger from "../ledger/AssumptionLedger";
import type { OeeResult, OeeInput } from "../models";

export interface TraceabilityViewProps {
  result: OeeResult;
  input: OeeInput | null;
}

type TraceabilityTab = "ledger" | "formulas" | "dependencies";

const TraceabilityView: React.FC<TraceabilityViewProps> = ({
  result,
  input,
}) => {
  const [activeTab, setActiveTab] = useState<TraceabilityTab>("ledger");

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
        <h2 className="section-heading">Traceability & Transparency</h2>
        <p className="helper-text">
          Every metric can answer: "How was this calculated?" and "Which
          assumptions does this depend on?"
        </p>
      </div>

      {/* Tabs */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft overflow-hidden">
        <div className="border-b border-charcoal-200 dark:border-charcoal-700">
          <nav className="flex -mb-px">
            <button
              onClick={() => setActiveTab("ledger")}
              className={`px-6 py-3 text-sm font-medium border-b-2 transition-colors ${
                activeTab === "ledger"
                  ? "border-steel-600 text-steel-600 dark:text-steel-400"
                  : "border-transparent text-charcoal-600 dark:text-charcoal-400 hover:text-charcoal-900 dark:hover:text-charcoal-200"
              }`}
            >
              Assumption Ledger
            </button>

            <button
              onClick={() => setActiveTab("formulas")}
              className={`px-6 py-3 text-sm font-medium border-b-2 transition-colors ${
                activeTab === "formulas"
                  ? "border-steel-600 text-steel-600 dark:text-steel-400"
                  : "border-transparent text-charcoal-600 dark:text-charcoal-400 hover:text-charcoal-900 dark:hover:text-charcoal-200"
              }`}
            >
              Formula Registry
            </button>

            <button
              onClick={() => setActiveTab("dependencies")}
              className={`px-6 py-3 text-sm font-medium border-b-2 transition-colors ${
                activeTab === "dependencies"
                  ? "border-steel-600 text-steel-600 dark:text-steel-400"
                  : "border-transparent text-charcoal-600 dark:text-charcoal-400 hover:text-charcoal-900 dark:hover:text-charcoal-200"
              }`}
            >
              Dependency Graph
            </button>
          </nav>
        </div>

        <div className="p-6">
          {activeTab === "ledger" && (
            <AssumptionLedger ledger={result.ledger} />
          )}
          {activeTab === "formulas" && <FormulasTab result={result} />}
          {activeTab === "dependencies" && (
            <DependenciesTab result={result} input={input} />
          )}
        </div>
      </div>
    </div>
  );
};

// Formulas Tab
const FormulasTab: React.FC<{ result: OeeResult }> = ({ result }) => {
  const allMetrics = [
    ...Object.values(result.core_metrics),
    ...Object.values(result.extended_metrics).filter((m) => m !== null),
  ].filter(Boolean);

  return (
    <div className="space-y-4">
      <div>
        <h3 className="section-subheading mb-2">All Formulas Used</h3>
        <p className="text-sm text-charcoal-600 dark:text-charcoal-400">
          Complete registry of every formula used in this analysis.
        </p>
      </div>

      <div className="space-y-3">
        {allMetrics.map((metric: any, idx: number) => (
          <div
            key={idx}
            className="bg-sand-50 dark:bg-charcoal-800 rounded-lg p-4 border border-charcoal-200 dark:border-charcoal-700"
          >
            <div className="flex items-start justify-between mb-2">
              <div>
                <div className="font-semibold text-charcoal-900 dark:text-charcoal-100">
                  {metric.name_key.replace("metrics.", "")}
                </div>
                <div className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
                  Result:{" "}
                  {metric.unit_key === "units.percentage"
                    ? `${(metric.value * 100).toFixed(2)}%`
                    : metric.value.toFixed(4)}
                </div>
              </div>
              <div
                className={`confidence-badge confidence-badge--${metric.confidence.toLowerCase()}`}
              >
                {metric.confidence}
              </div>
            </div>

            <code className="block text-xs font-mono bg-white dark:bg-charcoal-900 p-3 rounded border border-charcoal-200 dark:border-charcoal-700 overflow-x-auto">
              {metric.formula_key}
            </code>

            <div className="mt-2 grid grid-cols-2 gap-2">
              {Object.entries(metric.formula_params).map(
                ([key, value]: [string, any]) => (
                  <div
                    key={key}
                    className="flex items-center justify-between text-xs bg-white dark:bg-charcoal-900 px-2 py-1 rounded"
                  >
                    <span className="text-charcoal-600 dark:text-charcoal-400">
                      {key}
                    </span>
                    <code className="font-mono text-steel-600 dark:text-steel-400">
                      {typeof value === "number"
                        ? value.toFixed(4)
                        : String(value)}
                    </code>
                  </div>
                )
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

// Dependencies Tab
const DependenciesTab: React.FC<{
  result: OeeResult;
  input: OeeInput | null;
}> = ({ result, input }) => {
  return (
    <div className="space-y-4">
      <div>
        <h3 className="section-subheading mb-2">Dependency Mapping</h3>
        <p className="text-sm text-charcoal-600 dark:text-charcoal-400">
          Visual representation of how assumptions flow through to results.
        </p>
      </div>

      {/* Core Metrics Dependencies */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
        <h4 className="font-semibold text-charcoal-900 dark:text-charcoal-100 mb-4">
          Core OEE Components
        </h4>
        <div className="space-y-4">
          <DependencyCard
            metric="OEE"
            dependsOn={["Availability", "Performance", "Quality"]}
            type="multiplicative"
          />
          <DependencyCard
            metric="Availability"
            dependsOn={["Planned Production Time", "Total Downtime"]}
            type="ratio"
          />
          <DependencyCard
            metric="Performance"
            dependsOn={["Ideal Cycle Time", "Actual Units", "Operating Time"]}
            type="ratio"
          />
          <DependencyCard
            metric="Quality"
            dependsOn={["Good Units", "Total Units"]}
            type="ratio"
          />
        </div>
      </div>

      {/* Input Dependencies */}
      {input && (
        <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
          <h4 className="font-semibold text-charcoal-900 dark:text-charcoal-100 mb-4">
            Input Assumptions Used
          </h4>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
            <AssumptionBadge label="Planned Time" impact="Critical" />
            <AssumptionBadge label="Total Units" impact="Critical" />
            <AssumptionBadge label="Good Units" impact="Critical" />
            <AssumptionBadge label="Ideal Cycle Time" impact="High" />
            <AssumptionBadge label="Scrap Units" impact="Medium" />
            <AssumptionBadge label="Rework Units" impact="Medium" />
          </div>
        </div>
      )}

      {/* Threshold Dependencies */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
        <h4 className="font-semibold text-charcoal-900 dark:text-charcoal-100 mb-4">
          Thresholds Applied
        </h4>
        <div className="space-y-2">
          {result.ledger.thresholds.map((threshold, idx) => (
            <div
              key={idx}
              className="flex items-center justify-between text-sm bg-sand-50 dark:bg-charcoal-800 px-3 py-2 rounded"
            >
              <span className="text-charcoal-700 dark:text-charcoal-300">
                {threshold.threshold_key}
              </span>
              <code className="font-mono text-steel-600 dark:text-steel-400">
                {threshold.value} {threshold.unit_key}
              </code>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

// Helper Components
const DependencyCard: React.FC<{
  metric: string;
  dependsOn: string[];
  type: "multiplicative" | "ratio" | "additive";
}> = ({ metric, dependsOn, type }) => {
  const operators = {
    multiplicative: "×",
    ratio: "÷",
    additive: "+",
  };

  return (
    <div className="flex items-center gap-4 p-3 bg-sand-50 dark:bg-charcoal-800 rounded-lg">
      <div className="flex-shrink-0 w-24 text-right">
        <span className="font-semibold text-charcoal-900 dark:text-charcoal-100">
          {metric}
        </span>
      </div>
      <div className="flex-shrink-0">
        <svg
          className="w-4 h-4 text-charcoal-400"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M17 8l4 4m0 0l-4 4m4-4H3"
          />
        </svg>
      </div>
      <div className="flex-1 flex items-center gap-2 flex-wrap">
        {dependsOn.map((dep, idx) => (
          <React.Fragment key={dep}>
            <span className="text-sm px-2 py-1 bg-white dark:bg-charcoal-900 rounded border border-charcoal-200 dark:border-charcoal-700">
              {dep}
            </span>
            {idx < dependsOn.length - 1 && (
              <span className="text-charcoal-400">{operators[type]}</span>
            )}
          </React.Fragment>
        ))}
      </div>
    </div>
  );
};

const AssumptionBadge: React.FC<{
  label: string;
  impact: "Critical" | "High" | "Medium" | "Low";
}> = ({ label, impact }) => {
  const colors = {
    Critical:
      "bg-red-100 dark:bg-red-900/20 text-red-700 dark:text-red-400 border-red-300 dark:border-red-800",
    High: "bg-orange-100 dark:bg-orange-900/20 text-orange-700 dark:text-orange-400 border-orange-300 dark:border-orange-800",
    Medium:
      "bg-yellow-100 dark:bg-yellow-900/20 text-yellow-700 dark:text-yellow-400 border-yellow-300 dark:border-yellow-800",
    Low: "bg-blue-100 dark:bg-blue-900/20 text-blue-700 dark:text-blue-400 border-blue-300 dark:border-blue-800",
  };

  return (
    <div className={`px-3 py-2 rounded border ${colors[impact]}`}>
      <div className="text-xs font-semibold">{label}</div>
      <div className="text-xs opacity-75">{impact} Impact</div>
    </div>
  );
};

export default TraceabilityView;
