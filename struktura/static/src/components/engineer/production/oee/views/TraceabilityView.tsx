/**
 * TraceabilityView Component
 *
 * Layout orchestration for assumption ledger and audit trail.
 * This is the trust anchor of the entire application.
 *
 * CRITICAL: If this view is weak, the entire product is weak.
 */

import React from "react";
import AssumptionLedger from "../components/ledger/AssumptionLedger";
import type { AssumptionLedger as AssumptionLedgerType } from "../models";

export interface TraceabilityViewProps {
  ledger: AssumptionLedgerType;
}

const TraceabilityView: React.FC<TraceabilityViewProps> = ({ ledger }) => {
  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h2 className="section-heading">Traceability</h2>
        <p className="helper-text">
          Complete audit trail for every assumption in this analysis. This is
          your trust anchor.
        </p>
      </div>

      {/* Trust Statement */}
      <div className="bg-gradient-to-br from-steel-50 to-sand-50 dark:from-charcoal-800 dark:to-charcoal-900 rounded-lg shadow-medium p-6 border-2 border-steel-300 dark:border-steel-700">
        <h3 className="text-lg font-display font-bold text-charcoal-900 dark:text-charcoal-100 mb-3">
          🎯 Traceability Guarantee
        </h3>
        <div className="space-y-2 text-sm text-charcoal-700 dark:text-charcoal-300">
          <p>
            Every metric in this analysis can answer:{" "}
            <strong>"How was this calculated?"</strong> and{" "}
            <strong>"Which assumptions does this depend on?"</strong>
          </p>
          <p className="italic">
            If a result cannot be traced back to its inputs, it does not belong
            in this component.
          </p>
        </div>
      </div>

      {/* Ledger */}
      <AssumptionLedger ledger={ledger} />

      {/* Metadata Section */}
      {Object.keys(ledger.metadata).length > 0 && (
        <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
          <h3 className="section-subheading mb-4">Analysis Metadata</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {Object.entries(ledger.metadata).map(([key, value]) => (
              <div key={key} className="border-l-2 border-steel-400 pl-3">
                <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide">
                  {key}
                </div>
                <div className="font-mono text-sm font-semibold text-charcoal-900 dark:text-charcoal-100 mt-1">
                  {value}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Threshold Configuration */}
      {ledger.thresholds.length > 0 && (
        <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
          <h3 className="section-subheading mb-4">Applied Thresholds</h3>
          <div className="space-y-3">
            {ledger.thresholds.map((threshold, idx) => (
              <div
                key={idx}
                className="flex items-start justify-between p-3 bg-sand-50 dark:bg-charcoal-800 rounded-md"
              >
                <div className="flex-1">
                  <div className="font-medium text-sm text-charcoal-900 dark:text-charcoal-100">
                    {threshold.threshold_key}
                  </div>
                  <div className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
                    {threshold.rationale_key}
                  </div>
                </div>
                <div className="text-right ml-4">
                  <div className="font-semibold text-steel-600 dark:text-steel-400">
                    {threshold.value} {threshold.unit_key}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Reproducibility Statement */}
      <div className="bg-sand-50 dark:bg-charcoal-800 rounded-lg p-6">
        <h3 className="text-base font-display font-semibold text-charcoal-900 dark:text-charcoal-100 mb-3">
          🔒 Reproducibility
        </h3>
        <div className="space-y-2 text-sm text-charcoal-700 dark:text-charcoal-300">
          <p>
            This analysis is <strong>deterministic by construction</strong>:
          </p>
          <ul className="list-disc list-inside space-y-1 ml-4">
            <li>Same inputs → same outputs</li>
            <li>No time dependency</li>
            <li>No hidden state</li>
            <li>No stochastic behavior</li>
          </ul>
          <p className="mt-3 italic">
            Re-run this calculation with identical inputs and you will get
            identical results. Every time.
          </p>
        </div>
      </div>

      {/* Export Options */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
        <h3 className="section-subheading mb-4">Export Ledger</h3>
        <div className="flex gap-3 flex-wrap">
          <button
            onClick={() => {
              const json = JSON.stringify(ledger, null, 2);
              const blob = new Blob([json], { type: "application/json" });
              const url = URL.createObjectURL(blob);
              const a = document.createElement("a");
              a.href = url;
              a.download = `oee-ledger-${Date.now()}.json`;
              a.click();
              URL.revokeObjectURL(url);
            }}
            className="px-4 py-2 bg-steel-600 hover:bg-steel-700 text-white rounded-md 
                     font-medium text-sm transition-colors flex items-center gap-2"
          >
            <span>📄</span>
            <span>Export as JSON</span>
          </button>

          <button
            onClick={() => {
              // Generate CSV of assumptions
              const csv = [
                [
                  "Assumption Key",
                  "Description",
                  "Value",
                  "Source",
                  "Impact",
                  "Timestamp",
                ].join(","),
                ...ledger.assumptions.map((a) =>
                  [
                    a.assumption_key,
                    a.description_key,
                    JSON.stringify(a.value),
                    a.source,
                    a.impact,
                    a.timestamp,
                  ].join(",")
                ),
              ].join("\n");

              const blob = new Blob([csv], { type: "text/csv" });
              const url = URL.createObjectURL(blob);
              const a = document.createElement("a");
              a.href = url;
              a.download = `oee-assumptions-${Date.now()}.csv`;
              a.click();
              URL.revokeObjectURL(url);
            }}
            className="px-4 py-2 bg-sand-200 hover:bg-sand-300 dark:bg-charcoal-700 dark:hover:bg-charcoal-600 
                     text-charcoal-900 dark:text-charcoal-100 rounded-md font-medium text-sm 
                     transition-colors flex items-center gap-2"
          >
            <span>📊</span>
            <span>Export as CSV</span>
          </button>

          <button
            onClick={async () => {
              const text = JSON.stringify(ledger, null, 2);
              try {
                await navigator.clipboard.writeText(text);
                alert("Ledger copied to clipboard");
              } catch {
                alert("Failed to copy to clipboard");
              }
            }}
            className="px-4 py-2 bg-sand-200 hover:bg-sand-300 dark:bg-charcoal-700 dark:hover:bg-charcoal-600 
                     text-charcoal-900 dark:text-charcoal-100 rounded-md font-medium text-sm 
                     transition-colors flex items-center gap-2"
          >
            <span>📋</span>
            <span>Copy to Clipboard</span>
          </button>
        </div>
      </div>

      {/* Documentation Reference */}
      <div className="bg-blue-50 dark:bg-blue-900/20 border-l-4 border-blue-400 p-4 rounded-md">
        <h4 className="text-sm font-semibold text-blue-800 dark:text-blue-400 mb-2">
          📚 Using This Ledger
        </h4>
        <div className="text-sm text-blue-700 dark:text-blue-400 space-y-2">
          <p>This ledger serves as:</p>
          <ul className="list-disc list-inside ml-4 space-y-1">
            <li>Audit trail for regulatory compliance</li>
            <li>Input for post-mortem analysis</li>
            <li>Reference for training and documentation</li>
            <li>Baseline for scenario comparison</li>
          </ul>
          <p className="mt-3">
            Store this with your analysis results for complete traceability.
          </p>
        </div>
      </div>
    </div>
  );
};

export default TraceabilityView;
