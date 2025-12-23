/**
 * General utility helpers for OEE calculator
 * 
 * Miscellaneous functions that don't fit into other categories.
 */

import type { InputValue, ReasonCode, LossTreeNode } from '../models';
import { InputValueHelpers } from '../models';

/**
 * Deep clone an object (via JSON serialization)
 * Warning: Loses functions, undefined values, and special objects
 */
export function deepClone<T>(obj: T): T {
  return JSON.parse(JSON.stringify(obj));
}

/**
 * Deep equality check (via JSON serialization)
 */
export function deepEqual<T>(a: T, b: T): boolean {
  return JSON.stringify(a) === JSON.stringify(b);
}

/**
 * Debounce a function
 */
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout | null = null;

  return (...args: Parameters<T>) => {
    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(() => func(...args), wait);
  };
}

/**
 * Throttle a function
 */
export function throttle<T extends (...args: any[]) => any>(
  func: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle = false;

  return (...args: Parameters<T>) => {
    if (!inThrottle) {
      func(...args);
      inThrottle = true;
      setTimeout(() => (inThrottle = false), limit);
    }
  };
}

/**
 * Sleep/delay utility
 */
export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * Retry an async function with exponential backoff
 */
export async function retry<T>(
  fn: () => Promise<T>,
  options: {
    maxAttempts?: number;
    delayMs?: number;
    backoffMultiplier?: number;
    onRetry?: (attempt: number, error: Error) => void;
  } = {}
): Promise<T> {
  const {
    maxAttempts = 3,
    delayMs = 1000,
    backoffMultiplier = 2,
    onRetry = () => {},
  } = options;

  let lastError: Error | null = null;

  for (let attempt = 1; attempt <= maxAttempts; attempt++) {
    try {
      return await fn();
    } catch (error) {
      lastError = error instanceof Error ? error : new Error(String(error));

      if (attempt < maxAttempts) {
        onRetry(attempt, lastError);
        const delay = delayMs * Math.pow(backoffMultiplier, attempt - 1);
        await sleep(delay);
      }
    }
  }

  throw lastError;
}

/**
 * Group array by key function
 */
export function groupBy<T, K extends string | number | symbol>(
  array: T[],
  keyFn: (item: T) => K
): Record<K, T[]> {
  return array.reduce((result, item) => {
    const key = keyFn(item);
    if (!result[key]) {
      result[key] = [];
    }
    result[key].push(item);
    return result;
  }, {} as Record<K, T[]>);
}

/**
 * Sum array of numbers
 */
export function sum(numbers: number[]): number {
  return numbers.reduce((a, b) => a + b, 0);
}

/**
 * Average of array of numbers
 */
export function average(numbers: number[]): number {
  if (numbers.length === 0) return 0;
  return sum(numbers) / numbers.length;
}

/**
 * Median of array of numbers
 */
export function median(numbers: number[]): number {
  if (numbers.length === 0) return 0;

  const sorted = [...numbers].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);

  if (sorted.length % 2 === 0) {
    return (sorted[mid - 1] + sorted[mid]) / 2;
  }

  return sorted[mid];
}

/**
 * Clamp a number between min and max
 */
export function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

/**
 * Linear interpolation
 */
export function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

/**
 * Map a value from one range to another
 */
export function mapRange(
  value: number,
  inMin: number,
  inMax: number,
  outMin: number,
  outMax: number
): number {
  return ((value - inMin) * (outMax - outMin)) / (inMax - inMin) + outMin;
}

/**
 * Round to specified decimal places
 */
export function roundTo(value: number, decimals: number): number {
  const factor = Math.pow(10, decimals);
  return Math.round(value * factor) / factor;
}

/**
 * Check if value is between two numbers (inclusive)
 */
export function between(value: number, min: number, max: number): boolean {
  return value >= min && value <= max;
}

/**
 * Generate a unique ID (simple implementation)
 */
export function generateId(prefix: string = ''): string {
  const timestamp = Date.now().toString(36);
  const randomPart = Math.random().toString(36).substring(2, 9);
  return `${prefix}${prefix ? '-' : ''}${timestamp}-${randomPart}`;
}

/**
 * Format reason code path to readable string
 */
export function formatReasonCodePath(reasonCode: ReasonCode): string {
  if (reasonCode.path.length === 0) return 'Unknown';
  return reasonCode.path.join(' → ');
}

/**
 * Get leaf (most specific) reason from reason code
 */
export function getLeafReason(reasonCode: ReasonCode): string {
  if (reasonCode.path.length === 0) return 'Unknown';
  return reasonCode.path[reasonCode.path.length - 1];
}

/**
 * Get root (top-level) reason from reason code
 */
export function getRootReason(reasonCode: ReasonCode): string {
  if (reasonCode.path.length === 0) return 'Unknown';
  return reasonCode.path[0];
}

/**
 * Flatten a loss tree to array of nodes
 */
export function flattenLossTree(root: LossTreeNode): LossTreeNode[] {
  const result: LossTreeNode[] = [];

  function traverse(node: LossTreeNode) {
    result.push(node);
    node.children.forEach(traverse);
  }

  traverse(root);
  return result;
}

/**
 * Find node in loss tree by category key
 */
export function findLossTreeNode(
  root: LossTreeNode,
  categoryKey: string
): LossTreeNode | null {
  if (root.category_key === categoryKey) return root;

  for (const child of root.children) {
    const found = findLossTreeNode(child, categoryKey);
    if (found) return found;
  }

  return null;
}

/**
 * Calculate percentage with safe division
 */
export function safePercentage(numerator: number, denominator: number): number {
  if (denominator === 0) return 0;
  return (numerator / denominator) * 100;
}

/**
 * Calculate ratio with safe division
 */
export function safeRatio(numerator: number, denominator: number): number {
  if (denominator === 0) return 0;
  return numerator / denominator;
}

/**
 * Batch array into chunks
 */
export function chunk<T>(array: T[], size: number): T[][] {
  const chunks: T[][] = [];
  for (let i = 0; i < array.length; i += size) {
    chunks.push(array.slice(i, i + size));
  }
  return chunks;
}

/**
 * Remove duplicates from array
 */
export function unique<T>(array: T[]): T[] {
  return Array.from(new Set(array));
}

/**
 * Remove duplicates from array based on key function
 */
export function uniqueBy<T, K>(array: T[], keyFn: (item: T) => K): T[] {
  const seen = new Set<K>();
  return array.filter(item => {
    const key = keyFn(item);
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

/**
 * Sort array by key function
 */
export function sortBy<T>(
  array: T[],
  keyFn: (item: T) => number | string,
  order: 'asc' | 'desc' = 'asc'
): T[] {
  const sorted = [...array].sort((a, b) => {
    const aKey = keyFn(a);
    const bKey = keyFn(b);

    if (aKey < bKey) return order === 'asc' ? -1 : 1;
    if (aKey > bKey) return order === 'asc' ? 1 : -1;
    return 0;
  });

  return sorted;
}

/**
 * Partition array into two arrays based on predicate
 */
export function partition<T>(
  array: T[],
  predicate: (item: T) => boolean
): [T[], T[]] {
  const pass: T[] = [];
  const fail: T[] = [];

  for (const item of array) {
    if (predicate(item)) {
      pass.push(item);
    } else {
      fail.push(item);
    }
  }

  return [pass, fail];
}

/**
 * Pick properties from object
 */
export function pick<T extends object, K extends keyof T>(
  obj: T,
  keys: K[]
): Pick<T, K> {
  const result = {} as Pick<T, K>;
  for (const key of keys) {
    if (key in obj) {
      result[key] = obj[key];
    }
  }
  return result;
}

/**
 * Omit properties from object
 */
export function omit<T extends object, K extends keyof T>(
  obj: T,
  keys: K[]
): Omit<T, K> {
  const result = { ...obj };
  for (const key of keys) {
    delete result[key];
  }
  return result as Omit<T, K>;
}

/**
 * Check if object is empty
 */
export function isEmpty(obj: object): boolean {
  return Object.keys(obj).length === 0;
}

/**
 * Merge objects deeply (simple implementation)
 */
export function deepMerge<T extends object>(target: T, ...sources: Partial<T>[]): T {
  if (!sources.length) return target;

  const source = sources.shift();
  if (!source) return target;

  for (const key in source) {
    const sourceValue = source[key];
    const targetValue = target[key];

    if (
      sourceValue &&
      typeof sourceValue === 'object' &&
      !Array.isArray(sourceValue) &&
      targetValue &&
      typeof targetValue === 'object' &&
      !Array.isArray(targetValue)
    ) {
      target[key] = deepMerge({ ...targetValue }, sourceValue) as any;
    } else {
      target[key] = sourceValue as any;
    }
  }

  return deepMerge(target, ...sources);
}

/**
 * Create a download link for data
 */
export function downloadData(
  data: string | Blob,
  filename: string,
  mimeType: string = 'application/octet-stream'
): void {
  const blob = typeof data === 'string' ? new Blob([data], { type: mimeType }) : data;
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
}

/**
 * Copy text to clipboard
 */
export async function copyToClipboard(text: string): Promise<boolean> {
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch {
    // Fallback for older browsers
    const textarea = document.createElement('textarea');
    textarea.value = text;
    textarea.style.position = 'fixed';
    textarea.style.opacity = '0';
    document.body.appendChild(textarea);
    textarea.select();
    const success = document.execCommand('copy');
    document.body.removeChild(textarea);
    return success;
  }
}

/**
 * Compare InputValue wrappers for equality
 */
export function inputValueEquals<T>(a: InputValue<T>, b: InputValue<T>): boolean {
  return a.type === b.type && deepEqual(a.value, b.value);
}

/**
 * Extract all values from an array of InputValues
 */
export function extractInputValues<T>(inputs: InputValue<T>[]): T[] {
  return inputs.map(input => InputValueHelpers.getValue(input));
}

/**
 * Count InputValues by source type
 */
export function countInputValuesBySource<T>(
  inputs: InputValue<T>[]
): { explicit: number; inferred: number; default: number } {
  const counts = { explicit: 0, inferred: 0, default: 0 };

  for (const input of inputs) {
    if (InputValueHelpers.isExplicit(input)) counts.explicit++;
    else if (InputValueHelpers.isInferred(input)) counts.inferred++;
    else if (InputValueHelpers.isDefault(input)) counts.default++;
  }

  return counts;
}