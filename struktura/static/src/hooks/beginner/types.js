/**
 * @file hooks/beginner/types.js
 * @description Type definitions and initial state for beginner calculator
 * Mission briefing: Data structures for civilian operations
 */

/**
 * Initial form state with sensible tactical defaults
 */
export const INITIAL_FORM_STATE = {
  width: 3,
  length: 4,
  height: 0.15,
  depth: 0,
  thickness: 0,
};

/**
 * Initial input specification state
 */
export const INITIAL_INPUT_STATE = {
  hints: {},
  ranges: {},
  metadata: null,
  required: [],
  optional: [],
};

/**
 * Default category for initial deployment
 */
export const DEFAULT_CATEGORY = "outdoors";
