/**
 * Unit conversion and handling utilities
 * 
 * Time conversions, rate conversions, and unit validation.
 */

/**
 * Time unit conversions (all to/from seconds)
 */
export const TimeUnits = {
  /**
   * Convert seconds to milliseconds
   */
  secondsToMilliseconds: (seconds: number): number => seconds * 1000,

  /**
   * Convert milliseconds to seconds
   */
  millisecondsToSeconds: (ms: number): number => ms / 1000,

  /**
   * Convert seconds to minutes
   */
  secondsToMinutes: (seconds: number): number => seconds / 60,

  /**
   * Convert minutes to seconds
   */
  minutesToSeconds: (minutes: number): number => minutes * 60,

  /**
   * Convert seconds to hours
   */
  secondsToHours: (seconds: number): number => seconds / 3600,

  /**
   * Convert hours to seconds
   */
  hoursToSeconds: (hours: number): number => hours * 3600,

  /**
   * Convert seconds to days
   */
  secondsToDays: (seconds: number): number => seconds / 86400,

  /**
   * Convert days to seconds
   */
  daysToSeconds: (days: number): number => days * 86400,

  /**
   * Parse duration string to seconds
   * Supports formats like: "1h 30m", "90m", "5400s", "1.5h"
   */
  parseDuration: (duration: string): number | null => {
    const patterns = [
      // Hours, minutes, seconds: "1h 30m 45s"
      /(?:(\d+(?:\.\d+)?)\s*h(?:ours?)?)?(?:\s*(\d+(?:\.\d+)?)\s*m(?:in(?:utes?)?)?)?(?:\s*(\d+(?:\.\d+)?)\s*s(?:ec(?:onds?)?)?)?/i,
    ];

    for (const pattern of patterns) {
      const match = duration.match(pattern);
      if (match) {
        const hours = parseFloat(match[1] || '0');
        const minutes = parseFloat(match[2] || '0');
        const seconds = parseFloat(match[3] || '0');

        return TimeUnits.hoursToSeconds(hours) +
               TimeUnits.minutesToSeconds(minutes) +
               seconds;
      }
    }

    // Try parsing as a plain number (assume seconds)
    const num = parseFloat(duration);
    return isNaN(num) ? null : num;
  },
};

/**
 * Production rate conversions
 */
export const RateUnits = {
  /**
   * Convert units per second to units per minute
   */
  unitsPerSecondToMinute: (rate: number): number => rate * 60,

  /**
   * Convert units per minute to units per second
   */
  unitsPerMinuteToSecond: (rate: number): number => rate / 60,

  /**
   * Convert units per second to units per hour
   */
  unitsPerSecondToHour: (rate: number): number => rate * 3600,

  /**
   * Convert units per hour to units per second
   */
  unitsPerHourToSecond: (rate: number): number => rate / 3600,

  /**
   * Calculate cycle time from production rate
   * Rate is units per second, result is seconds per unit
   */
  rateToCalcycleTime: (rate: number): number => {
    return rate > 0 ? 1 / rate : 0;
  },

  /**
   * Calculate production rate from cycle time
   * Cycle time is seconds per unit, result is units per second
   */
  cycleTimeToRate: (cycleTime: number): number => {
    return cycleTime > 0 ? 1 / cycleTime : 0;
  },

  /**
   * Calculate theoretical capacity
   * Given ideal cycle time and available time
   */
  calculateCapacity: (
    idealCycleTimeSeconds: number,
    availableTimeSeconds: number
  ): number => {
    if (idealCycleTimeSeconds <= 0) return 0;
    return Math.floor(availableTimeSeconds / idealCycleTimeSeconds);
  },

  /**
   * Calculate required cycle time to meet target
   * Given target units and available time
   */
  calculateRequiredCycleTime: (
    targetUnits: number,
    availableTimeSeconds: number
  ): number => {
    if (targetUnits <= 0) return 0;
    return availableTimeSeconds / targetUnits;
  },
};

/**
 * Percentage conversions
 */
export const PercentageUnits = {
  /**
   * Convert decimal to percentage (0.85 → 85)
   */
  decimalToPercentage: (decimal: number): number => decimal * 100,

  /**
   * Convert percentage to decimal (85 → 0.85)
   */
  percentageToDecimal: (percentage: number): number => percentage / 100,

  /**
   * Convert basis points to decimal (850 → 0.085)
   */
  basisPointsToDecimal: (bps: number): number => bps / 10000,

  /**
   * Convert decimal to basis points (0.085 → 850)
   */
  decimalToBasisPoints: (decimal: number): number => decimal * 10000,

  /**
   * Clamp percentage to valid range [0, 1]
   */
  clamp: (value: number): number => Math.max(0, Math.min(1, value)),

  /**
   * Calculate percentage change
   */
  calculateChange: (oldValue: number, newValue: number): number => {
    if (oldValue === 0) return newValue > 0 ? Infinity : 0;
    return (newValue - oldValue) / oldValue;
  },
};

/**
 * Temperature conversions (if needed for equipment monitoring)
 */
export const TemperatureUnits = {
  /**
   * Celsius to Fahrenheit
   */
  celsiusToFahrenheit: (celsius: number): number => (celsius * 9/5) + 32,

  /**
   * Fahrenheit to Celsius
   */
  fahrenheitToCelsius: (fahrenheit: number): number => (fahrenheit - 32) * 5/9,

  /**
   * Celsius to Kelvin
   */
  celsiusToKelvin: (celsius: number): number => celsius + 273.15,

  /**
   * Kelvin to Celsius
   */
  kelvinToCelsius: (kelvin: number): number => kelvin - 273.15,
};

/**
 * Speed/velocity conversions (for machine speed monitoring)
 */
export const SpeedUnits = {
  /**
   * Meters per second to kilometers per hour
   */
  metersPerSecondToKmPerHour: (mps: number): number => mps * 3.6,

  /**
   * Kilometers per hour to meters per second
   */
  kmPerHourToMetersPerSecond: (kmh: number): number => kmh / 3.6,

  /**
   * RPM (rotations per minute) to radians per second
   */
  rpmToRadiansPerSecond: (rpm: number): number => (rpm * 2 * Math.PI) / 60,

  /**
   * Radians per second to RPM
   */
  radiansPerSecondToRpm: (rps: number): number => (rps * 60) / (2 * Math.PI),
};

/**
 * Currency conversions (basic - for more complex, use external service)
 */
export interface CurrencyConversion {
  from: string;
  to: string;
  rate: number;
}

export const CurrencyUnits = {
  /**
   * Convert amount between currencies
   */
  convert: (
    amount: number,
    conversion: CurrencyConversion
  ): number => {
    return amount * conversion.rate;
  },

  /**
   * Format with uncertainty bounds conversion
   */
  convertRange: (
    low: number,
    central: number,
    high: number,
    rate: number
  ): [number, number, number] => {
    return [low * rate, central * rate, high * rate];
  },
};

/**
 * Unit validation utilities
 */
export const UnitValidation = {
  /**
   * Check if duration is valid (non-negative)
   */
  isValidDuration: (seconds: number): boolean => {
    return seconds >= 0 && isFinite(seconds);
  },

  /**
   * Check if count is valid (non-negative integer)
   */
  isValidCount: (count: number): boolean => {
    return count >= 0 && Number.isInteger(count);
  },

  /**
   * Check if percentage is in valid range [0, 1]
   */
  isValidPercentage: (value: number): boolean => {
    return value >= 0 && value <= 1 && isFinite(value);
  },

  /**
   * Check if rate is valid (positive)
   */
  isValidRate: (rate: number): boolean => {
    return rate > 0 && isFinite(rate);
  },

  /**
   * Check if cycle time is physically plausible
   * (between 0.1 seconds and 1 hour)
   */
  isPlausibleCycleTime: (seconds: number): boolean => {
    return seconds >= 0.1 && seconds <= 3600;
  },

  /**
   * Check if time allocation is consistent
   */
  isConsistentTimeAllocation: (
    allocations: number[],
    total: number,
    tolerance: number = 0.01
  ): boolean => {
    const sum = allocations.reduce((a, b) => a + b, 0);
    const diff = Math.abs(sum - total);
    return diff <= tolerance * total;
  },
};

/**
 * Common unit constants
 */
export const UnitConstants = {
  SECONDS_PER_MINUTE: 60,
  SECONDS_PER_HOUR: 3600,
  SECONDS_PER_DAY: 86400,
  SECONDS_PER_WEEK: 604800,
  MINUTES_PER_HOUR: 60,
  HOURS_PER_DAY: 24,
  DAYS_PER_WEEK: 7,
  WEEKS_PER_YEAR: 52,
  
  // Common working schedules
  WORKING_HOURS_PER_SHIFT: 8,
  SHIFTS_PER_DAY: 3,
  WORKING_DAYS_PER_WEEK: 5,
  WORKING_DAYS_PER_YEAR: 260, // Approximate
};

/**
 * Calculate effective working time
 */
export const WorkingTimeCalculations = {
  /**
   * Calculate weekly operating hours
   */
  weeklyOperatingHours: (
    shiftsPerDay: number,
    daysPerWeek: number,
    hoursPerShift: number = UnitConstants.WORKING_HOURS_PER_SHIFT
  ): number => {
    return shiftsPerDay * daysPerWeek * hoursPerShift;
  },

  /**
   * Calculate annual operating hours
   */
  annualOperatingHours: (
    weeklyHours: number,
    weeksPerYear: number = UnitConstants.WEEKS_PER_YEAR
  ): number => {
    return weeklyHours * weeksPerYear;
  },

  /**
   * Calculate 24/7 time for a period
   */
  calculate24x7Time: (durationSeconds: number): number => {
    return durationSeconds; // Already in seconds
  },

  /**
   * Calculate working time from 24/7 time
   */
  calculate24x7ToWorking: (
    fullTimeSeconds: number,
    utilizationFactor: number
  ): number => {
    return fullTimeSeconds * utilizationFactor;
  },
};