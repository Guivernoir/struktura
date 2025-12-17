/**
 * @file lib/engineering-helpers.js
 * @description Engineering parameter helpers with type safety
 * Mission objective: Ensure data integrity in hostile environments
 */

import { ValidationError } from "./models.js";

/**
 * Engineering parameter builder with surgical type preservation
 * Ensures backend receives data in the exact format it expects
 */
export const EngineeringHelpers = {
  /**
   * Create engineering parameters from form data
   * Maintains type safety and proper serialization
   */
  createParameters: (formData) => {
    const params = {};

    // Dimensions: Always include, convert to numbers
    if (formData.dimensions && Object.keys(formData.dimensions).length > 0) {
      params.dimensions = {};
      for (const [key, value] of Object.entries(formData.dimensions)) {
        if (value !== null && value !== undefined && value !== "") {
          params.dimensions[key] = parseFloat(value);
        }
      }
    } else {
      params.dimensions = {};
    }

    // Material: Include if any property is set
    if (formData.material && Object.keys(formData.material).length > 0) {
      const material = {};
      let hasAnyValue = false;

      for (const [key, value] of Object.entries(formData.material)) {
        if (value !== null && value !== undefined && value !== "") {
          hasAnyValue = true;

          // material_type is string, everything else is number
          if (key === "material_type") {
            material[key] = String(value);
          } else {
            material[key] = parseFloat(value);
          }
        }
      }

      if (hasAnyValue) {
        // Ensure material_type is always present
        material.material_type = material.material_type || "Steel";
        params.material = material;
      }
    }

    // Loads: Include if any property is set
    if (formData.loads && Object.keys(formData.loads).length > 0) {
      const loads = {};
      let hasAnyValue = false;

      for (const [key, value] of Object.entries(formData.loads)) {
        if (value !== null && value !== undefined && value !== "") {
          hasAnyValue = true;

          // load_combination is string, everything else is number
          if (key === "load_combination") {
            loads[key] = String(value);
          } else {
            loads[key] = parseFloat(value);
          }
        }
      }

      if (hasAnyValue) {
        // Ensure load_combination has a default
        loads.load_combination = loads.load_combination || "LRFD";

        // Ensure required fields have defaults
        if (loads.dead_load === undefined) loads.dead_load = 1.0;
        if (loads.live_load === undefined) loads.live_load = 1.0;

        params.loads = loads;
      }
    }

    // Safety Factors: Include if any property is set
    if (
      formData.safetyFactors &&
      Object.keys(formData.safetyFactors).length > 0
    ) {
      const factors = {};
      let hasAnyValue = false;

      for (const [key, value] of Object.entries(formData.safetyFactors)) {
        if (value !== null && value !== undefined && value !== "") {
          hasAnyValue = true;
          factors[key] = parseFloat(value);
        }
      }

      if (hasAnyValue) {
        params.safety_factors = factors;
      }
    }

    // Design Code: String value
    if (formData.designCode) {
      params.design_code = String(formData.designCode);
    }

    // Exposure Class: String value
    if (formData.exposureClass) {
      params.exposure_class = String(formData.exposureClass);
    }

    // Environmental: Numbers
    if (formData.temperature !== null && formData.temperature !== undefined) {
      params.temperature = parseFloat(formData.temperature);
    }
    if (formData.humidity !== null && formData.humidity !== undefined) {
      params.humidity = parseFloat(formData.humidity);
    }

    // Additional: All numbers, preserve string keys
    if (formData.additional && Object.keys(formData.additional).length > 0) {
      const additional = {};

      for (const [key, value] of Object.entries(formData.additional)) {
        if (value !== null && value !== undefined && value !== "") {
          // CRITICAL: Check if this is supposed to be a string enum
          // If the key ends with "_type", "_mode", "_method", keep as string
          if (
            key.endsWith("_type") ||
            key.endsWith("_mode") ||
            key.endsWith("_method") ||
            key.endsWith("_class") ||
            key.endsWith("_category")
          ) {
            additional[key] = String(value);
          } else {
            // Try to parse as number, fall back to string
            const num = parseFloat(value);
            additional[key] = isNaN(num) ? String(value) : num;
          }
        }
      }

      if (Object.keys(additional).length > 0) {
        params.additional = additional;
      }
    }

    // Project Metadata: All strings
    if (
      formData.projectMetadata &&
      Object.keys(formData.projectMetadata).length > 0
    ) {
      const metadata = {};
      let hasAnyValue = false;

      for (const [key, value] of Object.entries(formData.projectMetadata)) {
        if (value !== null && value !== undefined && value !== "") {
          hasAnyValue = true;
          metadata[key] = String(value);
        }
      }

      if (hasAnyValue) {
        params.project_metadata = metadata;
      }
    }

    return params;
  },

  /**
   * Format calculation results for display
   */
  formatResults: (response) => {
    if (!response?.results) return [];

    return response.results.map((result) => ({
      label: result.label,
      value: result.value,
      unit: result.unit,
      tolerance: result.tolerance,
      formatted_value:
        result.formatted_value || `${result.value.toFixed(2)} ${result.unit}`,
      is_critical: result.is_critical || false,
    }));
  },

  /**
   * Group structured warnings by severity
   */
  groupWarnings: (response) => {
    if (!response?.structured_warnings) return null;

    const grouped = {
      CRITICAL: [],
      HIGH: [],
      MEDIUM: [],
      LOW: [],
    };

    response.structured_warnings.forEach((warning) => {
      const severity = warning.severity || "MEDIUM";
      if (grouped[severity]) {
        grouped[severity].push(warning);
      }
    });

    return grouped;
  },

  /**
   * Check if calculation requires PE review
   */
  requiresPEReview: (response) => {
    return response?.calculation_metadata?.requires_pe_review === true;
  },

  /**
   * Extract critical results only
   */
  getCriticalResults: (results) => {
    return results.filter((r) => r.is_critical);
  },

  /**
   * Calculate utilization ratio from results
   */
  getUtilizationRatio: (response) => {
    if (response?.analysis?.utilization_ratio !== undefined) {
      return response.analysis.utilization_ratio;
    }

    // Try to find it in results
    const result = response?.results?.find(
      (r) =>
        r.label.toLowerCase().includes("utilization") ||
        r.label.toLowerCase().includes("demand/capacity")
    );

    return result?.value;
  },

  /**
   * Validate parameter types before sending to backend
   * Throws ValidationError if types don't match expected format
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
        if (expectedType?.enum && !expectedType.enum.includes(value)) {
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
 * Safe number parser that handles empty strings and invalid input
 */
export function safeParseFloat(value, defaultValue = null) {
  if (value === null || value === undefined || value === "") {
    return defaultValue;
  }

  const parsed = parseFloat(value);
  return isNaN(parsed) ? defaultValue : parsed;
}

/**
 * Safe string parser that handles null/undefined
 */
export function safeString(value, defaultValue = null) {
  if (value === null || value === undefined || value === "") {
    return defaultValue;
  }
  return String(value);
}
