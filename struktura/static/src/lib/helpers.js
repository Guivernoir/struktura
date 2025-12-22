/**
 * @file helpers.js
 * @description Engineering parameter builders and result formatters
 * Mission objective: Strategic asset construction and intelligence formatting
 * Now with extended_parameters support - proper type safety, no JSON archaeology
 */

import { Validators } from "./validators.js";
import { WarningSeverity, ParameterType } from "./models.js";

// Helper to safely parse a float and return null if invalid.
const safeFloat = (val) => {
  if (val === null || val === undefined || val === "") return null;
  const num = parseFloat(val);
  return Number.isFinite(num) ? num : null;
};

// [NEW] Recursive sanitizer to clean nested structures
// Converts "" to null, and ensures strings meant to be numbers are parsed
const deepSanitize = (data) => {
  if (data === null || data === undefined) return null;

  // Convert empty strings to null immediately
  if (data === "") return null;

  // Handle Arrays
  if (Array.isArray(data)) {
    return data.map(deepSanitize);
  }

  // Handle Objects
  if (typeof data === "object") {
    const cleaned = {};
    for (const [key, val] of Object.entries(data)) {
      cleaned[key] = deepSanitize(val);
    }
    return cleaned;
  }

  // Auto-convert numeric strings that are definitely numbers
  // This prevents "300" (string) from breaking an f64 expectation in untyped maps
  if (typeof data === "string" && !isNaN(data) && !isNaN(parseFloat(data))) {
    // Check if it looks like a date first to avoid converting timestamps to NaNs or weird numbers
    if (!data.includes("-") && !data.includes("T")) {
      return parseFloat(data);
    }
  }

  return data;
};

export const EngineeringHelpers = {
  sanitizeNumbers: (obj) => {
    const sanitized = {};
    for (const [key, value] of Object.entries(obj)) {
      sanitized[key] = parseFloat(value);
    }
    return sanitized;
  },

  /**
   * Build ParameterValue object for backend
   * Converts frontend values to proper ParameterValue enum format
   */
  buildParameterValue: (value, dataType) => {
    if (value === null || value === undefined || value === "") {
      return null;
    }

    switch (dataType) {
      case ParameterType.NUMBER:
        return {
          type: "Number",
          value: typeof value === "number" ? value : parseFloat(value),
        };

      case ParameterType.INTEGER:
        return {
          type: "Integer",
          value:
            typeof value === "number" ? Math.floor(value) : parseInt(value, 10),
        };

      case ParameterType.STRING:
        return {
          type: "String",
          value: String(value),
        };

      case ParameterType.DATETIME: {
        // Convert datetime-local to ISO 8601 if needed (adds missing Z)
        let isoDate = value;
        if (!value.includes("Z") && !value.includes("+")) {
          try {
            isoDate = new Date(value).toISOString();
          } catch (e) {
            isoDate = value; // Fallback if invalid
          }
        }
        return {
          type: "DateTime",
          value: isoDate,
        };
      }

      case ParameterType.BOOLEAN:
        return {
          type: "Boolean",
          value: Boolean(value),
        };

      case ParameterType.ARRAY:
        return {
          type: "Array",
          value: Array.isArray(value) ? value : [],
        };

      case ParameterType.OBJECT:
        return {
          type: "Object",
          value: typeof value === "object" ? value : {},
        };

      default:
        // Default to string
        return {
          type: "String",
          value: String(value),
        };
    }
  },

  /**
   * Build extended parameters with proper ParameterValue types
   *
   * @param {Object} extendedParams - Raw extended parameter values
   * @param {Array} parameterMetadata - Parameter metadata from calculator
   * @returns {Object} Properly typed extended parameters
   */
  buildExtendedParameters: (extendedParams, parameterMetadata = []) => {
    if (!extendedParams || Object.keys(extendedParams).length === 0) {
      return null;
    }

    const extended = {};

    Object.entries(extendedParams).forEach(([key, value]) => {
      // Find parameter metadata to get data type
      const param = parameterMetadata.find(
        (p) =>
          p.path === `extended_parameters.${key}` ||
          p.path === `extendedParameters.${key}`
      );

      if (param && param.data_type) {
        const paramValue = EngineeringHelpers.buildParameterValue(
          value,
          param.data_type
        );
        if (paramValue !== null) {
          extended[key] = paramValue;
        }
      } else {
        // Fallback: Inference with Deep Sanitization
        const cleanValue = deepSanitize(value);

        if (cleanValue === null) return;

        if (Array.isArray(cleanValue)) {
          extended[key] = { type: "Array", value: cleanValue };
        } else if (typeof cleanValue === "number") {
          if (Number.isInteger(cleanValue)) {
            extended[key] = { type: "Integer", value: cleanValue };
          } else {
            extended[key] = { type: "Number", value: cleanValue };
          }
        } else if (typeof cleanValue === "boolean") {
          extended[key] = { type: "Boolean", value: cleanValue };
        } else if (typeof cleanValue === "string") {
          // [FIX] Improved DateTime detection and normalization
          // Detects strings starting with YYYY-MM-DD and containing T
          if (/^\d{4}-\d{2}-\d{2}T/.test(cleanValue)) {
            try {
              // Force ISO format (adds Z)
              const iso = new Date(cleanValue).toISOString();
              extended[key] = { type: "DateTime", value: iso };
            } catch (e) {
              // If date parsing fails, treat as string
              extended[key] = { type: "String", value: cleanValue };
            }
          } else {
            extended[key] = { type: "String", value: cleanValue };
          }
        } else if (typeof cleanValue === "object") {
          extended[key] = { type: "Object", value: cleanValue };
        }
      }
    });

    return Object.keys(extended).length > 0 ? extended : null;
  },

  /**
   * Construct load case with military precision
   */
  createLoadCase: (loads) => {
    if (!loads) return null;

    const deadLoad = safeFloat(loads.dead_load);
    const liveLoad = safeFloat(loads.live_load);

    if (deadLoad === null || liveLoad === null) {
      console.warn(
        "EngineeringHelpers: Missing required numeric values for dead_load or live_load"
      );
    }

    return {
      dead_load: deadLoad ?? 0.0,
      live_load: liveLoad ?? 0.0,
      wind_load: safeFloat(loads.wind_load),
      seismic_load: safeFloat(loads.seismic_load),
      snow_load: safeFloat(loads.snow_load),
      impact_load: safeFloat(loads.impact_load),
      tension_load: safeFloat(loads.tension_load),
      shear_load: safeFloat(loads.shear_load),
      load_combination: Validators.oneOf(
        loads.load_combination || "LRFD",
        ["LRFD", "ASD", "Eurocode"],
        "load_combination"
      ),
    };
  },

  /**
   * Construct material properties specification
   */
  createMaterial: (mat) => {
    if (!mat || Object.keys(mat).length === 0) return null;

    const material = {
      material_type: mat.material_type || "Generic",
    };

    const addIfValid = (key, sourceKey) => {
      const val = safeFloat(mat[sourceKey]);
      if (val !== null) {
        material[key] = Validators.inRange(val, 0, null, sourceKey);
      }
    };

    // Strength properties
    addIfValid("compressive_strength", "compressive_strength");
    addIfValid("tensile_strength", "tensile_strength");
    addIfValid("yield_strength", "yield_strength");
    addIfValid("ultimate_strength", "ultimate_strength");

    // Elastic properties
    addIfValid("elastic_modulus", "elastic_modulus");
    addIfValid("shear_modulus", "shear_modulus");

    const poisson = safeFloat(mat.poisson_ratio);
    if (poisson !== null) {
      material.poisson_ratio = Validators.inRange(
        poisson,
        0,
        0.5,
        "poisson_ratio"
      );
    }

    // Physical properties
    addIfValid("density", "density");

    // Thermal properties
    const k = safeFloat(mat.thermal_conductivity);
    if (k !== null) material.thermal_conductivity = k;

    const alpha = safeFloat(mat.thermal_expansion);
    if (alpha !== null) material.thermal_expansion = alpha;

    const cp = safeFloat(mat.specific_heat);
    if (cp !== null) material.specific_heat = cp;

    return material;
  },

  /**
   * Construct safety factors with sensible operational defaults
   */
  createSafetyFactors: (sf = {}) => ({
    dead_load_factor: Validators.inRange(
      safeFloat(sf.dead_load_factor) ?? 1.2,
      0,
      10,
      "dead_load_factor"
    ),
    live_load_factor: Validators.inRange(
      safeFloat(sf.live_load_factor) ?? 1.6,
      0,
      10,
      "live_load_factor"
    ),
    material_reduction_factor: Validators.inRange(
      safeFloat(sf.material_reduction_factor) ?? 0.9,
      0,
      1,
      "material_reduction_factor"
    ),
    importance_factor: Validators.inRange(
      safeFloat(sf.importance_factor) ?? 1.0,
      0,
      2,
      "importance_factor"
    ),
    overturning: safeFloat(sf.overturning),
    bearing: safeFloat(sf.bearing),
  }),

  /**
   * Construct project metadata dossier
   */
  createProjectMetadata: (metadata = {}) => ({
    project_name: metadata.project_name || "Untitled Project",
    engineer_name: metadata.engineer_name || "Unknown Engineer",
    project_location: metadata.project_location || null,
    calculation_date: metadata.calculation_date || new Date().toISOString(),
  }),

  /**
   * Construct complete engineering parameters package
   * Now with extended_parameters support - the future is typed
   */
  createParameters: ({
    dimensions = {},
    material = null,
    loads = null,
    safetyFactors = null,
    designCode = null,
    exposureClass = null,
    temperature = null,
    humidity = null,
    calculationDate = null,
    extendedParameters = null,
    parameterMetadata = [],
    additional = null,
    structured_data = null,
    projectMetadata = null,
  }) => {
    const params = {
      dimensions: {},
    };

    // Add calculation date
    params.calculation_date = calculationDate || new Date().toISOString();

    // Process dimensions
    if (dimensions) {
      for (const [key, value] of Object.entries(dimensions)) {
        const parsed = parseFloat(value);
        if (Number.isFinite(parsed)) {
          params.dimensions[key] = parsed;
        } else {
          // Optional: log warning only in debug mode
        }
      }
    }

    // Process material
    if (material) {
      const mat = EngineeringHelpers.createMaterial(material);
      if (mat) params.material = mat;
    }

    // Process loads
    if (loads) {
      const l = EngineeringHelpers.createLoadCase(loads);
      if (l) params.loads = l;
    }

    // Process safety factors
    if (safetyFactors) {
      params.safety_factors =
        EngineeringHelpers.createSafetyFactors(safetyFactors);
    }

    // Process design code
    if (designCode) {
      params.design_code = Validators.designCode(designCode);
    }

    // Process environmental conditions
    if (exposureClass) params.exposure_class = exposureClass;

    const temp = safeFloat(temperature);
    if (temp !== null) params.temperature = temp;

    const hum = safeFloat(humidity);
    if (hum !== null) params.humidity = hum;

    // NEW: Process extended parameters with proper ParameterValue typing
    if (extendedParameters) {
      const extended = EngineeringHelpers.buildExtendedParameters(
        extendedParameters,
        parameterMetadata
      );
      if (extended) {
        params.extended_parameters = extended;
      }
    }

    // LEGACY: Process additional parameters (HashMap<String, f64>)
    if (additional) {
      params.additional = {};
      for (const [key, value] of Object.entries(additional)) {
        const parsed = parseFloat(value);
        if (Number.isFinite(parsed)) {
          params.additional[key] = parsed;
        }
      }
    }

    // LEGACY: Process structured_data (deprecated, but kept for backward compatibility)
    if (structured_data) {
      params.structured_data = structured_data;
    }

    // Process project metadata
    if (projectMetadata) {
      params.project_metadata =
        EngineeringHelpers.createProjectMetadata(projectMetadata);
    }

    return params;
  },

  /**
   * Format calculation results for human consumption
   */
  formatResults: (results) => {
    if (!results?.results) return [];

    return results.results.map((item) => ({
      ...item,
      displayValue:
        item.formatted_value || `${item.value.toFixed(2)} ${item.unit}`,
      isCritical: item.is_critical || false,
      tolerancePercent: item.tolerance
        ? (item.tolerance * 100).toFixed(1)
        : null,
    }));
  },

  /**
   * Organize warnings by threat level
   */
  groupWarnings: (warnings) => {
    if (!warnings?.structured_warnings) {
      return {
        [WarningSeverity.CRITICAL]: [],
        [WarningSeverity.HIGH]: [],
        [WarningSeverity.MEDIUM]: [],
        [WarningSeverity.LOW]: [],
      };
    }

    return warnings.structured_warnings.reduce((acc, warning) => {
      const severity = warning.severity || WarningSeverity.LOW;
      if (!acc[severity]) acc[severity] = [];
      acc[severity].push(warning);
      return acc;
    }, {});
  },

  /**
   * Determine if professional engineer review is required
   */
  requiresPEReview: (response) => {
    return (
      response?.calculation_metadata?.requires_pe_review ||
      response?.structured_warnings?.some(
        (w) => w.severity === WarningSeverity.CRITICAL
      ) ||
      false
    );
  },

  /**
   * Convert datetime-local input to ISO 8601
   */
  datetimeLocalToISO: (datetimeLocal) => {
    if (!datetimeLocal) return "";
    return new Date(datetimeLocal).toISOString();
  },

  /**
   * Convert ISO 8601 to datetime-local input format
   */
  isoToDatetimeLocal: (iso) => {
    if (!iso) return "";
    const date = new Date(iso);
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    const hours = String(date.getHours()).padStart(2, "0");
    const minutes = String(date.getMinutes()).padStart(2, "0");
    return `${year}-${month}-${day}T${hours}:${minutes}`;
  },
};
