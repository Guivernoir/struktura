/**
 * @file engineer.js
 * @description Professional engineering calculations
 * Mission objective: Advanced tactical operations for certified personnel
 */

import { OutputFormat } from "./models.js";

export function createEngineerModule(requestHandler, cache) {
  return {
    /**
     * Retrieve professional-grade operations catalog
     */
    getCatalogue: async () => {
      if (!cache.engineerCatalogue) {
        cache.engineerCatalogue = await requestHandler(
          "/calculus/engineer/catalogue"
        );
      }
      return cache.engineerCatalogue;
    },

    /**
     * Execute professional calculation - strategic deployment
     */
    calculate: (type, params = {}, outputFormat = null) => {
      const body = {
        calculation_type: type,
        parameters: params,
      };

      if (outputFormat) {
        body.output_format = outputFormat;
      }

      return requestHandler("/calculus/engineer/calculate", {
        method: "POST",
        body,
      });
    },

    /**
     * Execute with detailed intelligence report
     * Includes intermediate steps and tactical breakdown
     */
    calculateDetailed: (type, params = {}) => {
      return requestHandler("/calculus/engineer/calculate", {
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
      return requestHandler("/calculus/engineer/calculate", {
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
