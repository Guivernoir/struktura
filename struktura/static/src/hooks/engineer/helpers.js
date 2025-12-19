/**
 * @file lib/engineering-helpers.js
 * @description Engineering parameter builders aligned with Rust backend
 * Mission objective: Surgical type preservation and data integrity
 */

import { ValidationError } from "./models.js";

/**
 * Safe number parser - returns null for empty/invalid values
 */
const safeFloat = (val) => {
  if (val === null || val === undefined || val === "") return null;
  const num = parseFloat(val);
  return Number.isFinite(num) ? num : null;
};

/**
 * Safe string parser - returns null for empty values
 */
const safeString = (val) => {
  if (val === null || val === undefined || val === "") return null;
  return String(val).trim();
};

/**
 * Check if an object has any valid values
 */
const hasValidValues = (obj) => {
  if (!obj || typeof obj !== "object") return false;
  return Object.values(obj).some(
    (v) => v !== null && v !== undefined && v !== ""
  );
};

export const EngineeringHelpers = {
  /**
   * Create MaterialProperties matching backend struct
   * Backend expects: material_type (String) + optional f64 fields
   */
  createMaterial: (material) => {
    if (!material || !hasValidValues(material)) return null;

    const result = {
      material_type: safeString(material.material_type) || "Steel",
    };

    // Add optional numeric fields only if they have valid values
    const numericFields = [
      "compressive_strength",
      "tensile_strength",
      "yield_strength",
      "ultimate_strength",
      "elastic_modulus",
      "shear_modulus",
      "poisson_ratio",
      "density",
      "thermal_conductivity",
      "thermal_expansion",
      "specific_heat",
    ];

    numericFields.forEach((field) => {
      const value = safeFloat(material[field]);
      if (value !== null) {
        result[field] = value;
      }
    });

    return result;
  },

  /**
   * Create LoadCase matching backend struct
   * Backend expects: dead_load (f64), live_load (f64), load_combination (String)
   * + optional f64 fields
   */
  createLoadCase: (loads) => {
    if (!loads || !hasValidValues(loads)) return null;

    const deadLoad = safeFloat(loads.dead_load);
    const liveLoad = safeFloat(loads.live_load);

    // Backend requires dead_load and live_load, but we'll use defaults if missing
    const result = {
      dead_load: deadLoad !== null ? deadLoad : 1.0,
      live_load: liveLoad !== null ? liveLoad : 1.0,
      load_combination: safeString(loads.load_combination) || "LRFD",
    };

    // Add optional load types
    const optionalLoads = [
      "wind_load",
      "seismic_load",
      "snow_load",
      "impact_load",
      "tension_load",
      "shear_load",
    ];

    optionalLoads.forEach((field) => {
      const value = safeFloat(loads[field]);
      if (value !== null) {
        result[field] = value;
      }
    });

    return result;
  },

  /**
   * Create SafetyFactors matching backend struct
   * Backend expects: all fields as f64 (with defaults)
   */
  createSafetyFactors: (factors) => {
    if (!factors || !hasValidValues(factors)) return null;

    const result = {
      dead_load_factor: safeFloat(factors.dead_load_factor) ?? 1.2,
      live_load_factor: safeFloat(factors.live_load_factor) ?? 1.6,
      material_reduction_factor:
        safeFloat(factors.material_reduction_factor) ?? 0.9,
      importance_factor: safeFloat(factors.importance_factor) ?? 1.0,
    };

    // Optional fields
    const overturning = safeFloat(factors.overturning);
    if (overturning !== null) result.overturning = overturning;

    const bearing = safeFloat(factors.bearing);
    if (bearing !== null) result.bearing = bearing;

    return result;
  },

  /**
   * Create ProjectMetadata matching backend struct
   * All fields are Strings
   */
  createProjectMetadata: (metadata) => {
    if (!metadata || !hasValidValues(metadata)) return null;

    const result = {};
    let hasAnyField = false;

    const fields = [
      "project_name",
      "engineer_name",
      "project_location",
      "calculation_date",
    ];

    fields.forEach((field) => {
      const value = safeString(metadata[field]);
      if (value !== null) {
        result[field] = value;
        hasAnyField = true;
      }
    });

    return hasAnyField ? result : null;
  },

  /**
   * Create complete EngineeringParameters matching backend struct
   * This is the main function that builds the request payload
   */
  createParameters: (formData) => {
    const params = {};

    // 1. DIMENSIONS (required HashMap<String, f64>)
    params.dimensions = {};
    if (formData.dimensions && typeof formData.dimensions === "object") {
      Object.entries(formData.dimensions).forEach(([key, value]) => {
        const num = safeFloat(value);
        if (num !== null) {
          params.dimensions[key] = num;
        }
      });
    }

    // 2. MATERIAL (optional MaterialProperties)
    const material = EngineeringHelpers.createMaterial(formData.material);
    if (material) {
      params.material = material;
    }

    // 3. LOADS (optional LoadCase)
    const loads = EngineeringHelpers.createLoadCase(formData.loads);
    if (loads) {
      params.loads = loads;
    }

    // 4. SAFETY_FACTORS (optional SafetyFactors)
    const safetyFactors = EngineeringHelpers.createSafetyFactors(
      formData.safetyFactors
    );
    if (safetyFactors) {
      params.safety_factors = safetyFactors;
    }

    // 5. DESIGN_CODE (optional String)
    const designCode = safeString(formData.designCode);
    if (designCode) {
      params.design_code = designCode;
    }

    // 6. EXPOSURE_CLASS (optional String)
    const exposureClass = safeString(formData.exposureClass);
    if (exposureClass) {
      params.exposure_class = exposureClass;
    }

    // 7. TEMPERATURE (optional f64)
    const temperature = safeFloat(formData.temperature);
    if (temperature !== null) {
      params.temperature = temperature;
    }

    // 8. HUMIDITY (optional f64)
    const humidity = safeFloat(formData.humidity);
    if (humidity !== null) {
      params.humidity = humidity;
    }

    // 9. ADDITIONAL (optional HashMap<String, f64>)
    // Backend expects only numeric values here
    if (formData.additional && hasValidValues(formData.additional)) {
      params.additional = {};
      Object.entries(formData.additional).forEach(([key, value]) => {
        const num = safeFloat(value);
        if (num !== null) {
          params.additional[key] = num;
        }
      });
    }

    // 10. STRUCTURED_DATA (optional HashMap<String, JsonValue>)
    // This can contain any JSON-serializable values
    if (formData.structured_data && hasValidValues(formData.structured_data)) {
      params.structured_data = {};
      Object.entries(formData.structured_data).forEach(([key, value]) => {
        if (value !== null && value !== undefined && value !== "") {
          // Try to parse as number first, otherwise keep as string or object
          const num = safeFloat(value);
          if (num !== null && typeof value !== "object") {
            params.structured_data[key] = num;
          } else {
            params.structured_data[key] = value;
          }
        }
      });
    }

    // 11. PROJECT_METADATA (optional ProjectMetadata)
    const projectMetadata = EngineeringHelpers.createProjectMetadata(
      formData.projectMetadata
    );
    if (projectMetadata) {
      params.project_metadata = projectMetadata;
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
      displayValue:
        result.formatted_value || `${result.value.toFixed(2)} ${result.unit}`,
      isCritical: result.is_critical || false,
      tolerancePercent: result.tolerance
        ? (result.tolerance * 100).toFixed(1)
        : null,
    }));
  },

  /**
   * Group structured warnings by severity
   */
  groupWarnings: (response) => {
    if (!response?.structured_warnings) {
      return {
        CRITICAL: [],
        HIGH: [],
        MEDIUM: [],
        LOW: [],
      };
    }

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
    return (
      response?.calculation_metadata?.requires_pe_review === true ||
      response?.structured_warnings?.some((w) => w.severity === "CRITICAL") ||
      false
    );
  },

  /**
   * Extract critical results only
   */
  getCriticalResults: (results) => {
    return results.filter((r) => r.is_critical || r.isCritical);
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
 * Export utility functions for direct use
 */
export { safeFloat as safeParseFloat, safeString };
