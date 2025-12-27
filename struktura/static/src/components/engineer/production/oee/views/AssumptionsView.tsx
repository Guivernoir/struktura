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
    new Set(["window", "machine", "time", "production", "cycle"])
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

  // Helper to get default input when none exists
  const getDefaultInput = (): OeeInput => {
    const now = new Date();
    const shiftStart = new Date(now);
    shiftStart.setHours(8, 0, 0, 0);
    const shiftEnd = new Date(shiftStart);
    shiftEnd.setHours(16, 0, 0, 0);

    return {
      window: {
        start: shiftStart.toISOString(),
        end: shiftEnd.toISOString(),
      },
      machine: {
        machine_id: "",
        line_id: "",
        product_id: "",
        shift_id: "",
      },
      time_model: {
        planned_production_time: explicit(0),
        allocations: [
          {
            state: MachineState.Running,
            duration: explicit(0),
            reason: null,
            notes: null,
          },
        ],
        all_time: explicit(86400), // 24 hours for TEEP
      },
      production: {
        total_units: explicit(0),
        good_units: explicit(0),
        scrap_units: explicit(0),
        reworked_units: explicit(0),
      },
      cycle_time: {
        ideal_cycle_time: explicit(0),
        average_cycle_time: explicit(0),
      },
      downtimes: {
        records: [],
      },
      thresholds: {
        micro_stoppage_threshold: 30,
        small_stop_threshold: 300,
        speed_loss_threshold: 0.05,
        high_scrap_rate_threshold: 0.2,
        low_utilization_threshold: 0.3,
      },
    };
  };

  // Use existing input or create empty structure
  const currentInput = input || getDefaultInput();

  // Generate validation hints only if we have input
  const hints = input ? MathHints.checkAllInputWarnings(input) : [];
  const timeHints = hints.filter((h) => h.field === "time");
  const productionHints = hints.filter((h) => h.field === "production");
  const cycleHints = hints.filter((h) => h.field === "cycle_time");
  const downtimeHints = hints.filter((h) => h.field === "downtime");

  // Helper to update input (creates input if it doesn't exist)
  const updateField = <K extends keyof OeeInput>(
    field: K,
    value: OeeInput[K]
  ) => {
    onChange({ ...currentInput, [field]: value });
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

      {/* Welcome Banner for New Input */}
      {!input && (
        <div className="bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-blue-950/30 dark:to-indigo-950/30 border border-blue-200 dark:border-blue-800 rounded-lg p-6 mb-6">
          <div className="flex items-start gap-4">
            <div className="text-4xl">📝</div>
            <div className="flex-1">
              <h3 className="text-lg font-semibold text-blue-900 dark:text-blue-100 mb-2">
                Start Your OEE Analysis
              </h3>
              <p className="text-sm text-blue-700 dark:text-blue-300 mb-3">
                Enter your production data below. All fields are editable -
                start by filling in the basic information like time window,
                machine context, and production counts.
              </p>
              <div className="flex items-center gap-2 text-xs text-blue-600 dark:text-blue-400">
                <span>💡 Tip:</span>
                <span>
                  You can also load sample data from the Quick Start section
                  above
                </span>
              </div>
            </div>
          </div>
        </div>
      )}

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
          value={convertInputValue(
            currentInput.time_model.planned_production_time
          )}
          unit="seconds"
          helpText="Total available time for production in this period"
          impact={ImpactLevel.Critical}
          type="duration"
          onChange={(value) =>
            updateField("time_model", {
              ...currentInput.time_model,
              planned_production_time: explicit(value),
            })
          }
        />

        {currentInput.time_model.all_time && (
          <AssumptionField
            label="All Time (24/7)"
            value={convertInputValue(currentInput.time_model.all_time)}
            unit="seconds"
            helpText="Calendar time for TEEP calculation"
            impact={ImpactLevel.High}
            type="duration"
            onChange={(value) =>
              updateField("time_model", {
                ...currentInput.time_model,
                all_time: explicit(value),
              })
            }
          />
        )}

        <div className="mt-4 text-sm text-charcoal-600 dark:text-charcoal-400">
          <p className="font-semibold mb-2">Time Allocations:</p>
          <p className="text-xs">
            {currentInput.time_model.allocations.length} allocation(s) defined.
            Running:{" "}
            {currentInput.time_model.allocations
              .filter((a) => a.state === MachineState.Running)
              .reduce((sum, a) => sum + getInputValue(a.duration), 0)}{" "}
            seconds, Stopped:{" "}
            {currentInput.time_model.allocations
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
            value={convertInputValue(currentInput.production.total_units)}
            unit="units"
            helpText="All units produced (good + scrap + rework)"
            impact={ImpactLevel.Critical}
            type="number"
            onChange={(value) =>
              updateField("production", {
                ...currentInput.production,
                total_units: explicit(value),
              })
            }
          />

          <AssumptionField
            label="Good Units"
            value={convertInputValue(currentInput.production.good_units)}
            unit="units"
            helpText="Units that passed quality inspection"
            impact={ImpactLevel.Critical}
            type="number"
            onChange={(value) =>
              updateField("production", {
                ...currentInput.production,
                good_units: explicit(value),
              })
            }
          />

          <AssumptionField
            label="Scrap Units"
            value={convertInputValue(currentInput.production.scrap_units)}
            unit="units"
            helpText="Units that cannot be recovered"
            impact={ImpactLevel.High}
            type="number"
            onChange={(value) =>
              updateField("production", {
                ...currentInput.production,
                scrap_units: explicit(value),
              })
            }
          />

          <AssumptionField
            label="Reworked Units"
            value={convertInputValue(currentInput.production.reworked_units)}
            unit="units"
            helpText="Units that required rework"
            impact={ImpactLevel.Medium}
            type="number"
            onChange={(value) =>
              updateField("production", {
                ...currentInput.production,
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
          value={convertInputValue(currentInput.cycle_time.ideal_cycle_time)}
          unit="seconds/unit"
          helpText="Theoretical minimum time per unit (design spec)"
          impact={ImpactLevel.Critical}
          type="number"
          step={0.1}
          onChange={(value) =>
            updateField("cycle_time", {
              ...currentInput.cycle_time,
              ideal_cycle_time: explicit(value),
            })
          }
        />

        {currentInput.cycle_time.average_cycle_time && (
          <AssumptionField
            label="Average Cycle Time"
            value={convertInputValue(
              currentInput.cycle_time.average_cycle_time
            )}
            unit="seconds/unit"
            helpText="Observed average time per unit"
            impact={ImpactLevel.High}
            type="number"
            step={0.1}
            onChange={(value) =>
              updateField("cycle_time", {
                ...currentInput.cycle_time,
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
            <p>Total Events: {currentInput.downtimes.records.length}</p>
            <p>
              Total Duration:{" "}
              {currentInput.downtimes.records.reduce(
                (sum, dt) => sum + getInputValue(dt.duration),
                0
              )}{" "}
              seconds
            </p>
          </div>
        </div>

        {currentInput.downtimes.records.length === 0 && (
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
              value={currentInput.thresholds.micro_stoppage_threshold}
              onChange={(e) =>
                updateField("thresholds", {
                  ...currentInput.thresholds,
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
              value={currentInput.thresholds.small_stop_threshold}
              onChange={(e) =>
                updateField("thresholds", {
                  ...currentInput.thresholds,
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
              value={currentInput.thresholds.speed_loss_threshold}
              onChange={(e) =>
                updateField("thresholds", {
                  ...currentInput.thresholds,
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
              value={currentInput.thresholds.high_scrap_rate_threshold}
              onChange={(e) =>
                updateField("thresholds", {
                  ...currentInput.thresholds,
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
      <AssumptionPanel
        title="Machine Context"
        description="Machine, line, product, and shift identifiers"
        icon="🏭"
        expanded={expandedPanels.has("machine")}
        onToggle={() => togglePanel("machine")}
      >
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              Machine ID
            </label>
            <input
              type="text"
              value={currentInput.machine.machine_id}
              onChange={(e) =>
                updateField("machine", {
                  ...currentInput.machine,
                  machine_id: e.target.value,
                })
              }
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
              placeholder="e.g., M-001"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              Line ID
            </label>
            <input
              type="text"
              value={currentInput.machine.line_id || ""}
              onChange={(e) =>
                updateField("machine", {
                  ...currentInput.machine,
                  line_id: e.target.value,
                })
              }
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
              placeholder="e.g., Line-A"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              Product ID
            </label>
            <input
              type="text"
              value={currentInput.machine.product_id || ""}
              onChange={(e) =>
                updateField("machine", {
                  ...currentInput.machine,
                  product_id: e.target.value,
                })
              }
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
              placeholder="e.g., PRODUCT-X"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              Shift ID
            </label>
            <input
              type="text"
              value={currentInput.machine.shift_id || ""}
              onChange={(e) =>
                updateField("machine", {
                  ...currentInput.machine,
                  shift_id: e.target.value,
                })
              }
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
              placeholder="e.g., SHIFT-1"
            />
          </div>
        </div>
      </AssumptionPanel>

      {/* Analysis Window */}
      <AssumptionPanel
        title="Analysis Window"
        description="Time period for this OEE calculation"
        icon="📅"
        expanded={expandedPanels.has("window")}
        onToggle={() => togglePanel("window")}
      >
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              Start Time
            </label>
            <input
              type="datetime-local"
              value={new Date(currentInput.window.start)
                .toISOString()
                .slice(0, 16)}
              onChange={(e) => {
                const newStart = new Date(e.target.value).toISOString();
                updateField("window", {
                  ...currentInput.window,
                  start: newStart,
                });
              }}
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
            />
            <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
              Beginning of the analysis period
            </p>
          </div>

          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              End Time
            </label>
            <input
              type="datetime-local"
              value={new Date(currentInput.window.end)
                .toISOString()
                .slice(0, 16)}
              onChange={(e) => {
                const newEnd = new Date(e.target.value).toISOString();
                updateField("window", {
                  ...currentInput.window,
                  end: newEnd,
                });
              }}
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
            />
            <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
              End of the analysis period
            </p>
          </div>
        </div>
        <div className="mt-4 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-md">
          <p className="text-sm text-blue-700 dark:text-blue-400">
            Duration:{" "}
            {(
              (new Date(currentInput.window.end).getTime() -
                new Date(currentInput.window.start).getTime()) /
              1000
            ).toFixed(0)}{" "}
            seconds (
            {(
              (new Date(currentInput.window.end).getTime() -
                new Date(currentInput.window.start).getTime()) /
              3600000
            ).toFixed(2)}{" "}
            hours)
          </p>
        </div>
      </AssumptionPanel>
    </div>
  );
};

export default AssumptionsView;
