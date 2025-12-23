/**
 * @file hooks/engineer/useCalculation.js
 * @description Professional calculation execution and result management
 * Mission objective: Strategic operations with comprehensive result processing
 * FIXED: Now accepts metadata for proper extended_parameters type inference
 */

import { useState, useCallback } from "react";
import {
  api,
  EngineeringHelpers,
  ValidationError,
  getErrorMessage,
} from "../../lib";
import { INITIAL_RESULTS_STATE } from "./types";

export function useCalculation(
  selectedCalculator,
  formData,
  outputFormat,
  calculatorMetadata, // FIXED: Added metadata parameter
  validateOrThrow
) {
  const [results, setResults] = useState([]);
  const [warnings, setWarnings] = useState([]);
  const [structuredWarnings, setStructuredWarnings] = useState(null);
  const [recommendations, setRecommendations] = useState([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  /**
   * Execute professional engineering calculation
   */
  const handleCalculate = useCallback(async () => {
    if (!selectedCalculator) {
      setError("Please select a calculator");
      return;
    }

    try {
      setIsLoading(true);
      setError(null);
      clearResults();

      // Pre-flight validation
      validateOrThrow();

      // Build parameters using engineering helpers with metadata for type inference
      const params = EngineeringHelpers.createParameters({
        dimensions: formData.dimensions || {},
        material: formData.material,
        loads: formData.loads,
        safetyFactors: formData.safetyFactors,
        designCode: formData.designCode,
        exposureClass: formData.exposureClass,
        temperature: formData.temperature,
        humidity: formData.humidity,
        calculationDate: formData.calculationDate,
        extendedParameters: formData.extendedParameters || {},
        parameterMetadata: calculatorMetadata?.parameters || [], // FIXED: Pass metadata
        additional: formData.additional,
        projectMetadata: formData.projectMetadata,
      });

      console.log("🎯 Executing calculation:", {
        calculator: selectedCalculator,
        outputFormat,
        params,
      });

      // Execute calculation
      const response = await api.calculus.engineer.calculate(
        selectedCalculator,
        params,
        outputFormat
      );

      console.log(
        "📦 RAW BACKEND RESPONSE:",
        JSON.stringify(response, null, 2)
      );

      // Process results
      processResults(response);

      console.log("✅ Professional calculation complete");
    } catch (err) {
      console.error("❌ Calculation error:", err);

      if (err instanceof ValidationError) {
        setError(`Validation Error: ${err.message}`);
      } else {
        setError(getErrorMessage(err));
      }

      setResults([]);
    } finally {
      setIsLoading(false);
    }
  }, [
    selectedCalculator,
    formData,
    outputFormat,
    calculatorMetadata,
    validateOrThrow,
  ]);

  /**
   * Process and format calculation results
   */
  const processResults = useCallback((response) => {
    // Format results
    if (response?.results) {
      const formatted = EngineeringHelpers.formatResults(response);
      console.log("📊 FORMATTED RESULTS:", JSON.stringify(formatted, null, 2));
      setResults(formatted);
    } else {
      setResults([]);
    }

    // Process warnings (legacy format)
    if (response?.warnings && response.warnings.length > 0) {
      setWarnings(response.warnings);
    }

    // Process structured warnings
    if (response?.structured_warnings) {
      const grouped = EngineeringHelpers.groupWarnings(response);
      setStructuredWarnings(grouped);
    }

    // Process recommendations
    if (response?.recommendations) {
      setRecommendations(response.recommendations);
    }

    // Check for PE review requirement
    if (EngineeringHelpers.requiresPEReview(response)) {
      setWarnings((prev) => [
        ...prev,
        "⚠️ CRITICAL: This calculation requires Professional Engineer review before implementation.",
      ]);
    }
  }, []);

  /**
   * Clear all results - sanitize operational memory
   */
  const clearResults = useCallback(() => {
    setResults([]);
    setWarnings([]);
    setStructuredWarnings(null);
    setRecommendations([]);
  }, []);

  return {
    results,
    warnings,
    structuredWarnings,
    recommendations,
    isLoading,
    error,
    handleCalculate,
    clearResults,
  };
}
