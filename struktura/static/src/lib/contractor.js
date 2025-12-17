/**
 * @file contractor.js
 * @description Professional contracting calculations
 * Mission objective: Advanced tactical operations for certified personnel
 */

import { OutputFormat } from "./models.js";

export function createContractorModule(requestHandler, cache) {
  return {
    /**
     * Retrieve professional-grade operations catalog
     */
    getCatalogue: async () => {
      if (!cache.contractorCatalogue) {
        cache.contractorCatalogue = await requestHandler(
          "/calculus/contractor/catalogue"
        );
      }
      return cache.contractorCatalogue;
    },

    /**
     * Execute professional calculation - strategic deployment
     * @param {string} type - Calculator type ID
     * @param {Object} params - ContractingParameters structure
     * @param {string|null} outputFormat - Optional output format
     */
    calculate: (type, params = {}, outputFormat = null) => {
      const body = {
        calculation_type: type,
        parameters: params,
      };

      if (outputFormat) {
        body.output_format = outputFormat;
      }

      return requestHandler("/calculus/contractor/calculate", {
        method: "POST",
        body,
      });
    },

    /**
     * Execute with detailed intelligence report
     * Includes intermediate steps and tactical breakdown
     */
    calculateDetailed: (type, params = {}) => {
      return requestHandler("/calculus/contractor/calculate", {
        method: "POST",
        body: {
          calculation_type: type,
          parameters: params,
          output_format: OutputFormat.DETAILED,
        },
      });
    },

    /**
     * Execute with executive summary
     * Critical results only - for command briefings
     */
    calculateSummary: (type, params = {}) => {
      return requestHandler("/calculus/contractor/calculate", {
        method: "POST",
        body: {
          calculation_type: type,
          parameters: params,
          output_format: OutputFormat.SUMMARY,
        },
      });
    },
  };
}
