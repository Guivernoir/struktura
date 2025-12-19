/**
 * @file models.js
 * @description Data structures, enums, and type definitions for Struktura API
 * Mission briefing: Central repository for all domain models
 */

// =============================================================================
// DESIGN CODES & STANDARDS
// =============================================================================

export const DesignCodes = Object.freeze({
  // Structural
  ACI_318: "ACI318",
  AISC_360: "AISC360",
  ASCE_7: "ASCE7",
  EUROCODE_EC2: "EurocodeEC2",
  EUROCODE_EC3: "EurocodeEC3",

  // Civil/Geotechnical
  AASHTO: "AASHTO",
  AASHTO_LRFD: "AASHTOLrfd",

  // Mechanical
  ASME: "ASME",
  ASME_BPVC: "ASMEBPVC",
  API_610: "API610",
  TEMA: "TEMA",

  // General
  ISO: "ISO",
  ASTM: "ASTM",

  // Manufacturing
  CEMA: "CEMA",
  LEAN_MANUFACTURING: "LeanManufacturing",
});

export const DesignCodeNames = Object.freeze({
  [DesignCodes.ACI_318]: "ACI 318",
  [DesignCodes.AISC_360]: "AISC 360",
  [DesignCodes.ASCE_7]: "ASCE 7",
  [DesignCodes.EUROCODE_EC2]: "Eurocode 2",
  [DesignCodes.EUROCODE_EC3]: "Eurocode 3",
  [DesignCodes.AASHTO]: "AASHTO",
  [DesignCodes.AASHTO_LRFD]: "AASHTO LRFD",
  [DesignCodes.ASME]: "ASME",
  [DesignCodes.ASME_BPVC]: "ASME BPVC",
  [DesignCodes.API_610]: "API 610",
  [DesignCodes.TEMA]: "TEMA",
  [DesignCodes.ISO]: "ISO",
  [DesignCodes.ASTM]: "ASTM",
  [DesignCodes.CEMA]: "CEMA",
  [DesignCodes.LEAN_MANUFACTURING]: "Lean Manufacturing",
});

// =============================================================================
// REGULATION CODES & STANDARDS
// =============================================================================

export const RegulationCodes = Object.freeze({
  // Construction
  IBC: "IBC",
  NEC: "NEC",
  OSHA: "OSHA",
  LEED: "LEED",

  // General
  ISO: "ISO",
  ASTM: "ASTM",

  // Management
  PMP: "PMP",
  Agile: "Agile",
});

export const RegulationCodeNames = Object.freeze({
  [RegulationCodes.IBC]: "International Building Code (IBC)",
  [RegulationCodes.NEC]: "National Electrical Code (NEC)",
  [RegulationCodes.OSHA]:
    "Occupational Safety and Health Administration (OSHA)",
  [RegulationCodes.LEED]:
    "Leadership in Energy and Environmental Design (LEED)",
  [RegulationCodes.ISO]: "ISO Standards",
  [RegulationCodes.ASTM]: "ASTM Standards",
  [RegulationCodes.PMP]: "Project Management Professional (PMP)",
  [RegulationCodes.Agile]: "Agile Methodology",
});

// =============================================================================
// SEVERITY & COMPLEXITY LEVELS
// =============================================================================

export const WarningSeverity = Object.freeze({
  CRITICAL: "CRITICAL",
  HIGH: "HIGH",
  MEDIUM: "MEDIUM",
  LOW: "LOW",
});

export const ComplexityLevel = Object.freeze({
  BASIC: "basic",
  INTERMEDIATE: "intermediate",
  ADVANCED: "advanced",
});

// =============================================================================
// OUTPUT & PARAMETER FORMATS
// =============================================================================

export const OutputFormat = Object.freeze({
  STANDARD: "standard",
  DETAILED: "detailed",
  SUMMARY: "summary",
});

export const ParameterType = Object.freeze({
  NUMBER: "number",
  INTEGER: "integer",
  STRING: "string",
  BOOLEAN: "boolean",
  ENUM: "enum",
  ARRAY: "array",
  OBJECT: "object",
  DATETIME: "datetime",
  JSON: "json",
});

// =============================================================================
// ERROR CLASSES
// =============================================================================

export class ApiError extends Error {
  constructor(message, status, data = null) {
    super(message);
    this.name = "ApiError";
    this.status = status;
    this.data = data;
  }
}

export class ValidationError extends Error {
  constructor(message, field = null, value = null) {
    super(message);
    this.name = "ValidationError";
    this.field = field;
    this.value = value;
  }
}

// =============================================================================
// DESIGN CODE UTILITIES
// =============================================================================

export function formatDesignCode(code) {
  return DesignCodeNames[code] || code;
}

export function getDesignCodesByCategory(category) {
  const mapping = {
    structural: [
      DesignCodes.ACI_318,
      DesignCodes.AISC_360,
      DesignCodes.ASCE_7,
      DesignCodes.EUROCODE_EC2,
      DesignCodes.EUROCODE_EC3,
    ],
    civil: [DesignCodes.AASHTO, DesignCodes.AASHTO_LRFD],
    geotechnical: [DesignCodes.AASHTO, DesignCodes.AASHTO_LRFD],
    mechanical: [
      DesignCodes.ASME,
      DesignCodes.ASME_BPVC,
      DesignCodes.API_610,
      DesignCodes.TEMA,
    ],
    general: [DesignCodes.ISO, DesignCodes.ASTM],
    production: [DesignCodes.CEMA, DesignCodes.LEAN_MANUFACTURING],
  };

  return mapping[category.toLowerCase()] || [];
}

// =============================================================================
// REGULATION CODE UTILITIES
// =============================================================================

export function formatRegulationCode(code) {
  return RegulationCodeNames[code] || code;
}

export function getRegulationCodesByCategory(category) {
  const mapping = {
    construction: [
      RegulationCodes.IBC,
      RegulationCodes.NEC,
      RegulationCodes.OSHA,
      RegulationCodes.LEED,
    ],
    general: [RegulationCodes.ISO, RegulationCodes.ASTM],
    management: [RegulationCodes.PMP, RegulationCodes.Agile],
  };

  return mapping[category.toLowerCase()] || [];
}

// =============================================================================
// ERROR UTILITIES
// =============================================================================

export function getErrorMessage(error) {
  // 1. Check for specific API error structure
  if (error instanceof ApiError) {
    // If the backend sent a detailed JSON error (e.g. { error: "X", reason: "Y" })
    if (error.data) {
      // Rust backend often sends: { "error": "...", "reason": "..." }
      if (error.data.reason) return `${error.data.error}: ${error.data.reason}`;
      if (error.data.message) return error.data.message;
      if (error.data.error) return error.data.error;
    }
    return error.message;
  }

  // 2. Check for Validation Error
  if (error instanceof ValidationError) {
    return `Validation Error (${error.field}): ${error.message}`;
  }

  // 3. Fallback for generic Axios/Fetch errors with response data
  if (error?.response?.data) {
    const d = error.response.data;
    if (d.reason) return `${d.error}: ${d.reason}`;
    if (d.error) return d.error;
    if (d.message) return d.message;
  }

  // 4. Standard Error object
  if (error?.message) {
    return error.message;
  }

  return "An unexpected error occurred";
}
