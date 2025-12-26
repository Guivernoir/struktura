/**
 * AssumptionsView Component
 *
 * Layout orchestration for assumption declaration and interrogation.
 * Composes AssumptionPanel components for different domains.
 *
 * CRITICAL: This is layout only. No calculation logic. No API calls.
 */

import React, { useState } from "react";
import AssumptionPanel from "../components/assumptions/AssumptionPanel";
import AssumptionField from "../components/assumptions/AssumptionField";
import ValidationBanner from "../components/validation/ValidationBanner";
import type { OeeInput, ValidationResult } from "../models";
import { MachineState, ImpactLevel } from "../models";
import { getInputValue, explicit } from "../models/input";
import { MathHints } from "../utils";

export interface AssumptionsViewProps {
  input: OeeInput | null;
  validation: ValidationResult | null;
  onChange: (input: OeeInput) => void;
  onCalculate: () => void;
  isCalculating: boolean;
}

const AssumptionsView: React.FC<AssumptionsViewProps> = ({
  input,
  validation,
  onChange,
  onCalculate,
  isCalculating,
}) => {
  // Panel expansion state
  const [expandedPanels, setExpandedPanels] = useState<Set<string>>(
    new Set(["time", "production", "cycle"])
  );

  // Helper to convert from Rust InputValue format to component format
  const convertInputValue = <T,>(
    rustValue: import("../models/input").InputValue<T>
  ): import("../models").InputValue<T> => {
    if ("Explicit" in rustValue) {
      return { type: "Explicit", value: rustValue.Explicit };
    }
    if ("Inferred" in rustValue) {
      return { type: "Inferred", value: rustValue.Inferred };
    }
    if ("Default" in rustValue) {
      return { type: "Default", value: rustValue.Default };
    }
    throw new Error("Invalid InputValue format");
  };

  const togglePanel = (panelId: string) => {
    setExpandedPanels((prev) => {
      const next = new Set(prev);
      if (next.has(panelId)) {
        next.delete(panelId);
      } else {
        next.add(panelId);
      }
      return next;
    });
  };

  if (!input) {
    return (
      <div className="text-center py-12">
        <p className="text-charcoal-600 dark:text-charcoal-400">
          No input data available. Initialize input to begin.
        </p>
      </div>
    );
  }

  // Generate validation hints
  const hints = MathHints.checkAllInputWarnings(input);
  const timeHints = hints.filter((h) => h.field === "time");
  const productionHints = hints.filter((h) => h.field === "production");
  const cycleHints = hints.filter((h) => h.field === "cycle_time");
  const downtimeHints = hints.filter((h) => h.field === "downtime");

  // Helper to update input
  const updateField = <K extends keyof OeeInput>(
    field: K,
    value: OeeInput[K]
  ) => {
    onChange({ ...input, [field]: value });
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="section-heading">Assumptions</h2>
          <p className="helper-text">
            Declare your curated inputs. Each value tracks its source
            (explicit/inferred/default).
          </p>
        </div>
        <button
          onClick={onCalculate}
          disabled={isCalculating}
          className="px-6 py-3 bg-steel-600 hover:bg-steel-700 text-white rounded-lg
                   font-semibold transition-colors disabled:opacity-50 disabled:cursor-not-allowed
                   flex items-center gap-2"
        >
          {isCalculating ? (
            <>
              <div className="oee-spinner w-4 h-4" />
              <span>Calculating...</span>
            </>
          ) : (
            <>
              <span>⚙</span>
              <span>Calculate OEE</span>
            </>
          )}
        </button>
      </div>

      {/* Global Validation Banner */}
      {validation && <ValidationBanner validation={validation} />}

      {/* Time Model Panel */}
      <AssumptionPanel
        title="Time Model"
        description="Planned time and state allocations"
        icon="⏱️"
        warnings={timeHints}
        expanded={expandedPanels.has("time")}
        onToggle={() => togglePanel("time")}
      >
        <AssumptionField
          label="Planned Production Time"
          value={convertInputValue(input.time_model.planned_production_time)}
          unit="seconds"
          helpText="Total available time for production in this period"
          impact={ImpactLevel.Critical}
          type="duration"
          onChange={(value) =>
            updateField("time_model", {
              ...input.time_model,
              planned_production_time: explicit(value),
            })
          }
        />

        {input.time_model.all_time && (
          <AssumptionField
            label="All Time (24/7)"
            value={convertInputValue(input.time_model.all_time)}
            unit="seconds"
            helpText="Calendar time for TEEP calculation"
            impact={ImpactLevel.High}
            type="duration"
            onChange={(value) =>
              updateField("time_model", {
                ...input.time_model,
                all_time: explicit(value),
              })
            }
          />
        )}

        <div className="mt-4 text-sm text-charcoal-600 dark:text-charcoal-400">
          <p className="font-semibold mb-2">Time Allocations:</p>
          <p className="text-xs">
            {input.time_model.allocations.length} allocation(s) defined.
            Running:{" "}
            {input.time_model.allocations
              .filter((a) => a.state === MachineState.Running)
              .reduce((sum, a) => sum + getInputValue(a.duration), 0)}{" "}
            seconds, Stopped:{" "}
            {input.time_model.allocations
              .filter((a) => a.state === MachineState.Stopped)
              .reduce((sum, a) => sum + getInputValue(a.duration), 0)}{" "}
            seconds
          </p>
        </div>
      </AssumptionPanel>

      {/* Production Counts Panel */}
      <AssumptionPanel
        title="Production Counts"
        description="Unit counts by category"
        icon="📦"
        warnings={productionHints}
        expanded={expandedPanels.has("production")}
        onToggle={() => togglePanel("production")}
      >
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <AssumptionField
            label="Total Units"
            value={convertInputValue(input.production.total_units)}
            unit="units"
            helpText="All units produced (good + scrap + rework)"
            impact={ImpactLevel.Critical}
            type="number"
            onChange={(value) =>
              updateField("production", {
                ...input.production,
                total_units: explicit(value),
              })
            }
          />

          <AssumptionField
            label="Good Units"
            value={convertInputValue(input.production.good_units)}
            unit="units"
            helpText="Units that passed quality inspection"
            impact={ImpactLevel.Critical}
            type="number"
            onChange={(value) =>
              updateField("production", {
                ...input.production,
                good_units: explicit(value),
              })
            }
          />

          <AssumptionField
            label="Scrap Units"
            value={convertInputValue(input.production.scrap_units)}
            unit="units"
            helpText="Units that cannot be recovered"
            impact={ImpactLevel.High}
            type="number"
            onChange={(value) =>
              updateField("production", {
                ...input.production,
                scrap_units: explicit(value),
              })
            }
          />

          <AssumptionField
            label="Reworked Units"
            value={convertInputValue(input.production.reworked_units)}
            unit="units"
            helpText="Units that required rework"
            impact={ImpactLevel.Medium}
            type="number"
            onChange={(value) =>
              updateField("production", {
                ...input.production,
                reworked_units: explicit(value),
              })
            }
          />
        </div>
      </AssumptionPanel>

      {/* Cycle Time Panel */}
      <AssumptionPanel
        title="Cycle Time Model"
        description="Ideal and observed cycle times"
        icon="⚡"
        warnings={cycleHints}
        expanded={expandedPanels.has("cycle")}
        onToggle={() => togglePanel("cycle")}
      >
        <AssumptionField
          label="Ideal Cycle Time"
          value={convertInputValue(input.cycle_time.ideal_cycle_time)}
          unit="seconds/unit"
          helpText="Theoretical minimum time per unit (design spec)"
          impact={ImpactLevel.Critical}
          type="number"
          step={0.1}
          onChange={(value) =>
            updateField("cycle_time", {
              ...input.cycle_time,
              ideal_cycle_time: explicit(value),
            })
          }
        />

        {input.cycle_time.average_cycle_time && (
          <AssumptionField
            label="Average Cycle Time"
            value={convertInputValue(input.cycle_time.average_cycle_time)}
            unit="seconds/unit"
            helpText="Observed average time per unit"
            impact={ImpactLevel.High}
            type="number"
            step={0.1}
            onChange={(value) =>
              updateField("cycle_time", {
                ...input.cycle_time,
                average_cycle_time: explicit(value),
              })
            }
          />
        )}
      </AssumptionPanel>

      {/* Downtime Panel */}
      <AssumptionPanel
        title="Downtime Events"
        description="Categorized downtime records"
        icon="🔧"
        warnings={downtimeHints}
        expanded={expandedPanels.has("downtime")}
        onToggle={() => togglePanel("downtime")}
      >
        <div className="text-sm text-charcoal-700 dark:text-charcoal-300">
          <p className="font-semibold mb-2">Downtime Summary:</p>
          <div className="space-y-1">
            <p>Total Events: {input.downtimes.records.length}</p>
            <p>
              Total Duration:{" "}
              {input.downtimes.records.reduce(
                (sum, dt) => sum + getInputValue(dt.duration),
                0
              )}{" "}
              seconds
            </p>
          </div>
        </div>

        {input.downtimes.records.length === 0 && (
          <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-md p-4 mt-4">
            <p className="text-sm text-blue-700 dark:text-blue-400">
              No downtime records provided. Availability will be calculated from
              time allocations.
            </p>
          </div>
        )}
      </AssumptionPanel>

      {/* Thresholds Panel */}
      <AssumptionPanel
        title="Thresholds & Configuration"
        description="Loss categorization thresholds"
        icon="⚙️"
        expanded={expandedPanels.has("thresholds")}
        onToggle={() => togglePanel("thresholds")}
      >
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              Micro Stoppage Threshold
            </label>
            <input
              type="number"
              value={input.thresholds.micro_stoppage_threshold}
              onChange={(e) =>
                updateField("thresholds", {
                  ...input.thresholds,
                  micro_stoppage_threshold: parseFloat(e.target.value) || 0,
                })
              }
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
            />
            <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
              Minimum duration (seconds) to count as downtime
            </p>
          </div>

          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              Small Stop Threshold
            </label>
            <input
              type="number"
              value={input.thresholds.small_stop_threshold}
              onChange={(e) =>
                updateField("thresholds", {
                  ...input.thresholds,
                  small_stop_threshold: parseFloat(e.target.value) || 0,
                })
              }
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
            />
            <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
              Maximum duration (seconds) for small stop category
            </p>
          </div>

          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              Speed Loss Threshold
            </label>
            <input
              type="number"
              step={0.01}
              value={input.thresholds.speed_loss_threshold}
              onChange={(e) =>
                updateField("thresholds", {
                  ...input.thresholds,
                  speed_loss_threshold: parseFloat(e.target.value) || 0,
                })
              }
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
            />
            <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
              % below ideal speed to trigger warning (0.05 = 5%)
            </p>
          </div>

          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              High Scrap Rate Threshold
            </label>
            <input
              type="number"
              step={0.01}
              value={input.thresholds.high_scrap_rate_threshold}
              onChange={(e) =>
                updateField("thresholds", {
                  ...input.thresholds,
                  high_scrap_rate_threshold: parseFloat(e.target.value) || 0,
                })
              }
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
            />
            <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
              Scrap rate (0.20 = 20%) to trigger warning
            </p>
          </div>
        </div>
      </AssumptionPanel>

      {/* Machine Context */}
      <div className="bg-sand-50 dark:bg-charcoal-800 rounded-lg p-4">
        <h4 className="text-sm font-semibold text-charcoal-700 dark:text-charcoal-300 mb-3">
          Machine Context
        </h4>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-3 text-sm">
          <div>
            <span className="text-charcoal-500 dark:text-charcoal-400">
              Machine ID:
            </span>
            <div className="font-mono font-semibold">
              {input.machine.machine_id}
            </div>
          </div>
          {input.machine.line_id && (
            <div>
              <span className="text-charcoal-500 dark:text-charcoal-400">
                Line ID:
              </span>
              <div className="font-mono font-semibold">
                {input.machine.line_id}
              </div>
            </div>
          )}
          {input.machine.product_id && (
            <div>
              <span className="text-charcoal-500 dark:text-charcoal-400">
                Product ID:
              </span>
              <div className="font-mono font-semibold">
                {input.machine.product_id}
              </div>
            </div>
          )}
          {input.machine.shift_id && (
            <div>
              <span className="text-charcoal-500 dark:text-charcoal-400">
                Shift ID:
              </span>
              <div className="font-mono font-semibold">
                {input.machine.shift_id}
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Analysis Window */}
      <div className="bg-sand-50 dark:bg-charcoal-800 rounded-lg p-4">
        <h4 className="text-sm font-semibold text-charcoal-700 dark:text-charcoal-300 mb-3">
          Analysis Window
        </h4>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-sm">
          <div>
            <span className="text-charcoal-500 dark:text-charcoal-400">
              Start:
            </span>
            <div className="font-mono">
              {new Date(input.window.start).toLocaleString()}
            </div>
          </div>
          <div>
            <span className="text-charcoal-500 dark:text-charcoal-400">
              End:
            </span>
            <div className="font-mono">
              {new Date(input.window.end).toLocaleString()}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AssumptionsView;
