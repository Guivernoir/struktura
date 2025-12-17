import { useOutletContext } from "react-router-dom";
import { useState, useEffect } from "react";
import { useEngineerCalculator } from "../hooks";
import CategorySelector from "../components/engineer/CategorySelector";
import CalculatorForm from "../components/engineer/CalculatorForm";
import EngineerResults from "../components/engineer/EngineerResults";
import VisualizationPanel from "../components/engineer/VisualizationPanel";
import Disclaimer from "../components/beginner/Disclaimer";
import Icon from "../components/Icon";
import { api } from "../lib";

const EngineerCalculator = () => {
  const { t, theme } = useOutletContext();

  const {
    categories,
    selectedCategory,
    setSelectedCategory,
    selectedCalculator,
    setSelectedCalculator,
    calculatorMetadata,
    formData,
    handleInputChange,
    handleFormEvent,
    outputFormat,
    setOutputFormat,
    handleCalculate,
    results,
    warnings,
    structuredWarnings,
    recommendations,
    isLoading,
    error,
  } = useEngineerCalculator();

  // Fetch calculators for selected category
  const [calculators, setCalculators] = useState([]);
  useEffect(() => {
    async function loadCalculators() {
      if (selectedCategory) {
        try {
          const calcs = await api.calculus.getCalculatorsByCategory(
            selectedCategory,
            "engineer"
          );
          setCalculators(calcs || []);
          setSelectedCalculator(null);
        } catch (err) {
          console.error("Failed to load calculators:", err);
          setCalculators([]);
        }
      }
    }
    loadCalculators();
  }, [selectedCategory, setSelectedCalculator]);

  // Define supported 3D types
  const supported3DTypes = [
    "deck",
    "concrete_slab",
    "planter_box",
    "wall_framing",
    "mulch_bed",
  ];
  const show3D = supported3DTypes.includes(selectedCalculator);

  const getLabel = (key) => {
    return (
      key
        .split(".")
        .reduce(
          (obj, prop) => (obj && obj[prop] !== undefined ? obj[prop] : key),
          t
        ) || key
    );
  };

  return (
    <div className="container mx-auto px-4 md:px-6 py-12 relative z-10">
      {/* Header */}
      <div className="mb-12 text-center">
        <div className="inline-flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-red-100 to-rose-100 dark:from-red-950/50 dark:to-rose-950/50 rounded-full mb-6 border border-red-200 dark:border-red-800">
          <span className="text-xs font-bold text-red-900 dark:text-red-100 uppercase tracking-wider">
            Advanced Engineering Analysis
          </span>
        </div>

        <h1 className="font-display text-5xl md:text-6xl font-black mb-6 text-transparent bg-clip-text bg-gradient-to-r from-red-600 to-rose-600 dark:from-red-400 dark:to-rose-400">
          {getLabel("engineer.title") || "Structural & Material Calculators"}
        </h1>

        <p className="text-lg text-charcoal-600 dark:text-steel-400 max-w-3xl mx-auto leading-relaxed">
          {getLabel("engineer.subtitle") ||
            "Precision tools for structural integrity, materials science, and civil planning."}
        </p>
      </div>

      {/* Category Selection */}
      <div className="mb-8 flex flex-col items-center">
        <CategorySelector
          t={t}
          categories={categories}
          selectedCategory={selectedCategory}
          setSelectedCategory={setSelectedCategory}
        />

        {/* Output Format Selector (Moved from sticky header) */}
        {selectedCalculator && (
          <div className="mt-4 flex items-center gap-2 bg-white dark:bg-charcoal-900 rounded-lg px-3 py-2 shadow-sm border border-sand-200 dark:border-charcoal-800">
            <Icon
              name="FileText"
              size={16}
              className="text-charcoal-500 dark:text-steel-500"
            />
            <select
              value={outputFormat}
              onChange={(e) => setOutputFormat(e.target.value)}
              className="text-sm font-medium text-charcoal-800 dark:text-steel-200 bg-transparent border-none focus:outline-none"
            >
              <option value="standard">Standard Output</option>
              <option value="detailed">Detailed Analysis</option>
              <option value="summary">Summary Report</option>
            </select>
          </div>
        )}
      </div>

      {/* Main Content Grid - Standard 2-column layout */}
      <div className={`grid lg:grid-cols-2 gap-8`}>
        {/* Left Column: Form */}
        <div className="lg:col-span-1">
          <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-xl border border-sand-200 dark:border-charcoal-800 p-6 sticky top-24">
            <h2 className="text-xl font-semibold text-charcoal-900 dark:text-white mb-6">
              {getLabel("engineer.form.title") || "Input Parameters"}
            </h2>

            <CalculatorForm
              t={t}
              selectedCategory={selectedCategory}
              selectedCalculator={selectedCalculator}
              setSelectedCalculator={setSelectedCalculator}
              calculators={calculators}
              calculatorMeta={calculatorMetadata}
              formData={formData}
              handleInputChange={handleInputChange}
              handleFormEvent={handleFormEvent}
              handleCalculate={handleCalculate}
              isLoading={isLoading}
              error={error}
            />

            {!show3D && selectedCalculator && (
              <div className="mt-6 p-3 rounded-lg bg-sand-100 dark:bg-charcoal-800 border border-sand-200 dark:border-charcoal-700 flex items-center gap-3">
                <Icon
                  name="BoxSelect"
                  size={18}
                  className="text-charcoal-400 dark:text-steel-500"
                />
                <span className="text-xs text-charcoal-500 dark:text-steel-400">
                  3D preview unavailable for this selected calculation.
                </span>
              </div>
            )}
          </div>
        </div>

        {/* Right Column: Results & Visualization */}
        <div className="lg:col-span-1 space-y-8">
          {/* 3D Visualization Panel (if supported and a calculator is selected) */}
          {show3D && selectedCalculator && (
            <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-xl border border-sand-200 dark:border-charcoal-800 p-6">
              <h2 className="text-xl font-semibold text-charcoal-900 dark:text-white mb-4">
                3D Visualization
              </h2>
              <VisualizationPanel t={t} formData={formData} theme={theme} />
            </div>
          )}

          {/* Engineer Results */}
          <EngineerResults
            t={t}
            results={results}
            isLoading={isLoading}
            error={error}
            warnings={warnings}
            structuredWarnings={structuredWarnings}
            recommendations={recommendations}
            theme={theme}
            calculatorMetadata={calculatorMetadata}
          />
        </div>
      </div>

      {/* Disclaimer Footer (Re-added to the bottom) */}
      <div className="mt-12">
        <Disclaimer t={t} />
      </div>
    </div>
  );
};

export default EngineerCalculator;
