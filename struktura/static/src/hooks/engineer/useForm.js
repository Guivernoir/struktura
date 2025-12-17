/**
 * @file hooks/engineer/useForm.js
 * @description Professional form state management
 * Mission objective: Complex parameter control with nested structures
 */

import { useState, useCallback } from "react";
import { INITIAL_FORM_STATE } from "./types";

/**
 * Utility function to safely update a nested object property by string path
 * (e.g., 'dimensions.height'). This ensures React's state immutability
 * requirements are met, preventing state loss for nested fields.
 * * @param {Object} obj The previous state object.
 * @param {string} path The dot-separated path to the field (e.g., 'dimensions.height').
 * @param {*} value The new value for the field.
 * @returns {Object} A new state object with the value updated.
 */
const setDeepValue = (obj, path, value) => {
  const parts = path.split(".");

  // Defensive copy of the top level
  let current = { ...obj };
  let pointer = current;

  for (let i = 0; i < parts.length; i++) {
    const part = parts[i];

    // If it's the last part, set the value
    if (i === parts.length - 1) {
      pointer[part] = value;
    }
    // Otherwise, clone the next level down to maintain immutability
    else {
      // Ensure the key exists and is an object before cloning
      if (!pointer[part] || typeof pointer[part] !== "object") {
        // If the path doesn't exist, initialize it as an object
        pointer[part] = {};
      }

      pointer[part] = { ...pointer[part] };
      pointer = pointer[part];
    }
  }
  return current;
};

export function useForm() {
  const [formData, setFormData] = useState(INITIAL_FORM_STATE);

  /**
   * Handle input changes for the nested form structure.
   * This function now expects the native event object from the input field.
   * * @param {Event} event The native change event from an input field.
   * The input name must be a dot-separated path (e.g., 'dimensions.height').
   */
  const handleInputChange = useCallback((event) => {
    const { name, value, type, checked } = event.target;

    // Determine value based on input type (e.g., checkbox uses 'checked')
    const newValue = type === "checkbox" ? checked : value;

    setFormData((prevData) => setDeepValue(prevData, name, newValue));
  }, []);

  /**
   * Handle standard form event
   */
  const handleFormEvent = useCallback((e) => {
    const { name, value } = e.target;
    const numValue = parseFloat(value);
    const finalValue = isNaN(numValue) ? value : numValue;

    setFormData((prevData) => setDeepValue(prevData, name, finalValue));
  }, []);

  /**
   * Batch update form data with defaults
   */
  const applyDefaults = useCallback((defaults) => {
    // This is still fragile. Let's make it deep and robust.
    const deepMerge = (target, source) => {
      const output = { ...target };
      for (const key of Object.keys(source)) {
        if (source[key] instanceof Object && target[key]) {
          output[key] = deepMerge(target[key], source[key]);
        } else {
          output[key] = source[key];
        }
      }
      return output;
    };

    setFormData((prev) => deepMerge(prev, defaults));
  }, []);

  /**
   * Reset form to initial state
   */
  const resetForm = useCallback(() => {
    setFormData(INITIAL_FORM_STATE);
  }, []);

  /**
   * Update entire section
   */
  const updateSection = useCallback((section, data) => {
    setFormData((prev) => ({
      ...prev,
      [section]: data,
    }));
  }, []);

  return {
    formData,
    setFormData,
    handleInputChange,
    handleFormEvent,
    applyDefaults,
    resetForm,
    updateSection,
  };
}
