/**
 * @file hooks/engineer/useValidation.js
 * @description Form validation against calculator metadata with backend alignment
 * Mission objective: Pre-flight checks matching Rust backend expectations
 * Now with extended_parameters support - because validation is surgical, not optional
 */

import { useCallback } from "react";
import { ValidationError } from "../../lib";

/**
 * Get nested value from object using dot notation path
 * Handles extended_parameters specially
 */
function getNestedValue(obj, path) {
  const pathParts = path.split(".");

  // Handle extended_parameters
  if (
    pathParts[0] === "extended_parameters" ||
    pathParts[0] === "extendedParameters"
  ) {
    const paramName = pathParts.slice(1).join(".");
    return obj.extendedParameters?.[paramName];
  }

  let value = obj;

  for (const part of pathParts) {
    value = value?.[part];
    if (value === undefined) break;
  }

  return value;
}

/**
 * Check if a value is considered "empty" for validation purposes
 */
function isEmpty(value) {
  if (value === null || value === undefined || value === "") return true;

  // Empty arrays are considered empty
  if (Array.isArray(value) && value.length === 0) return true;

  return false;
}

/**
 * Parse value as number if possible
 */
function parseNumeric(value) {
  if (typeof value === "number") return value;
  if (typeof value === "string" && value.trim() !== "") {
    const num = parseFloat(value);
    return isNaN(num) ? null : num;
  }
  return null;
}

/**
 * Validate array items
 */
function validateArrayItems(items, param) {
  const errors = [];

  if (!Array.isArray(items)) {
    errors.push(
      new ValidationError(`${param.name} must be an array`, param.path, items)
    );
    return errors;
  }

  // Each item should be an object with required fields
  items.forEach((item, index) => {
    if (typeof item !== "object" || item === null) {
      errors.push(
        new ValidationError(
          `${param.name}[${index}] must be an object`,
          `${param.path}[${index}]`,
          item
        )
      );
    }
  });

  return errors;
}

export function useValidation(calculatorMetadata, formData) {
  /**
   * Validate form data against metadata constraints
   * Returns: { valid: boolean, errors: ValidationError[], errorMessages: string[] }
   */
  const validate = useCallback(() => {
    if (!calculatorMetadata) {
      return {
        valid: false,
        errors: [new ValidationError("Calculator metadata not loaded")],
        errorMessages: ["Calculator metadata not loaded"],
      };
    }

    const errors = [];
    const errorMessages = [];

    // Check required parameters from metadata
    if (calculatorMetadata.parameters) {
      calculatorMetadata.parameters.forEach((param) => {
        // Skip if not required
        if (!param.required) return;

        const value = getNestedValue(formData, param.path);

        // Check if value is empty
        if (isEmpty(value)) {
          const error = new ValidationError(
            `${param.name} is required`,
            param.path,
            value
          );
          errors.push(error);
          errorMessages.push(`${param.name} is required`);
          return;
        }

        // Validate based on data type
        if (param.data_type === "number" || param.data_type === "integer") {
          const numValue = parseNumeric(value);

          if (numValue === null) {
            const error = new ValidationError(
              `${param.name} must be a valid number`,
              param.path,
              value
            );
            errors.push(error);
            errorMessages.push(`${param.name} must be a valid number`);
            return;
          }

          // Check min value constraint
          if (param.min_value !== null && param.min_value !== undefined) {
            if (numValue < param.min_value) {
              const error = new ValidationError(
                `${param.name} must be >= ${param.min_value}`,
                param.path,
                numValue
              );
              errors.push(error);
              errorMessages.push(`${param.name} must be >= ${param.min_value}`);
            }
          }

          // Check max value constraint
          if (param.max_value !== null && param.max_value !== undefined) {
            if (numValue > param.max_value) {
              const error = new ValidationError(
                `${param.name} must be <= ${param.max_value}`,
                param.path,
                numValue
              );
              errors.push(error);
              errorMessages.push(`${param.name} must be <= ${param.max_value}`);
            }
          }

          // Integer-specific validation
          if (param.data_type === "integer" && !Number.isInteger(numValue)) {
            const error = new ValidationError(
              `${param.name} must be an integer`,
              param.path,
              numValue
            );
            errors.push(error);
            errorMessages.push(`${param.name} must be an integer`);
          }
        }

        // String validation
        if (param.data_type === "string") {
          if (typeof value !== "string") {
            const error = new ValidationError(
              `${param.name} must be a string`,
              param.path,
              value
            );
            errors.push(error);
            errorMessages.push(`${param.name} must be a string`);
          }
        }

        // DateTime validation
        if (param.data_type === "datetime") {
          if (typeof value !== "string" || !value.includes("T")) {
            const error = new ValidationError(
              `${param.name} must be a valid ISO 8601 datetime`,
              param.path,
              value
            );
            errors.push(error);
            errorMessages.push(`${param.name} must be a valid datetime`);
          }
        }

        // Boolean validation
        if (param.data_type === "boolean") {
          if (typeof value !== "boolean") {
            const error = new ValidationError(
              `${param.name} must be a boolean`,
              param.path,
              value
            );
            errors.push(error);
            errorMessages.push(`${param.name} must be true or false`);
          }
        }

        // Array validation
        if (param.data_type === "array") {
          const arrayErrors = validateArrayItems(value, param);
          errors.push(...arrayErrors);
          arrayErrors.forEach((err) => errorMessages.push(err.message));
        }

        // Enum validation
        if (param.data_type?.enum && Array.isArray(param.data_type.enum)) {
          if (!param.data_type.enum.includes(value)) {
            const error = new ValidationError(
              `${param.name} must be one of: ${param.data_type.enum.join(
                ", "
              )}`,
              param.path,
              value
            );
            errors.push(error);
            errorMessages.push(
              `${param.name} must be one of: ${param.data_type.enum.join(", ")}`
            );
          }
        }
      });
    }

    // Legacy validation using required_parameters array
    if (calculatorMetadata.required_parameters) {
      calculatorMetadata.required_parameters.forEach((paramPath) => {
        const value = getNestedValue(formData, paramPath);

        if (isEmpty(value)) {
          // Skip if already reported by parameter metadata validation
          if (!errors.some((e) => e.field === paramPath)) {
            const error = new ValidationError(
              `${paramPath} is required`,
              paramPath,
              value
            );
            errors.push(error);
            errorMessages.push(`${paramPath} is required`);
          }
        }
      });
    }

    return {
      valid: errors.length === 0,
      errors,
      errorMessages,
    };
  }, [formData, calculatorMetadata]);

  /**
   * Validate and throw on error
   */
  const validateOrThrow = useCallback(() => {
    const { valid, errorMessages } = validate();

    if (!valid) {
      throw new ValidationError(errorMessages.join("; "));
    }
  }, [validate]);

  /**
   * Validate a specific field
   */
  const validateField = useCallback(
    (path) => {
      if (!calculatorMetadata?.parameters) {
        return { valid: true, error: null };
      }

      const param = calculatorMetadata.parameters.find((p) => p.path === path);
      if (!param) {
        return { valid: true, error: null };
      }

      const value = getNestedValue(formData, path);

      // Check required
      if (param.required && isEmpty(value)) {
        return {
          valid: false,
          error: `${param.name} is required`,
        };
      }

      // If empty and not required, it's valid
      if (isEmpty(value)) {
        return { valid: true, error: null };
      }

      // Type-specific validation
      if (param.data_type === "number" || param.data_type === "integer") {
        const numValue = parseNumeric(value);

        if (numValue === null) {
          return {
            valid: false,
            error: `${param.name} must be a valid number`,
          };
        }

        if (
          param.min_value !== null &&
          param.min_value !== undefined &&
          numValue < param.min_value
        ) {
          return {
            valid: false,
            error: `${param.name} must be >= ${param.min_value}`,
          };
        }

        if (
          param.max_value !== null &&
          param.max_value !== undefined &&
          numValue > param.max_value
        ) {
          return {
            valid: false,
            error: `${param.name} must be <= ${param.max_value}`,
          };
        }

        if (param.data_type === "integer" && !Number.isInteger(numValue)) {
          return {
            valid: false,
            error: `${param.name} must be an integer`,
          };
        }
      }

      // Array validation
      if (param.data_type === "array") {
        if (!Array.isArray(value)) {
          return {
            valid: false,
            error: `${param.name} must be an array`,
          };
        }

        if (param.required && value.length === 0) {
          return {
            valid: false,
            error: `${param.name} must have at least one item`,
          };
        }
      }

      // DateTime validation
      if (param.data_type === "datetime") {
        if (typeof value !== "string" || !value.includes("T")) {
          return {
            valid: false,
            error: `${param.name} must be a valid datetime`,
          };
        }
      }

      return { valid: true, error: null };
    },
    [formData, calculatorMetadata]
  );

  /**
   * Get all validation errors without throwing
   */
  const getValidationErrors = useCallback(() => {
    const { errors, errorMessages } = validate();
    return { errors, messages: errorMessages };
  }, [validate]);

  return {
    validate,
    validateOrThrow,
    validateField,
    getValidationErrors,
  };
}
