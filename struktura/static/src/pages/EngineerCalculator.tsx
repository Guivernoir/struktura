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

import { useState } from "react";
import { useOutletContext } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import CalculatorChooser from "../components/engineer/Chooser";
import OeeCalculator from "../components/engineer/production/oee/index";
import Disclaimer from "../components/engineer/Disclaimer";
import { OeeApiProvider } from "../components/engineer/production/oee/useOee"; // Import Provider
import { OeeApiClient } from "../components/engineer/production/oee/api"; // Import Client

// ============================================================================
// Initialize Clients
// ============================================================================

const queryClient = new QueryClient();

const oeeClient = new OeeApiClient({
  baseUrl: "http://localhost:8000", // Replace with your actual API URL
  timeout: 30000, // Optional: override default 30s timeout
});

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
  icon: string;
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
// Component Map - Lazy Loading with Style
// ============================================================================

/**
 * Map calculator IDs to their components.
 * Currently only OEE is implemented, others return placeholders.
 * This is called "iterative development" in polite company.
 */
const CALCULATOR_COMPONENTS: Record<CalculatorId, React.ComponentType<any>> = {
  oee: OeeCalculator,
  beam_deflection: () => <ComingSoon calculatorName="Beam Deflection" />,
  concrete_mix: () => <ComingSoon calculatorName="Concrete Mix Design" />,
  soil_bearing: () => <ComingSoon calculatorName="Soil Bearing Capacity" />,
  hvac_load: () => <ComingSoon calculatorName="HVAC Load Calculation" />,
};

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
// Main Component - The Command Center
// ============================================================================

const EngineerCalculator: React.FC = () => {
  const { t, theme } = useOutletContext<OutletContext>();
  const [selectedCalculator, setSelectedCalculator] =
    useState<CalculatorId | null>(null);

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
   * Load the selected calculator component
   * Type-safe because we're not animals
   */
  const CalculatorComponent = selectedCalculator
    ? CALCULATOR_COMPONENTS[selectedCalculator]
    : null;

  return (
    <QueryClientProvider client={queryClient}>
      <div className="container mx-auto px-4 md:px-6 py-12 relative z-10">
        {/* Header - Because First Impressions Matter */}
        <div className="mb-12 text-center">
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-red-100 to-rose-100 dark:from-red-950/50 dark:to-rose-950/50 rounded-full mb-6 border border-red-200 dark:border-red-800">
            <span className="text-xs font-bold text-red-900 dark:text-red-100 uppercase tracking-wider">
              {getLabel("engineer.badge")}
            </span>
          </div>

          <h1 className="font-display text-5xl md:text-6xl font-black mb-6 text-transparent bg-clip-text bg-gradient-to-r from-red-600 to-rose-600 dark:from-red-400 dark:to-rose-400">
            {getLabel("engineer.title")}
          </h1>

          <p className="text-lg text-charcoal-600 dark:text-steel-400 max-w-3xl mx-auto leading-relaxed">
            {getLabel("engineer.subtitle")}
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
          <>
            {/* Back Button - Tactical Retreat Option */}
            <div className="mb-6">
              <button
                onClick={() => setSelectedCalculator(null)}
                className="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-charcoal-700 dark:text-steel-300 hover:text-red-600 dark:hover:text-red-400 transition-colors"
              >
                <svg
                  className="w-4 h-4"
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
            </div>

            {/* Wrap the Calculator Component with the Provider */}
            {selectedCalculator === "oee" ? (
              <OeeApiProvider client={oeeClient}>
                <OeeCalculator t={t} theme={theme} getLabel={getLabel} />
              </OeeApiProvider>
            ) : (
              CalculatorComponent && (
                <CalculatorComponent t={t} theme={theme} getLabel={getLabel} />
              )
            )}
          </>
        )}

        {/* Disclaimer - Legal's Favorite Section */}
        <div className="mt-12">
          <Disclaimer t={t} />
        </div>
      </div>
    </QueryClientProvider>
  );
};

export default EngineerCalculator;
