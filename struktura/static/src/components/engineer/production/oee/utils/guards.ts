/**
 * Type guards and runtime validation utilities
 * 
 * These functions provide runtime type checking and validation
 * to ensure data integrity at component boundaries.
 */

import type {
  OeeInput,
  OeeResult,
  ProductionSummary,
  CycleTimeModel,
  TimeModel,
  DowntimeCollection,
  ThresholdConfiguration,
  TrackedMetric,
  CoreMetrics,
  LossTree,
  AssumptionLedger,
  InputValue,
} from '../models';

/**
 * Check if a value is a valid InputValue
 */
export function isInputValue<T>(value: unknown): value is InputValue<T> {
  if (typeof value !== 'object' || value === null) return false;
  
  const v = value as any;
  
  return (
    ('type' in v) &&
    ('value' in v) &&
    (v.type === 'Explicit' || v.type === 'Inferred' || v.type === 'Default')
  );
}

/**
 * Check if a value is a valid number (finite, not NaN)
 */
export function isValidNumber(value: unknown): value is number {
  return typeof value === 'number' && isFinite(value) && !isNaN(value);
}

/**
 * Check if a value is a non-negative number
 */
export function isNonNegativeNumber(value: unknown): value is number {
  return isValidNumber(value) && value >= 0;
}

/**
 * Check if a value is a positive number
 */
export function isPositiveNumber(value: unknown): value is number {
  return isValidNumber(value) && value > 0;
}

/**
 * Check if a value is a valid integer
 */
export function isInteger(value: unknown): value is number {
  return isValidNumber(value) && Number.isInteger(value);
}

/**
 * Check if a value is a non-negative integer
 */
export function isNonNegativeInteger(value: unknown): value is number {
  return isInteger(value) && value >= 0;
}

/**
 * Check if a value is a valid percentage (0-1)
 */
export function isPercentage(value: unknown): value is number {
  return isValidNumber(value) && value >= 0 && value <= 1;
}

/**
 * Check if a value is a valid ISO 8601 date string
 */
export function isISODateString(value: unknown): value is string {
  if (typeof value !== 'string') return false;
  
  const date = new Date(value);
  return !isNaN(date.getTime());
}

/**
 * Check if ProductionSummary is valid
 */
export function isValidProductionSummary(value: unknown): value is ProductionSummary {
  if (typeof value !== 'object' || value === null) return false;
  
  const ps = value as any;
  
  return (
    isInputValue(ps.total_units) &&
    isInputValue(ps.good_units) &&
    isInputValue(ps.scrap_units) &&
    isInputValue(ps.reworked_units)
  );
}

/**
 * Check if CycleTimeModel is valid
 */
export function isValidCycleTimeModel(value: unknown): value is CycleTimeModel {
  if (typeof value !== 'object' || value === null) return false;
  
  const ctm = value as any;
  
  return (
    isInputValue(ctm.ideal_cycle_time) &&
    (ctm.average_cycle_time === undefined || isInputValue(ctm.average_cycle_time))
  );
}

/**
 * Check if TimeModel is valid
 */
export function isValidTimeModel(value: unknown): value is TimeModel {
  if (typeof value !== 'object' || value === null) return false;
  
  const tm = value as any;
  
  return (
    isInputValue(tm.planned_production_time) &&
    Array.isArray(tm.allocations) &&
    (tm.all_time === undefined || isInputValue(tm.all_time))
  );
}

/**
 * Check if DowntimeCollection is valid
 */
export function isValidDowntimeCollection(value: unknown): value is DowntimeCollection {
  if (typeof value !== 'object' || value === null) return false;
  
  const dc = value as any;
  
  return Array.isArray(dc.records);
}

/**
 * Check if ThresholdConfiguration is valid
 */
export function isValidThresholdConfiguration(value: unknown): value is ThresholdConfiguration {
  if (typeof value !== 'object' || value === null) return false;
  
  const tc = value as any;
  
  return (
    isNonNegativeNumber(tc.micro_stoppage_threshold) &&
    isNonNegativeNumber(tc.small_stop_threshold) &&
    isPercentage(tc.speed_loss_threshold) &&
    isPercentage(tc.high_scrap_rate_threshold) &&
    isPercentage(tc.low_utilization_threshold)
  );
}

/**
 * Check if OeeInput is valid
 */
export function isValidOeeInput(value: unknown): value is OeeInput {
  if (typeof value !== 'object' || value === null) return false;
  
  const input = value as any;
  
  return (
    input.window !== undefined &&
    input.machine !== undefined &&
    isValidTimeModel(input.time_model) &&
    isValidProductionSummary(input.production) &&
    isValidCycleTimeModel(input.cycle_time) &&
    isValidDowntimeCollection(input.downtimes) &&
    isValidThresholdConfiguration(input.thresholds)
  );
}

/**
 * Check if TrackedMetric is valid
 */
export function isValidTrackedMetric(value: unknown): value is TrackedMetric {
  if (typeof value !== 'object' || value === null) return false;
  
  const tm = value as any;
  
  return (
    typeof tm.name_key === 'string' &&
    isValidNumber(tm.value) &&
    typeof tm.unit_key === 'string' &&
    typeof tm.formula_key === 'string' &&
    typeof tm.formula_params === 'object' &&
    typeof tm.confidence === 'string'
  );
}

/**
 * Check if CoreMetrics is valid
 */
export function isValidCoreMetrics(value: unknown): value is CoreMetrics {
  if (typeof value !== 'object' || value === null) return false;
  
  const cm = value as any;
  
  return (
    isValidTrackedMetric(cm.availability) &&
    isValidTrackedMetric(cm.performance) &&
    isValidTrackedMetric(cm.quality) &&
    isValidTrackedMetric(cm.oee)
  );
}

/**
 * Check if LossTree is valid
 */
export function isValidLossTree(value: unknown): value is LossTree {
  if (typeof value !== 'object' || value === null) return false;
  
  const lt = value as any;
  
  return (
    typeof lt.root === 'object' &&
    isNonNegativeNumber(lt.planned_time)
  );
}

/**
 * Check if AssumptionLedger is valid
 */
export function isValidAssumptionLedger(value: unknown): value is AssumptionLedger {
  if (typeof value !== 'object' || value === null) return false;
  
  const al = value as any;
  
  return (
    isISODateString(al.analysis_timestamp) &&
    Array.isArray(al.assumptions) &&
    Array.isArray(al.warnings) &&
    Array.isArray(al.thresholds) &&
    typeof al.source_statistics === 'object' &&
    typeof al.metadata === 'object'
  );
}

/**
 * Check if OeeResult is valid
 */
export function isValidOeeResult(value: unknown): value is OeeResult {
  if (typeof value !== 'object' || value === null) return false;
  
  const result = value as any;
  
  return (
    isValidCoreMetrics(result.core_metrics) &&
    typeof result.extended_metrics === 'object' &&
    isValidLossTree(result.loss_tree) &&
    isValidAssumptionLedger(result.ledger) &&
    typeof result.validation === 'object'
  );
}

/**
 * Array validation helpers
 */
export const ArrayGuards = {
  /**
   * Check if all elements in array satisfy predicate
   */
  all: <T>(arr: unknown[], predicate: (item: unknown) => item is T): arr is T[] => {
    return arr.every(predicate);
  },

  /**
   * Check if array is non-empty
   */
  isNonEmpty: <T>(arr: T[]): arr is [T, ...T[]] => {
    return arr.length > 0;
  },

  /**
   * Check if array has unique elements
   */
  hasUniqueElements: <T>(arr: T[], keyFn?: (item: T) => unknown): boolean => {
    if (!keyFn) {
      return new Set(arr).size === arr.length;
    }
    
    const keys = arr.map(keyFn);
    return new Set(keys).size === keys.length;
  },
};

/**
 * Object validation helpers
 */
export const ObjectGuards = {
  /**
   * Check if object has required keys
   */
  hasKeys: <K extends string>(
    obj: unknown,
    keys: K[]
  ): obj is Record<K, unknown> => {
    if (typeof obj !== 'object' || obj === null) return false;
    
    return keys.every(key => key in obj);
  },

  /**
   * Check if object has no unexpected keys
   */
  hasOnlyKeys: (obj: unknown, allowedKeys: string[]): boolean => {
    if (typeof obj !== 'object' || obj === null) return false;
    
    const objKeys = Object.keys(obj);
    return objKeys.every(key => allowedKeys.includes(key));
  },

  /**
   * Check if value is a plain object (not array, null, etc.)
   */
  isPlainObject: (value: unknown): value is Record<string, unknown> => {
    return (
      typeof value === 'object' &&
      value !== null &&
      !Array.isArray(value) &&
      Object.getPrototypeOf(value) === Object.prototype
    );
  },
};

/**
 * Range validation helpers
 */
export const RangeGuards = {
  /**
   * Check if value is within range (inclusive)
   */
  inRange: (value: number, min: number, max: number): boolean => {
    return isValidNumber(value) && value >= min && value <= max;
  },

  /**
   * Check if value is within percentage range (0-100)
   */
  inPercentageRange: (value: number): boolean => {
    return RangeGuards.inRange(value, 0, 100);
  },

  /**
   * Check if value is within normalized range (0-1)
   */
  inNormalizedRange: (value: number): boolean => {
    return RangeGuards.inRange(value, 0, 1);
  },
};

/**
 * String validation helpers
 */
export const StringGuards = {
  /**
   * Check if string is non-empty
   */
  isNonEmpty: (value: string): boolean => {
    return value.trim().length > 0;
  },

  /**
   * Check if string matches pattern
   */
  matchesPattern: (value: string, pattern: RegExp): boolean => {
    return pattern.test(value);
  },

  /**
   * Check if string is valid JSON
   */
  isValidJson: (value: string): boolean => {
    try {
      JSON.parse(value);
      return true;
    } catch {
      return false;
    }
  },

  /**
   * Check if string is a valid currency code (ISO 4217)
   */
  isValidCurrencyCode: (value: string): boolean => {
    return /^[A-Z]{3}$/.test(value);
  },
};

/**
 * Assertion helpers that throw errors
 */
export const Assertions = {
  /**
   * Assert value is defined (not null or undefined)
   */
  assertDefined: <T>(
    value: T | null | undefined,
    message: string = 'Value must be defined'
  ): T => {
    if (value === null || value === undefined) {
      throw new Error(message);
    }
    return value;
  },

  /**
   * Assert value is a valid number
   */
  assertValidNumber: (
    value: unknown,
    message: string = 'Value must be a valid number'
  ): number => {
    if (!isValidNumber(value)) {
      throw new Error(message);
    }
    return value;
  },

  /**
   * Assert value is non-negative
   */
  assertNonNegative: (
    value: number,
    message: string = 'Value must be non-negative'
  ): number => {
    if (value < 0) {
      throw new Error(message);
    }
    return value;
  },

  /**
   * Assert array is non-empty
   */
  assertNonEmpty: <T>(
    arr: T[],
    message: string = 'Array must not be empty'
  ): [T, ...T[]] => {
    if (arr.length === 0) {
      throw new Error(message);
    }
    return arr as [T, ...T[]];
  },

  /**
   * Assert condition is true
   */
  assert: (
    condition: boolean,
    message: string = 'Assertion failed'
  ): void => {
    if (!condition) {
      throw new Error(message);
    }
  },
};

/**
 * Safe parsing helpers that return null on failure
 */
export const SafeParsing = {
  /**
   * Safely parse number
   */
  parseNumber: (value: unknown): number | null => {
    if (typeof value === 'number' && isValidNumber(value)) {
      return value;
    }
    
    if (typeof value === 'string') {
      const parsed = parseFloat(value);
      return isValidNumber(parsed) ? parsed : null;
    }
    
    return null;
  },

  /**
   * Safely parse integer
   */
  parseInteger: (value: unknown): number | null => {
    const num = SafeParsing.parseNumber(value);
    return num !== null && Number.isInteger(num) ? num : null;
  },

  /**
   * Safely parse boolean
   */
  parseBoolean: (value: unknown): boolean | null => {
    if (typeof value === 'boolean') return value;
    if (value === 'true') return true;
    if (value === 'false') return false;
    if (value === 1) return true;
    if (value === 0) return false;
    return null;
  },

  /**
   * Safely parse JSON
   */
  parseJson: <T = unknown>(value: string): T | null => {
    try {
      return JSON.parse(value) as T;
    } catch {
      return null;
    }
  },

  /**
   * Safely parse date
   */
  parseDate: (value: unknown): Date | null => {
    if (value instanceof Date) return value;
    if (typeof value === 'string' || typeof value === 'number') {
      const date = new Date(value);
      return isNaN(date.getTime()) ? null : date;
    }
    return null;
  },
};