/**
 * @file hooks/engineer/index.js
 * @description Professional engineering calculator orchestration
 * Mission objective: Command center for special forces operations
 *
 * Architecture: Composed from specialized sub-hooks with military precision
 */

import { useState, useEffect } from "react";
import { useMetadata } from "./useMetadata";
import { useForm } from "./useForm";
import { useValidation } from "./useValidation";
import { useCalculation } from "./useCalculation";
import { api } from "../../lib"; // Added import for API
import { DEFAULT_CATEGORY, DEFAULT_OUTPUT_FORMAT } from "./types";

export const useEngineerCalculator = () => {
  const [selectedCategory, setSelectedCategory] = useState(DEFAULT_CATEGORY);
  const [selectedCalculator, setSelectedCalculator] = useState(null);
  const [outputFormat, setOutputFormat] = useState(DEFAULT_OUTPUT_FORMAT);
  const [categories, setCategories] = useState([]); // Added for categories

  // Added fetch for categories
  useEffect(() => {
    async function fetchCategories() {
      try {
        const catalogue = await api.calculus.engineer.getCatalogue();
        setCategories(catalogue.categories || []);
      } catch (err) {
        console.error("Failed to load categories:", err);
      }
    }
    fetchCategories();
  }, []);

  // Form management
  const {
    formData,
    setFormData,
    handleInputChange,
    handleFormEvent,
    applyDefaults,
    resetForm,
    updateSection,
  } = useForm();

  // Calculator metadata
  const {
    calculatorMetadata,
    isLoading: isLoadingMetadata,
    error: metadataError,
  } = useMetadata(selectedCalculator, applyDefaults);

  // Form validation
  const { validate, validateOrThrow } = useValidation(
    calculatorMetadata,
    formData
  );

  // Calculation execution
  const {
    results,
    warnings,
    structuredWarnings,
    recommendations,
    isLoading,
    error: calculationError,
    handleCalculate,
    clearResults,
  } = useCalculation(
    selectedCalculator,
    formData,
    outputFormat,
    validateOrThrow
  );

  // Reset form when calculator changes
  useEffect(() => {
    resetForm();
    clearResults();
  }, [selectedCalculator, resetForm, clearResults]);

  // Consolidated error state
  const error = metadataError || calculationError;

  return {
    // Added
    categories,
    // Category selection
    selectedCategory,
    setSelectedCategory,

    // Calculator selection
    selectedCalculator,
    setSelectedCalculator,

    // Metadata
    calculatorMetadata,
    isLoadingMetadata,

    // Form data
    formData,
    handleInputChange,
    handleFormEvent,
    updateSection,
    setFormData, // For advanced manual updates

    // Output format
    outputFormat,
    setOutputFormat,

    // Validation
    validate,

    // Calculation
    handleCalculate,
    results,
    warnings,
    structuredWarnings,
    recommendations,
    isLoading,
    clearResults,

    // Error handling
    error,
  };
};
