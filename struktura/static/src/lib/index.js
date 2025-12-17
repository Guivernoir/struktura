/**
 * @file index.js
 * @description Public API surface - the front desk
 * Mission objective: Clean, organized access to all operational capabilities
 *
 * Usage:
 *   import { api, DesignCodes, EngineeringHelpers } from './struktura-api'
 */

// Primary operational assets
export { api, StrukturaClient } from "./api.js";

// Data models and enumerations
export {
  DesignCodes,
  DesignCodeNames,
  RegulationCodes,
  RegulationCodeNames,
  WarningSeverity,
  ComplexityLevel,
  OutputFormat,
  ParameterType,
  ApiError,
  ValidationError,
  formatDesignCode,
  getDesignCodesByCategory,
  getErrorMessage,
} from "./models.js";

// Validation utilities
export { Validators } from "./validators.js";

// Engineering construction utilities
export { EngineeringHelpers } from "./helpers.js";

/**
 * Quick start example:
 *
 * import { api, DesignCodes, EngineeringHelpers } from './struktura-api'
 *
 * // Initialize
 * await api.init()
 *
 * // Beginner calculation
 * const result = await api.calculus.beginner.calculate('concrete_slab', {
 *   width: 10,
 *   length: 20,
 *   thickness: 0.15
 * })
 *
 * // Engineer calculation with helpers
 * const params = EngineeringHelpers.createParameters({
 *   dimensions: { width: 5, height: 3, thickness: 0.2 },
 *   material: { material_type: 'concrete', compressive_strength: 30 },
 *   loads: { dead_load: 5, live_load: 2, load_combination: 'LRFD' },
 *   designCode: DesignCodes.ACI_318
 * })
 *
 * const engineering = await api.calculus.engineer.calculate('beam_design', params)
 */
