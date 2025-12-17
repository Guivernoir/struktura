/**
 * @file hooks/beginner/useForm.js
 * @description Form state management
 * Mission objective: Tactical parameter control
 */

import { useState, useCallback } from "react";
import { INITIAL_FORM_STATE } from "./types";

export function useForm() {
  const [formData, setFormData] = useState(INITIAL_FORM_STATE);

  /**
   * Handle individual input changes
   */
  const handleInputChange = useCallback((e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: parseFloat(value) || 0,
    }));
  }, []);

  /**
   * Update form data programmatically
   */
  const updateFormData = useCallback((updates) => {
    setFormData((prev) => ({ ...prev, ...updates }));
  }, []);

  /**
   * Reset form to default state
   */
  const resetForm = useCallback((newDefaults = INITIAL_FORM_STATE) => {
    setFormData(newDefaults);
  }, []);

  return {
    formData,
    setFormData,
    handleInputChange,
    updateFormData,
    resetForm,
  };
}
