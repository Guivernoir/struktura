/**
 * ResultsView Component
 *
 * Layout orchestration for computed outputs.
 * Displays OEE metrics, loss tree, leverage, and economics.
 *
 * CRITICAL: This is presentation only. No calculation logic.
 */

import React, { useState } from "react";
import OeeSummary from "../components/results/OeeSummary";
import MetricCard from "../components/results/MetricCard";
import LossTreeView from "../components/loss_tree/LossTreeView";
import LeverageTable from "../components/leverage/LeverageTable";
import EconomicImpactPanel from "../components/economics/EconomicImpactPanel";
import type { OeeResult } from "../models";
import type { LeverageResponse, SensitivityAnalysis } from "../api";

export interface ResultsViewProps {
  result: OeeResult;
  leverage?: LeverageResponse;
  sensitivity?: SensitivityAnalysis;
  periodStart?: string;
  periodEnd?: string;
}

type ResultSection =
  | "overview"
  | "extended"
  | "losses"
  | "leverage"
  | "economics";

const ResultsView: React.FC<ResultsViewProps> = ({
  result,
  leverage,
  sensitivity,
  periodStart,
  periodEnd,
}) => {
  const [activeSection, setActiveSection] = useState<ResultSection>("overview");

  const sections: { id: ResultSection; label: string; icon: string }[] = [
    { id: "overview", label: "Overview", icon: "📊" },
    { id: "extended", label: "Extended Metrics", icon: "📈" },
    { id: "losses", label: "Loss Tree", icon: "🌳" },
    ...(leverage
      ? [{ id: "leverage" as ResultSection, label: "Leverage", icon: "⚖️" }]
      : []),
    ...(result.economic_analysis
      ? [{ id: "economics" as ResultSection, label: "Economics", icon: "💰" }]
      : []),
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h2 className="section-heading">Results</h2>
        <p className="helper-text">
          Calculated metrics with full formula traceability. All outputs are
          deterministic.
        </p>
      </div>

      {/* Section Navigation */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-2">
        <div className="flex flex-wrap gap-2">
          {sections.map((section) => (
            <button
              key={section.id}
              onClick={() => setActiveSection(section.id)}
              className={`px-4 py-2 rounded-md font-medium text-sm transition-colors flex items-center gap-2
                ${
                  activeSection === section.id
                    ? "bg-steel-600 text-white"
                    : "bg-sand-100 dark:bg-charcoal-800 text-charcoal-700 dark:text-charcoal-300 hover:bg-sand-200 dark:hover:bg-charcoal-700"
                }`}
            >
              <span>{section.icon}</span>
              <span>{section.label}</span>
            </button>
          ))}
        </div>
      </div>

      {/* Section Content */}
      <div className="animate-fade-in">
        {activeSection === "overview" && (
          <div className="space-y-6">
            <OeeSummary
              metrics={result.core_metrics}
              periodStart={periodStart}
              periodEnd={periodEnd}
            />

            {/* Confidence Summary */}
            <div className="bg-sand-50 dark:bg-charcoal-800 rounded-lg p-6">
              <h3 className="section-subheading mb-4">
                Data Quality Assessment
              </h3>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <div className="text-sm text-charcoal-600 dark:text-charcoal-400 mb-2">
                    Source Distribution
                  </div>
                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-medium">Explicit</span>
                      <span className="text-sm font-semibold text-blue-600">
                        {result.ledger.source_statistics.explicit_percentage.toFixed(
                          0
                        )}
                        %
                      </span>
                    </div>
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-medium">Inferred</span>
                      <span className="text-sm font-semibold text-purple-600">
                        {result.ledger.source_statistics.inferred_percentage.toFixed(
                          0
                        )}
                        %
                      </span>
                    </div>
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-medium">Default</span>
                      <span className="text-sm font-semibold text-gray-600">
                        {result.ledger.source_statistics.default_percentage.toFixed(
                          0
                        )}
                        %
                      </span>
                    </div>
                  </div>
                </div>

                <div>
                  <div className="text-sm text-charcoal-600 dark:text-charcoal-400 mb-2">
                    Validation Status
                  </div>
                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-medium">Valid</span>
                      <span
                        className={`text-sm font-semibold ${
                          result.validation.is_valid
                            ? "text-green-600"
                            : "text-red-600"
                        }`}
                      >
                        {result.validation.is_valid ? "Yes" : "No"}
                      </span>
                    </div>
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-medium">Issues</span>
                      <span className="text-sm font-semibold">
                        {result.validation.issues.length}
                      </span>
                    </div>
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-medium">Warnings</span>
                      <span className="text-sm font-semibold text-yellow-600">
                        {result.ledger.warnings.length}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Sensitivity Overview */}
            {sensitivity && (
              <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
                <h3 className="section-subheading mb-4">
                  Sensitivity Analysis
                </h3>
                <p className="text-sm text-charcoal-600 dark:text-charcoal-400 mb-4">
                  Impact of ±10% variation in key assumptions
                </p>
                <div className="space-y-3">
                  {Object.entries(sensitivity.factors)
                    .sort(
                      (a, b) => Math.abs(b[1].impact) - Math.abs(a[1].impact)
                    )
                    .slice(0, 5)
                    .map(([key, factor]) => (
                      <div
                        key={key}
                        className="flex items-center justify-between p-3 bg-sand-50 dark:bg-charcoal-800 rounded-md"
                      >
                        <span className="text-sm font-medium">{key}</span>
                        <div className="flex items-center gap-4">
                          <span className="text-sm text-charcoal-600 dark:text-charcoal-400">
                            Impact: {(factor.impact * 100).toFixed(2)}%
                          </span>
                          <span className="text-sm text-steel-600 dark:text-steel-400">
                            Sensitivity: {(factor.sensitivity * 100).toFixed(1)}
                            %
                          </span>
                        </div>
                      </div>
                    ))}
                </div>
              </div>
            )}
          </div>
        )}

        {activeSection === "extended" && (
          <div className="space-y-6">
            <h3 className="section-subheading">Extended Metrics</h3>

            {/* Grid of Extended Metrics */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {result.extended_metrics.teep && (
                <MetricCard
                  metric={result.extended_metrics.teep}
                  color="purple"
                  icon="📅"
                />
              )}
              <MetricCard
                metric={result.extended_metrics.utilization}
                color="blue"
                icon="⚡"
              />
              {result.extended_metrics.mtbf && (
                <MetricCard
                  metric={result.extended_metrics.mtbf}
                  color="green"
                  icon="🔧"
                />
              )}
              {result.extended_metrics.mttr && (
                <MetricCard
                  metric={result.extended_metrics.mttr}
                  color="yellow"
                  icon="⏱️"
                />
              )}
              <MetricCard
                metric={result.extended_metrics.scrap_rate}
                color="red"
                icon="🗑️"
              />
              <MetricCard
                metric={result.extended_metrics.rework_rate}
                color="yellow"
                icon="🔄"
              />
              <MetricCard
                metric={result.extended_metrics.net_operating_time}
                color="blue"
                icon="⏰"
              />
            </div>

            {/* Explanation */}
            <div className="bg-blue-50 dark:bg-blue-900/20 border-l-4 border-blue-400 p-4 rounded-md">
              <p className="text-sm text-blue-700 dark:text-blue-400">
                <strong>Note:</strong> Extended metrics provide additional
                context beyond core OEE. They help identify specific improvement
                areas but are not part of the standard OEE calculation.
              </p>
            </div>
          </div>
        )}

        {activeSection === "losses" && (
          <div className="space-y-6">
            <LossTreeView lossTree={result.loss_tree} expandAll={false} />

            <div className="bg-yellow-50 dark:bg-yellow-900/20 border-l-4 border-yellow-400 p-4 rounded-md">
              <p className="text-sm text-yellow-700 dark:text-yellow-400">
                <strong>Important:</strong> This tree shows where losses
                <em> accumulate</em>, not what <em>caused</em> them. Categories
                represent mathematical partitions, not root causes.
              </p>
            </div>
          </div>
        )}

        {activeSection === "leverage" && leverage && (
          <div className="space-y-6">
            <LeverageTable leverage={leverage} sortBy="impact" />
          </div>
        )}

        {activeSection === "economics" && result.economic_analysis && (
          <div className="space-y-6">
            <EconomicImpactPanel analysis={result.economic_analysis} />
          </div>
        )}
      </div>

      {/* Footer Info */}
      <div className="bg-sand-50 dark:bg-charcoal-800 rounded-lg p-4 text-sm">
        <div className="flex items-center justify-between flex-wrap gap-4">
          <div>
            <span className="text-charcoal-500 dark:text-charcoal-400">
              Analysis Timestamp:
            </span>
            <span className="ml-2 font-mono font-semibold">
              {new Date(result.ledger.analysis_timestamp).toLocaleString()}
            </span>
          </div>
          <div>
            <span className="text-charcoal-500 dark:text-charcoal-400">
              Total Assumptions:
            </span>
            <span className="ml-2 font-semibold">
              {result.ledger.source_statistics.total_count}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ResultsView;
