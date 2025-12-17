/**
 * @file hooks/contractor/useForm.js
 * @description Form state management for contracting parameters
 * Mission objective: Tactical parameter control for professional operations
 */

import { useState, useCallback } from "react";
import { INITIAL_FORM_STATE } from "./types";

export function useForm() {
  const [formData, setFormData] = useState(INITIAL_FORM_STATE);

  /**
   * Handle nested input changes (e.g., dimensions.area, material.unit_cost)
   */
  const handleInputChange = useCallback((path, value) => {
    setFormData((prev) => {
      const newData = { ...prev };
      const parts = path.split(".");

      if (parts.length === 1) {
        // Top-level field
        newData[parts[0]] = value;
      } else if (parts.length === 2) {
        // Nested field (e.g., dimensions.area)
        newData[parts[0]] = {
          ...prev[parts[0]],
          [parts[1]]: value,
        };
      } else if (parts.length === 3) {
        // Double-nested field (e.g., project_metadata.project_name)
        newData[parts[0]] = {
          ...prev[parts[0]],
          [parts[1]]: {
            ...prev[parts[0]][parts[1]],
            [parts[2]]: value,
          },
        };
      }

      return newData;
    });
  }, []);

  /**
   * Handle standard form event
   */
  const handleFormEvent = useCallback(
    (e) => {
      const { name, value } = e.target;
      const numValue = parseFloat(value);
      handleInputChange(name, isNaN(numValue) ? value : numValue);
    },
    [handleInputChange]
  );

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

  /**
   * Update dimension field
   */
  const updateDimension = useCallback(
    (key, value) => {
      handleInputChange(`dimensions.${key}`, parseFloat(value) || 0);
    },
    [handleInputChange]
  );

  /**
   * Update material property
   */
  const updateMaterial = useCallback(
    (key, value) => {
      handleInputChange(`material.${key}`, value);
    },
    [handleInputChange]
  );

  /**
   * Update resource requirement
   */
  const updateResource = useCallback(
    (key, value) => {
      handleInputChange(`resources.${key}`, parseFloat(value) || 0);
    },
    [handleInputChange]
  );

  /**
   * Update safety factor
   */
  const updateSafetyFactor = useCallback(
    (key, value) => {
      handleInputChange(`safety_factors.${key}`, parseFloat(value) || 0);
    },
    [handleInputChange]
  );

  /**
   * Update additional parameter
   */
  const updateAdditional = useCallback((key, value) => {
    setFormData((prev) => ({
      ...prev,
      additional: {
        ...prev.additional,
        [key]: parseFloat(value) || 0,
      },
    }));
  }, []);

  return {
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
  };
}
