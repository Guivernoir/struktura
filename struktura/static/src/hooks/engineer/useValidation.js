/**
 * @file hooks/engineer/useValidation.js
 * @description Form validation against calculator metadata
 * Mission objective: Pre-flight checks and quality assurance
 */

import { useCallback } from "react";
import { ValidationError } from "../../lib";

export function useValidation(calculatorMetadata, formData) {
  /**
   * Validate form data against metadata constraints
   * Returns: { valid: boolean, errors: string[] }
   */
  const validate = useCallback(() => {
    if (!calculatorMetadata) {
      return {
        valid: false,
        errors: ["Calculator metadata not loaded"],
      };
    }

    const errors = [];

    // Check required parameters
    const requiredParams = calculatorMetadata.required_parameters || [];
    requiredParams.forEach((paramPath) => {
      const value = getNestedValue(formData, paramPath);

      if (value === undefined || value === null || value === "") {
        errors.push(`${paramPath} is required`);
      }
    });

    // Validate against parameter constraints
    if (calculatorMetadata.parameters) {
      calculatorMetadata.parameters.forEach((param) => {
        if (!param.required) return;

        const value = getNestedValue(formData, param.path);

        if (value !== undefined && value !== null && value !== "") {
          const numValue = parseFloat(value);

          if (!isNaN(numValue)) {
            if (param.min_value !== null && numValue < param.min_value) {
              errors.push(`${param.name} must be >= ${param.min_value}`);
            }
            if (param.max_value !== null && numValue > param.max_value) {
              errors.push(`${param.name} must be <= ${param.max_value}`);
            }
          }
        }
      });
    }

    return {
      valid: errors.length === 0,
      errors,
    };
  }, [formData, calculatorMetadata]);

  /**
   * Validate and throw on error
   */
  const validateOrThrow = useCallback(() => {
    const { valid, errors } = validate();

    if (!valid) {
      throw new ValidationError(errors.join("; "));
    }
  }, [validate]);

  return {
    validate,
    validateOrThrow,
  };
}

/**
 * Get nested value from object using dot notation path
 */
function getNestedValue(obj, path) {
  const pathParts = path.split(".");
  let value = obj;

  for (const part of pathParts) {
    value = value?.[part];
    if (value === undefined) break;
  }

  return value;
}
