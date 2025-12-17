/**
 * @file beginner.js
 * @description Beginner-friendly calculation operations
 * Mission objective: Entry-level tactical support for civilian operations
 */

export function createBeginnerModule(requestHandler, cache) {
  return {
    /**
     * Retrieve available calculation operations catalog
     */
    getCatalogue: async () => {
      if (!cache.beginnerCatalogue) {
        cache.beginnerCatalogue = await requestHandler(
          "/calculus/beginner/catalogue"
        );
      }
      return cache.beginnerCatalogue;
    },

    /**
     * Execute simplified calculation - basic field operations
     * @param {string} type - Calculator type ID
     * @param {Object} params - BeginnerParameters structure
     */
    calculate: (type, params = {}) => {
      // Build parameters object matching backend BeginnerParameters structure
      const parameters = {
        width: parseFloat(params.width) || 0,
        length: parseFloat(params.length) || 0,
        height: parseFloat(params.height) || 0,
      };

      // Handle additional parameters if provided
      const additionalKeys = Object.keys(params).filter(
        (k) => !["width", "length", "height"].includes(k)
      );

      if (additionalKeys.length > 0) {
        parameters.additional = {};
        additionalKeys.forEach((key) => {
          parameters.additional[key] = parseFloat(params[key]) || 0;
        });
      }

      return requestHandler("/calculus/beginner/calculate", {
        method: "POST",
        body: {
          calculation_type: type,
          parameters: parameters,
        },
      });
    },
  };
}
