/**
 * @file hooks/engineer/types.js
 * @description Type definitions for professional engineering operations
 * Mission briefing: Data structures aligned with backend Rust models
 * Now with extended_parameters support - proper type safety, no JSON archaeology
 */

import { OutputFormat } from "../../lib";

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

  // NEW: Calculation date/time (ISO 8601)
  calculationDate: null,

  // NEW: Extended parameters with full type support (HashMap<String, ParameterValue>)
  // This replaces structured_data for new calculators
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
    if (section === "extended_parameters") {
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
 */
export function datetimeLocalToISO(datetimeLocal) {
  if (!datetimeLocal) return "";
  // Browser datetime-local is "YYYY-MM-DDTHH:MM"
  // Convert to ISO 8601: "YYYY-MM-DDTHH:MM:SSZ"
  return new Date(datetimeLocal).toISOString();
}

/**
 * Convert ISO 8601 to datetime-local input format
 */
export function isoToDatetimeLocal(iso) {
  if (!iso) return "";
  // ISO is "YYYY-MM-DDTHH:MM:SS.sssZ"
  // datetime-local needs "YYYY-MM-DDTHH:MM"
  const date = new Date(iso);
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  const hours = String(date.getHours()).padStart(2, "0");
  const minutes = String(date.getMinutes()).padStart(2, "0");
  return `${year}-${month}-${day}T${hours}:${minutes}`;
}

/**
 * Build ParameterValue object for backend
 * Converts frontend values to proper ParameterValue enum format
 */
export function buildParameterValue(value, dataType) {
  if (value === null || value === undefined || value === "") {
    return null;
  }

  switch (dataType) {
    case "number":
      return {
        type: "Number",
        value: typeof value === "number" ? value : parseFloat(value),
      };

    case "integer":
      return {
        type: "Integer",
        value:
          typeof value === "number" ? Math.floor(value) : parseInt(value, 10),
      };

    case "string":
      return {
        type: "String",
        value: String(value),
      };

    case "datetime": {
      // Convert datetime-local to ISO 8601 if needed
      const isoDate = value.includes("Z") ? value : datetimeLocalToISO(value);
      return {
        type: "DateTime",
        value: isoDate,
      };
    }

    case "boolean":
      return {
        type: "Boolean",
        value: Boolean(value),
      };

    case "array":
      return {
        type: "Array",
        value: Array.isArray(value) ? value : [],
      };

    case "object":
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
}

/**
 * Build complete parameters object for API request
 * Constructs the new extended_parameters format
 */
export function buildCalculationParameters(formData, metadata) {
  const parameters = {
    dimensions: {},
    calculationDate: formData.calculationDate || new Date().toISOString(),
    extendedParameters: {},
  };

  // Add non-empty dimensions
  if (formData.dimensions && Object.keys(formData.dimensions).length > 0) {
    Object.entries(formData.dimensions).forEach(([key, value]) => {
      if (value !== "" && value !== null && value !== undefined) {
        parameters.dimensions[key] =
          typeof value === "number" ? value : parseFloat(value);
      }
    });
  }

  // Add material if present
  if (formData.material && hasNonEmptyValues(formData.material)) {
    parameters.material = cleanObject(formData.material);
  }

  // Add loads if present
  if (formData.loads && hasNonEmptyValues(formData.loads)) {
    parameters.loads = cleanObject(formData.loads);
  }

  // Add safety factors if present
  if (formData.safetyFactors && hasNonEmptyValues(formData.safetyFactors)) {
    parameters.safety_factors = cleanObject(formData.safetyFactors);
  }

  // Add design code if present
  if (formData.designCode) {
    parameters.design_code = formData.designCode;
  }

  // Add environmental conditions
  if (formData.exposureClass) {
    parameters.exposure_class = formData.exposureClass;
  }
  if (formData.temperature !== null && formData.temperature !== "") {
    parameters.temperature = parseFloat(formData.temperature);
  }
  if (formData.humidity !== null && formData.humidity !== "") {
    parameters.humidity = parseFloat(formData.humidity);
  }

  // Build extended_parameters with proper ParameterValue types
  if (formData.extendedParameters && metadata?.parameters) {
    Object.entries(formData.extendedParameters).forEach(([key, value]) => {
      // Find parameter metadata to get data type
      const param = metadata.parameters.find(
        (p) => p.path === `extended_parameters.${key}`
      );

      if (param) {
        const paramValue = buildParameterValue(value, param.data_type);
        if (paramValue !== null) {
          parameters.extendedParameters[key] = paramValue;
        }
      }
    });
  }

  // Add legacy additional parameters if present
  if (formData.additional && hasNonEmptyValues(formData.additional)) {
    parameters.additional = {};
    Object.entries(formData.additional).forEach(([key, value]) => {
      if (value !== "" && value !== null && value !== undefined) {
        parameters.additional[key] =
          typeof value === "number" ? value : parseFloat(value);
      }
    });
  }

  // Add project metadata if present
  if (formData.projectMetadata && hasNonEmptyValues(formData.projectMetadata)) {
    parameters.project_metadata = cleanObject(formData.projectMetadata);
  }

  return parameters;
}

/**
 * Check if object has any non-empty values
 */
function hasNonEmptyValues(obj) {
  return Object.values(obj).some(
    (v) => v !== "" && v !== null && v !== undefined
  );
}

/**
 * Remove empty values from object
 */
function cleanObject(obj) {
  const cleaned = {};
  Object.entries(obj).forEach(([key, value]) => {
    if (value !== "" && value !== null && value !== undefined) {
      cleaned[key] = value;
    }
  });
  return Object.keys(cleaned).length > 0 ? cleaned : undefined;
}
