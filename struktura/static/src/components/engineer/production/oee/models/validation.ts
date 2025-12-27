/**
 * OEE Validation Layer - Data Structures
 * 
 * Bright-line rule: We validate logic, not realism.
 * If numbers are mathematically impossible, we flag it.
 * If they're merely "unlikely," we shut up.
 */

/**
 * Severity levels for validation issues
 */
export type Severity = 'fatal' | 'warning' | 'info';

/**
 * A validation issue with translation-ready messaging
 */
export interface ValidationIssue {
  /** Translation key for the message */
  messageKey: string;
  /** Parameters for translation interpolation */
  params: Record<string, any>;
  /** Severity level */
  severity: Severity;
  /** Field path (e.g., "time_allocations[2].duration") */
  fieldPath?: string;
  /** Error code for programmatic handling */
  code: string;
}

/**
 * Complete validation result
 */
export interface ValidationResult {
  isValid: boolean;
  issues: ValidationIssue[];
}