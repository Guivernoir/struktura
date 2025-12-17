import { useState } from "react";
import PropTypes from "prop-types";
import Icon from "../Icon";
import { DesignCodes, DesignCodeNames, formatDesignCode } from "../../lib";

const CalculatorForm = ({
  t,
  selectedCategory,
  selectedCalculator,
  setSelectedCalculator,
  calculators,
  calculatorMeta,
  formData,
  handleInputChange,
  handleFormEvent,
  handleCalculate,
  isLoading,
  error,
}) => {
  const [expandedSections, setExpandedSections] = useState({
    dimensions: true,
    material: false,
    loads: false,
    advanced: false,
  });

  const toggleSection = (section) => {
    setExpandedSections((prev) => ({ ...prev, [section]: !prev[section] }));
  };

  const renderParameterInput = (param) => {
    const pathParts = param.path.split(".");
    const section = pathParts[0];
    const field = pathParts.slice(1).join(".");

    // Helper to get deep value
    let value = formData;
    pathParts.forEach((part) => {
      value = value?.[part] ?? ""; // Use nullish coalescing for safety
    });

    // Check if data_type is an Enum (which comes from Rust as an object: { "enum": [...] })
    // or if validation_rules imply a selection.
    const isEnum = typeof param.data_type === "object" && param.data_type.enum;
    const options = isEnum ? param.data_type.enum : [];

    const renderInput = () => {
      // 1. Render Dropdown for Enums
      if (isEnum) {
        return (
          <select
            name={param.path}
            value={value}
            onChange={(e) => {
              // Pass raw value for enums (strings)
              const event = {
                target: { name: param.path, value: e.target.value },
              };
              handleFormEvent(event);
            }}
            className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white focus:ring-blue-500 focus:border-blue-500 transition"
          >
            <option value="">Select...</option>
            {options.map((opt) => (
              <option key={opt} value={opt}>
                {opt.charAt(0).toUpperCase() + opt.slice(1)} {/* Capitalize */}
              </option>
            ))}
          </select>
        );
      }

      // 2. Render Text Input for Strings
      if (param.data_type === "string") {
        return (
          <input
            type="text"
            name={param.path}
            value={value}
            onChange={handleInputChange} // This handles strings correctly now
            className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white"
          />
        );
      }

      // 3. Default: Render Number Input
      return (
        <input
          type="number"
          name={param.path}
          value={value}
          onChange={handleFormEvent} // Use form event to parse float
          min={param.min_value ?? undefined}
          max={param.max_value ?? undefined}
          step={param.data_type === "integer" ? "1" : "0.1"}
          className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white"
        />
      );
    };

    return (
      <div key={param.path} className="space-y-1.5">
        <div className="flex items-center justify-between">
          <label className="text-xs font-medium text-charcoal-700 dark:text-steel-300 flex items-center gap-1.5">
            {param.name || field}
            {param.required && <span className="text-red-500 text-sm">*</span>}
          </label>
          {param.unit && (
            <span className="text-xs text-charcoal-400 dark:text-steel-600 font-mono">
              {param.unit}
            </span>
          )}
        </div>
        {renderInput()}
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
          className="w-full flex items-center justify-between px-4 py-3 bg-sand-100 dark:bg-charcoal-800 rounded-xl text-sm font-medium text-charcoal-800 dark:text-steel-200"
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

  const renderDesignCodeSelector = () => {
    if (!calculatorMeta?.design_codes?.length) return null;

    return (
      <div className="space-y-1.5">
        <label className="text-xs font-medium text-charcoal-700 dark:text-steel-300">
          Design Code
        </label>
        <select
          value={formData.design_code || ""}
          onChange={(e) => {
            const event = {
              target: {
                name: "design_code",
                value: e.target.value,
              },
            };
            handleFormEvent(event);
          }}
          className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white focus:ring-amber-500 focus:border-amber-500"
        >
          <option value="">Select Design Code</option>
          {calculatorMeta.design_codes.map((code) => (
            <option key={code} value={code}>
              {DesignCodeNames[code] || code}
            </option>
          ))}
        </select>
      </div>
    );
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
          className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white"
          disabled={calculators.length === 0}
        >
          <option value="">Choose a calculator...</option>
          {calculators.map((calc) => (
            <option key={calc.id} value={calc.id}>
              {calc.name}
            </option>
          ))}
        </select>
      </div>

      {selectedCalculator && calculatorMeta && (
        <>
          {/* Sections */}
          {renderSection("dimensions", "Dimensions")}
          {renderSection("material", "Material Properties")}
          {renderSection("loads", "Load Cases")}
          {renderSection("safetyFactors", "Safety Factors")}
          {renderSection("additional", "Additional Parameters")}

          {/* Top-level fields like designCode, exposureClass, etc. */}
          <div className="space-y-4">
            {renderDesignCodeSelector()}
            {/* Add similar for exposureClass, temperature, humidity if in metadata */}
          </div>
        </>
      )}

      {error && (
        <div className="p-3 bg-red-50 dark:bg-red-900/20 rounded-lg text-red-700 dark:text-red-300 flex items-center gap-2">
          <Icon name="AlertTriangle" size={16} />
          {error}
        </div>
      )}

      <button
        onClick={handleCalculate}
        disabled={!selectedCalculator || isLoading}
        className={`w-full flex items-center justify-center gap-2 px-6 py-3 rounded-lg font-semibold text-sm transition-all duration-200 ${
          !selectedCalculator || isLoading
            ? "bg-sand-200 dark:bg-charcoal-700 text-charcoal-400 dark:text-steel-600 cursor-not-allowed"
            : "bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-700 hover:to-purple-700 text-white shadow-lg shadow-purple-600/30 hover:shadow-xl hover:shadow-purple-600/40"
        }`}
      >
        {isLoading ? (
          <>
            <Icon name="Loader2" size={18} className="animate-spin" />
            Calculating...
          </>
        ) : (
          <>
            <Icon name="Play" size={18} />
            Calculate
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

CalculatorForm.propTypes = {
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

export default CalculatorForm;
