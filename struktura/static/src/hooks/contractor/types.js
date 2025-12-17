/**
 * @file hooks/contractor/types.js
 * @description Type definitions and initial state for contractor calculator
 * Mission briefing: Data structures for professional contracting operations
 */

/**
 * Initial form state for contracting operations
 */
export const INITIAL_FORM_STATE = {
  // Dimensions
  dimensions: {
    area: 100,
    volume: 0,
    length: 10,
    width: 10,
    height: 3,
  },

  // Material properties
  material: {
    material_type: "Concrete",
    unit_cost: null,
    waste_factor: null,
    density: null,
    availability: null,
  },

  // Resources
  resources: {
    labor_hours: 1,
    equipment_hours: 1,
    material_quantity: null,
    subcontractor_cost: null,
    overhead: null,
  },

  // Safety factors
  safety_factors: {
    cost_factor: 1.1,
    time_factor: 1.2,
    risk_reduction_factor: 0.9,
    importance_factor: 1.0,
  },

  // Regulation code
  regulation_code: null,

  // Environmental
  exposure_class: null,
  temperature: null,
  humidity: null,

  // Additional parameters
  additional: {},

  // Project metadata
  project_metadata: {
    project_name: null,
    contractor_name: null,
    project_location: null,
    calculation_date: null,
  },
};

/**
 * Initial input specification state
 */
export const INITIAL_INPUT_STATE = {
  required: [],
  optional: [],
  parameters: [],
  codes: [],
  metadata: null,
};

/**
 * Default category for initial deployment
 */
export const DEFAULT_CATEGORY = "estimation";

/**
 * Available regulation codes
 */
export const REGULATION_CODES = {
  IBC: "IBC",
  NEC: "NEC",
  OSHA: "OSHA",
  LEED: "LEED",
  ISO: "ISO",
  ASTM: "ASTM",
  PMP: "PMP",
  AGILE: "Agile",
};

/**
 * Calculators that ALWAYS require certain parameter groups
 * Intelligence gathered from backend validator specifications
 */
const REQUIRED_PARAMETER_GROUPS = {
  safety_planning: ["safety_factors"],
  risk_assessment: ["safety_factors"],
  quality_control: ["safety_factors"],
  // Add more as needed
};

/**
 * Helper to build ContractingParameters from form data
 * FIXED: Now respects calculator-specific requirements
 */
export function buildContractingParameters(formData, calculatorId = null) {
  const params = {
    dimensions: { ...formData.dimensions },
  };

  // Check if calculator requires specific parameter groups
  const requiredGroups = calculatorId
    ? REQUIRED_PARAMETER_GROUPS[calculatorId] || []
    : [];

  // Add material if any field is set OR if required
  const materialRequired = requiredGroups.includes("material");
  if (
    materialRequired ||
    formData.material.unit_cost ||
    formData.material.waste_factor ||
    formData.material.density ||
    formData.material.availability
  ) {
    params.material = { ...formData.material };
  }

  // Add resources if non-default OR if required
  const resourcesRequired = requiredGroups.includes("resources");
  if (
    resourcesRequired ||
    formData.resources.labor_hours !== 1 ||
    formData.resources.equipment_hours !== 1 ||
    formData.resources.material_quantity ||
    formData.resources.subcontractor_cost ||
    formData.resources.overhead
  ) {
    params.resources = { ...formData.resources };
  }

  // Add safety factors if non-default OR if required
  const safetyRequired = requiredGroups.includes("safety_factors");
  const defaultSafety = INITIAL_FORM_STATE.safety_factors;
  if (
    safetyRequired ||
    formData.safety_factors.cost_factor !== defaultSafety.cost_factor ||
    formData.safety_factors.time_factor !== defaultSafety.time_factor ||
    formData.safety_factors.risk_reduction_factor !==
      defaultSafety.risk_reduction_factor ||
    formData.safety_factors.importance_factor !==
      defaultSafety.importance_factor
  ) {
    params.safety_factors = { ...formData.safety_factors };
  }

  // Add optional fields if present
  if (formData.regulation_code)
    params.regulation_code = formData.regulation_code;
  if (formData.exposure_class) params.exposure_class = formData.exposure_class;
  if (formData.temperature) params.temperature = formData.temperature;
  if (formData.humidity) params.humidity = formData.humidity;

  // Add additional parameters if any
  if (Object.keys(formData.additional).length > 0) {
    params.additional = { ...formData.additional };
  }

  // Add project metadata if any field is set
  const meta = formData.project_metadata;
  if (
    meta.project_name ||
    meta.contractor_name ||
    meta.project_location ||
    meta.calculation_date
  ) {
    params.project_metadata = { ...meta };
  }

  return params;
}
