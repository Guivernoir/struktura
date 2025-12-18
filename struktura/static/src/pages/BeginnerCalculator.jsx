import { useOutletContext } from "react-router-dom";
import { useBeginnerCalculator } from "../hooks/beginner";
import BeginnerCategorySelector from "../components/beginner/CategorySelector";
import BeginnerForm from "../components/beginner/CalculatorForm";
import BeginnerResults from "../components/beginner/BeginnerResults";

const BeginnerCalculator = () => {
  const { lang, theme, t } = useOutletContext();

  const {
    catalogue,
    categories,
    calculatorsInCategory,
    selectedCategory,
    setSelectedCategory,
    selectedCalculator,
    setSelectedCalculator,
    inputs,
    isLoadingInputs,
    formData,
    handleInputChange,
    handleCalculate,
    results,
    warnings,
    isLoading,
    clearResults,
    error,
  } = useBeginnerCalculator();

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
        <div className="inline-flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-green-100 to-emerald-100 dark:from-green-950/50 dark:to-emerald-950/50 rounded-full mb-6 border border-green-200 dark:border-green-800">
          <span className="text-xs font-bold text-green-900 dark:text-green-100 uppercase tracking-wider">
            {getLabel("beginner.badge")}
          </span>
        </div>

        <h1 className="font-display text-5xl md:text-6xl font-black mb-6 text-transparent bg-clip-text bg-gradient-to-r from-green-600 to-emerald-600 dark:from-green-400 dark:to-emerald-400">
          {getLabel("beginner.title")}
        </h1>

        <p className="text-lg text-charcoal-600 dark:text-steel-400 max-w-3xl mx-auto leading-relaxed">
          {getLabel("beginner.subtitle")}
        </p>

        {catalogue?.disclaimer && (
          <div className="mt-6 p-4 bg-green-50 dark:bg-green-900/10 rounded-xl border-l-4 border-green-500 max-w-2xl mx-auto">
            <p className="text-sm text-green-900 dark:text-green-100">
              <strong className="font-semibold">
                {getLabel("beginner.note_label")}:
              </strong>{" "}
              {catalogue.disclaimer}
            </p>
          </div>
        )}
      </div>

      {/* Category Selection */}
      <div className="mb-8">
        <BeginnerCategorySelector
          t={t}
          categories={categories}
          selectedCategory={selectedCategory}
          setSelectedCategory={setSelectedCategory}
        />
      </div>

      {/* Main Content Grid */}
      <div className="grid lg:grid-cols-2 gap-8">
        {/* Left Column: Form */}
        <div className="lg:col-span-1">
          <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-xl border border-sand-200 dark:border-charcoal-800 p-6 sticky top-24">
            <h2 className="text-xl font-semibold text-charcoal-900 dark:text-white mb-6">
              {getLabel("beginner.form.title")}
            </h2>

            <BeginnerForm
              t={t}
              selectedCategory={selectedCategory}
              selectedCalculator={selectedCalculator}
              setSelectedCalculator={setSelectedCalculator}
              calculators={calculatorsInCategory}
              calculatorMeta={inputs.metadata}
              formData={formData}
              handleInputChange={handleInputChange}
              handleCalculate={handleCalculate}
              isLoading={isLoading}
              error={error}
            />
          </div>
        </div>

        {/* Right Column: Results */}
        <div className="lg:col-span-1">
          <BeginnerResults
            t={t}
            results={results}
            isLoading={isLoading}
            error={error}
            warnings={warnings}
            theme={theme}
            calculatorMetadata={inputs.metadata}
          />
        </div>
      </div>

      {/* Tips Section */}
      {inputs.metadata && (
        <div className="mt-12 grid md:grid-cols-3 gap-6">
          {/* Tips Card */}
          <div className="bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-blue-950/30 dark:to-indigo-950/30 rounded-xl p-6 border border-blue-200 dark:border-blue-800">
            <h3 className="text-sm font-bold text-blue-900 dark:text-blue-100 uppercase tracking-wide mb-3 flex items-center gap-2">
              <span className="text-blue-600 dark:text-blue-400">💡</span>
              {getLabel("beginner.info_cards.tips.title")}
            </h3>
            <ul className="space-y-2 text-sm text-blue-800 dark:text-blue-200">
              <li className="flex items-start gap-2">
                <span className="w-1.5 h-1.5 bg-blue-500 rounded-full mt-1.5 flex-shrink-0" />
                <span>{getLabel("beginner.info_cards.tips.item_1")}</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="w-1.5 h-1.5 bg-blue-500 rounded-full mt-1.5 flex-shrink-0" />
                <span>{getLabel("beginner.info_cards.tips.item_2")}</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="w-1.5 h-1.5 bg-blue-500 rounded-full mt-1.5 flex-shrink-0" />
                <span>{getLabel("beginner.info_cards.tips.item_3")}</span>
              </li>
            </ul>
          </div>

          {/* Complexity Info */}
          {inputs.metadata.typical_applications && (
            <div className="bg-gradient-to-br from-purple-50 to-pink-50 dark:from-purple-950/30 dark:to-pink-950/30 rounded-xl p-6 border border-purple-200 dark:border-purple-800">
              <h3 className="text-sm font-bold text-purple-900 dark:text-purple-100 uppercase tracking-wide mb-3">
                {getLabel("beginner.info_cards.common_uses")}
              </h3>
              <ul className="space-y-2 text-sm text-purple-800 dark:text-purple-200">
                {inputs.metadata.typical_applications
                  .slice(0, 3)
                  .map((app, idx) => (
                    <li key={idx} className="flex items-start gap-2">
                      <span className="w-1.5 h-1.5 bg-purple-500 rounded-full mt-1.5 flex-shrink-0" />
                      <span>{app}</span>
                    </li>
                  ))}
              </ul>
            </div>
          )}

          {/* Safety Notice */}
          <div className="bg-gradient-to-br from-orange-50 to-red-50 dark:from-orange-950/30 dark:to-red-950/30 rounded-xl p-6 border border-orange-200 dark:border-orange-800">
            <h3 className="text-sm font-bold text-orange-900 dark:text-orange-100 uppercase tracking-wide mb-3 flex items-center gap-2">
              <span className="text-orange-600 dark:text-orange-400">⚠️</span>
              {getLabel("beginner.info_cards.safety.title")}
            </h3>
            <p className="text-sm text-orange-800 dark:text-orange-200">
              {getLabel("beginner.info_cards.safety.text")}
            </p>
          </div>
        </div>
      )}

      {/* Disclaimer Footer */}
      <div className="mt-12 p-4 bg-sand-50 dark:bg-charcoal-900 rounded-xl border border-sand-200 dark:border-charcoal-800">
        <p className="text-xs text-center text-charcoal-500 dark:text-steel-500">
          <strong>{getLabel("disclaimer.title")}:</strong>{" "}
          {getLabel("disclaimer.text")}
        </p>
      </div>
    </div>
  );
};

export default BeginnerCalculator;
