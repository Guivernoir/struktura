import PropTypes from "prop-types";
import Icon from "../Icon";

const BeginnerForm = ({
  t,
  selectedCategory,
  selectedCalculator,
  setSelectedCalculator,
  calculators,
  calculatorMeta,
  formData,
  handleInputChange,
  handleCalculate,
  isLoading,
  error,
}) => {
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

  // Get hints and ranges from metadata
  const hints = calculatorMeta?.input_hints || {};
  const ranges = calculatorMeta?.typical_ranges || {};

  // Standard fields for beginner calculators
  const standardFields = ["width", "length", "height", "depth", "thickness"];

  const renderInputField = (field) => {
    const label =
      hints[field] || field.charAt(0).toUpperCase() + field.slice(1);
    const range = ranges[field] || [0.1, 50];
    const value = formData[field] ?? "";

    // Skip if range is [1,1] (field not used)
    if (range[0] === 1 && range[1] === 1) return null;

    return (
      <div key={field} className="space-y-1.5">
        <div className="flex items-center justify-between">
          <label className="text-xs font-medium text-charcoal-700 dark:text-steel-300">
            {label}
          </label>
          <span className="text-xs text-charcoal-400 dark:text-steel-600 font-mono">
            m
          </span>
        </div>
        <input
          type="number"
          name={field}
          value={value}
          onChange={handleInputChange}
          min={range[0]}
          max={range[1]}
          step="0.05"
          placeholder={`${range[0]} - ${range[1]}`}
          className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white focus:ring-green-500 focus:border-green-500 transition"
        />
        {range[0] !== 0.1 && (
          <p className="text-xs text-charcoal-400 dark:text-steel-600">
            {t?.beginner?.form?.typical_range || "Typical"}: {range[0]} -{" "}
            {range[1]} m
          </p>
        )}
      </div>
    );
  };

  return (
    <div className="space-y-6">
      {/* Calculator Selector */}
      <div>
        <label className="text-sm font-medium text-charcoal-700 dark:text-steel-300 block mb-2">
          {getLabel("beginner.form.select_project")}
        </label>
        <select
          value={selectedCalculator || ""}
          onChange={(e) => setSelectedCalculator(e.target.value)}
          className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white focus:ring-green-500 focus:border-green-500"
          disabled={calculators.length === 0}
        >
          <option value="">
            {t?.beginner?.form?.choose_project || "Choose a project..."}
          </option>
          {calculators.map((calc) => (
            <option key={calc.value} value={calc.value}>
              {calc.label}
            </option>
          ))}
        </select>
      </div>

      {selectedCalculator && calculatorMeta && (
        <>
          {/* Calculator Description */}
          {calculatorMeta.description && (
            <div className="p-3 bg-green-50 dark:bg-green-900/10 rounded-lg text-xs text-green-900 dark:text-green-100 border border-green-200 dark:border-green-800">
              {calculatorMeta.description}
            </div>
          )}

          {/* Input Fields */}
          <div className="space-y-4">
            <h3 className="text-sm font-semibold text-charcoal-700 dark:text-steel-300 uppercase tracking-wide">
              {getLabel("beginner.form.dimensions")}
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {standardFields.map(renderInputField)}
            </div>
          </div>
        </>
      )}

      {error && (
        <div className="p-3 bg-red-50 dark:bg-red-900/20 rounded-lg text-red-700 dark:text-red-300 flex items-center gap-2 text-sm">
          <Icon name="AlertTriangle" size={16} />
          {error}
        </div>
      )}

      <button
        onClick={handleCalculate}
        disabled={!selectedCalculator || isLoading}
        className={`w-full flex items-center justify-center gap-2 px-6 py-4 rounded-lg font-semibold text-base transition-all duration-200 ${
          !selectedCalculator || isLoading
            ? "bg-sand-200 dark:bg-charcoal-700 text-charcoal-400 dark:text-steel-600 cursor-not-allowed"
            : "bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-700 hover:to-emerald-700 text-white shadow-lg shadow-green-600/30 hover:shadow-xl hover:shadow-green-600/40"
        }`}
      >
        {isLoading ? (
          <>
            <Icon name="Loader2" size={20} className="animate-spin" />
            {t?.beginner?.form?.calculating || "Calculating..."}
          </>
        ) : (
          <>
            <Icon name="Calculator" size={20} />
            {getLabel("beginner.form.calculate")}
          </>
        )}
      </button>

      {/* Help Text */}
      <p className="text-xs text-center text-charcoal-500 dark:text-steel-500">
        {getLabel("beginner.form.help_text") ||
          "Enter your project dimensions above and click Calculate"}
      </p>
    </div>
  );
};

BeginnerForm.propTypes = {
  t: PropTypes.object.isRequired,
  selectedCategory: PropTypes.string.isRequired,
  selectedCalculator: PropTypes.string,
  setSelectedCalculator: PropTypes.func.isRequired,
  calculators: PropTypes.array.isRequired,
  calculatorMeta: PropTypes.object,
  formData: PropTypes.object.isRequired,
  handleInputChange: PropTypes.func.isRequired,
  handleCalculate: PropTypes.func.isRequired,
  isLoading: PropTypes.bool.isRequired,
  error: PropTypes.string,
};

export default BeginnerForm;
