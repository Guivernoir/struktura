/**
 * @file hooks/engineer/helpers.js
 * @description Thin wrapper around lib/helpers for hook-specific utilities
 * Mission objective: Avoid code duplication, maintain single source of truth
 *
 * IMPORTANT: This file now re-exports from lib/helpers.js to avoid duplication.
 * All parameter building logic lives in lib/helpers.js with proper extended_parameters support.
 */

import { EngineeringHelpers as LibHelpers } from "../../lib";
import { ValidationError } from "../../lib";

/**
 * Re-export all helpers from lib
 * This ensures we use the same implementation with proper extended_parameters support
 */
export const EngineeringHelpers = {
  // Core parameter builders (from lib)
  createMaterial: LibHelpers.createMaterial,
  createLoadCase: LibHelpers.createLoadCase,
  createSafetyFactors: LibHelpers.createSafetyFactors,
  createProjectMetadata: LibHelpers.createProjectMetadata,
  createParameters: LibHelpers.createParameters,
  buildParameterValue: LibHelpers.buildParameterValue,
  buildExtendedParameters: LibHelpers.buildExtendedParameters,

  // Result formatters (from lib)
  formatResults: LibHelpers.formatResults,
  groupWarnings: LibHelpers.groupWarnings,
  requiresPEReview: LibHelpers.requiresPEReview,
  getCriticalResults: LibHelpers.getCriticalResults,
  getUtilizationRatio: LibHelpers.getUtilizationRatio,

  // Datetime utilities (from lib)
  datetimeLocalToISO: LibHelpers.datetimeLocalToISO,
  isoToDatetimeLocal: LibHelpers.isoToDatetimeLocal,

  // Number sanitization (from lib)
  sanitizeNumbers: LibHelpers.sanitizeNumbers,

  /**
   * Hook-specific: Validate parameter types before sending to backend
   * This is hook-specific because it works with React form state
   */
  validateParameterTypes: (params, calculatorMetadata) => {
    if (!calculatorMetadata?.parameters) return;

    calculatorMetadata.parameters.forEach((paramMeta) => {
      const path = paramMeta.path.split(".");
      let value = params;

      // Navigate to the nested value
      for (const key of path) {
        value = value?.[key];
      }

      // Skip if optional and not provided
      if (!paramMeta.required && (value === undefined || value === null)) {
        return;
      }

      // Validate type
      if (value !== undefined && value !== null) {
        const actualType = typeof value;
        const expectedType = paramMeta.data_type;

        if (expectedType === "number" && actualType !== "number") {
          throw new ValidationError(
            `Parameter '${paramMeta.path}' must be a number, got ${actualType}`,
            paramMeta.path,
            value
          );
        }

        if (expectedType === "string" && actualType !== "string") {
          throw new ValidationError(
            `Parameter '${paramMeta.path}' must be a string, got ${actualType}`,
            paramMeta.path,
            value
          );
        }

        // Enum validation
        if (
          Array.isArray(expectedType?.enum) &&
          !expectedType.enum.includes(value)
        ) {
          throw new ValidationError(
            `Parameter '${
              paramMeta.path
            }' must be one of: ${expectedType.enum.join(", ")}`,
            paramMeta.path,
            value
          );
        }
      }
    });
  },
};

/**
 * Safe number parser - returns null for empty/invalid values
 */
export const safeFloat = (val) => {
  if (val === null || val === undefined || val === "") return null;
  const num = parseFloat(val);
  return Number.isFinite(num) ? num : null;
};

/**
 * Safe string parser - returns null for empty values
 */
export const safeString = (val) => {
  if (val === null || val === undefined || val === "") return null;
  return String(val).trim();
};

/**
 * Check if an object has any valid values
 */
export const hasValidValues = (obj) => {
  if (!obj || typeof obj !== "object") return false;
  return Object.values(obj).some(
    (v) => v !== null && v !== undefined && v !== ""
  );
};

// Also export as named exports for convenience
export const {
  createMaterial,
  createLoadCase,
  createSafetyFactors,
  createProjectMetadata,
  createParameters,
  buildParameterValue,
  buildExtendedParameters,
  formatResults,
  groupWarnings,
  requiresPEReview,
  getCriticalResults,
  getUtilizationRatio,
  datetimeLocalToISO,
  isoToDatetimeLocal,
  sanitizeNumbers,
  validateParameterTypes,
} = EngineeringHelpers;
