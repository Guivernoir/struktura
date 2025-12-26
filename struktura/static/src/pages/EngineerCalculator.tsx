/**
 * Engineer Calculator Page
 *
 * Strategic redesign: No more dynamic loading, no more uncertainty.
 * We know what calculators we have, so we declare them upfront.
 * Component-based loading because code splitting is a virtue.
 *
 * This is what happens when you stop trusting the backend
 * to tell you what you already know.
 */

import { useState, useCallback } from "react";
import { useOutletContext } from "react-router-dom";
import CalculatorChooser from "../components/engineer/Chooser";
import OeeEngine from "../components/engineer/production/oee/OeeEngine";
import Disclaimer from "../components/engineer/Disclaimer";
import type { IconName } from "../components/engineer/Icon";
import type {
  OeeResult,
  OeeInput,
  EconomicParameters,
} from "../components/engineer/production/oee/models";
import {
  getScenario,
  SAMPLE_SCENARIOS,
} from "../components/engineer/production/oee/sampleData";

// ============================================================================
// Type Definitions - Know Your Battlefield
// ============================================================================

interface OutletContext {
  t: Record<string, any>;
  theme: "light" | "dark";
}

export type CalculatorId =
  | "oee"
  | "beam_deflection"
  | "concrete_mix"
  | "soil_bearing"
  | "hvac_load";

export interface CalculatorDefinition {
  id: CalculatorId;
  nameKey: string;
  descriptionKey: string;
  category: string;
  icon: IconName;
  complexity: "basic" | "intermediate" | "advanced";
}

// ============================================================================
// Calculator Registry - The Single Source of Truth
// ============================================================================

/**
 * All available calculators, declared with military precision.
 * Add new calculators here, not in some database table.
 */
const CALCULATOR_REGISTRY: CalculatorDefinition[] = [
  {
    id: "oee",
    nameKey: "engineer.calculators.oee.name",
    descriptionKey: "engineer.calculators.oee.description",
    category: "production",
    icon: "Activity",
    complexity: "advanced",
  },
  {
    id: "beam_deflection",
    nameKey: "engineer.calculators.beam_deflection.name",
    descriptionKey: "engineer.calculators.beam_deflection.description",
    category: "structural",
    icon: "Columns",
    complexity: "intermediate",
  },
  {
    id: "concrete_mix",
    nameKey: "engineer.calculators.concrete_mix.name",
    descriptionKey: "engineer.calculators.concrete_mix.description",
    category: "materials",
    icon: "Box",
    complexity: "basic",
  },
  {
    id: "soil_bearing",
    nameKey: "engineer.calculators.soil_bearing.name",
    descriptionKey: "engineer.calculators.soil_bearing.description",
    category: "geotechnical",
    icon: "Mountain",
    complexity: "advanced",
  },
  {
    id: "hvac_load",
    nameKey: "engineer.calculators.hvac_load.name",
    descriptionKey: "engineer.calculators.hvac_load.description",
    category: "mechanical",
    icon: "Wind",
    complexity: "intermediate",
  },
];

// ============================================================================
// Placeholder Component - Honesty Over False Promises
// ============================================================================

function ComingSoon({ calculatorName }: { calculatorName: string }) {
  return (
    <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-xl border border-sand-200 dark:border-charcoal-800 p-12 text-center">
      <div className="mb-6">
        <div className="w-16 h-16 mx-auto bg-sand-100 dark:bg-charcoal-800 rounded-full flex items-center justify-center">
          <span className="text-3xl">🚧</span>
        </div>
      </div>
      <h3 className="text-2xl font-bold text-charcoal-900 dark:text-white mb-3">
        {calculatorName}
      </h3>
      <p className="text-charcoal-600 dark:text-steel-400 max-w-md mx-auto">
        This calculator is under development. Like all good engineering
        projects, it's taking slightly longer than originally estimated.
      </p>
    </div>
  );
}

// ============================================================================
// Component Map - Lazy Loading with Style
// ============================================================================

/**
 * Map calculator IDs to their components.
 * Currently only OEE is implemented, others return placeholders.
 */
const CALCULATOR_COMPONENTS: Record<CalculatorId, React.ComponentType<any>> = {
  oee: OeeEngine,
  beam_deflection: () => <ComingSoon calculatorName="Beam Deflection" />,
  concrete_mix: () => <ComingSoon calculatorName="Concrete Mix Design" />,
  soil_bearing: () => <ComingSoon calculatorName="Soil Bearing Capacity" />,
  hvac_load: () => <ComingSoon calculatorName="HVAC Load Calculation" />,
};

// ============================================================================
// Main Component - The Command Center
// ============================================================================

const EngineerCalculator: React.FC = () => {
  const { t, theme } = useOutletContext<OutletContext>();
  const [selectedCalculator, setSelectedCalculator] =
    useState<CalculatorId | null>(null);

  // OEE-specific state
  const [oeeInitialInput, setOeeInitialInput] = useState<OeeInput | undefined>(
    undefined
  );
  const [oeeEconomicParams, setOeeEconomicParams] = useState<
    EconomicParameters | undefined
  >(undefined);
  const [showSampleLoader, setShowSampleLoader] = useState(false);

  /**
   * Load a sample scenario for testing
   */
  const loadSampleScenario = useCallback(
    (scenarioName: keyof typeof SAMPLE_SCENARIOS) => {
      const { input, economic } = getScenario(scenarioName);
      setOeeInitialInput(input);
      setOeeEconomicParams(economic);
      setShowSampleLoader(false);
    },
    []
  );

  /**
   * Translate a key, or admit defeat gracefully
   */
  const getLabel = (key: string): string => {
    return (
      key
        .split(".")
        .reduce(
          (obj, prop) => (obj && obj[prop] !== undefined ? obj[prop] : key),
          t as any
        ) || key
    );
  };

  /**
   * Handle OEE calculation completion
   * This is where you'd persist results, trigger notifications, etc.
   */
  const handleOeeResultsReady = useCallback((result: OeeResult) => {
    console.log("OEE Calculation Complete:", {
      oee: result.core_metrics.oee.value,
      availability: result.core_metrics.availability.value,
      performance: result.core_metrics.performance.value,
      quality: result.core_metrics.quality.value,
      timestamp: result.ledger.analysis_timestamp,
    });

    // Future: Save to backend, show toast notification, etc.
  }, []);

  /**
   * Handle OEE input changes
   * This is where you'd auto-save drafts, validate, etc.
   */
  const handleOeeInputChange = useCallback((input: any) => {
    // Future: Auto-save to localStorage or backend
    console.log("OEE Input Updated:", input.window);
  }, []);

  /**
   * Render the active calculator
   */
  const renderCalculator = () => {
    if (!selectedCalculator) return null;

    const CalculatorComponent = CALCULATOR_COMPONENTS[selectedCalculator];

    // OEE gets special treatment with sample data loader and callbacks
    if (selectedCalculator === "oee") {
      return (
        <div className="space-y-6">
          {/* Sample Data Loader */}
          <div className="bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-blue-950/30 dark:to-indigo-950/30 rounded-lg border border-blue-200 dark:border-blue-800 p-6">
            <div className="flex items-start justify-between mb-4">
              <div>
                <h3 className="text-lg font-display font-bold text-blue-900 dark:text-blue-100 mb-1">
                  🎯 Quick Start with Sample Data
                </h3>
                <p className="text-sm text-blue-700 dark:text-blue-300">
                  Load a pre-configured scenario to explore the calculator
                </p>
              </div>
              <button
                onClick={() => setShowSampleLoader(!showSampleLoader)}
                className="text-sm text-blue-600 dark:text-blue-400 hover:underline"
              >
                {showSampleLoader ? "Hide" : "Show"} Options
              </button>
            </div>

            {showSampleLoader && (
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 animate-slide-down">
                {Object.entries(SAMPLE_SCENARIOS).map(([key, scenario]) => (
                  <button
                    key={key}
                    onClick={() =>
                      loadSampleScenario(key as keyof typeof SAMPLE_SCENARIOS)
                    }
                    className="text-left p-4 bg-white dark:bg-charcoal-800 rounded-lg border-2 border-blue-200 dark:border-blue-800 
                             hover:border-blue-400 dark:hover:border-blue-600 transition-colors group"
                  >
                    <div className="font-semibold text-blue-900 dark:text-blue-100 mb-1 group-hover:text-blue-600 dark:group-hover:text-blue-400">
                      {scenario.name}
                    </div>
                    <p className="text-xs text-blue-700 dark:text-blue-300">
                      {scenario.description}
                    </p>
                  </button>
                ))}
              </div>
            )}
          </div>

          {/* OEE Engine */}
          <OeeEngine
            initialInput={oeeInitialInput}
            initialEconomicParams={oeeEconomicParams}
            onResultsReady={handleOeeResultsReady}
            onInputChange={handleOeeInputChange}
            autoCalculate={false}
            className="animate-fade-in"
          />
        </div>
      );
    }

    // Other calculators get standard props
    return <CalculatorComponent t={t} theme={theme} getLabel={getLabel} />;
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-sand-50 via-white to-steel-50 dark:from-charcoal-950 dark:via-charcoal-900 dark:to-charcoal-950">
      <div className="container mx-auto px-4 md:px-6 py-12 relative z-10">
        {/* Header - Because First Impressions Matter */}
        <div className="mb-12 text-center">
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-red-100 to-rose-100 dark:from-red-950/50 dark:to-rose-950/50 rounded-full mb-6 border border-red-200 dark:border-red-800">
            <span className="text-xs font-bold text-red-900 dark:text-red-100 uppercase tracking-wider">
              {getLabel("engineer.badge") || "Engineering Tools"}
            </span>
          </div>

          <h1 className="font-display text-5xl md:text-6xl font-black mb-6 text-transparent bg-clip-text bg-gradient-to-r from-red-600 to-rose-600 dark:from-red-400 dark:to-rose-400">
            {getLabel("engineer.title") || "Engineering Calculators"}
          </h1>

          <p className="text-lg text-charcoal-600 dark:text-steel-400 max-w-3xl mx-auto leading-relaxed">
            {getLabel("engineer.subtitle") ||
              "Precision tools for engineering analysis and decision support"}
          </p>
        </div>

        {/* Strategic Decision Point: Show Chooser or Calculator */}
        {!selectedCalculator ? (
          <CalculatorChooser
            calculators={CALCULATOR_REGISTRY}
            onSelect={setSelectedCalculator}
            getLabel={getLabel}
          />
        ) : (
          <div className="space-y-6">
            {/* Back Button - Tactical Retreat Option */}
            <div className="flex items-center justify-between">
              <button
                onClick={() => setSelectedCalculator(null)}
                className="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium 
                         text-charcoal-700 dark:text-steel-300 
                         hover:text-red-600 dark:hover:text-red-400 
                         transition-colors group"
              >
                <svg
                  className="w-4 h-4 transition-transform group-hover:-translate-x-1"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M15 19l-7-7 7-7"
                  />
                </svg>
                Back to Calculator Selection
              </button>

              {/* Calculator Info Badge */}
              <div className="flex items-center gap-2 text-xs text-charcoal-500 dark:text-charcoal-400">
                <span className="px-2 py-1 bg-sand-100 dark:bg-charcoal-800 rounded-full font-medium">
                  {CALCULATOR_REGISTRY.find((c) => c.id === selectedCalculator)
                    ?.category || "Unknown"}
                </span>
                <span className="px-2 py-1 bg-sand-100 dark:bg-charcoal-800 rounded-full font-medium">
                  {CALCULATOR_REGISTRY.find((c) => c.id === selectedCalculator)
                    ?.complexity || "Unknown"}
                </span>
              </div>
            </div>

            {/* Calculator Component */}
            <div className="animate-fade-in">{renderCalculator()}</div>
          </div>
        )}

        {/* Disclaimer - Legal's Favorite Section */}
        <div className="mt-16">
          <Disclaimer t={t} />
        </div>
      </div>

      {/* Background Decoration */}
      <div className="fixed inset-0 pointer-events-none overflow-hidden -z-10">
        <div className="absolute top-0 right-0 w-96 h-96 bg-red-100/30 dark:bg-red-900/10 rounded-full blur-3xl" />
        <div className="absolute bottom-0 left-0 w-96 h-96 bg-rose-100/30 dark:bg-rose-900/10 rounded-full blur-3xl" />
      </div>
    </div>
  );
};

export default EngineerCalculator;
