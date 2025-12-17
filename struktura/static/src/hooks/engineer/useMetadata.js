/**
 * @file hooks/engineer/useMetadata.js
 * @description Calculator metadata loading and processing
 * Mission objective: Intelligence briefing for professional operations
 */

import { useState, useEffect } from "react";
import { api } from "../../lib";

export function useMetadata(selectedCalculator, onDefaultsExtracted) {
  const [calculatorMetadata, setCalculatorMetadata] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  useEffect(() => {
    const loadMetadata = async () => {
      if (!selectedCalculator) {
        setCalculatorMetadata(null);
        return;
      }

      setIsLoading(true);
      setError(null);

      try {
        const metadata = await api.calculus.getCalculatorInputs(
          selectedCalculator,
          "engineer"
        );

        setCalculatorMetadata(metadata.metadata);

        // Extract default values from parameters
        const defaults = extractDefaults(metadata.metadata);

        if (onDefaultsExtracted && Object.keys(defaults).length > 0) {
          onDefaultsExtracted(defaults);
        }

        console.log("📋 Metadata loaded:", {
          calculator: selectedCalculator,
          required: metadata.required_parameters?.length || 0,
          optional: metadata.optional_parameters?.length || 0,
          designCodes: metadata.design_codes?.length || 0,
        });
      } catch (err) {
        setError("Failed to load calculator metadata");
        console.error("Metadata load failed:", err);
        setCalculatorMetadata(null);
      } finally {
        setIsLoading(false);
      }
    };

    loadMetadata();
  }, [selectedCalculator, onDefaultsExtracted]);

  return {
    calculatorMetadata,
    isLoading,
    error,
  };
}

/**
 * Extract default values from parameter metadata
 */
function extractDefaults(metadata) {
  if (!metadata?.parameters) return {};

  const defaults = {};

  metadata.parameters.forEach((param) => {
    if (param.default_value !== undefined && param.default_value !== null) {
      const pathParts = param.path.split(".");

      // Build nested structure
      if (pathParts.length === 2) {
        const [section, field] = pathParts;
        if (!defaults[section]) defaults[section] = {};
        defaults[section][field] = param.default_value;
      } else if (pathParts.length === 1) {
        defaults[pathParts[0]] = param.default_value;
      }
    }
  });

  return defaults;
}
