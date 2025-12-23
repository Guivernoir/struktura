/**
 * @file hooks/engineer/types.js
 * @description Type definitions for professional engineering operations
 * Mission briefing: Data structures aligned with backend Rust models
 * FIXED: Removed duplicate builders - now imports from lib for consistency
 */

import { OutputFormat, EngineeringHelpers } from "../../lib";

/**
 * Initial form state for professional engineering
 * Matches backend EngineeringParameters structure with new fields
 */
export const INITIAL_FORM_STATE = {
  // Geometric dimensions (always required, HashMap<String, f64>)
  dimensions: {},

  // Material properties (optional MaterialProperties struct)
  material: {
    material_type: "Steel",
    compressive_strength: "",
    tensile_strength: "",
    yield_strength: "",
    ultimate_strength: "",
    elastic_modulus: "",
    shear_modulus: "",
    poisson_ratio: "",
    density: "",
    thermal_conductivity: "",
    thermal_expansion: "",
    specific_heat: "",
  },

  // Loading conditions (optional LoadCase struct)
  loads: {
    dead_load: "",
    live_load: "",
    wind_load: "",
    seismic_load: "",
    snow_load: "",
    impact_load: "",
    tension_load: "",
    shear_load: "",
    load_combination: "LRFD",
  },

  // Safety factors (optional SafetyFactors struct)
  safetyFactors: {
    dead_load_factor: "",
    live_load_factor: "",
    material_reduction_factor: "",
    importance_factor: "",
    overturning: "",
    bearing: "",
  },

  // Design code (optional String)
  designCode: null,

  // Environmental conditions (optional)
  exposureClass: null,
  temperature: null,
  humidity: null,

  // Calculation date/time (ISO 8601)
  calculationDate: null,

  // Extended parameters with full type support (HashMap<String, ParameterValue>)
  extendedParameters: {},

  // LEGACY: Additional calculator-specific parameters (HashMap<String, f64>)
  additional: {},

  // LEGACY: Structured data (HashMap<String, JsonValue>) - deprecated
  structured_data: {},

  // Project metadata (optional ProjectMetadata)
  projectMetadata: {
    project_name: "",
    engineer_name: "",
    project_location: "",
    calculation_date: "",
  },
};

/**
 * Initial results state
 */
export const INITIAL_RESULTS_STATE = {
  results: [],
  warnings: [],
  structuredWarnings: null,
  recommendations: [],
};

/**
 * Default category for initial deployment
 */
export const DEFAULT_CATEGORY = "structural";

/**
 * Default output format
 */
export const DEFAULT_OUTPUT_FORMAT = OutputFormat.STANDARD;

/**
 * Array item schemas for different event types
 * These define the structure of objects within arrays
 */
export const ARRAY_SCHEMAS = {
  downtime_events: {
    fields: [
      {
        name: "start_time",
        label: "Start Time",
        type: "datetime",
        required: true,
        helpText: "When the downtime event started (ISO 8601)",
      },
      {
        name: "end_time",
        label: "End Time",
        type: "datetime",
        required: true,
        helpText: "When the downtime event ended (ISO 8601)",
      },
      {
        name: "category",
        label: "Category",
        type: "string",
        required: true,
        enum: [
          "equipment_failure",
          "setup_and_adjustment",
          "idling_and_minor_stops",
        ],
        helpText: "Type of downtime loss",
      },
      {
        name: "reason",
        label: "Reason",
        type: "string",
        required: true,
        placeholder: "e.g., Motor bearing seized",
        helpText: "Description of what caused the downtime",
      },
      {
        name: "equipment_id",
        label: "Equipment ID",
        type: "string",
        required: false,
        placeholder: "e.g., CONV-02",
        helpText: "Optional equipment identifier",
      },
    ],
  },
  production_runs: {
    fields: [
      {
        name: "start_time",
        label: "Start Time",
        type: "datetime",
        required: true,
        helpText: "When production run started",
      },
      {
        name: "end_time",
        label: "End Time",
        type: "datetime",
        required: true,
        helpText: "When production run ended",
      },
      {
        name: "pieces_produced",
        label: "Pieces Produced",
        type: "number",
        required: true,
        unit: "pieces",
        helpText: "Total units produced during this run",
      },
      {
        name: "ideal_cycle_time",
        label: "Ideal Cycle Time",
        type: "number",
        required: true,
        unit: "seconds/piece",
        helpText: "Target time per piece at optimal speed",
      },
      {
        name: "actual_cycle_time",
        label: "Actual Cycle Time",
        type: "number",
        required: false,
        unit: "seconds/piece",
        helpText: "Measured time per piece (if available)",
      },
    ],
  },
  quality_events: {
    fields: [
      {
        name: "timestamp",
        label: "Timestamp",
        type: "datetime",
        required: true,
        helpText: "When quality issue occurred",
      },
      {
        name: "loss_type",
        label: "Loss Type",
        type: "string",
        required: true,
        enum: ["process_defect", "startup_loss", "rework"],
        helpText: "Category of quality loss",
      },
      {
        name: "quantity",
        label: "Quantity",
        type: "number",
        required: true,
        unit: "pieces",
        helpText: "Number of defective/lost pieces",
      },
      {
        name: "reason",
        label: "Reason",
        type: "string",
        required: true,
        placeholder: "e.g., Out of spec",
        helpText: "Description of quality issue",
      },
    ],
  },
};

/**
 * Helper to create empty form state based on calculator metadata
 */
export function createFormStateFromMetadata(metadata) {
  if (!metadata?.parameters) {
    return { ...INITIAL_FORM_STATE };
  }

  const formState = {
    dimensions: {},
    material: { material_type: "Steel" },
    loads: { load_combination: "LRFD" },
    safetyFactors: {},
    designCode: null,
    exposureClass: null,
    temperature: null,
    humidity: null,
    calculationDate: new Date().toISOString(),
    extendedParameters: {},
    additional: {},
    structured_data: {},
    projectMetadata: {},
  };

  // Populate default values from metadata
  metadata.parameters.forEach((param) => {
    const path = param.path.split(".");

    // Determine which section this parameter belongs to
    const section = path[0];

    // Handle extended_parameters specially
    if (section === "extended_parameters" || section === "extendedParameters") {
      const paramName = path.slice(1).join(".");

      // Initialize based on data type
      if (param.data_type === "array") {
        formState.extendedParameters[paramName] = [];
      } else if (param.data_type === "datetime") {
        formState.extendedParameters[paramName] = "";
      } else if (
        param.data_type === "number" ||
        param.data_type === "integer"
      ) {
        formState.extendedParameters[paramName] =
          param.default_value !== undefined ? param.default_value : "";
      } else if (param.data_type === "boolean") {
        formState.extendedParameters[paramName] =
          param.default_value !== undefined ? param.default_value : false;
      } else {
        formState.extendedParameters[paramName] =
          param.default_value !== undefined ? param.default_value : "";
      }
    } else {
      // Handle regular nested parameters
      if (param.default_value !== undefined && param.default_value !== null) {
        setNestedValue(formState, path, param.default_value);
      } else {
        setNestedValue(formState, path, "");
      }
    }
  });

  return formState;
}

/**
 * Helper to set nested value in object
 */
function setNestedValue(obj, pathArray, value) {
  let current = obj;

  for (let i = 0; i < pathArray.length - 1; i++) {
    const key = pathArray[i];
    if (!current[key]) {
      current[key] = {};
    }
    current = current[key];
  }

  current[pathArray[pathArray.length - 1]] = value;
}

/**
 * Convert datetime-local input to ISO 8601
 * FIXED: Now uses lib helper for consistency
 */
export function datetimeLocalToISO(datetimeLocal) {
  return EngineeringHelpers.datetimeLocalToISO(datetimeLocal);
}

/**
 * Convert ISO 8601 to datetime-local input format
 * FIXED: Now uses lib helper for consistency
 */
export function isoToDatetimeLocal(iso) {
  return EngineeringHelpers.isoToDatetimeLocal(iso);
}

/**
 * Build ParameterValue object for backend
 * FIXED: Now uses lib helper for consistency and proper type handling
 */
export function buildParameterValue(value, dataType) {
  return EngineeringHelpers.buildParameterValue(value, dataType);
}

/**
 * Build complete parameters object for API request
 * FIXED: Now uses lib's createParameters with proper metadata support
 *
 * @param {Object} formData - Form data from hooks
 * @param {Object} metadata - Calculator metadata (REQUIRED for proper extended_parameters typing)
 * @returns {Object} Properly formatted parameters for backend
 */
export function buildCalculationParameters(formData, metadata) {
  return EngineeringHelpers.createParameters({
    dimensions: formData.dimensions || {},
    material: formData.material,
    loads: formData.loads,
    safetyFactors: formData.safetyFactors,
    designCode: formData.designCode,
    exposureClass: formData.exposureClass,
    temperature: formData.temperature,
    humidity: formData.humidity,
    calculationDate: formData.calculationDate,
    extendedParameters: formData.extendedParameters || {},
    parameterMetadata: metadata?.parameters || [], // CRITICAL: Pass metadata for proper typing
    additional: formData.additional,
    projectMetadata: formData.projectMetadata,
  });
}
