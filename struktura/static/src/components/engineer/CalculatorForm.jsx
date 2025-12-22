import { useState } from "react";
import PropTypes from "prop-types";
import Icon from "../Icon";
import ArrayInput from "./ArrayInput";
import { DesignCodes, DesignCodeNames, formatDesignCode } from "../../lib";
import {
  ARRAY_SCHEMAS,
  isoToDatetimeLocal,
  datetimeLocalToISO,
} from "../../hooks/engineer/types";

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
    extended: true,
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
    let value;

    if (section === "extended_parameters" || section === "extendedParameters") {
      // Direct lookup in the flat extendedParameters object using the dotted field name
      value = formData.extendedParameters?.[field] ?? "";
    } else {
      // Standard recursive lookup for dimensions, material, etc.
      value = formData;
      pathParts.forEach((part) => {
        value = value?.[part] ?? "";
      });
    }

    // Check if data_type is an Enum (which comes from Rust as an object: { "enum": [...] })
    const isEnum = typeof param.data_type === "object" && param.data_type.enum;
    const options = isEnum ? param.data_type.enum : [];

    // Handle Array type
    if (param.data_type === "array") {
      // Determine which array schema to use
      const arrayKey = pathParts[pathParts.length - 1];
      const itemSchema = ARRAY_SCHEMAS[arrayKey] || {
        fields: [
          {
            name: "value",
            label: "Value",
            type: "string",
            required: true,
          },
        ],
      };

      return (
        <div key={param.path} className="col-span-1 md:col-span-2">
          <ArrayInput
            label={param.name}
            name={param.path}
            value={Array.isArray(value) ? value : []}
            onChange={handleFormEvent}
            itemSchema={itemSchema}
            required={param.required}
            helpText={param.description}
            t={t}
          />
        </div>
      );
    }

    const renderInput = () => {
      // 1. Render Dropdown for Enums
      if (isEnum) {
        return (
          <select
            name={param.path}
            value={value}
            onChange={(e) => {
              const event = {
                target: { name: param.path, value: e.target.value },
              };
              handleFormEvent(event);
            }}
            className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white focus:ring-blue-500 focus:border-blue-500 transition"
          >
            <option value="">
              {t?.engineer?.form?.select_placeholder || "Select..."}
            </option>
            {options.map((opt) => (
              <option key={opt} value={opt}>
                {opt.charAt(0).toUpperCase() + opt.slice(1)}
              </option>
            ))}
          </select>
        );
      }

      // 2. Render DateTime Input
      if (param.data_type === "datetime") {
        // Convert ISO to datetime-local format for input
        const localValue = value ? isoToDatetimeLocal(value) : "";

        return (
          <input
            type="datetime-local"
            name={param.path}
            value={localValue}
            onChange={(e) => {
              // Convert datetime-local back to ISO 8601 for storage
              const isoValue = datetimeLocalToISO(e.target.value);
              const event = {
                target: { name: param.path, value: isoValue },
              };
              handleFormEvent(event);
            }}
            className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white"
          />
        );
      }

      // 3. Render Text Input for Strings
      if (param.data_type === "string") {
        return (
          <input
            type="text"
            name={param.path}
            value={value}
            onChange={handleInputChange}
            placeholder={param.description}
            className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white"
          />
        );
      }

      // 4. Render Boolean Checkbox
      if (param.data_type === "boolean") {
        return (
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              name={param.path}
              checked={Boolean(value)}
              onChange={(e) => {
                const event = {
                  target: { name: param.path, value: e.target.checked },
                };
                handleFormEvent(event);
              }}
              className="w-5 h-5 text-indigo-600 border-sand-300 dark:border-charcoal-700 rounded focus:ring-indigo-500"
            />
            <span className="text-sm text-charcoal-700 dark:text-steel-300">
              {param.description || "Enable"}
            </span>
          </label>
        );
      }

      // 5. Default: Render Number Input
      return (
        <input
          type="number"
          name={param.path}
          value={value}
          onChange={handleFormEvent}
          min={param.min_value ?? undefined}
          max={param.max_value ?? undefined}
          step={param.data_type === "integer" ? "1" : "0.1"}
          placeholder={param.description}
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
        {param.description && param.data_type !== "boolean" && (
          <p className="text-xs text-charcoal-500 dark:text-steel-500">
            {param.description}
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

  const renderSection = (section, title, defaultExpanded = false) => {
    const params = getSectionParameters(section);
    if (params.length === 0) return null;

    return (
      <div className="space-y-3">
        <button
          onClick={() => toggleSection(section)}
          className="w-full flex items-center justify-between px-4 py-3 bg-sand-100 dark:bg-charcoal-800 rounded-xl text-sm font-medium text-charcoal-800 dark:text-steel-200 hover:bg-sand-200 dark:hover:bg-charcoal-700 transition"
        >
          <div className="flex items-center gap-2">
            <span>{title}</span>
            {params.some((p) => p.required) && (
              <span className="px-2 py-0.5 text-xs bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400 rounded">
                {t?.engineer?.form?.required || "Required"}
              </span>
            )}
          </div>
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
          {t?.engineer?.form?.design_code || "Design Code"}
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
          <option value="">
            {t?.engineer?.form?.select_design_code || "Select Design Code"}
          </option>
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
          {t?.engineer?.form?.select_calculator || "Select Calculator"}
        </label>
        <select
          value={selectedCalculator || ""}
          onChange={(e) => setSelectedCalculator(e.target.value)}
          className="w-full p-3 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white"
          disabled={calculators.length === 0}
        >
          <option value="">
            {t?.engineer?.form?.choose_calculator || "Choose a calculator..."}
          </option>
          {calculators.map((calc) => (
            <option key={calc.id} value={calc.id}>
              {calc.name}
            </option>
          ))}
        </select>

        {selectedCalculator && calculatorMeta && (
          <div className="mt-2 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
            <p className="text-sm text-blue-800 dark:text-blue-300">
              {calculatorMeta.description}
            </p>
            {calculatorMeta.complexity_level && (
              <div className="mt-2 flex items-center gap-2">
                <span className="text-xs text-blue-600 dark:text-blue-400 font-medium">
                  Complexity:
                </span>
                <span
                  className={`px-2 py-0.5 text-xs rounded ${
                    calculatorMeta.complexity_level === "basic"
                      ? "bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300"
                      : calculatorMeta.complexity_level === "intermediate"
                      ? "bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-300"
                      : "bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300"
                  }`}
                >
                  {calculatorMeta.complexity_level.charAt(0).toUpperCase() +
                    calculatorMeta.complexity_level.slice(1)}
                </span>
              </div>
            )}
          </div>
        )}
      </div>

      {selectedCalculator && calculatorMeta && (
        <>
          {/* Extended Parameters Section (NEW) */}
          {renderSection(
            "extended_parameters",
            t?.engineer?.form?.sections?.extended_parameters ||
              "Calculator Parameters",
            true
          )}

          {/* Standard Sections */}
          {renderSection(
            "dimensions",
            t?.engineer?.form?.sections?.dimensions || "Dimensions"
          )}
          {renderSection(
            "material",
            t?.engineer?.form?.sections?.material || "Material Properties"
          )}
          {renderSection(
            "loads",
            t?.engineer?.form?.sections?.loads || "Load Cases"
          )}
          {renderSection(
            "safetyFactors",
            t?.engineer?.form?.sections?.safety_factors || "Safety Factors"
          )}
          {renderSection(
            "additional",
            t?.engineer?.form?.sections?.additional || "Additional Parameters"
          )}

          {/* Top-level fields like designCode, exposureClass, etc. */}
          <div className="space-y-4">{renderDesignCodeSelector()}</div>
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
            {t?.engineer?.form?.calculating || "Calculating..."}
          </>
        ) : (
          <>
            <Icon name="Play" size={18} />
            {t?.engineer?.form?.calculate || "Calculate"}
          </>
        )}
      </button>

      {calculatorMeta?.required_parameters?.length > 0 && (
        <p className="text-xs text-charcoal-500 dark:text-steel-500 text-center">
          <span className="text-red-500">*</span>{" "}
          {t?.engineer?.form?.required_fields ||
            "Required fields must be filled"}
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
  handleFormEvent: PropTypes.func.isRequired,
  handleCalculate: PropTypes.func.isRequired,
  isLoading: PropTypes.bool.isRequired,
  error: PropTypes.string,
};

export default CalculatorForm;
