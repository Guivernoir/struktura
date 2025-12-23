// Chooser.tsx
import React from "react";

interface ChooserProps {
  t: any;
  theme: string;
  categories: string[];
  selectedCategory: string | null;
  setSelectedCategory: (cat: string) => void;
  calculators: { id: string; title: string }[];
  selectedCalculator: string | null;
  setSelectedCalculator: (calc: string) => void;
  calculatorMetadata: {
    parameters: { name: string; type: string; unit?: string; }[];
  } | null;
  formData: Record<string, string>;
  handleInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  outputFormat: string;
  setOutputFormat: (format: string) => void;
  handleCalculate: () => void;
  results: { key: string; value: number; unit?: string }[];
  warnings: string[];
  structuredWarnings: any;
  recommendations: string[];
  isLoading: boolean;
  error: string | null;
  show3D: boolean;
  getLabel: (key: string) => string;
}

const Chooser: React.FC<ChooserProps> = ({
  t,
  theme,
  categories,
  selectedCategory,
  setSelectedCategory,
  calculators,
  selectedCalculator,
  setSelectedCalculator,
  calculatorMetadata,
  formData,
  handleInputChange,
  outputFormat,
  setOutputFormat,
  handleCalculate,
  results,
  warnings,
  structuredWarnings,
  recommendations,
  isLoading,
  error,
  show3D,
  getLabel,
}) => {
  return (
    <div className="grid lg:grid-cols-2 gap-8">
      {/* Left Column: Selections and Form */}
      <div className="lg:col-span-1">
        <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-xl border border-sand-200 dark:border-charcoal-800 p-6 sticky top-24">
          <h2 className="text-xl font-semibold text-charcoal-900 dark:text-white mb-6">
            {getLabel("engineer.form.title")}
          </h2>

          {/* Category Selector */}
          <select
            value={selectedCategory || ""}
            onChange={(e) => setSelectedCategory(e.target.value)}
            className="w-full mb-6 p-3 bg-white dark:bg-charcoal-800 border border-sand-200 dark:border-charcoal-700 rounded-lg text-charcoal-900 dark:text-steel-100 focus:ring-2 focus:ring-red-500 focus:border-transparent"
          >
            <option value="">{getLabel("engineer.select_category") || "Select Category"}</option>
            {categories.map((cat) => (
              <option key={cat} value={cat}>
                {getLabel(`categories.${cat}`)}
              </option>
            ))}
          </select>

          {/* Calculator Selector */}
          {selectedCategory && (
            <select
              value={selectedCalculator || ""}
              onChange={(e) => setSelectedCalculator(e.target.value)}
              className="w-full mb-6 p-3 bg-white dark:bg-charcoal-800 border border-sand-200 dark:border-charcoal-700 rounded-lg text-charcoal-900 dark:text-steel-100 focus:ring-2 focus:ring-red-500 focus:border-transparent"
            >
              <option value="">{getLabel("engineer.select_calculator") || "Select Calculator"}</option>
              {calculators.map((calc) => (
                <option key={calc.id} value={calc.id}>
                  {getLabel(`calculators.${calc.id}.title`)}
                </option>
              ))}
            </select>
          )}

          {/* Output Format Selector */}
          {selectedCalculator && (
            <div className="mb-6">
              <label className="block text-sm font-medium mb-2 text-charcoal-700 dark:text-steel-300">
                {getLabel("engineer.output_format.label") || "Output Format"}
              </label>
              <select
                value={outputFormat}
                onChange={(e) => setOutputFormat(e.target.value)}
                className="w-full p-3 bg-white dark:bg-charcoal-800 border border-sand-200 dark:border-charcoal-700 rounded-lg text-charcoal-900 dark:text-steel-100 focus:ring-2 focus:ring-red-500 focus:border-transparent"
              >
                <option value="standard">{getLabel("engineer.output_format.standard")}</option>
                <option value="detailed">{getLabel("engineer.output_format.detailed")}</option>
                <option value="summary">{getLabel("engineer.output_format.summary")}</option>
              </select>
            </div>
          )}

          {/* Dynamic Form */}
          {selectedCalculator && calculatorMetadata && calculatorMetadata.parameters && (
            <form
              onSubmit={(e) => {
                e.preventDefault();
                handleCalculate();
              }}
            >
              {calculatorMetadata.parameters.map((param) => (
                <div key={param.name} className="mb-4">
                  <label className="block text-sm font-medium mb-2 text-charcoal-700 dark:text-steel-300">
                    {getLabel(`calculators.${selectedCalculator}.${param.name}`)}
                  </label>
                  <div className="relative">
                    <input
                      type={param.type === "number" ? "number" : "text"}
                      name={param.name}
                      value={formData[param.name] || ""}
                      onChange={handleInputChange}
                      className="w-full p-3 bg-white dark:bg-charcoal-800 border border-sand-200 dark:border-charcoal-700 rounded-lg text-charcoal-900 dark:text-steel-100 focus:ring-2 focus:ring-red-500 focus:border-transparent"
                      placeholder={getLabel(`calculators.${selectedCalculator}.${param.name}.placeholder`) || ""}
                    />
                    {param.unit && (
                      <span className="absolute right-3 top-1/2 -translate-y-1/2 text-sm text-charcoal-500 dark:text-steel-500">
                        {param.unit}
                      </span>
                    )}
                  </div>
                </div>
              ))}
              <button
                type="submit"
                disabled={isLoading}
                className="w-full bg-gradient-to-r from-red-600 to-rose-600 text-white font-medium py-3 px-4 rounded-lg hover:from-red-700 hover:to-rose-700 disabled:opacity-50 transition-colors"
              >
                {isLoading ? getLabel("engineer.calculating") : getLabel("engineer.calculate")}
              </button>
            </form>
          )}

          {error && (
            <p className="mt-4 text-red-600 dark:text-red-400">{error}</p>
          )}

          {!show3D && selectedCalculator && (
            <div className="mt-6 p-3 rounded-lg bg-sand-100 dark:bg-charcoal-800 border border-sand-200 dark:border-charcoal-700">
              <span className="text-xs text-charcoal-500 dark:text-steel-400">
                {getLabel("engineer.visualization.unavailable")}
              </span>
            </div>
          )}
        </div>
      </div>

      {/* Right Column: Visualization & Results */}
      <div className="lg:col-span-1 space-y-8">
        {/* Visualization Panel */}
        {show3D && selectedCalculator && (
          <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-xl border border-sand-200 dark:border-charcoal-800 p-6">
            <h2 className="text-xl font-semibold text-charcoal-900 dark:text-white mb-4">
              {getLabel("engineer.visualization.title")}
            </h2>
            <div className="h-96 bg-sand-50 dark:bg-charcoal-800 rounded-lg flex items-center justify-center text-charcoal-500 dark:text-steel-400">
              <p>3D Visualization Placeholder</p>
            </div>
          </div>
        )}

        {/* Results Panel */}
        {(results.length > 0 || warnings.length > 0 || recommendations.length > 0) && (
          <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-xl border border-sand-200 dark:border-charcoal-800 p-6">
            <h2 className="text-xl font-semibold text-charcoal-900 dark:text-white mb-6">
              {getLabel("engineer.results.title") || "Results"}
            </h2>

            {results.length > 0 && (
              <div className="mb-6">
                <h3 className="text-lg font-medium mb-3 text-charcoal-800 dark:text-steel-200">
                  {getLabel("engineer.results.calculations") || "Calculations"}
                </h3>
                <div className="space-y-2">
                  {results.map((res, index) => (
                    <div
                      key={index}
                      className="flex justify-between items-center border-b border-sand-100 dark:border-charcoal-700 pb-2"
                    >
                      <span className="text-charcoal-700 dark:text-steel-300">
                        {getLabel(res.key)}
                      </span>
                      <span className="font-medium text-charcoal-900 dark:text-white">
                        {res.value} {res.unit || ""}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {warnings.length > 0 && (
              <div className="mb-6">
                <h3 className="text-lg font-medium mb-3 text-yellow-700 dark:text-yellow-300">
                  {getLabel("engineer.results.warnings") || "Warnings"}
                </h3>
                <ul className="space-y-2">
                  {warnings.map((w, i) => (
                    <li key={i} className="text-sm text-yellow-600 dark:text-yellow-400">
                      {w}
                    </li>
                  ))}
                </ul>
              </div>
            )}

            {structuredWarnings && (
              <div className="mb-6">
                <h3 className="text-lg font-medium mb-3 text-yellow-700 dark:text-yellow-300">
                  {getLabel("engineer.results.structured_warnings") || "Detailed Warnings"}
                </h3>
                <pre className="bg-sand-50 dark:bg-charcoal-800 p-3 rounded-lg text-sm overflow-auto">
                  {JSON.stringify(structuredWarnings, null, 2)}
                </pre>
              </div>
            )}

            {recommendations.length > 0 && (
              <div>
                <h3 className="text-lg font-medium mb-3 text-green-700 dark:text-green-300">
                  {getLabel("engineer.results.recommendations") || "Recommendations"}
                </h3>
                <ul className="space-y-2">
                  {recommendations.map((r, i) => (
                    <li key={i} className="text-sm text-green-600 dark:text-green-400">
                      {r}
                    </li>
                  ))}
                </ul>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};

export default Chooser;