/**
 * @file hooks/engineer/types.js
 * @description Type definitions for professional engineering operations
 * Mission briefing: Data structures aligned with backend Rust models
 */

import { OutputFormat } from "../../lib";

/**
 * Initial form state for professional engineering
 * Matches backend EngineeringParameters structure
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

  // Additional calculator-specific parameters (HashMap<String, f64>)
  additional: {},

  // Structured data (HashMap<String, JsonValue>)
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
    additional: {},
    structured_data: {},
    projectMetadata: {},
  };

  // Populate default values from metadata
  metadata.parameters.forEach((param) => {
    const path = param.path.split(".");

    // Set default value if available
    if (param.default_value !== undefined && param.default_value !== null) {
      setNestedValue(formState, path, param.default_value);
    } else {
      // Initialize with empty string for form inputs
      setNestedValue(formState, path, "");
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
