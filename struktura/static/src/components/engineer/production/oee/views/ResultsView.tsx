/**
 * Results View
 *
 * Presents computed outputs - nothing upstream of calculation.
 * Shows: Core OEE, Extended Metrics, Loss Tree, Leverage, Sensitivity
 */

import React, { useState } from "react";
import type { OeeResult } from "../models";
import type { SensitivityAnalysis, LeverageResponse } from "../api";
import {
  formatPercentage,
  formatDuration,
  formatCurrency,
  getOeeColorClass,
} from "../utils";

interface ResultsViewProps {
  result: OeeResult;
  sensitivity?: SensitivityAnalysis;
  leverage?: LeverageResponse;
}

type ResultsTab = "overview" | "loss-tree" | "leverage" | "sensitivity";

const ResultsView: React.FC<ResultsViewProps> = ({
  result,
  sensitivity,
  leverage,
}) => {
  const [activeTab, setActiveTab] = useState<ResultsTab>("overview");

  const oeeValue = result.core_metrics.oee.value;
  const oeePercentage = (oeeValue * 100).toFixed(1);
  const oeeClass = getOeeColorClass(oeeValue);

  return (
    <div className="space-y-6">
      {/* OEE Hero Card */}
      <div className="bg-gradient-to-br from-steel-50 to-sand-50 dark:from-charcoal-800 dark:to-charcoal-900 rounded-xl shadow-medium p-8 text-center">
        <h2 className="text-sm font-medium text-charcoal-600 dark:text-charcoal-400 uppercase tracking-wide mb-2">
          Overall Equipment Effectiveness
        </h2>
        <div className={`text-6xl font-display font-bold ${oeeClass} mb-4`}>
          {oeePercentage}%
        </div>
        <div className="inline-flex items-center gap-2 px-4 py-2 bg-white dark:bg-charcoal-800 rounded-full">
          <div
            className={`w-3 h-3 rounded-full ${
              oeeValue >= 0.85
                ? "bg-green-500"
                : oeeValue >= 0.6
                ? "bg-yellow-500"
                : "bg-red-500"
            }`}
          />
          <span className="text-sm font-medium text-charcoal-700 dark:text-charcoal-300">
            {oeeValue >= 0.85
              ? "World Class"
              : oeeValue >= 0.6
              ? "Acceptable"
              : "Needs Improvement"}
          </span>
        </div>
      </div>

      {/* Core Metrics Grid */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* Availability */}
        <div className="oee-metric-card">
          <div className="metric-label">Availability</div>
          <div className="metric-value text-availability">
            {formatPercentage(result.core_metrics.availability)}
          </div>
          <div className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-2">
            Operating Time / Planned Time
          </div>
        </div>

        {/* Performance */}
        <div className="oee-metric-card">
          <div className="metric-label">Performance</div>
          <div className="metric-value text-performance">
            {formatPercentage(result.core_metrics.performance)}
          </div>
          <div className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-2">
            Ideal Production / Actual Production
          </div>
        </div>

        {/* Quality */}
        <div className="oee-metric-card">
          <div className="metric-label">Quality</div>
          <div className="metric-value text-quality">
            {formatPercentage(result.core_metrics.quality)}
          </div>
          <div className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-2">
            Good Units / Total Units
          </div>
        </div>
      </div>

      {/* Extended Metrics */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
        <h3 className="section-subheading mb-4">Extended Metrics</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mb-1">
              Utilization
            </div>
            <div className="text-2xl font-display font-semibold text-charcoal-900 dark:text-charcoal-100">
              {formatPercentage(result.extended_metrics.utilization)}
            </div>
          </div>

          <div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mb-1">
              Scrap Rate
            </div>
            <div className="text-2xl font-display font-semibold text-charcoal-900 dark:text-charcoal-100">
              {formatPercentage(result.extended_metrics.scrap_rate)}
            </div>
          </div>

          {result.extended_metrics.teep && (
            <div>
              <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mb-1">
                TEEP
              </div>
              <div className="text-2xl font-display font-semibold text-charcoal-900 dark:text-charcoal-100">
                {formatPercentage(result.extended_metrics.teep)}
              </div>
            </div>
          )}

          {result.extended_metrics.mtbf && (
            <div>
              <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide mb-1">
                MTBF
              </div>
              <div className="text-2xl font-display font-semibold text-charcoal-900 dark:text-charcoal-100">
                {formatDuration(result.extended_metrics.mtbf.value, {
                  compact: true,
                  maxParts: 2,
                })}
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Economic Analysis */}
      {result.economic_analysis && (
        <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
          <h3 className="section-subheading mb-4">
            Economic Impact (Estimates)
          </h3>
          <div className="space-y-3">
            <div className="economic-impact">
              <span className="text-sm font-medium text-charcoal-700 dark:text-charcoal-300">
                Throughput Loss
              </span>
              <div className="economic-range">
                <span className="economic-range-low">
                  {formatCurrency(
                    result.economic_analysis.throughput_loss.low_estimate,
                    result.economic_analysis.throughput_loss.currency,
                    0
                  )}
                </span>
                <span className="economic-range-central">
                  {formatCurrency(
                    result.economic_analysis.throughput_loss.central_estimate,
                    result.economic_analysis.throughput_loss.currency,
                    0
                  )}
                </span>
                <span className="economic-range-high">
                  {formatCurrency(
                    result.economic_analysis.throughput_loss.high_estimate,
                    result.economic_analysis.throughput_loss.currency,
                    0
                  )}
                </span>
              </div>
            </div>

            <div className="economic-impact">
              <span className="text-sm font-medium text-charcoal-700 dark:text-charcoal-300">
                Material Waste
              </span>
              <div className="economic-range">
                <span className="economic-range-low">
                  {formatCurrency(
                    result.economic_analysis.material_waste.low_estimate,
                    result.economic_analysis.material_waste.currency,
                    0
                  )}
                </span>
                <span className="economic-range-central">
                  {formatCurrency(
                    result.economic_analysis.material_waste.central_estimate,
                    result.economic_analysis.material_waste.currency,
                    0
                  )}
                </span>
                <span className="economic-range-high">
                  {formatCurrency(
                    result.economic_analysis.material_waste.high_estimate,
                    result.economic_analysis.material_waste.currency,
                    0
                  )}
                </span>
              </div>
            </div>

            <div className="pt-3 border-t border-charcoal-200 dark:border-charcoal-700">
              <div className="economic-impact">
                <span className="text-sm font-bold text-charcoal-900 dark:text-charcoal-100">
                  Total Impact
                </span>
                <div className="economic-range">
                  <span className="economic-range-low">
                    {formatCurrency(
                      result.economic_analysis.total_impact.low_estimate,
                      result.economic_analysis.total_impact.currency,
                      0
                    )}
                  </span>
                  <span className="economic-range-central">
                    {formatCurrency(
                      result.economic_analysis.total_impact.central_estimate,
                      result.economic_analysis.total_impact.currency,
                      0
                    )}
                  </span>
                  <span className="economic-range-high">
                    {formatCurrency(
                      result.economic_analysis.total_impact.high_estimate,
                      result.economic_analysis.total_impact.currency,
                      0
                    )}
                  </span>
                </div>
              </div>
            </div>
          </div>
          <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-4 italic">
            These are estimates with uncertainty bounds. NOT accounting-grade
            figures.
          </p>
        </div>
      )}

      {/* Tabs for Additional Analysis */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft overflow-hidden">
        <div className="border-b border-charcoal-200 dark:border-charcoal-700">
          <nav className="flex -mb-px">
            <button
              onClick={() => setActiveTab("overview")}
              className={`px-6 py-3 text-sm font-medium border-b-2 transition-colors ${
                activeTab === "overview"
                  ? "border-steel-600 text-steel-600 dark:text-steel-400"
                  : "border-transparent text-charcoal-600 dark:text-charcoal-400 hover:text-charcoal-900 dark:hover:text-charcoal-200"
              }`}
            >
              Overview
            </button>

            <button
              onClick={() => setActiveTab("loss-tree")}
              className={`px-6 py-3 text-sm font-medium border-b-2 transition-colors ${
                activeTab === "loss-tree"
                  ? "border-steel-600 text-steel-600 dark:text-steel-400"
                  : "border-transparent text-charcoal-600 dark:text-charcoal-400 hover:text-charcoal-900 dark:hover:text-charcoal-200"
              }`}
            >
              Loss Tree
            </button>

            {leverage && (
              <button
                onClick={() => setActiveTab("leverage")}
                className={`px-6 py-3 text-sm font-medium border-b-2 transition-colors ${
                  activeTab === "leverage"
                    ? "border-steel-600 text-steel-600 dark:text-steel-400"
                    : "border-transparent text-charcoal-600 dark:text-charcoal-400 hover:text-charcoal-900 dark:hover:text-charcoal-200"
                }`}
              >
                Leverage
              </button>
            )}

            {sensitivity && (
              <button
                onClick={() => setActiveTab("sensitivity")}
                className={`px-6 py-3 text-sm font-medium border-b-2 transition-colors ${
                  activeTab === "sensitivity"
                    ? "border-steel-600 text-steel-600 dark:text-steel-400"
                    : "border-transparent text-charcoal-600 dark:text-charcoal-400 hover:text-charcoal-900 dark:hover:text-charcoal-200"
                }`}
              >
                Sensitivity
              </button>
            )}
          </nav>
        </div>

        <div className="p-6">
          {activeTab === "overview" && <OverviewTab result={result} />}
          {activeTab === "loss-tree" && (
            <LossTreeTab lossTree={result.loss_tree} />
          )}
          {activeTab === "leverage" && leverage && (
            <LeverageTab leverage={leverage} />
          )}
          {activeTab === "sensitivity" && sensitivity && (
            <SensitivityTab sensitivity={sensitivity} />
          )}
        </div>
      </div>
    </div>
  );
};

// Overview Tab
const OverviewTab: React.FC<{ result: OeeResult }> = ({ result }) => {
  return (
    <div className="space-y-4">
      <h4 className="font-medium text-charcoal-900 dark:text-charcoal-100 mb-3">
        Formula Breakdown
      </h4>

      <div className="formula-display">
        OEE = Availability × Performance × Quality
        <br />
        OEE = {(result.core_metrics.availability.value * 100).toFixed(1)}% ×{" "}
        {(result.core_metrics.performance.value * 100).toFixed(1)}% ×{" "}
        {(result.core_metrics.quality.value * 100).toFixed(1)}%
        <br />
        OEE = {(result.core_metrics.oee.value * 100).toFixed(1)}%
      </div>

      <div className="mt-6">
        <h5 className="text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
          Confidence Level
        </h5>
        <div
          className={`inline-flex items-center gap-2 px-3 py-1 rounded-full ${
            result.core_metrics.oee.confidence === "High"
              ? "bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400"
              : result.core_metrics.oee.confidence === "Medium"
              ? "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400"
              : "bg-red-100 text-red-800 dark:bg-red-900/20 dark:text-red-400"
          }`}
        >
          <span className="font-semibold">
            {result.core_metrics.oee.confidence}
          </span>
        </div>
        <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-2">
          Based on input source quality (Explicit/Inferred/Default)
        </p>
      </div>
    </div>
  );
};

// Loss Tree Tab
const LossTreeTab: React.FC<{ lossTree: any }> = ({ lossTree }) => {
  return (
    <div>
      <p className="text-sm text-charcoal-600 dark:text-charcoal-400 mb-4 italic">
        This is attribution, not causality. These categories represent where
        impact accumulates, not why it exists.
      </p>

      <div className="loss-tree">
        <LossTreeNode node={lossTree.root} level={0} />
      </div>
    </div>
  );
};

// Recursive Loss Tree Node
const LossTreeNode: React.FC<{ node: any; level: number }> = ({
  node,
  level,
}) => {
  const [expanded, setExpanded] = useState(level < 2);

  return (
    <div className={`${level > 0 ? "loss-tree-children" : ""}`}>
      <div
        className="loss-tree-node loss-tree-node--expandable"
        onClick={() => setExpanded(!expanded)}
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            {node.children && node.children.length > 0 && (
              <svg
                className={`w-4 h-4 transition-transform ${
                  expanded ? "rotate-90" : ""
                }`}
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 5l7 7-7 7"
                />
              </svg>
            )}
            <span className="category-label">{node.category_key}</span>
          </div>
          <div className="flex items-center gap-3">
            <span className="duration text-sm">
              {formatDuration(node.duration, { compact: true, maxParts: 2 })}
            </span>
            <span className="percentage text-sm text-charcoal-600 dark:text-charcoal-400">
              {(node.percentage_of_planned * 100).toFixed(1)}%
            </span>
          </div>
        </div>
      </div>

      {expanded && node.children && node.children.length > 0 && (
        <div className="mt-2">
          {node.children.map((child: any, idx: number) => (
            <LossTreeNode key={idx} node={child} level={level + 1} />
          ))}
        </div>
      )}
    </div>
  );
};

// Leverage Tab
const LeverageTab: React.FC<{ leverage: LeverageResponse }> = ({
  leverage,
}) => {
  return (
    <div className="space-y-4">
      <div className="bg-blue-50 dark:bg-blue-900/20 border-l-4 border-blue-400 p-4 mb-4">
        <p className="text-sm text-blue-800 dark:text-blue-400">
          <strong>Theoretical Impact Analysis</strong> - These show potential if
          categories were eliminated. This is "if X were reduced", never
          "should".
        </p>
      </div>

      <div className="space-y-3">
        {leverage.leverage_impacts.map((impact, idx) => (
          <div
            key={idx}
            className="border border-charcoal-200 dark:border-charcoal-700 rounded-lg p-4"
          >
            <div className="flex items-center justify-between mb-2">
              <span className="font-medium text-charcoal-900 dark:text-charcoal-100">
                {impact.category_key}
              </span>
              <span className="text-xl font-display font-bold text-steel-600 dark:text-steel-400">
                +{impact.oee_opportunity_points.toFixed(1)}%
              </span>
            </div>
            <div className="text-sm text-charcoal-600 dark:text-charcoal-400">
              Potential throughput gain:{" "}
              <span className="font-semibold">
                {impact.throughput_gain_units} units
              </span>
            </div>
            <div className="mt-2 text-xs text-charcoal-500 dark:text-charcoal-400">
              Sensitivity: {(impact.sensitivity_score * 100).toFixed(0)}%
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

// Sensitivity Tab
const SensitivityTab: React.FC<{ sensitivity: SensitivityAnalysis }> = ({
  sensitivity,
}) => {
  return (
    <div className="space-y-4">
      <p className="text-sm text-charcoal-600 dark:text-charcoal-400 italic">
        Shows how OEE changes with ±10% variations in key parameters
      </p>

      <div className="space-y-2">
        {Object.entries(sensitivity.factors).map(([param, data]) => (
          <div key={param} className="sensitivity-impact">
            <div className="font-medium text-charcoal-900 dark:text-charcoal-100">
              {param.replace(/_/g, " ")}
            </div>
            <div className="flex-1">
              <div className="sensitivity-bar">
                <div
                  className="sensitivity-bar-fill"
                  style={{
                    width: `${Math.abs(data.impact)}%`,
                    left:
                      data.impact > 0
                        ? "50%"
                        : `${50 - Math.abs(data.impact)}%`,
                    backgroundColor: data.impact > 0 ? "#10b981" : "#ef4444",
                  }}
                />
              </div>
              <div className="flex justify-between text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
                <span>
                  Impact: {data.impact > 0 ? "+" : ""}
                  {data.impact.toFixed(1)}%
                </span>
                <span>Sensitivity: {(data.sensitivity * 100).toFixed(0)}%</span>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ResultsView;
