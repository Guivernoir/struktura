import PropTypes from "prop-types";
import WheelDateTimePicker from "./WheelDateTimePicker"; // Updated import

/**
 * InputField component with proper type handling for engineering parameters
 * Updated to support the WheelDateTimePicker for datetime-local types
 */
const InputField = ({
  label,
  name,
  type = "number",
  unit,
  value,
  onChange,
  min,
  max,
  step = "0.1",
  placeholder,
  required = false,
  disabled = false,
  error,
  helpText,
  options,
  // New props specific to the Wheel Picker logic
  stepMinutes = 15,
  daysRange = 30,
}) => {
  const hasError = Boolean(error);

  // 1. Render Select Input
  if (type === "select" && options) {
    return (
      <div className="flex flex-col space-y-1.5">
        <label
          htmlFor={name}
          className="text-sm font-medium text-charcoal-700 dark:text-steel-300"
        >
          {label}
          {required && <span className="text-red-500 ml-1">*</span>}
        </label>
        <select
          id={name}
          name={name}
          value={value || ""}
          onChange={onChange}
          disabled={disabled}
          className={`w-full p-3 border rounded-xl transition shadow-sm
            ${
              hasError
                ? "border-red-500 focus:ring-red-500 focus:border-red-500"
                : "border-sand-300 dark:border-charcoal-700 focus:ring-indigo-500 focus:border-indigo-500 focus:shadow-md"
            }
            bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-white
            disabled:opacity-50 disabled:cursor-not-allowed`}
        >
          <option value="">Select {label}</option>
          {options.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
        {error && <span className="text-sm text-red-500">{error}</span>}
        {helpText && !error && (
          <span className="text-xs text-charcoal-500 dark:text-steel-500">
            {helpText}
          </span>
        )}
      </div>
    );
  }

  // 2. Render Wheel-based DateTimePicker
  if (type === "datetime-local") {
    return (
      <WheelDateTimePicker
        label={label}
        name={name}
        value={value}
        onChange={onChange}
        required={required}
        disabled={disabled}
        error={error}
        helpText={helpText}
        stepMinutes={stepMinutes}
        daysRange={daysRange}
      />
    );
  }

  // 3. Render Standard Text or Number Input
  return (
    <div className="flex flex-col space-y-1.5">
      <label
        htmlFor={name}
        className="text-sm font-medium text-charcoal-700 dark:text-steel-300"
      >
        {label}
        {required && <span className="text-red-500 ml-1">*</span>}
      </label>
      <div className="relative">
        <input
          id={name}
          name={name}
          type={type}
          min={min}
          max={max}
          step={step}
          value={value ?? ""}
          onChange={onChange}
          placeholder={placeholder}
          disabled={disabled}
          className={`w-full p-3 border rounded-xl transition shadow-sm
            ${unit ? "pr-16" : "pr-3"}
            ${
              hasError
                ? "border-red-500 focus:ring-red-500 focus:border-red-500"
                : "border-sand-300 dark:border-charcoal-700 focus:ring-indigo-500 focus:border-indigo-500 focus:shadow-md"
            }
            bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-white
            disabled:opacity-50 disabled:cursor-not-allowed`}
        />
        {unit && (
          <span className="absolute right-3 top-1/2 -translate-y-1/2 text-sm text-charcoal-500 dark:text-steel-500 font-medium pointer-events-none">
            {unit}
          </span>
        )}
      </div>
      {error && <span className="text-sm text-red-500">{error}</span>}
      {helpText && !error && (
        <span className="text-xs text-charcoal-500 dark:text-steel-500">
          {helpText}
        </span>
      )}
    </div>
  );
};

InputField.propTypes = {
  label: PropTypes.string.isRequired,
  name: PropTypes.string.isRequired,
  type: PropTypes.oneOf([
    "number",
    "text",
    "email",
    "tel",
    "select",
    "datetime-local",
  ]),
  unit: PropTypes.string,
  value: PropTypes.oneOfType([PropTypes.string, PropTypes.number]),
  onChange: PropTypes.func.isRequired,
  min: PropTypes.oneOfType([PropTypes.number, PropTypes.string]),
  max: PropTypes.oneOfType([PropTypes.number, PropTypes.string]),
  step: PropTypes.string,
  placeholder: PropTypes.string,
  required: PropTypes.bool,
  disabled: PropTypes.bool,
  error: PropTypes.string,
  helpText: PropTypes.string,
  options: PropTypes.arrayOf(
    PropTypes.shape({
      value: PropTypes.string.isRequired,
      label: PropTypes.string.isRequired,
    })
  ),
  stepMinutes: PropTypes.number,
  daysRange: PropTypes.number,
};

export default InputField;
