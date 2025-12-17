import { useState } from "react";
import PropTypes from "prop-types";
import Icon from "../Icon";
import { RegulationCodeNames } from "../../lib";

const ContractorForm = ({
  t,
  selectedCategory,
  selectedCalculator,
  setSelectedCalculator,
  calculators,
  calculatorMeta,
  formData,
  handleFormEvent,
  updateDimension,
  updateMaterial,
  updateResource,
  updateSafetyFactor,
  updateAdditional,
  handleCalculate,
  handleCalculateDetailed,
  handleCalculateSummary,
  isLoading,
  error,
}) => {
  const [expandedSections, setExpandedSections] = useState({
    dimensions: true,
    material: false,
    resources: false,
    safety_factors: false,
    additional: false,
    project_metadata: false,
  });

  const [outputFormat, setOutputFormat] = useState("standard");

  const toggleSection = (section) => {
    setExpandedSections((prev) => ({ ...prev, [section]: !prev[section] }));
  };

  const renderParameterInput = (param) => {
    const pathParts = param.path.split(".");
    const section = pathParts[0];
    let value = formData;
    pathParts.forEach((part) => {
      value = value?.[part];
    });

    // Handle different data types
    const isNumber =
      param.data_type === "number" || param.data_type === "integer";
    const isString = param.data_type === "string";

    return (
      <div key={param.path} className="space-y-1.5">
        <div className="flex items-center justify-between">
          <label className="text-xs font-medium text-charcoal-700 dark:text-steel-300 flex items-center gap-1.5">
            {param.name}
            {param.required && <span className="text-red-500 text-sm">*</span>}
          </label>
          {param.unit && (
            <span className="text-xs text-charcoal-400 dark:text-steel-600 font-mono">
              {param.unit}
            </span>
          )}
        </div>
        <input
          type={isNumber ? "number" : "text"}
          name={param.path}
          value={value ?? ""}
          onChange={handleFormEvent}
          min={isNumber ? param.min_value ?? undefined : undefined}
          max={isNumber ? param.max_value ?? undefined : undefined}
          step={param.data_type === "integer" ? "1" : "0.1"}
          placeholder={param.description}
          className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white focus:ring-amber-500 focus:border-amber-500 transition"
        />
        {param.typical_range && (
          <p className="text-xs text-charcoal-400 dark:text-steel-600">
            Typical: {param.typical_range[0]} - {param.typical_range[1]}{" "}
            {param.unit}
          </p>
        )}
      </div>
    );
  };

  const getSectionParameters = (section) => {
    return (calculatorMeta?.parameters || []).filter((param) =>
      param.path.startsWith(section + ".")
    );
  };

  const renderSection = (section, title) => {
    const params = getSectionParameters(section);
    if (params.length === 0) return null;

    return (
      <div className="space-y-3">
        <button
          onClick={() => toggleSection(section)}
          className="w-full flex items-center justify-between px-4 py-3 bg-sand-100 dark:bg-charcoal-800 rounded-xl text-sm font-medium text-charcoal-800 dark:text-steel-200 hover:bg-sand-200 dark:hover:bg-charcoal-700 transition"
        >
          <span>{title}</span>
          <Icon
            name={expandedSections[section] ? "ChevronUp" : "ChevronDown"}
            size={16}
          />
        </button>
        {expandedSections[section] && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pl-4">
            {params.map(renderParameterInput)}
          </div>
        )}
      </div>
    );
  };

  const renderRegulationCodeSelector = () => {
    if (!calculatorMeta?.regulation_codes?.length) return null;

    return (
      <div className="space-y-1.5">
        <label className="text-xs font-medium text-charcoal-700 dark:text-steel-300">
          Regulation Code
        </label>
        <select
          value={formData.regulation_code || ""}
          onChange={(e) => {
            const event = {
              target: {
                name: "regulation_code",
                value: e.target.value,
              },
            };
            handleFormEvent(event);
          }}
          className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white focus:ring-amber-500 focus:border-amber-500"
        >
          <option value="">Select Regulation Code</option>
          {calculatorMeta.regulation_codes.map((code) => (
            <option key={code} value={code}>
              {RegulationCodeNames[code] || code}
            </option>
          ))}
        </select>
      </div>
    );
  };

  const handleCalculateClick = () => {
    if (outputFormat === "detailed") {
      handleCalculateDetailed();
    } else if (outputFormat === "summary") {
      handleCalculateSummary();
    } else {
      handleCalculate();
    }
  };

  return (
    <div className="space-y-6">
      {/* Calculator Selector */}
      <div>
        <label className="text-sm font-medium text-charcoal-700 dark:text-steel-300 block mb-2">
          Select Calculator
        </label>
        <select
          value={selectedCalculator || ""}
          onChange={(e) => setSelectedCalculator(e.target.value)}
          className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white focus:ring-amber-500 focus:border-amber-500"
          disabled={calculators.length === 0}
        >
          <option value="">Choose a calculator...</option>
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
            <div className="p-3 bg-amber-50 dark:bg-amber-900/10 rounded-lg text-xs text-amber-900 dark:text-amber-100 border border-amber-200 dark:border-amber-800">
              {calculatorMeta.description}
            </div>
          )}

          {/* Sections */}
          {renderSection("dimensions", "Dimensions")}
          {renderSection("material", "Material Properties")}
          {renderSection("resources", "Resource Requirements")}
          {renderSection("safety_factors", "Safety Factors")}
          {renderSection("additional", "Additional Parameters")}
          {renderSection("project_metadata", "Project Information")}

          {/* Top-level fields */}
          <div className="space-y-4">
            {renderRegulationCodeSelector()}

            {/* Exposure Class */}
            {calculatorMeta.parameters?.some(
              (p) => p.path === "exposure_class"
            ) && (
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-charcoal-700 dark:text-steel-300">
                  Exposure Class
                </label>
                <input
                  type="text"
                  name="exposure_class"
                  value={formData.exposure_class || ""}
                  onChange={handleFormEvent}
                  placeholder="e.g., C1, C2, C3"
                  className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white focus:ring-amber-500 focus:border-amber-500"
                />
              </div>
            )}
          </div>

          {/* Output Format Selector */}
          <div className="space-y-1.5">
            <label className="text-xs font-medium text-charcoal-700 dark:text-steel-300">
              Output Format
            </label>
            <div className="grid grid-cols-3 gap-2">
              {["standard", "detailed", "summary"].map((format) => (
                <button
                  key={format}
                  onClick={() => setOutputFormat(format)}
                  className={`px-3 py-2 rounded-lg text-xs font-medium transition ${
                    outputFormat === format
                      ? "bg-amber-600 text-white shadow-md"
                      : "bg-sand-100 dark:bg-charcoal-800 text-charcoal-700 dark:text-steel-300 hover:bg-sand-200 dark:hover:bg-charcoal-700"
                  }`}
                >
                  {format.charAt(0).toUpperCase() + format.slice(1)}
                </button>
              ))}
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
        onClick={handleCalculateClick}
        disabled={!selectedCalculator || isLoading}
        className={`w-full flex items-center justify-center gap-2 px-6 py-3 rounded-lg font-semibold text-sm transition-all duration-200 ${
          !selectedCalculator || isLoading
            ? "bg-sand-200 dark:bg-charcoal-700 text-charcoal-400 dark:text-steel-600 cursor-not-allowed"
            : "bg-gradient-to-r from-amber-600 to-orange-600 hover:from-amber-700 hover:to-orange-700 text-white shadow-lg shadow-orange-600/30 hover:shadow-xl hover:shadow-orange-600/40"
        }`}
      >
        {isLoading ? (
          <>
            <Icon name="Loader2" size={18} className="animate-spin" />
            Calculating...
          </>
        ) : (
          <>
            <Icon name="Calculator" size={18} />
            Calculate {outputFormat !== "standard" && `(${outputFormat})`}
          </>
        )}
      </button>

      {calculatorMeta?.required_parameters?.length > 0 && (
        <p className="text-xs text-charcoal-500 dark:text-steel-500 text-center">
          <span className="text-red-500">*</span> Required fields must be filled
        </p>
      )}
    </div>
  );
};

ContractorForm.propTypes = {
  t: PropTypes.object.isRequired,
  selectedCategory: PropTypes.string.isRequired,
  selectedCalculator: PropTypes.string,
  setSelectedCalculator: PropTypes.func.isRequired,
  calculators: PropTypes.array.isRequired,
  calculatorMeta: PropTypes.object,
  formData: PropTypes.object.isRequired,
  handleFormEvent: PropTypes.func.isRequired,
  updateDimension: PropTypes.func.isRequired,
  updateMaterial: PropTypes.func.isRequired,
  updateResource: PropTypes.func.isRequired,
  updateSafetyFactor: PropTypes.func.isRequired,
  updateAdditional: PropTypes.func.isRequired,
  handleCalculate: PropTypes.func.isRequired,
  handleCalculateDetailed: PropTypes.func.isRequired,
  handleCalculateSummary: PropTypes.func.isRequired,
  isLoading: PropTypes.bool.isRequired,
  error: PropTypes.string,
};

export default ContractorForm;
