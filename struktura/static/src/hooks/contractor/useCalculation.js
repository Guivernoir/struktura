/**
 * @file hooks/contractor/useCalculation.js
 * @description Calculation execution and result management
 * Mission objective: Professional field operations execution
 */

import { useState, useCallback } from "react";
import { api, getErrorMessage } from "../../lib";
import { buildContractingParameters } from "./types";

export function useCalculation(selectedCalculator, formData) {
  const [results, setResults] = useState(null);
  const [warnings, setWarnings] = useState([]);
  const [structuredWarnings, setStructuredWarnings] = useState([]);
  const [recommendations, setRecommendations] = useState([]);
  const [complianceNotes, setComplianceNotes] = useState([]);
  const [analysis, setAnalysis] = useState(null);
  const [metadata, setMetadata] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  /**
   * Execute calculation - deploy professional operations
   * FIXED: Now passes calculator ID to parameter builder
   */
  const handleCalculate = useCallback(
    async (outputFormat = null) => {
      if (!selectedCalculator) {
        setError("No calculator selected");
        return;
      }

      setIsLoading(true);
      setError(null);

      try {
        // Build ContractingParameters structure with calculator context
        const parameters = buildContractingParameters(
          formData,
          selectedCalculator
        );

        console.log("🚀 Executing contractor calculation:", {
          calculator: selectedCalculator,
          parameters,
          outputFormat,
        });

        const response = await api.calculus.contractor.calculate(
          selectedCalculator,
          parameters,
          outputFormat
        );

        // Process results
        setResults(response.results || []);
        setWarnings(response.warnings || []);
        setStructuredWarnings(response.structured_warnings || []);
        setRecommendations(response.recommendations || []);
        setComplianceNotes(response.compliance_notes || []);
        setAnalysis(response.analysis || null);
        setMetadata(response.calculation_metadata || null);

        console.log("✅ Contractor calculation complete:", {
          calculator: selectedCalculator,
          results: response.results?.length || 0,
          warnings: response.warnings?.length || 0,
          structured_warnings: response.structured_warnings?.length || 0,
          has_analysis: !!response.analysis,
        });
      } catch (err) {
        const errorMsg = getErrorMessage(err);
        setError(errorMsg);
        setResults(null);
        console.error("Contractor calculation failed:", err);
      } finally {
        setIsLoading(false);
      }
    },
    [selectedCalculator, formData]
  );

  /**
   * Execute with detailed output
   */
  const handleCalculateDetailed = useCallback(async () => {
    return handleCalculate("detailed");
  }, [handleCalculate]);

  /**
   * Execute with summary output
   */
  const handleCalculateSummary = useCallback(async () => {
    return handleCalculate("summary");
  }, [handleCalculate]);

  /**
   * Clear results - sanitize operational memory
   */
  const clearResults = useCallback(() => {
    setResults(null);
    setWarnings([]);
    setStructuredWarnings([]);
    setRecommendations([]);
    setComplianceNotes([]);
    setAnalysis(null);
    setMetadata(null);
    setError(null);
  }, []);

  return {
    results,
    warnings,
    structuredWarnings,
    recommendations,
    complianceNotes,
    analysis,
    metadata,
    isLoading,
    error,
    handleCalculate,
    handleCalculateDetailed,
    handleCalculateSummary,
    clearResults,
  };
}
