/**
 * @file helpers.js
 * @description Engineering parameter builders and result formatters
 * Mission objective: Strategic asset construction and intelligence formatting
 */

import { Validators } from "./validators.js";
import { WarningSeverity } from "./models.js";

// Helper to safely parse a float and return null if invalid.
const safeFloat = (val) => {
  if (val === null || val === undefined || val === "") return null;
  const num = parseFloat(val);
  return Number.isFinite(num) ? num : null;
};

export const EngineeringHelpers = {
  /**
   * Construct load case with military precision
   */
  createLoadCase: (loads) => {
    if (!loads) return null;

    //Validators.required(loads, ["dead_load", "live_load", "load_combination"]);

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
    // FIX: Check for empty objects to prevent validation errors on initialized state
    if (!mat || Object.keys(mat).length === 0) return null;

    // FIX: Removed strict validation for "material_type" to allow the default "Generic" to work
    // Validators.required(mat, ["material_type"]);

    const material = {
      material_type: mat.material_type || "Generic", // Fallback now functions correctly
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
    additional = null,
    projectMetadata = null,
  }) => {
    const params = {
      dimensions: {},
    };

    if (dimensions) {
      for (const [key, value] of Object.entries(dimensions)) {
        const parsed = parseFloat(value);
        if (Number.isFinite(parsed)) {
          params.dimensions[key] = parsed;
        } else {
          console.warn(
            `[EngineeringHelpers] Skipping invalid dimension: ${key} = ${value}`
          );
        }
      }
    }

    if (material) {
      const mat = EngineeringHelpers.createMaterial(material);
      if (mat) params.material = mat;
    }

    if (loads) {
      const l = EngineeringHelpers.createLoadCase(loads);
      if (l) params.loads = l;
    }

    if (safetyFactors) {
      params.safety_factors =
        EngineeringHelpers.createSafetyFactors(safetyFactors);
    }

    if (designCode) {
      params.design_code = Validators.designCode(designCode);
    }

    if (exposureClass) params.exposure_class = exposureClass;

    const temp = safeFloat(temperature);
    if (temp !== null) params.temperature = temp;

    const hum = safeFloat(humidity);
    if (hum !== null) params.humidity = hum;

    if (additional) {
      params.additional = {};
      for (const [key, value] of Object.entries(additional)) {
        const parsed = parseFloat(value);
        if (Number.isFinite(parsed)) {
          params.additional[key] = parsed;
        }
      }
    }

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
};
