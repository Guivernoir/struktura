/**
 * @file hooks/engineer/useForm.js
 * @description Professional form state management with robust nested handling
 * Mission objective: Complex parameter control with type-safe updates
 * Now with extended_parameters support - because JSON archaeology is so last season
 */

import { useState, useCallback } from "react";
import { INITIAL_FORM_STATE, createFormStateFromMetadata } from "./types";

/**
 * Utility function to safely update a nested object property by string path
 * Ensures React's state immutability requirements are met
 *
 * @param {Object} obj The previous state object
 * @param {string} path The dot-separated path (e.g., 'dimensions.height')
 * @param {*} value The new value for the field
 * @returns {Object} A new state object with the value updated
 */
const setDeepValue = (obj, path, value) => {
  const parts = path.split(".");

  // Handle simple top-level updates
  if (parts.length === 1) {
    return { ...obj, [parts[0]]: value };
  }

  // Handle extended_parameters specially
  if (parts[0] === "extended_parameters" || parts[0] === "extendedParameters") {
    const paramName = parts.slice(1).join(".");
    return {
      ...obj,
      extendedParameters: {
        ...obj.extendedParameters,
        [paramName]: value,
      },
    };
  }

  // Clone the top level
  const result = { ...obj };
  let current = result;

  // Navigate and clone each level
  for (let i = 0; i < parts.length - 1; i++) {
    const part = parts[i];

    // Ensure the intermediate path exists
    if (!current[part] || typeof current[part] !== "object") {
      current[part] = {};
    } else {
      current[part] = { ...current[part] };
    }

    current = current[part];
  }

  // Set the final value
  current[parts[parts.length - 1]] = value;

  return result;
};

/**
 * Get nested value from object using dot notation path
 */
const getDeepValue = (obj, path) => {
  const parts = path.split(".");

  // Handle extended_parameters specially
  if (parts[0] === "extended_parameters" || parts[0] === "extendedParameters") {
    const paramName = parts.slice(1).join(".");
    return obj.extendedParameters?.[paramName];
  }

  let value = obj;

  for (const part of parts) {
    value = value?.[part];
    if (value === undefined) break;
  }

  return value;
};

export function useForm(initialMetadata = null) {
  // Initialize form state based on metadata if provided
  const [formData, setFormData] = useState(() => {
    if (initialMetadata) {
      return createFormStateFromMetadata(initialMetadata);
    }
    return INITIAL_FORM_STATE;
  });

  /**
   * Handle input changes for nested form structure
   * Accepts native event object from input field
   * Input name must be dot-separated path (e.g., 'dimensions.height')
   *
   * @param {Event} event The native change event from an input field
   */
  const handleInputChange = useCallback((event) => {
    const { name, value, type, checked } = event.target;

    // Determine the appropriate value based on input type
    let finalValue;

    if (type === "checkbox") {
      finalValue = checked;
    } else if (type === "number") {
      // For number inputs, keep as string for controlled input
      // Conversion to number happens in parameter builder
      finalValue = value;
    } else {
      finalValue = value;
    }

    setFormData((prevData) => setDeepValue(prevData, name, finalValue));
  }, []);

  /**
   * Handle form event with auto-parsing of numeric values
   * Use this when you want automatic number conversion
   *
   * @param {Event} e The change event
   */
  const handleFormEvent = useCallback((e) => {
    const { name, value, type } = e.target;

    let finalValue;

    if (type === "number") {
      // Auto-parse numbers
      const numValue = parseFloat(value);
      finalValue = isNaN(numValue) ? value : numValue;
    } else if (type === "checkbox") {
      finalValue = e.target.checked;
    } else {
      finalValue = value;
    }

    setFormData((prevData) => setDeepValue(prevData, name, finalValue));
  }, []);

  /**
   * Update a specific field by path
   *
   * @param {string} path Dot-separated path (e.g., 'material.elastic_modulus')
   * @param {*} value The new value
   */
  const updateField = useCallback((path, value) => {
    setFormData((prevData) => setDeepValue(prevData, path, value));
  }, []);

  /**
   * Get a specific field value by path
   *
   * @param {string} path Dot-separated path
   * @returns {*} The field value
   */
  const getField = useCallback(
    (path) => {
      return getDeepValue(formData, path);
    },
    [formData]
  );

  /**
   * Batch update form data with defaults
   * Deep merges to preserve existing nested structures
   *
   * @param {Object} defaults Default values to merge
   */
  const applyDefaults = useCallback((defaults) => {
    const deepMerge = (target, source) => {
      const output = { ...target };

      for (const key of Object.keys(source)) {
        const sourceValue = source[key];
        const targetValue = target[key];

        // Deep merge objects
        if (
          sourceValue &&
          typeof sourceValue === "object" &&
          !Array.isArray(sourceValue) &&
          targetValue &&
          typeof targetValue === "object" &&
          !Array.isArray(targetValue)
        ) {
          output[key] = deepMerge(targetValue, sourceValue);
        } else {
          output[key] = sourceValue;
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
   * Reset form based on metadata
   */
  const resetFormWithMetadata = useCallback((metadata) => {
    if (metadata) {
      setFormData(createFormStateFromMetadata(metadata));
    } else {
      setFormData(INITIAL_FORM_STATE);
    }
  }, []);

  /**
   * Update entire section at once
   *
   * @param {string} section Top-level key (e.g., 'material', 'loads')
   * @param {Object} data The new data for that section
   */
  const updateSection = useCallback((section, data) => {
    setFormData((prev) => ({
      ...prev,
      [section]: data,
    }));
  }, []);

  /**
   * Bulk update multiple fields at once
   *
   * @param {Object} updates Object with path: value pairs
   * @example bulkUpdate({ 'dimensions.height': 10, 'material.density': 7850 })
   */
  const bulkUpdate = useCallback((updates) => {
    setFormData((prev) => {
      let result = prev;

      for (const [path, value] of Object.entries(updates)) {
        result = setDeepValue(result, path, value);
      }

      return result;
    });
  }, []);

  /**
   * Update an extended parameter (new typed parameter system)
   *
   * @param {string} name Parameter name (without 'extended_parameters.' prefix)
   * @param {*} value The new value
   */
  const updateExtendedParameter = useCallback((name, value) => {
    setFormData((prev) => ({
      ...prev,
      extendedParameters: {
        ...prev.extendedParameters,
        [name]: value,
      },
    }));
  }, []);

  /**
   * Get an extended parameter value
   *
   * @param {string} name Parameter name
   * @returns {*} The parameter value
   */
  const getExtendedParameter = useCallback(
    (name) => {
      return formData.extendedParameters?.[name];
    },
    [formData]
  );

  /**
   * Check if form has any data
   */
  const hasData = useCallback(() => {
    const checkObject = (obj) => {
      if (!obj || typeof obj !== "object") return false;

      return Object.values(obj).some((value) => {
        if (value && typeof value === "object") {
          return checkObject(value);
        }
        return value !== null && value !== undefined && value !== "";
      });
    };

    return checkObject(formData);
  }, [formData]);

  return {
    formData,
    setFormData,
    handleInputChange,
    handleFormEvent,
    updateField,
    getField,
    applyDefaults,
    resetForm,
    resetFormWithMetadata,
    updateSection,
    bulkUpdate,
    updateExtendedParameter,
    getExtendedParameter,
    hasData,
  };
}
