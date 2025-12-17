/**
 * @file hooks/index.js
 * @description Central export point for all calculator hooks
 * Mission briefing: Clean access to all operational capabilities
 *
 * Usage:
 *   import { useBeginnerCalculator, useEngineerCalculator } from './hooks'
 */

// Beginner calculator hook (civilian operations)
export { useBeginnerCalculator } from "./beginner";

// Contractor calculator hook (professional contracting)
export { useContractorCalculator } from "./contractor";

// Engineer calculator hook (special forces)
export { useEngineerCalculator } from "./engineer";

/**
 * Quick start examples:
 *
 * // Beginner Calculator
 * const {
 *   selectedCalculator,
 *   setSelectedCalculator,
 *   formData,
 *   handleInputChange,
 *   handleCalculate,
 *   results,
 *   isLoading
 * } = useBeginnerCalculator()
 *
 * // Engineer Calculator
 * const {
 *   selectedCalculator,
 *   setSelectedCalculator,
 *   calculatorMetadata,
 *   formData,
 *   handleInputChange,
 *   handleCalculate,
 *   results,
 *   structuredWarnings,
 *   isLoading
 * } = useEngineerCalculator()
 */
