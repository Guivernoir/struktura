/**
 * Assumptions View
 *
 * Orchestrates assumption declaration and interrogation.
 * This view DOES NOT calculate anything - it only collects curated input.
 */

import React, { useState } from "react";
import type { OeeInput, EconomicParameters } from "../models";
import { DEFAULT_THRESHOLDS, InputValueHelpers, MachineState } from "../models";

interface AssumptionsViewProps {
  input: OeeInput | null;
  economicParams: EconomicParameters | null;
  config: {
    includeSensitivity: boolean;
    includeTemporalScrap: boolean;
    includeLeverage: boolean;
    sensitivityVariation: number;
  };
  onInputChange: (input: OeeInput) => void;
  onEconomicParamsChange: (params: EconomicParameters | null) => void;
  onToggleSensitivity: (enabled: boolean) => void;
  onToggleLeverage: (enabled: boolean) => void;
}

type AssumptionSection =
  | "time"
  | "production"
  | "cycle"
  | "downtime"
  | "economics"
  | "config";

const AssumptionsView: React.FC<AssumptionsViewProps> = ({
  input,
  economicParams,
  config,
  onInputChange,
  onEconomicParamsChange,
  onToggleSensitivity,
  onToggleLeverage,
}) => {
  const [expandedSection, setExpandedSection] =
    useState<AssumptionSection>("time");

  // Initialize empty input if none exists
  React.useEffect(() => {
    if (!input) {
      const now = new Date().toISOString();
      const startDate = new Date();
      startDate.setHours(8, 0, 0, 0);
      const endDate = new Date();
      endDate.setHours(16, 0, 0, 0);

      const newInput: OeeInput = {
        window: {
          start: startDate.toISOString(),
          end: endDate.toISOString(),
        },
        machine: {
          machine_id: "",
          line_id: null,
          product_id: null,
          shift_id: null,
        },
        time_model: {
          planned_production_time: InputValueHelpers.explicit(28800), // 8 hours
          allocations: [
            {
              state: MachineState.Running,
              duration: InputValueHelpers.explicit(25200), // 7 hours
              reason: null,
              notes: null,
            },
            {
              state: MachineState.Stopped,
              duration: InputValueHelpers.explicit(3600), // 1 hour
              reason: null,
              notes: null,
            },
          ],
          all_time: null,
        },
        production: {
          total_units: InputValueHelpers.explicit(0),
          good_units: InputValueHelpers.explicit(0),
          scrap_units: InputValueHelpers.explicit(0),
          reworked_units: InputValueHelpers.explicit(0),
        },
        cycle_time: {
          ideal_cycle_time: InputValueHelpers.explicit(25), // 25 seconds
          average_cycle_time: null,
        },
        downtimes: {
          records: [],
        },
        thresholds: DEFAULT_THRESHOLDS,
      };

      onInputChange(newInput);
    }
  }, [input, onInputChange]);

  if (!input) {
    return (
      <div className="oee-loading-text justify-center py-12">
        <div className="oee-spinner" />
        <span>Initializing assumptions...</span>
      </div>
    );
  }

  const sections = [
    { id: "time" as const, label: "Time Allocation", icon: "⏱️" },
    { id: "production" as const, label: "Production Counts", icon: "📊" },
    { id: "cycle" as const, label: "Cycle Time", icon: "🔄" },
    { id: "downtime" as const, label: "Downtime Records", icon: "🔴" },
    { id: "economics" as const, label: "Economic Parameters", icon: "💰" },
    { id: "config" as const, label: "Analysis Configuration", icon: "⚙️" },
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
        <h2 className="section-heading">Analyst-Curated Assumptions</h2>
        <p className="helper-text mb-4">
          Every value entered here is tracked with its source
          (Explicit/Inferred/Default). This is not data ingestion - this is
          structured reasoning.
        </p>

        {/* Warning Banner */}
        <div className="bg-yellow-50 dark:bg-yellow-900/20 border-l-4 border-yellow-400 p-4">
          <div className="flex">
            <div className="flex-shrink-0">
              <svg
                className="h-5 w-5 text-yellow-400"
                viewBox="0 0 20 20"
                fill="currentColor"
              >
                <path
                  fillRule="evenodd"
                  d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                  clipRule="evenodd"
                />
              </svg>
            </div>
            <div className="ml-3">
              <p className="text-sm text-yellow-700 dark:text-yellow-400">
                <strong>Reminder:</strong> If your inputs are wrong, your
                outputs will be wrong. That is intentional.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Machine Context */}
      <div className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft p-6">
        <h3 className="section-subheading mb-4">Machine Context</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              Machine ID *
            </label>
            <input
              type="text"
              value={input.machine.machine_id}
              onChange={(e) =>
                onInputChange({
                  ...input,
                  machine: { ...input.machine, machine_id: e.target.value },
                })
              }
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
              placeholder="M-001"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
              Line ID
            </label>
            <input
              type="text"
              value={input.machine.line_id || ""}
              onChange={(e) =>
                onInputChange({
                  ...input,
                  machine: {
                    ...input.machine,
                    line_id: e.target.value || null,
                  },
                })
              }
              className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                       rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
              placeholder="LINE-A"
            />
          </div>
        </div>
      </div>

      {/* Sections */}
      <div className="space-y-4">
        {sections.map((section) => (
          <div
            key={section.id}
            className="bg-white dark:bg-charcoal-900 rounded-lg shadow-soft overflow-hidden"
          >
            <button
              onClick={() =>
                setExpandedSection(
                  expandedSection === section.id ? (null as any) : section.id
                )
              }
              className="w-full px-6 py-4 flex items-center justify-between hover:bg-sand-50 dark:hover:bg-charcoal-800 transition-colors"
            >
              <div className="flex items-center gap-3">
                <span className="text-2xl">{section.icon}</span>
                <span className="font-medium text-charcoal-900 dark:text-charcoal-100">
                  {section.label}
                </span>
              </div>
              <svg
                className={`w-5 h-5 text-charcoal-600 dark:text-charcoal-400 transition-transform ${
                  expandedSection === section.id ? "rotate-180" : ""
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
            </button>

            {expandedSection === section.id && (
              <div className="px-6 py-4 border-t border-charcoal-200 dark:border-charcoal-700 animate-slide-down">
                {section.id === "time" && (
                  <TimeSection input={input} onChange={onInputChange} />
                )}
                {section.id === "production" && (
                  <ProductionSection input={input} onChange={onInputChange} />
                )}
                {section.id === "cycle" && (
                  <CycleTimeSection input={input} onChange={onInputChange} />
                )}
                {section.id === "downtime" && (
                  <DowntimeSection input={input} onChange={onInputChange} />
                )}
                {section.id === "economics" && (
                  <EconomicsSection
                    params={economicParams}
                    onChange={onEconomicParamsChange}
                  />
                )}
                {section.id === "config" && (
                  <ConfigSection
                    config={config}
                    onToggleSensitivity={onToggleSensitivity}
                    onToggleLeverage={onToggleLeverage}
                  />
                )}
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

// Time Section Component
const TimeSection: React.FC<{
  input: OeeInput;
  onChange: (input: OeeInput) => void;
}> = ({ input, onChange }) => {
  const plannedSeconds = InputValueHelpers.getValue(
    input.time_model.planned_production_time
  );
  const plannedHours = plannedSeconds / 3600;

  return (
    <div className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
          Planned Production Time (hours) *
          <span className="source-pill source-pill--explicit ml-2">
            Explicit
          </span>
        </label>
        <input
          type="number"
          step="0.5"
          value={plannedHours}
          onChange={(e) => {
            const hours = parseFloat(e.target.value) || 0;
            const seconds = hours * 3600;
            onChange({
              ...input,
              time_model: {
                ...input.time_model,
                planned_production_time: InputValueHelpers.explicit(seconds),
              },
            });
          }}
          className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                   rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
        />
        <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
          {plannedSeconds} seconds
        </p>
      </div>

      <p className="text-sm text-charcoal-600 dark:text-charcoal-400 italic">
        Time allocations and additional TEEP configuration can be added through
        advanced settings.
      </p>
    </div>
  );
};

// Production Section Component
const ProductionSection: React.FC<{
  input: OeeInput;
  onChange: (input: OeeInput) => void;
}> = ({ input, onChange }) => {
  const total = InputValueHelpers.getValue(input.production.total_units);
  const good = InputValueHelpers.getValue(input.production.good_units);
  const scrap = InputValueHelpers.getValue(input.production.scrap_units);
  const rework = InputValueHelpers.getValue(input.production.reworked_units);

  const sum = good + scrap + rework;
  const isConsistent = sum === total;

  return (
    <div className="space-y-4">
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
            Total Units *
          </label>
          <input
            type="number"
            value={total}
            onChange={(e) =>
              onChange({
                ...input,
                production: {
                  ...input.production,
                  total_units: InputValueHelpers.explicit(
                    parseInt(e.target.value) || 0
                  ),
                },
              })
            }
            className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                     rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
            Good Units *
          </label>
          <input
            type="number"
            value={good}
            onChange={(e) =>
              onChange({
                ...input,
                production: {
                  ...input.production,
                  good_units: InputValueHelpers.explicit(
                    parseInt(e.target.value) || 0
                  ),
                },
              })
            }
            className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                     rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
            Scrap Units
          </label>
          <input
            type="number"
            value={scrap}
            onChange={(e) =>
              onChange({
                ...input,
                production: {
                  ...input.production,
                  scrap_units: InputValueHelpers.explicit(
                    parseInt(e.target.value) || 0
                  ),
                },
              })
            }
            className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                     rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
            Reworked Units
          </label>
          <input
            type="number"
            value={rework}
            onChange={(e) =>
              onChange({
                ...input,
                production: {
                  ...input.production,
                  reworked_units: InputValueHelpers.explicit(
                    parseInt(e.target.value) || 0
                  ),
                },
              })
            }
            className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                     rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
          />
        </div>
      </div>

      {/* Validation Hint */}
      {!isConsistent && (
        <div className="validation-warning">
          <p className="warning-text">
            Count mismatch: {good} + {scrap} + {rework} = {sum}, but total is{" "}
            {total}
          </p>
          <p className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
            Good + Scrap + Rework must equal Total
          </p>
        </div>
      )}
    </div>
  );
};

// Cycle Time Section
const CycleTimeSection: React.FC<{
  input: OeeInput;
  onChange: (input: OeeInput) => void;
}> = ({ input, onChange }) => {
  const ideal = InputValueHelpers.getValue(input.cycle_time.ideal_cycle_time);

  return (
    <div className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-charcoal-700 dark:text-charcoal-300 mb-2">
          Ideal Cycle Time (seconds) *
          <span className="source-pill source-pill--explicit ml-2">
            Explicit
          </span>
        </label>
        <input
          type="number"
          step="0.1"
          value={ideal}
          onChange={(e) =>
            onChange({
              ...input,
              cycle_time: {
                ...input.cycle_time,
                ideal_cycle_time: InputValueHelpers.explicit(
                  parseFloat(e.target.value) || 0
                ),
              },
            })
          }
          className="w-full px-3 py-2 border border-charcoal-300 dark:border-charcoal-600 
                   rounded-md bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
        />
        <p className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-1">
          Theoretical minimum time per unit (design spec)
        </p>
      </div>
    </div>
  );
};

// Downtime Section
const DowntimeSection: React.FC<{
  input: OeeInput;
  onChange: (input: OeeInput) => void;
}> = ({ input }) => {
  return (
    <div className="text-center py-6 text-charcoal-600 dark:text-charcoal-400">
      <p>Advanced downtime record entry coming soon</p>
      <p className="text-sm mt-2">
        For now, use time allocations to model downtime
      </p>
    </div>
  );
};

// Economics Section
const EconomicsSection: React.FC<{
  params: EconomicParameters | null;
  onChange: (params: EconomicParameters | null) => void;
}> = ({ params, onChange }) => {
  const [enabled, setEnabled] = React.useState(!!params);

  const handleToggle = () => {
    if (enabled) {
      onChange(null);
    } else {
      onChange({
        unit_price: [90, 100, 110],
        marginal_contribution: [36, 40, 44],
        material_cost: [18, 20, 22],
        labor_cost_per_hour: [31.5, 35, 38.5],
        currency: "USD",
      });
    }
    setEnabled(!enabled);
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <label className="text-sm font-medium text-charcoal-700 dark:text-charcoal-300">
          Enable Economic Analysis
        </label>
        <button
          onClick={handleToggle}
          className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
            enabled ? "bg-steel-600" : "bg-charcoal-300"
          }`}
        >
          <span
            className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
              enabled ? "translate-x-6" : "translate-x-1"
            }`}
          />
        </button>
      </div>

      {enabled && params && (
        <div className="mt-4 p-4 bg-yellow-50 dark:bg-yellow-900/20 rounded-md">
          <p className="text-sm text-yellow-800 dark:text-yellow-400 mb-3">
            <strong>⚠️ Economic estimates only</strong> - These are NOT
            accounting-grade figures
          </p>
          <div className="space-y-3">
            <div>
              <label className="block text-xs font-medium text-charcoal-700 dark:text-charcoal-300 mb-1">
                Unit Price (Low, Central, High)
              </label>
              <div className="flex gap-2">
                {params.unit_price.map((val, i) => (
                  <input
                    key={i}
                    type="number"
                    step="0.01"
                    value={val}
                    onChange={(e) => {
                      const newPrice = [...params.unit_price];
                      newPrice[i] = parseFloat(e.target.value) || 0;
                      onChange({
                        ...params,
                        unit_price: newPrice as [number, number, number],
                      });
                    }}
                    className="flex-1 px-2 py-1 text-sm border border-charcoal-300 dark:border-charcoal-600 
                             rounded bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-charcoal-100"
                  />
                ))}
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

// Config Section
const ConfigSection: React.FC<{
  config: any;
  onToggleSensitivity: (enabled: boolean) => void;
  onToggleLeverage: (enabled: boolean) => void;
}> = ({ config, onToggleSensitivity, onToggleLeverage }) => {
  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <label className="text-sm font-medium text-charcoal-700 dark:text-charcoal-300">
            Sensitivity Analysis
          </label>
          <p className="text-xs text-charcoal-500 dark:text-charcoal-400">
            Test how results change with ±10% input variations
          </p>
        </div>
        <button
          onClick={() => onToggleSensitivity(!config.includeSensitivity)}
          className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
            config.includeSensitivity ? "bg-steel-600" : "bg-charcoal-300"
          }`}
        >
          <span
            className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
              config.includeSensitivity ? "translate-x-6" : "translate-x-1"
            }`}
          />
        </button>
      </div>

      <div className="flex items-center justify-between">
        <div>
          <label className="text-sm font-medium text-charcoal-700 dark:text-charcoal-300">
            Leverage Analysis
          </label>
          <p className="text-xs text-charcoal-500 dark:text-charcoal-400">
            Identify theoretical improvement opportunities
          </p>
        </div>
        <button
          onClick={() => onToggleLeverage(!config.includeLeverage)}
          className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
            config.includeLeverage ? "bg-steel-600" : "bg-charcoal-300"
          }`}
        >
          <span
            className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
              config.includeLeverage ? "translate-x-6" : "translate-x-1"
            }`}
          />
        </button>
      </div>
    </div>
  );
};

export default AssumptionsView;
