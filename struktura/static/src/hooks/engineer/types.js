/**
 * @file hooks/engineer/types.js
 * @description Type definitions for professional engineering operations
 * Mission briefing: Data structures for special forces
 */

import { OutputFormat } from "../../lib";

/**
 * Initial form state for professional engineering
 */
export const INITIAL_FORM_STATE = {
  dimensions: {},
  material: {},
  loads: {},
  safetyFactors: {},
  designCode: null,
  exposureClass: null,
  temperature: null,
  humidity: null,
  additional: {},
  projectMetadata: null,
};

/**
 * Initial results state
 */
export const INITIAL_RESULTS_STATE = {
  results: [],
  warnings: [],
  structuredWarnings: null,
  recommendations: [],
};

/**
 * Default category for initial deployment
 */
export const DEFAULT_CATEGORY = "structural";

/**
 * Default output format
 */
export const DEFAULT_OUTPUT_FORMAT = OutputFormat.STANDARD;
