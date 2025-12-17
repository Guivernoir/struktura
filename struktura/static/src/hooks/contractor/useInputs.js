/**
 * @file hooks/contractor/useInputs.js
 * @description Input specification loading and management
 * Mission objective: Field equipment preparation for professional operations
 */

import { useState, useEffect } from "react";
import { api } from "../../lib";
import { INITIAL_INPUT_STATE, INITIAL_FORM_STATE } from "./types";

export function useInputs(selectedCalculator, onFormReset) {
  const [inputs, setInputs] = useState(INITIAL_INPUT_STATE);
  const [isLoadingInputs, setIsLoadingInputs] = useState(false);
  const [error, setError] = useState(null);

  // Load input specifications when calculator changes
  useEffect(() => {
    const loadInputs = async () => {
      if (!selectedCalculator) {
        setInputs(INITIAL_INPUT_STATE);
        return;
      }

      setIsLoadingInputs(true);
      setError(null);

      try {
        const spec = await api.calculus.getCalculatorInputs(
          selectedCalculator,
          "contractor"
        );

        setInputs({
          required: spec.required || [],
          optional: spec.optional || [],
          parameters: spec.parameters || [],
          codes: spec.codes || [],
          metadata: spec.metadata || null,
        });

        // Trigger form reset with safe defaults
        if (onFormReset) {
          onFormReset(INITIAL_FORM_STATE);
        }

        console.log("📋 Contractor inputs loaded:", {
          calculator: selectedCalculator,
          required: spec.required?.length || 0,
          optional: spec.optional?.length || 0,
          codes: spec.codes?.length || 0,
        });
      } catch (err) {
        setError("Failed to load input specifications");
        console.error("Contractor input load failed:", err);
        setInputs(INITIAL_INPUT_STATE);
      } finally {
        setIsLoadingInputs(false);
      }
    };

    loadInputs();
  }, [selectedCalculator, onFormReset]);

  return {
    inputs,
    isLoadingInputs,
    error,
  };
}
