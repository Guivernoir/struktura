import { useOutletContext } from "react-router-dom";
import { useContractorCalculator } from "../hooks/contractor";
import ContractorCategorySelector from "../components/contractor/CategorySelector";
import ContractorForm from "../components/contractor/ContractorForm";
import ContractorResults from "../components/contractor/ContractorResults";
// Removed ContractorVisualizationPanel import

const ContractorCalculator = () => {
  const { lang, theme, t } = useOutletContext();

  const {
    // Catalogue
    catalogue,
    categories,
    calculatorsInCategory,

    // Selection
    selectedCategory,
    setSelectedCategory,
    selectedCalculator,
    setSelectedCalculator,

    // Inputs
    inputs,
    isLoadingInputs,

    // Form
    formData,
    handleFormEvent,
    updateDimension,
    updateMaterial,
    updateResource,
    updateSafetyFactor,
    updateAdditional,

    // Calculation
    handleCalculate,
    handleCalculateDetailed,
    handleCalculateSummary,
    results,
    warnings,
    structuredWarnings,
    recommendations,
    complianceNotes,
    analysis,
    metadata,
    isLoading,
    clearResults,

    // Error
    error,
  } = useContractorCalculator();

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

  // Removed hasVisualization check

  return (
    <div className="container mx-auto px-4 md:px-6 py-12 relative z-10">
      {/* Header */}
      <div className="mb-12 text-center">
        <div className="inline-flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-amber-100 to-orange-100 dark:from-amber-950/50 dark:to-orange-950/50 rounded-full mb-6 border border-amber-200 dark:border-amber-800">
          <span className="text-xs font-bold text-amber-900 dark:text-amber-100 uppercase tracking-wider">
            Professional Contracting Tools
          </span>
        </div>

        <h1 className="font-display text-5xl md:text-6xl font-black mb-6 text-transparent bg-clip-text bg-gradient-to-r from-amber-600 to-orange-600 dark:from-amber-400 dark:to-orange-400">
          {getLabel("contractor.title")}
        </h1>

        <p className="text-lg text-charcoal-600 dark:text-steel-400 max-w-3xl mx-auto leading-relaxed">
          {getLabel("contractor.subtitle")}
        </p>

        {catalogue?.disclaimer && (
          <div className="mt-6 p-4 bg-amber-50 dark:bg-amber-900/10 rounded-xl border-l-4 border-amber-500 max-w-2xl mx-auto">
            <p className="text-sm text-amber-900 dark:text-amber-100">
              <strong className="font-semibold">Professional Notice:</strong>{" "}
              {catalogue.disclaimer}
            </p>
          </div>
        )}
      </div>

      {/* Category Selection */}
      <div className="mb-8">
        <ContractorCategorySelector
          t={t}
          categories={categories}
          selectedCategory={selectedCategory}
          setSelectedCategory={setSelectedCategory}
        />
      </div>

      {/* Main Content Grid - Now a standard 2-column layout */}
      <div className="grid lg:grid-cols-2 gap-8">
        {/* Left Column: Form */}
        <div className="lg:col-span-1">
          <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-xl border border-sand-200 dark:border-charcoal-800 p-6 sticky top-24">
            <h2 className="text-xl font-semibold text-charcoal-900 dark:text-white mb-6">
              {getLabel("contractor.form.title")}
            </h2>

            <ContractorForm
              t={t}
              selectedCategory={selectedCategory}
              selectedCalculator={selectedCalculator}
              setSelectedCalculator={setSelectedCalculator}
              calculators={calculatorsInCategory}
              calculatorMeta={inputs.metadata}
              formData={formData}
              handleFormEvent={handleFormEvent}
              updateDimension={updateDimension}
              updateMaterial={updateMaterial}
              updateResource={updateResource}
              updateSafetyFactor={updateSafetyFactor}
              updateAdditional={updateAdditional}
              handleCalculate={handleCalculate}
              handleCalculateDetailed={handleCalculateDetailed}
              handleCalculateSummary={handleCalculateSummary}
              isLoading={isLoading}
              error={error}
            />
          </div>
        </div>

        {/* Right Column: Results */}
        <div className="lg:col-span-1">
          <ContractorResults
            t={t}
            results={results}
            isLoading={isLoading}
            error={error}
            warnings={warnings}
            structuredWarnings={structuredWarnings}
            recommendations={recommendations}
            complianceNotes={complianceNotes}
            analysis={analysis}
            theme={theme}
            calculatorMetadata={inputs.metadata}
          />
        </div>

        {/* Removed Right Column: Visualization (conditional) */}
      </div>

      {/* Additional Info Section */}
      {inputs.metadata && (
        <div className="mt-12 grid md:grid-cols-3 gap-6">
          {/* Regulation Codes */}
          {inputs.codes && inputs.codes.length > 0 && (
            <div className="bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-blue-950/30 dark:to-indigo-950/30 rounded-xl p-6 border border-blue-200 dark:border-blue-800">
              <h3 className="text-sm font-bold text-blue-900 dark:text-blue-100 uppercase tracking-wide mb-3 flex items-center gap-2">
                <span className="text-blue-600 dark:text-blue-400">📋</span>
                Applicable Codes
              </h3>
              <ul className="space-y-2">
                {inputs.codes.map((code) => (
                  <li
                    key={code}
                    className="text-sm text-blue-800 dark:text-blue-200 flex items-center gap-2"
                  >
                    <span className="w-1.5 h-1.5 bg-blue-500 rounded-full" />
                    {code}
                  </li>
                ))}
              </ul>
            </div>
          )}

          {/* Complexity Level */}
          {inputs.metadata.complexity_level && (
            <div className="bg-gradient-to-br from-purple-50 to-pink-50 dark:from-purple-950/30 dark:to-pink-950/30 rounded-xl p-6 border border-purple-200 dark:border-purple-800">
              <h3 className="text-sm font-bold text-purple-900 dark:text-purple-100 uppercase tracking-wide mb-3">
                Calculation Complexity
              </h3>
              <div className="text-2xl font-bold text-purple-700 dark:text-purple-300 capitalize">
                {inputs.metadata.complexity_level}
              </div>
              <p className="text-xs text-purple-600 dark:text-purple-400 mt-2">
                {inputs.metadata.calculation_time || "< 1s"}
              </p>
            </div>
          )}

          {/* Certification Requirements */}
          {inputs.metadata.requires_certification_review && (
            <div className="bg-gradient-to-br from-orange-50 to-red-50 dark:from-orange-950/30 dark:to-red-950/30 rounded-xl p-6 border border-orange-200 dark:border-orange-800">
              <h3 className="text-sm font-bold text-orange-900 dark:text-orange-100 uppercase tracking-wide mb-3 flex items-center gap-2">
                <span className="text-orange-600 dark:text-orange-400">⚠️</span>
                Certification Required
              </h3>
              <p className="text-sm text-orange-800 dark:text-orange-200">
                Professional certification review required before project
                execution.
              </p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default ContractorCalculator;
