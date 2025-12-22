/**
 * @file validators.js
 * @description Validation utilities with the precision of a sniper
 * Mission objective: Ensure data integrity before battlefield deployment
 * Now with extended_parameters support - because validation is surgical, not optional
 */

import {
  ValidationError,
  DesignCodes,
  RegulationCodes,
  ParameterType,
} from "./models.js";

export const Validators = {
  /**
   * Validates a number is within acceptable operational parameters
   */
  inRange: (value, min, max, field = "value") => {
    const num = parseFloat(value);
    if (isNaN(num)) {
      throw new ValidationError(
        `${field} must be a valid number`,
        field,
        value
      );
    }
    if (min !== null && num < min) {
      throw new ValidationError(`${field} must be >= ${min}`, field, value);
    }
    if (max !== null && num > max) {
      throw new ValidationError(`${field} must be <= ${max}`, field, value);
    }
    return num;
  },

  /**
   * Validates a value is among approved strategic options
   */
  oneOf: (value, options, field = "value") => {
    if (!options.includes(value)) {
      throw new ValidationError(
        `${field} must be one of: ${options.join(", ")}`,
        field,
        value
      );
    }
    return value;
  },

  /**
   * Validates mission-critical required fields exist
   */
  required: (obj, fields) => {
    const missing = fields.filter(
      (f) => obj[f] === undefined || obj[f] === null || obj[f] === ""
    );
    if (missing.length > 0) {
      throw new ValidationError(
        `Missing required fields: ${missing.join(", ")}`,
        missing[0]
      );
    }
  },

  /**
   * Validates design code against authorized standards
   */
  designCode: (code) => {
    const validCodes = Object.values(DesignCodes);
    if (!validCodes.includes(code)) {
      throw new ValidationError(
        `Invalid design code. Must be one of: ${validCodes.join(", ")}`,
        "design_code",
        code
      );
    }
    return code;
  },

  /**
   * Validates regulation code against authorized standards
   */
  regulationCode: (code) => {
    const validCodes = Object.values(RegulationCodes);
    if (!validCodes.includes(code)) {
      throw new ValidationError(
        `Invalid regulation code. Must be one of: ${validCodes.join(", ")}`,
        "regulation_code",
        code
      );
    }
    return code;
  },

  /**
   * Validates parameter type
   */
  parameterType: (type) => {
    const validTypes = Object.values(ParameterType);
    if (!validTypes.includes(type)) {
      throw new ValidationError(
        `Invalid parameter type. Must be one of: ${validTypes.join(", ")}`,
        "parameter_type",
        type
      );
    }
    return type;
  },

  /**
   * Validates ISO 8601 datetime string
   */
  datetime: (value, field = "datetime") => {
    if (typeof value !== "string") {
      throw new ValidationError(`${field} must be a string`, field, value);
    }

    // Check for ISO 8601 format (basic check)
    if (!value.includes("T")) {
      throw new ValidationError(
        `${field} must be in ISO 8601 format (e.g., 2025-12-18T06:00:00Z)`,
        field,
        value
      );
    }

    // Try to parse as date
    const date = new Date(value);
    if (isNaN(date.getTime())) {
      throw new ValidationError(
        `${field} is not a valid datetime`,
        field,
        value
      );
    }

    return value;
  },

  /**
   * Validates array parameter
   */
  array: (value, field = "array", minLength = 0, maxLength = null) => {
    if (!Array.isArray(value)) {
      throw new ValidationError(`${field} must be an array`, field, value);
    }

    if (minLength !== null && value.length < minLength) {
      throw new ValidationError(
        `${field} must have at least ${minLength} items`,
        field,
        value
      );
    }

    if (maxLength !== null && value.length > maxLength) {
      throw new ValidationError(
        `${field} must have at most ${maxLength} items`,
        field,
        value
      );
    }

    return value;
  },

  /**
   * Validates boolean parameter
   */
  boolean: (value, field = "boolean") => {
    if (typeof value !== "boolean") {
      throw new ValidationError(
        `${field} must be a boolean (true or false)`,
        field,
        value
      );
    }
    return value;
  },

  /**
   * Validates integer parameter
   */
  integer: (value, field = "integer", min = null, max = null) => {
    const num = parseInt(value, 10);
    if (isNaN(num) || !Number.isInteger(num)) {
      throw new ValidationError(
        `${field} must be a valid integer`,
        field,
        value
      );
    }

    if (min !== null && num < min) {
      throw new ValidationError(`${field} must be >= ${min}`, field, value);
    }

    if (max !== null && num > max) {
      throw new ValidationError(`${field} must be <= ${max}`, field, value);
    }

    return num;
  },

  /**
   * Validates ParameterValue structure
   */
  parameterValue: (value, field = "parameter_value") => {
    if (!value || typeof value !== "object") {
      throw new ValidationError(
        `${field} must be a ParameterValue object`,
        field,
        value
      );
    }

    if (!value.type || !value.value) {
      throw new ValidationError(
        `${field} must have 'type' and 'value' properties`,
        field,
        value
      );
    }

    const validTypes = [
      "Number",
      "Integer",
      "String",
      "Boolean",
      "Array",
      "Object",
      "DateTime",
      "NumberArray",
      "StringArray",
    ];

    if (!validTypes.includes(value.type)) {
      throw new ValidationError(
        `${field}.type must be one of: ${validTypes.join(", ")}`,
        field,
        value.type
      );
    }

    return value;
  },

  /**
   * Validates extended parameters structure
   */
  extendedParameters: (params, field = "extended_parameters") => {
    if (!params || typeof params !== "object") {
      throw new ValidationError(`${field} must be an object`, field, params);
    }

    // Validate each parameter value
    Object.entries(params).forEach(([key, value]) => {
      try {
        Validators.parameterValue(value, `${field}.${key}`);
      } catch (error) {
        throw new ValidationError(
          `Invalid parameter value for ${key}: ${error.message}`,
          `${field}.${key}`,
          value
        );
      }
    });

    return params;
  },

  /**
   * Validates complete calculation parameters
   */
  calculationParameters: (params) => {
    if (!params || typeof params !== "object") {
      throw new ValidationError(
        "Parameters must be an object",
        "parameters",
        params
      );
    }

    // Dimensions must exist (can be empty)
    if (!params.dimensions || typeof params.dimensions !== "object") {
      throw new ValidationError(
        "Parameters must include dimensions object",
        "parameters.dimensions",
        params.dimensions
      );
    }

    // Validate calculation_date if present
    if (params.calculation_date) {
      Validators.datetime(params.calculation_date, "calculation_date");
    }

    // Validate extended_parameters if present
    if (params.extended_parameters) {
      Validators.extendedParameters(
        params.extended_parameters,
        "extended_parameters"
      );
    }

    // Validate design_code if present
    if (params.design_code) {
      Validators.designCode(params.design_code);
    }

    return params;
  },
};
