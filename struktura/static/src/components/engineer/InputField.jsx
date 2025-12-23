import React from "react";
import PropTypes from "prop-types";
import DateTimePicker from "./DateTimePicker";

/**
 * A robust InputField that handles:
 * 1. Standard inputs (text, number, etc.)
 * 2. Select dropdowns
 * 3. Custom WheelDateTimePicker for dates
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
  stepMinutes = 15,
  daysRange = 30,
}) => {
  // Ensure we never pass 'undefined' to a form element to keep it "controlled"
  const safeValue = value ?? "";
  const hasError = Boolean(error);

  // Common Tailwind classes for consistency
  const baseInputClasses = `w-full p-3 border rounded-xl transition shadow-sm bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-white disabled:opacity-50 disabled:cursor-not-allowed ${
    hasError
      ? "border-red-500 focus:ring-red-500 focus:border-red-500"
      : "border-sand-300 dark:border-charcoal-700 focus:ring-indigo-500 focus:border-indigo-500 focus:shadow-md"
  }`;

  // 1. Special Case: Wheel Date Picker
  if (type === "datetime-local") {
    return (
      <DateTimePicker
        label={label}
        name={name}
        value={safeValue}
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

  return (
    <div className="flex flex-col space-y-1.5 w-full">
      {/* Label Section */}
      <label
        htmlFor={name}
        className="text-sm font-medium text-charcoal-700 dark:text-steel-300"
      >
        {label}
        {required && <span className="text-red-500 ml-1">*</span>}
      </label>

      <div className="relative">
        {/* 2. Select Input Case */}
        {type === "select" ? (
          <select
            id={name}
            name={name}
            value={safeValue}
            onChange={onChange}
            disabled={disabled}
            className={baseInputClasses}
          >
            <option value="" disabled>
              Select {label}
            </option>
            {options?.map((opt) => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
        ) : (
          /* 3. Standard Input Case (text, number, tel, etc.) */
          <>
            <input
              id={name}
              name={name}
              type={type}
              min={min}
              max={max}
              step={step}
              value={safeValue}
              onChange={onChange}
              placeholder={placeholder}
              disabled={disabled}
              className={`${baseInputClasses} ${unit ? "pr-16" : "pr-3"}`}
            />
            {unit && (
              <span className="absolute right-3 top-1/2 -translate-y-1/2 text-sm text-charcoal-500 dark:text-steel-500 font-medium pointer-events-none">
                {unit}
              </span>
            )}
          </>
        )}
      </div>

      {/* Messaging Section */}
      {error ? (
        <span className="text-xs text-red-500 animate-in fade-in duration-200">
          {error}
        </span>
      ) : helpText ? (
        <span className="text-xs text-charcoal-500 dark:text-steel-500">
          {helpText}
        </span>
      ) : null}
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
      value: PropTypes.oneOfType([PropTypes.string, PropTypes.number])
        .isRequired,
      label: PropTypes.string.isRequired,
    })
  ),
  stepMinutes: PropTypes.number,
  daysRange: PropTypes.number,
};

export default InputField;
