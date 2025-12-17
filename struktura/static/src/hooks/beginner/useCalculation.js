/**
 * @file hooks/beginner/useCalculation.js
 * @description Calculation execution and result management
 * Mission objective: Field operations execution
 */

import { useState, useCallback } from "react";
import { api, getErrorMessage } from "../../lib";

export function useCalculation(selectedCalculator, formData) {
  const [results, setResults] = useState(null);
  const [warnings, setWarnings] = useState([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  /**
   * Execute calculation - deploy field operations
   */
  const handleCalculate = useCallback(async () => {
    if (!selectedCalculator) {
      setError("No calculator selected");
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      // Build parameters matching BeginnerParameters structure
      const parameters = {
        width: formData.width,
        length: formData.length,
        height: formData.height,
      };

      // Collect additional parameters if present
      const additionalKeys = Object.keys(formData).filter(
        (k) => !["width", "length", "height"].includes(k) && formData[k] !== 0
      );

      if (additionalKeys.length > 0) {
        parameters.additional = {};
        additionalKeys.forEach((key) => {
          parameters.additional[key] = formData[key];
        });
      }

      const response = await api.calculus.beginner.calculate(
        selectedCalculator,
        parameters
      );

      // Process results
      setResults(response.results || []);
      setWarnings(response.warnings || []);

      console.log("✅ Beginner calculation complete:", {
        calculator: selectedCalculator,
        results: response.results?.length || 0,
        warnings: response.warnings?.length || 0,
      });
    } catch (err) {
      const errorMsg = getErrorMessage(err);
      setError(errorMsg);
      setResults(null);
      console.error("Calculation failed:", err);
    } finally {
      setIsLoading(false);
    }
  }, [selectedCalculator, formData]);

  /**
   * Clear results - sanitize operational memory
   */
  const clearResults = useCallback(() => {
    setResults(null);
    setWarnings([]);
    setError(null);
  }, []);

  return {
    results,
    warnings,
    isLoading,
    error,
    handleCalculate,
    clearResults,
  };
}
