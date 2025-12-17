/**
 * @file validators.js
 * @description Validation utilities with the precision of a sniper
 * Mission objective: Ensure data integrity before battlefield deployment
 */

import { ValidationError, DesignCodes, RegulationCodes } from "./models.js";

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
      (f) => obj[f] === undefined || obj[f] === null
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
};
