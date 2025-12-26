/**
 * EconomicImpactPanel Component
 *
 * Displays economic ranges for loss impacts.
 * CRITICAL: Every economic number must scream "estimate".
 * These are NOT accounting-grade figures.
 */

import React from "react";
import EconomicRangeBar from "./EconomicRangeBar";
import type { EconomicAnalysis } from "../../models";
import { formatCurrency } from "../../utils";

export interface EconomicImpactPanelProps {
  analysis: EconomicAnalysis;
}

const EconomicImpactPanel: React.FC<EconomicImpactPanelProps> = ({
  analysis,
}) => {
  return (
    <div className="space-y-6">
      {/* Warning Banner */}
      <div className="bg-yellow-50 dark:bg-yellow-900/20 border-l-4 border-yellow-400 p-4 rounded-md">
        <div className="flex items-start gap-2">
          <svg
            className="w-5 h-5 text-yellow-600 dark:text-yellow-400 flex-shrink-0 mt-0.5"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fillRule="evenodd"
              d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
              clipRule="evenodd"
            />
          </svg>
          <div>
            <p className="text-sm font-semibold text-yellow-800 dark:text-yellow-400">
              Economic Estimates Only
            </p>
            <p className="text-xs text-yellow-700 dark:text-yellow-400 mt-1">
              These figures are user-parameterized estimates with uncertainty
              bounds. They are NOT financial statements or accounting-grade
              data.
            </p>
          </div>
        </div>
      </div>

      {/* Impact Categories */}
      <div className="space-y-4">
        {/* Throughput Loss */}
        <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
          <div className="flex items-start justify-between mb-4">
            <div>
              <h4 className="font-semibold text-charcoal-900 dark:text-charcoal-100">
                Throughput Loss
              </h4>
              <p className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
                {analysis.throughput_loss.description_key}
              </p>
            </div>
          </div>
          <EconomicRangeBar
            low={analysis.throughput_loss.low_estimate}
            central={analysis.throughput_loss.central_estimate}
            high={analysis.throughput_loss.high_estimate}
            currency={analysis.throughput_loss.currency}
          />
          {analysis.throughput_loss.assumptions.length > 0 && (
            <div className="mt-3 text-xs text-charcoal-500 dark:text-charcoal-400">
              <span className="font-semibold">Assumptions:</span>{" "}
              {analysis.throughput_loss.assumptions.join(", ")}
            </div>
          )}
        </div>

        {/* Material Waste */}
        <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
          <div className="flex items-start justify-between mb-4">
            <div>
              <h4 className="font-semibold text-charcoal-900 dark:text-charcoal-100">
                Material Waste
              </h4>
              <p className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
                {analysis.material_waste.description_key}
              </p>
            </div>
          </div>
          <EconomicRangeBar
            low={analysis.material_waste.low_estimate}
            central={analysis.material_waste.central_estimate}
            high={analysis.material_waste.high_estimate}
            currency={analysis.material_waste.currency}
          />
        </div>

        {/* Rework Cost */}
        <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
          <div className="flex items-start justify-between mb-4">
            <div>
              <h4 className="font-semibold text-charcoal-900 dark:text-charcoal-100">
                Rework Cost
              </h4>
              <p className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
                {analysis.rework_cost.description_key}
              </p>
            </div>
          </div>
          <EconomicRangeBar
            low={analysis.rework_cost.low_estimate}
            central={analysis.rework_cost.central_estimate}
            high={analysis.rework_cost.high_estimate}
            currency={analysis.rework_cost.currency}
          />
        </div>

        {/* Opportunity Cost */}
        <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
          <div className="flex items-start justify-between mb-4">
            <div>
              <h4 className="font-semibold text-charcoal-900 dark:text-charcoal-100">
                Opportunity Cost
              </h4>
              <p className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
                {analysis.opportunity_cost.description_key}
              </p>
            </div>
          </div>
          <EconomicRangeBar
            low={analysis.opportunity_cost.low_estimate}
            central={analysis.opportunity_cost.central_estimate}
            high={analysis.opportunity_cost.high_estimate}
            currency={analysis.opportunity_cost.currency}
          />
        </div>

        {/* Total Impact */}
        <div className="bg-gradient-to-br from-steel-50 to-sand-50 dark:from-charcoal-800 dark:to-charcoal-900 rounded-lg shadow-medium p-6 border-2 border-steel-300 dark:border-steel-700">
          <div className="flex items-start justify-between mb-4">
            <div>
              <h3 className="text-lg font-display font-bold text-charcoal-900 dark:text-charcoal-100">
                Total Economic Impact
              </h3>
              <p className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
                {analysis.total_impact.description_key}
              </p>
            </div>
          </div>
          <EconomicRangeBar
            low={analysis.total_impact.low_estimate}
            central={analysis.total_impact.central_estimate}
            high={analysis.total_impact.high_estimate}
            currency={analysis.total_impact.currency}
            isTotal
          />
        </div>
      </div>

      {/* Disclaimer */}
      <div className="bg-sand-50 dark:bg-charcoal-800 rounded-lg p-4">
        <p className="text-xs text-charcoal-600 dark:text-charcoal-400 italic">
          <strong>Important:</strong> These economic figures support
          prioritization, scenario comparison, and executive communication. They
          are estimates with uncertainty bounds and should not be treated as
          financial statements. Accuracy depends entirely on the quality of
          economic parameters provided.
        </p>
      </div>
    </div>
  );
};

export default EconomicImpactPanel;
