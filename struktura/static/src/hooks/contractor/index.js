/**
 * @file hooks/contractor/index.js
 * @description Contractor calculator orchestration hook
 * Mission objective: Command center for professional contracting operations
 *
 * Architecture: Composed from specialized sub-hooks
 */

import { useState } from "react";
import { useCatalogue } from "./useCatalogue";
import { useInputs } from "./useInputs";
import { useForm } from "./useForm";
import { useCalculation } from "./useCalculation";

export const useContractorCalculator = () => {
  const [selectedCalculator, setSelectedCalculator] = useState(null);

  // Form management with nested structure support
  const {
    formData,
    setFormData,
    handleInputChange,
    handleFormEvent,
    updateFormData,
    resetForm,
    updateDimension,
    updateMaterial,
    updateResource,
    updateSafetyFactor,
    updateAdditional,
  } = useForm();

  // Catalogue and category management
  const {
    catalogue,
    categories,
    selectedCategory,
    setSelectedCategory,
    calculatorsInCategory,
    error: catalogueError,
  } = useCatalogue(setSelectedCalculator);

  // Input specifications
  const {
    inputs,
    isLoadingInputs,
    error: inputsError,
  } = useInputs(selectedCalculator, resetForm);

  // Calculation execution with professional features
  const {
    results,
    warnings,
    structuredWarnings,
    recommendations,
    complianceNotes,
    analysis,
    metadata,
    isLoading,
    error: calculationError,
    handleCalculate,
    handleCalculateDetailed,
    handleCalculateSummary,
    clearResults,
  } = useCalculation(selectedCalculator, formData);

  // Consolidated error state
  const error = catalogueError || inputsError || calculationError;

  return {
    // Catalogue
    catalogue,
    categories,
    calculatorsInCategory,

    // Category selection
    selectedCategory,
    setSelectedCategory,

    // Calculator selection
    selectedCalculator,
    setSelectedCalculator,

    // Input specifications
    inputs,
    isLoadingInputs,

    // Form data with specialized updaters
    formData,
    setFormData,
    handleInputChange,
    handleFormEvent,
    updateFormData,
    updateDimension,
    updateMaterial,
    updateResource,
    updateSafetyFactor,
    updateAdditional,

    // Calculation with output format options
    handleCalculate,
    handleCalculateDetailed,
    handleCalculateSummary,
    results,
    warnings,
    structuredWarnings,
    recommendations,
    complianceNotes,
    analysis,
    metadata,
    isLoading,
    clearResults,

    // Error handling
    error,
  };
};
