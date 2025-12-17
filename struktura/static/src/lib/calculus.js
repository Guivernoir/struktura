/**
 * @file calculus.js
 * @description Central calculation orchestration and catalog management
 * Mission objective: Command center for all computational operations
 */

import { createBeginnerModule } from "./beginner.js";
import { createEngineerModule } from "./engineer.js";
import { createContractorModule } from "./contractor.js";
import { ApiError, DesignCodeNames, RegulationCodeNames } from "./models.js";
import { Validators } from "./validators.js";

export function createCalculusModule(requestHandler, cache) {
  const beginner = createBeginnerModule(requestHandler, cache);
  const engineer = createEngineerModule(requestHandler, cache);
  const contractor = createContractorModule(requestHandler, cache);

  return {
    /**
     * Preload intelligence catalogs for rapid deployment
     */
    preloadCatalogues: async () => {
      try {
        const [beginnerResult, engineerResult, contractorResult] =
          await Promise.allSettled([
            requestHandler("/calculus/beginner/catalogue"),
            requestHandler("/calculus/engineer/catalogue"),
            requestHandler("/calculus/contractor/catalogue"),
          ]);

        if (beginnerResult.status === "fulfilled") {
          cache.beginnerCatalogue = beginnerResult.value;
        } else {
          console.warn(
            "Failed to preload beginner catalogue",
            beginnerResult.reason
          );
        }

        if (engineerResult.status === "fulfilled") {
          cache.engineerCatalogue = engineerResult.value;
        } else {
          console.warn(
            "Failed to preload engineer catalogue",
            engineerResult.reason
          );
        }

        if (contractorResult.status === "fulfilled") {
          cache.contractorCatalogue = contractorResult.value;
        } else {
          console.warn(
            "Failed to preload contractor catalogue",
            contractorResult.reason
          );
        }
      } catch (e) {
        console.warn("Catalogue preload failed", e);
      }
    },

    /**
     * Clear intelligence cache - sanitize operational memory
     */
    clearCache: () => {
      cache.beginnerCatalogue = null;
      cache.engineerCatalogue = null;
      cache.contractorCatalogue = null;
    },

    // Sub-modules for operational modes
    beginner,
    engineer,
    contractor,

    /**
     * Retrieve input requirements for specific calculator
     */
    getCalculatorInputs: async (calculationType, mode = "beginner") => {
      let catalogue;

      if (mode === "engineer") {
        catalogue = await engineer.getCatalogue();
      } else if (mode === "contractor") {
        catalogue = await contractor.getCatalogue();
      } else {
        catalogue = await beginner.getCatalogue();
      }

      const calculator = catalogue.calculators.find(
        (calc) => calc.id === calculationType
      );

      if (!calculator) {
        throw new ApiError(
          `Calculator '${calculationType}' not found in ${mode} catalogue`,
          404
        );
      }

      if (mode === "engineer" || mode === "contractor") {
        return {
          required: calculator.required_parameters || [],
          optional: calculator.optional_parameters || [],
          parameters: calculator.parameters || [],
          codes:
            mode === "engineer"
              ? calculator.design_codes || []
              : calculator.regulation_codes || [],
          metadata: calculator,
        };
      }

      return {
        required: ["width", "length", "height"],
        optional: Object.keys(calculator.input_hints || {}).filter(
          (key) => !["width", "length", "height"].includes(key)
        ),
        hints: calculator.input_hints,
        ranges: calculator.typical_ranges,
        metadata: calculator,
      };
    },

    /**
     * Retrieve unified catalog view - complete operational overview
     */
    getUnifiedCatalogue: async () => {
      const [beginnerCat, engineerCat, contractorCat] = await Promise.all([
        beginner
          .getCatalogue()
          .catch(() => ({ categories: [], calculators: [] })),
        engineer
          .getCatalogue()
          .catch(() => ({ categories: [], calculators: [], disclaimer: "" })),
        contractor
          .getCatalogue()
          .catch(() => ({ categories: [], calculators: [], disclaimer: "" })),
      ]);

      return {
        beginner: {
          title: "DIY & Home Projects",
          icon: "tools",
          ...beginnerCat,
        },
        engineer: {
          title: "Professional Engineering",
          icon: "hard-hat",
          disclaimer: engineerCat.disclaimer || null,
          ...engineerCat,
        },
        contractor: {
          title: "Professional Contracting",
          icon: "clipboard-list",
          disclaimer: contractorCat.disclaimer || null,
          ...contractorCat,
        },
      };
    },

    /**
     * Filter calculators by design code (engineer mode)
     */
    getCalculatorsByDesignCode: async (designCode) => {
      Validators.designCode(designCode);
      const catalogue = await engineer.getCatalogue();

      return catalogue.calculators.filter((calc) =>
        calc.design_codes?.includes(designCode)
      );
    },

    /**
     * Filter calculators by regulation code (contractor mode)
     */
    getCalculatorsByRegulationCode: async (regulationCode) => {
      Validators.regulationCode(regulationCode);
      const catalogue = await contractor.getCatalogue();

      return catalogue.calculators.filter((calc) =>
        calc.regulation_codes?.includes(regulationCode)
      );
    },

    /**
     * Filter calculators by operational category
     */
    getCalculatorsByCategory: async (category, mode = "engineer") => {
      let catalogue;

      if (mode === "engineer") {
        catalogue = await engineer.getCatalogue();
      } else if (mode === "contractor") {
        catalogue = await contractor.getCatalogue();
      } else {
        catalogue = await beginner.getCatalogue();
      }

      return catalogue.calculators.filter((calc) => calc.category === category);
    },

    /**
     * Search calculators by keyword - reconnaissance operation
     */
    searchCalculators: async (query, mode = "all") => {
      const lowerQuery = query.toLowerCase();
      const results = { beginner: [], engineer: [], contractor: [] };

      if (mode === "beginner" || mode === "all") {
        const beginnerCat = await beginner.getCatalogue();
        results.beginner = beginnerCat.calculators.filter(
          (calc) =>
            calc.name.toLowerCase().includes(lowerQuery) ||
            calc.description.toLowerCase().includes(lowerQuery) ||
            calc.id.toLowerCase().includes(lowerQuery)
        );
      }

      if (mode === "engineer" || mode === "all") {
        const engineerCat = await engineer.getCatalogue();
        results.engineer = engineerCat.calculators.filter(
          (calc) =>
            calc.name.toLowerCase().includes(lowerQuery) ||
            calc.description.toLowerCase().includes(lowerQuery) ||
            calc.id.toLowerCase().includes(lowerQuery) ||
            calc.design_codes?.some((code) =>
              DesignCodeNames[code]?.toLowerCase().includes(lowerQuery)
            )
        );
      }

      if (mode === "contractor" || mode === "all") {
        const contractorCat = await contractor.getCatalogue();
        results.contractor = contractorCat.calculators.filter(
          (calc) =>
            calc.name.toLowerCase().includes(lowerQuery) ||
            calc.description.toLowerCase().includes(lowerQuery) ||
            calc.id.toLowerCase().includes(lowerQuery) ||
            calc.regulation_codes?.some((code) =>
              RegulationCodeNames[code]?.toLowerCase().includes(lowerQuery)
            )
        );
      }

      return results;
    },
  };
}
