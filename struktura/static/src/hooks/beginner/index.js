/**
 * @file hooks/beginner/index.js
 * @description Beginner calculator orchestration hook
 * Mission objective: Command center for civilian operations
 *
 * Architecture: Composed from specialized sub-hooks
 */

import { useState } from "react";
import { useCatalogue } from "./useCatalogue";
import { useInputs } from "./useInputs";
import { useForm } from "./useForm";
import { useCalculation } from "./useCalculation";

export const useBeginnerCalculator = () => {
  const [selectedCalculator, setSelectedCalculator] = useState(null);

  // Form management
  const { formData, setFormData, handleInputChange, resetForm } = useForm();

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

  // Calculation execution
  const {
    results,
    warnings,
    isLoading,
    error: calculationError,
    handleCalculate,
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

    // Form data
    formData,
    setFormData,
    handleInputChange,

    // Calculation
    handleCalculate,
    results,
    warnings,
    isLoading,
    clearResults,

    // Error handling
    error,
  };
};
