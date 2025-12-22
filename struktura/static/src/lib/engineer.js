/**
 * @file engineer.js
 * @description Refactored Engineering Module API
 * @overview This refactor introduces an object-oriented approach for better usability and readability.
 *           - Uses a class-based API for calculations to encapsulate state, validation, and execution.
 *           - Improved parameter building with type safety aligned to backend models.
 *           - Enhanced error handling and logging.
 *           - Caching integrated more seamlessly.
 *           - Validation performed incrementally for better developer experience.
 *           - Aligns closely with backend models.rs for ParameterValue, DesignCode, etc.
 *           - Maintains backward compatibility with functional methods where possible.
 */

import { OutputFormat, DesignCodes, ApiError, ValidationError } from "./models.js";
import { EngineeringHelpers } from "./helpers.js";
import { Validators } from "./validators.js";

export class EngineeringCalculator {
  /**
   * @param {string} type - Calculator type (e.g., 'oee_calculation')
   * @param {Function} requestHandler - The API request handler function
   * @param {Object} cache - Shared cache object
   */
  constructor(type, requestHandler, cache) {
    this.type = type;
    this.requestHandler = requestHandler;
    this.cache = cache;
    this.parameters = {
      dimensions: {},
      extended_parameters: {},
    };
    this.outputFormat = OutputFormat.STANDARD;
    this.metadata = null; // Loaded lazily
  }

  /**
   * Load calculator metadata if not already loaded
   * @private
   */
  async _loadMetadata() {
    if (this.metadata) return;

    const catalogue = await this._getCatalogue();
    this.metadata = catalogue.calculators.find((calc) => calc.id === this.type);

    if (!this.metadata) {
      throw new ApiError(`Calculator '${this.type}' not found`, 404);
    }
  }

  /**
   * Get catalogue from cache or API
   * @private
   */
  async _getCatalogue() {
    if (!this.cache.engineerCatalogue) {
      this.cache.engineerCatalogue = await this.requestHandler(
        "/calculus/engineer/catalogue"
      );
    }
    return this.cache.engineerCatalogue;
  }

  /**
   * Set dimensions
   * @param {Object} dimensions - Key-value pairs for dimensions (e.g., { height: 10, width: 20 })
   * @returns {EngineeringCalculator} this for chaining
   */
  setDimensions(dimensions) {
    Validators.required(dimensions, Object.keys(dimensions)); // Basic check
    Object.assign(
      this.parameters.dimensions,
      EngineeringHelpers.sanitizeNumbers(dimensions)
    );
    return this;
  }

  /**
   * Set material properties
   * @param {Object} material - Material properties object
   * @returns {EngineeringCalculator} this for chaining
   */
  setMaterial(material) {
    this.parameters.material = EngineeringHelpers.createMaterial(material);
    return this;
  }

  /**
   * Set load case
   * @param {Object} loads - Load case object
   * @returns {EngineeringCalculator} this for chaining
   */
  setLoads(loads) {
    this.parameters.loads = EngineeringHelpers.createLoadCase(loads);
    return this;
  }

  /**
   * Set safety factors
   * @param {Object} safetyFactors - Safety factors object
   * @returns {EngineeringCalculator} this for chaining
   */
  setSafetyFactors(safetyFactors) {
    this.parameters.safety_factors =
      EngineeringHelpers.createSafetyFactors(safetyFactors);
    return this;
  }

  /**
   * Set design code
   * @param {string} designCode - Design code (from DesignCodes enum)
   * @returns {EngineeringCalculator} this for chaining
   */
  setDesignCode(designCode) {
    this.parameters.design_code = Validators.designCode(designCode);
    return this;
  }

  /**
   * Set environmental conditions
   * @param {Object} conditions - { exposureClass, temperature, humidity }
   * @returns {EngineeringCalculator} this for chaining
   */
  setEnvironmentalConditions(conditions) {
    if (conditions.exposureClass)
      this.parameters.exposure_class = conditions.exposureClass;
    if (conditions.temperature !== undefined)
      this.parameters.temperature = parseFloat(conditions.temperature);
    if (conditions.humidity !== undefined)
      this.parameters.humidity = parseFloat(conditions.humidity);
    return this;
  }

  /**
   * Set calculation date (ISO 8601)
   * @param {string} date - ISO 8601 datetime string
   * @returns {EngineeringCalculator} this for chaining
   */
  setCalculationDate(date) {
    this.parameters.calculation_date = Validators.datetime(
      date,
      "calculation_date"
    );
    return this;
  }

  /**
   * Set project metadata
   * @param {Object} metadata - Project metadata object
   * @returns {EngineeringCalculator} this for chaining
   */
  setProjectMetadata(metadata) {
    this.parameters.project_metadata =
      EngineeringHelpers.createProjectMetadata(metadata);
    return this;
  }

  /**
   * Set an extended parameter with type safety
   * @param {string} key - Parameter key
   * @param {any} value - Parameter value
   * @param {string} [dataType='number'] - ParameterType (e.g., 'number', 'string')
   * @returns {EngineeringCalculator} this for chaining
   */
  async setExtendedParameter(key, value, dataType = "number") {
    await this._loadMetadata();
    const paramMeta = this.metadata.parameters.find(
      (p) => p.path === `extended_parameters.${key}`
    );
    const effectiveType = paramMeta ? paramMeta.data_type : dataType;
    const paramValue = EngineeringHelpers.buildParameterValue(
      value,
      effectiveType.toUpperCase()
    );
    this.parameters.extended_parameters[key] = paramValue;
    return this;
  }

  /**
   * Set multiple extended parameters
   * @param {Object} extendedParams - Key-value pairs
   * @returns {EngineeringCalculator} this for chaining
   */
  async setExtendedParameters(extendedParams) {
    await this._loadMetadata();
    this.parameters.extended_parameters =
      EngineeringHelpers.buildExtendedParameters(
        extendedParams,
        this.metadata.parameters
      );
    return this;
  }

  /**
   * Set output format
   * @param {string} format - 'standard', 'detailed', 'summary'
   * @returns {EngineeringCalculator} this for chaining
   */
  setOutputFormat(format) {
    this.outputFormat = Validators.oneOf(format.toLowerCase(), [
      "standard",
      "detailed",
      "summary",
    ]);
    return this;
  }

  /**
   * Validate current parameters against metadata
   * @returns {Promise<{valid: boolean, errors: Array}>}
   */
  async validate() {
    await this._loadMetadata();
    return validateParametersInternal(
      this.type,
      this.parameters,
      this.metadata
    );
  }

  /**
   * Execute the calculation
   * @returns {Promise<Object>} Calculation response
   */
  async calculate() {
    // Validate before execution
    const validation = await this.validate();
    if (!validation.valid) {
      throw new ValidationError(
        `Validation failed: ${validation.errors.join(", ")}`
      );
    }

    const body = {
      calculation_type: this.type,
      parameters: this.parameters,
      output_format: this.outputFormat,
    };

    // Debug logging
    if (process.env.NODE_ENV === "development") {
      console.log("=== Engineering Calculation Request ===");
      console.log("Type:", this.type);
      console.log("Parameters:", JSON.stringify(this.parameters, null, 2));
      console.log("Output Format:", this.outputFormat);
    }

    return this.requestHandler("/calculus/engineer/calculate", {
      method: "POST",
      body,
    });
  }

  /**
   * Get calculator metadata
   * @returns {Promise<Object>}
   */
  async getMetadata() {
    await this._loadMetadata();
    return this.metadata;
  }
}

/**
 * Internal validation function (extracted for reuse)
 * @private
 */
async function validateParametersInternal(type, params, metadata) {
  const errors = [];

  // Required parameters check
  if (
    metadata.required_parameters &&
    Array.isArray(metadata.required_parameters)
  ) {
    metadata.required_parameters.forEach((paramPath) => {
      const value = getNestedValue(params, paramPath);
      if (value === undefined || value === null || value === "") {
        errors.push(`Required parameter missing: ${paramPath}`);
      }
    });
  }

  // Type and range validation
  if (metadata.parameters && Array.isArray(metadata.parameters)) {
    metadata.parameters.forEach((param) => {
      const value = getNestedValue(params, param.path);

      // Skip optional empty params
      if (
        !param.required &&
        (value === undefined || value === null || value === "")
      )
        return;

      // Required missing
      if (
        param.required &&
        (value === undefined || value === null || value === "")
      ) {
        errors.push(`${param.name} is required`);
        return;
      }

      if (value !== null && value !== undefined && value !== "") {
        const dt = param.data_type;

        if (dt === "number" || dt === "integer") {
          const num = parseFloat(value);
          if (isNaN(num)) {
            errors.push(`${param.name} must be a valid number`);
          } else {
            if (param.min_value != null && num < param.min_value) {
              errors.push(`${param.name} must be >= ${param.min_value}`);
            }
            if (param.max_value != null && num > param.max_value) {
              errors.push(`${param.name} must be <= ${param.max_value}`);
            }
          }
        } else if (dt === "array") {
          if (!Array.isArray(value)) {
            errors.push(`${param.name} must be an array`);
          } else if (param.required && value.length === 0) {
            errors.push(`${param.name} must have at least one item`);
          }
        } else if (dt === "datetime") {
          if (typeof value !== "string" || !value.includes("T")) {
            errors.push(`${param.name} must be a valid ISO 8601 datetime`);
          }
        }
      }
    });
  }

  return { valid: errors.length === 0, errors };
}

/**
 * Helper to get nested value by path (e.g., 'extended_parameters.foo')
 * @private
 */
function getNestedValue(obj, path) {
  return path
    .split(".")
    .reduce((acc, part) => (acc ? acc[part] : undefined), obj);
}

/**
 * Factory function for the module (backward compatible)
 */
export function createEngineerModule(requestHandler, cache) {
  return {
    EngineeringCalculator: (type) =>
      new EngineeringCalculator(type, requestHandler, cache),

    // Backward compatible functional API
    getCatalogue: async () => {
      if (!cache.engineerCatalogue) {
        cache.engineerCatalogue = await requestHandler(
          "/calculus/engineer/catalogue"
        );
      }
      return cache.engineerCatalogue;
    },

    calculate: async (type, params = {}, outputFormat = null) => {
      const calc = new EngineeringCalculator(type, requestHandler, cache);
      calc.parameters = EngineeringHelpers.createParameters({ ...params }); // Adapt old params
      if (outputFormat) calc.setOutputFormat(outputFormat);
      return calc.calculate();
    },

    calculateDetailed: async (type, params = {}) => {
      const calc = new EngineeringCalculator(type, requestHandler, cache);
      calc.parameters = EngineeringHelpers.createParameters({ ...params });
      calc.setOutputFormat("detailed");
      return calc.calculate();
    },

    calculateSummary: async (type, params = {}) => {
      const calc = new EngineeringCalculator(type, requestHandler, cache);
      calc.parameters = EngineeringHelpers.createParameters({ ...params });
      calc.setOutputFormat("summary");
      return calc.calculate();
    },

    getCalculatorMetadata: async (calculatorId) => {
      const catalogue = await this.getCatalogue();
      const calculator = catalogue.calculators.find(
        (calc) => calc.id === calculatorId
      );
      if (!calculator)
        throw new ApiError(`Calculator '${calculatorId}' not found`, 404);
      return calculator;
    },

    getCalculatorsByCategory: async (category) => {
      const catalogue = await this.getCatalogue();
      return catalogue.calculators.filter((calc) => calc.category === category);
    },

    validateParameters: async (calculatorId, params) => {
      const metadata = await this.getCalculatorMetadata(calculatorId);
      return validateParametersInternal(calculatorId, params, metadata);
    },

    dryRun: (type, params = {}, outputFormat = null) => {
      const body = {
        calculation_type: type,
        parameters: params,
      };
      if (outputFormat) body.output_format = outputFormat;
      console.log("=== DRY RUN - Request that would be sent ===");
      console.log(JSON.stringify(body, null, 2));
      return body;
    },
  };
}
