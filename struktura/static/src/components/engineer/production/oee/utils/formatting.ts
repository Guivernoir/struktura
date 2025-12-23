/**
 * Formatting utilities for OEE calculator
 * 
 * All formatting functions for display purposes.
 * Handles numbers, durations, percentages, currency, and more.
 */

import type { TrackedMetric, Confidence } from '../models';

/**
 * Format options for numbers
 */
export interface NumberFormatOptions {
  decimals?: number;
  locale?: string;
  useGrouping?: boolean;
  signDisplay?: 'auto' | 'never' | 'always' | 'exceptZero';
}

/**
 * Format a number with specified options
 */
export function formatNumber(
  value: number,
  options: NumberFormatOptions = {}
): string {
  const {
    decimals = 2,
    locale = 'en-US',
    useGrouping = true,
    signDisplay = 'auto',
  } = options;

  return new Intl.NumberFormat(locale, {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
    useGrouping,
    signDisplay,
  }).format(value);
}

/**
 * Format a percentage (0.0 - 1.0 → 0% - 100%)
 */
export function formatPercentage(
  value: number,
  decimals: number = 1
): string {
  return `${(value * 100).toFixed(decimals)}%`;
}

/**
 * Format a percentage from a TrackedMetric
 */
export function formatMetricPercentage(
  metric: TrackedMetric,
  decimals: number = 1
): string {
  return formatPercentage(metric.value, decimals);
}

/**
 * Format duration in seconds to human-readable string
 * 
 * Examples:
 * - 45 → "45s"
 * - 150 → "2m 30s"
 * - 3665 → "1h 1m 5s"
 * - 90000 → "1d 1h 0m 0s"
 */
export function formatDuration(
  seconds: number,
  options: {
    compact?: boolean;
    showSeconds?: boolean;
    maxParts?: number;
  } = {}
): string {
  const {
    compact = false,
    showSeconds = true,
    maxParts = 4,
  } = options;

  if (seconds === 0) return compact ? '0s' : '0 seconds';

  const parts: string[] = [];
  let remaining = Math.abs(seconds);

  const days = Math.floor(remaining / 86400);
  remaining -= days * 86400;

  const hours = Math.floor(remaining / 3600);
  remaining -= hours * 3600;

  const minutes = Math.floor(remaining / 60);
  remaining -= minutes * 60;

  const secs = Math.floor(remaining);

  if (days > 0) {
    parts.push(compact ? `${days}d` : `${days} day${days !== 1 ? 's' : ''}`);
  }

  if (hours > 0 || (days > 0 && (minutes > 0 || secs > 0))) {
    parts.push(compact ? `${hours}h` : `${hours} hour${hours !== 1 ? 's' : ''}`);
  }

  if (minutes > 0 || (hours > 0 && secs > 0)) {
    parts.push(compact ? `${minutes}m` : `${minutes} minute${minutes !== 1 ? 's' : ''}`);
  }

  if (showSeconds && (secs > 0 || parts.length === 0)) {
    parts.push(compact ? `${secs}s` : `${secs} second${secs !== 1 ? 's' : ''}`);
  }

  // Apply maxParts limit
  const limitedParts = parts.slice(0, maxParts);

  return (seconds < 0 ? '-' : '') + limitedParts.join(compact ? ' ' : ', ');
}

/**
 * Format duration as hours (decimal)
 * 
 * Example: 5400 seconds → "1.50 hours"
 */
export function formatDurationAsHours(
  seconds: number,
  decimals: number = 2
): string {
  const hours = seconds / 3600;
  return `${hours.toFixed(decimals)} hours`;
}

/**
 * Format duration as minutes (decimal)
 * 
 * Example: 150 seconds → "2.50 minutes"
 */
export function formatDurationAsMinutes(
  seconds: number,
  decimals: number = 2
): string {
  const minutes = seconds / 60;
  return `${minutes.toFixed(decimals)} minutes`;
}

/**
 * Format currency with proper symbol and formatting
 */
export function formatCurrency(
  value: number,
  currency: string = 'USD',
  decimals: number = 2,
  locale: string = 'en-US'
): string {
  return new Intl.NumberFormat(locale, {
    style: 'currency',
    currency: currency,
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(value);
}

/**
 * Format large numbers with K/M/B suffixes
 * 
 * Examples:
 * - 1500 → "1.5K"
 * - 2500000 → "2.5M"
 * - 3000000000 → "3.0B"
 */
export function formatLargeNumber(
  value: number,
  decimals: number = 1
): string {
  const absValue = Math.abs(value);

  if (absValue >= 1e9) {
    return `${(value / 1e9).toFixed(decimals)}B`;
  }

  if (absValue >= 1e6) {
    return `${(value / 1e6).toFixed(decimals)}M`;
  }

  if (absValue >= 1e3) {
    return `${(value / 1e3).toFixed(decimals)}K`;
  }

  return value.toFixed(decimals);
}

/**
 * Format a date/time string
 */
export function formatDateTime(
  isoString: string,
  options: {
    dateStyle?: 'full' | 'long' | 'medium' | 'short';
    timeStyle?: 'full' | 'long' | 'medium' | 'short';
    locale?: string;
  } = {}
): string {
  const {
    dateStyle = 'medium',
    timeStyle = 'short',
    locale = 'en-US',
  } = options;

  const date = new Date(isoString);

  return new Intl.DateTimeFormat(locale, {
    dateStyle,
    timeStyle,
  }).format(date);
}

/**
 * Format a date only (no time)
 */
export function formatDate(
  isoString: string,
  locale: string = 'en-US'
): string {
  const date = new Date(isoString);

  return new Intl.DateTimeFormat(locale, {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  }).format(date);
}

/**
 * Format a time only (no date)
 */
export function formatTime(
  isoString: string,
  locale: string = 'en-US'
): string {
  const date = new Date(isoString);

  return new Intl.DateTimeFormat(locale, {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  }).format(date);
}

/**
 * Format a relative time (e.g., "2 minutes ago")
 */
export function formatRelativeTime(
  isoString: string,
  locale: string = 'en-US'
): string {
  const date = new Date(isoString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSeconds = Math.floor(diffMs / 1000);

  const rtf = new Intl.RelativeTimeFormat(locale, { numeric: 'auto' });

  if (diffSeconds < 60) {
    return rtf.format(-diffSeconds, 'second');
  }

  const diffMinutes = Math.floor(diffSeconds / 60);
  if (diffMinutes < 60) {
    return rtf.format(-diffMinutes, 'minute');
  }

  const diffHours = Math.floor(diffMinutes / 60);
  if (diffHours < 24) {
    return rtf.format(-diffHours, 'hour');
  }

  const diffDays = Math.floor(diffHours / 24);
  if (diffDays < 30) {
    return rtf.format(-diffDays, 'day');
  }

  const diffMonths = Math.floor(diffDays / 30);
  if (diffMonths < 12) {
    return rtf.format(-diffMonths, 'month');
  }

  const diffYears = Math.floor(diffMonths / 12);
  return rtf.format(-diffYears, 'year');
}

/**
 * Format count with optional singular/plural forms
 */
export function formatCount(
  count: number,
  singular: string,
  plural?: string
): string {
  const pluralForm = plural || `${singular}s`;
  return `${count} ${count === 1 ? singular : pluralForm}`;
}

/**
 * Format ratio as "X:Y" (e.g., "3:1")
 */
export function formatRatio(
  numerator: number,
  denominator: number,
  simplify: boolean = true
): string {
  if (denominator === 0) return 'N/A';

  if (simplify) {
    // Find GCD to simplify ratio
    const gcd = (a: number, b: number): number => (b === 0 ? a : gcd(b, a % b));
    const divisor = gcd(Math.abs(numerator), Math.abs(denominator));
    numerator /= divisor;
    denominator /= divisor;
  }

  return `${numerator}:${denominator}`;
}

/**
 * Format confidence level with emoji/icon
 */
export function formatConfidence(confidence: Confidence): string {
  const icons = {
    High: '🟢',
    Medium: '🟡',
    Low: '🔴',
  };

  return `${icons[confidence]} ${confidence}`;
}

/**
 * Format a range (low, central, high)
 */
export function formatRange(
  low: number,
  central: number,
  high: number,
  formatter: (n: number) => string = (n) => n.toFixed(2)
): string {
  return `${formatter(low)} - ${formatter(central)} - ${formatter(high)}`;
}

/**
 * Format economic impact range
 */
export function formatEconomicRange(
  low: number,
  central: number,
  high: number,
  currency: string = 'USD'
): string {
  return formatRange(
    low,
    central,
    high,
    (n) => formatCurrency(n, currency)
  );
}

/**
 * Truncate text with ellipsis
 */
export function truncate(
  text: string,
  maxLength: number,
  ellipsis: string = '...'
): string {
  if (text.length <= maxLength) return text;
  return text.slice(0, maxLength - ellipsis.length) + ellipsis;
}

/**
 * Format file size in bytes to human-readable
 */
export function formatFileSize(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let size = bytes;
  let unitIndex = 0;

  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }

  return `${size.toFixed(2)} ${units[unitIndex]}`;
}