/**
 * Mathematical validation hints and helpers
 * 
 * Provides human-readable explanations for mathematical issues,
 * plausibility checks, and consistency validation.
 * 
 * These are HINTS, not errors - they suggest when something might be wrong
 * but never block computation.
 */

import type { ProductionSummary, TimeModel, CycleTimeModel, OeeInput, EconomicParameters } from '../models/input';
import { getInputValue } from '../models/input';
import { MachineState } from '../models/enums';

/**
 * Validation hint severity
 */
export type HintSeverity = 'info' | 'warning' | 'critical';

/**
 * Validation hint structure
 */
export interface ValidationHint {
  field?: string;
  severity: HintSeverity;
  message: string;
  suggestion?: string;
  explanation?: string;
}

/**
 * Mathematical consistency checks
 */
export const MathHints = {
  /**
   * Check if production counts are consistent
   * Good + Scrap + Rework should equal Total
   */
  checkProductionCountConsistency: (
    production: ProductionSummary
  ): ValidationHint | null => {
    const total = getInputValue(production.total_units);
    const good = getInputValue(production.good_units);
    const scrap = getInputValue(production.scrap_units);
    const rework = getInputValue(production.reworked_units);

    const sum = good + scrap + rework;
    const diff = Math.abs(sum - total);

    if (diff > 0) {
      return {
        field: 'production',
        severity: 'critical',
        message: `Count mismatch: ${good} + ${scrap} + ${rework} = ${sum}, but total is ${total}`,
        suggestion: 'Good + Scrap + Rework must equal Total',
        explanation: 'The sum of categorized units must match the total count for accurate OEE calculation.',
      };
    }

    return null;
  },

  /**
   * Check for zero production
   */
  checkZeroProduction: (
    production: ProductionSummary
  ): ValidationHint | null => {
    const total = getInputValue(production.total_units);

    if (total === 0) {
      return {
        field: 'production',
        severity: 'warning',
        message: 'No production units recorded',
        suggestion: 'Enter production counts to calculate quality metrics',
      };
    }

    return null;
  },

  /**
   * Check if good units exceed total
   */
  checkGoodUnitsExceedTotal: (
    production: ProductionSummary
  ): ValidationHint | null => {
    const total = getInputValue(production.total_units);
    const good = getInputValue(production.good_units);

    if (good > total) {
      return {
        field: 'production',
        severity: 'critical',
        message: `Good units (${good}) exceeds total units (${total})`,
        suggestion: 'Good units cannot be greater than total production',
      };
    }

    return null;
  },

  /**
   * Get all production warnings
   */
  checkProductionWarnings: (production: ProductionSummary): ValidationHint[] => {
    const warnings: ValidationHint[] = [];

    const consistencyHint = MathHints.checkProductionCountConsistency(production);
    if (consistencyHint) warnings.push(consistencyHint);

    const zeroHint = MathHints.checkZeroProduction(production);
    if (zeroHint) warnings.push(zeroHint);

    const exceedHint = MathHints.checkGoodUnitsExceedTotal(production);
    if (exceedHint) warnings.push(exceedHint);

    return warnings;
  },

  /**
   * Check if planned production time is valid
   */
  checkPlannedTimeValid: (timeModel: TimeModel): ValidationHint | null => {
    const plannedTime = getInputValue(timeModel.planned_production_time);

    if (plannedTime <= 0) {
      return {
        field: 'time',
        severity: 'critical',
        message: 'Planned production time must be greater than 0',
        suggestion: 'Enter the total available production time in seconds',
      };
    }

    return null;
  },

  /**
   * Check if time allocations sum to planned time
   */
  checkTimeAllocationConsistency: (
    timeModel: TimeModel,
    tolerance: number = 0.01
  ): ValidationHint | null => {
    const planned = getInputValue(timeModel.planned_production_time);
    const allocated = timeModel.allocations.reduce(
      (sum, alloc) => sum + getInputValue(alloc.duration),
      0
    );

    const diff = allocated - planned;

    if (diff > tolerance * planned) {
      return {
        field: 'time',
        severity: 'critical',
        message: `Time allocations (${allocated}s) exceed planned time (${planned}s)`,
        suggestion: 'Reduce allocation durations to match planned production time',
      };
    }

    if (diff < -tolerance * planned) {
      const unallocated = planned - allocated;
      return {
        field: 'time',
        severity: 'warning',
        message: `Unallocated time: ${unallocated} seconds (${(unallocated / 60).toFixed(1)} minutes)`,
        suggestion: 'Consider accounting for all time periods',
      };
    }

    return null;
  },

  /**
   * Get all time warnings
   */
  checkTimeWarnings: (timeModel: TimeModel): ValidationHint[] => {
    const warnings: ValidationHint[] = [];

    const plannedTimeHint = MathHints.checkPlannedTimeValid(timeModel);
    if (plannedTimeHint) warnings.push(plannedTimeHint);

    const allocationHint = MathHints.checkTimeAllocationConsistency(timeModel);
    if (allocationHint) warnings.push(allocationHint);

    return warnings;
  },

  /**
   * Check if ideal cycle time is valid
   */
  checkIdealCycleTimeValid: (cycleTime: CycleTimeModel): ValidationHint | null => {
    const idealCycleTime = getInputValue(cycleTime.ideal_cycle_time);

    if (idealCycleTime <= 0) {
      return {
        field: 'cycle_time',
        severity: 'critical',
        message: 'Ideal cycle time must be greater than 0',
        suggestion: 'Enter the theoretical minimum time per unit from design specs',
      };
    }

    return null;
  },

  /**
   * Check if cycle time is plausible
   */
  checkCycleTimePlausibility: (
    cycleTime: CycleTimeModel,
    totalUnits: number,
    runningTime: number
  ): ValidationHint[] => {
    const hints: ValidationHint[] = [];
    const ideal = getInputValue(cycleTime.ideal_cycle_time);

    // Check if ideal cycle time is realistic (0.1s to 1 hour)
    if (ideal < 0.1) {
      hints.push({
        field: 'cycle_time',
        severity: 'critical',
        message: `Ideal cycle time (${ideal}s) seems too fast to be realistic`,
        suggestion: 'Check if unit is correct - should be seconds per unit',
      });
    }

    if (ideal > 3600) {
      hints.push({
        field: 'cycle_time',
        severity: 'warning',
        message: `Ideal cycle time is ${ideal}s (${(ideal / 60).toFixed(1)} minutes)`,
        suggestion: 'Verify this is correct - unusually long cycle times may indicate an error',
      });
    }

    // Check if average cycle time makes sense given production
    if (totalUnits > 0 && runningTime > 0) {
      const impliedCycleTime = runningTime / totalUnits;
      const average = cycleTime.average_cycle_time
        ? getInputValue(cycleTime.average_cycle_time)
        : null;

      if (average && Math.abs(impliedCycleTime - average) > average * 0.2) {
        hints.push({
          field: 'cycle_time',
          severity: 'warning',
          message: `Average cycle time (${average.toFixed(2)}s) differs significantly from calculated (${impliedCycleTime.toFixed(2)}s)`,
          suggestion: 'Verify running time, unit counts, or average cycle time',
          explanation: 'Running time / Total units should approximately equal average cycle time.',
        });
      }

      // Check if ideal vs actual is reasonable
      if (impliedCycleTime < ideal * 0.8) {
        hints.push({
          field: 'cycle_time',
          severity: 'warning',
          message: `Actual cycle time (${impliedCycleTime.toFixed(2)}s) is faster than ideal (${ideal}s)`,
          suggestion: 'This is unusual - ideal should be the minimum possible. Verify your ideal cycle time.',
        });
      }

      if (impliedCycleTime > ideal * 2) {
        hints.push({
          field: 'cycle_time',
          severity: 'info',
          message: `Actual cycle time is ${((impliedCycleTime / ideal) * 100).toFixed(0)}% of ideal`,
          suggestion: 'Significant performance loss - investigate speed losses',
        });
      }
    }

    return hints;
  },

  /**
   * Get all cycle time warnings
   */
  checkCycleTimeWarnings: (
    cycleTime: CycleTimeModel,
    totalUnits: number,
    runningTime: number
  ): ValidationHint[] => {
    const warnings: ValidationHint[] = [];

    const validHint = MathHints.checkIdealCycleTimeValid(cycleTime);
    if (validHint) warnings.push(validHint);

    warnings.push(...MathHints.checkCycleTimePlausibility(cycleTime, totalUnits, runningTime));

    return warnings;
  },

  /**
   * Check scrap rate
   */
  checkScrapRate: (production: ProductionSummary): ValidationHint | null => {
    const total = getInputValue(production.total_units);
    const scrap = getInputValue(production.scrap_units);

    if (total === 0) return null;

    const scrapRate = scrap / total;

    if (scrapRate > 0.5) {
      return {
        field: 'production',
        severity: 'critical',
        message: `Extremely high scrap rate: ${(scrapRate * 100).toFixed(1)}%`,
        suggestion: 'More than half of production is scrap - verify counts',
      };
    }

    if (scrapRate > 0.2) {
      return {
        field: 'production',
        severity: 'warning',
        message: `High scrap rate: ${(scrapRate * 100).toFixed(1)}%`,
        suggestion: 'Quality is significantly impacting OEE',
      };
    }

    if (scrapRate > 0.1) {
      return {
        field: 'production',
        severity: 'info',
        message: `Moderate scrap rate: ${(scrapRate * 100).toFixed(1)}%`,
        suggestion: 'Room for quality improvement',
      };
    }

    return null;
  },

  /**
   * Check if downtime exceeds planned time
   */
  checkDowntimeExceedance: (
    totalDowntime: number,
    plannedTime: number
  ): ValidationHint | null => {
    if (totalDowntime > plannedTime) {
      return {
        field: 'time',
        severity: 'critical',
        message: `Downtime (${totalDowntime}s) exceeds planned production time (${plannedTime}s)`,
        suggestion: 'Verify time allocations - stopped time cannot exceed total time',
      };
    }

    if (totalDowntime > plannedTime * 0.8) {
      return {
        field: 'time',
        severity: 'warning',
        message: `Downtime is very high: ${((totalDowntime / plannedTime) * 100).toFixed(1)}% of planned time`,
        suggestion: 'This will result in very low availability (<20%). Verify downtime records.',
      };
    }

    return null;
  },

  /**
   * Check if economic parameters are reasonable
   */
  checkEconomicPlausibility: (
    unitPrice: [number, number, number],
    materialCost: [number, number, number]
  ): ValidationHint[] => {
    const hints: ValidationHint[] = [];
    const [, unitPriceCentral] = unitPrice;
    const [, materialCostCentral] = materialCost;

    // Material cost shouldn't exceed unit price
    if (materialCostCentral > unitPriceCentral) {
      hints.push({
        field: 'economics',
        severity: 'warning',
        message: `Material cost (${materialCostCentral}) exceeds unit price (${unitPriceCentral})`,
        suggestion: 'This implies negative margins. Verify economic parameters.',
      });
    }

    // Check uncertainty ranges
    const [lowPrice, centralPrice, highPrice] = unitPrice;
    const priceSpread = (highPrice - lowPrice) / centralPrice;

    if (priceSpread > 0.5) {
      hints.push({
        field: 'economics',
        severity: 'info',
        message: `Unit price uncertainty is high: ±${(priceSpread * 50).toFixed(0)}%`,
        suggestion: 'High uncertainty will produce wide economic impact ranges.',
      });
    }

    return hints;
  },

  /**
   * Check if TEEP parameters are valid
   */
  checkTeepValidity: (
    timeModel: TimeModel
  ): ValidationHint | null => {
    if (!timeModel.all_time) return null;

    const allTime = getInputValue(timeModel.all_time);
    const plannedTime = getInputValue(timeModel.planned_production_time);

    if (plannedTime > allTime) {
      return {
        field: 'time',
        severity: 'critical',
        message: `Planned time (${plannedTime}s) cannot exceed all time (${allTime}s)`,
        suggestion: 'All time should be the 24/7 calendar time for the period',
        explanation: 'TEEP requires all_time >= planned_production_time',
      };
    }

    const utilizationFactor = plannedTime / allTime;
    if (utilizationFactor < 0.2) {
      return {
        field: 'time',
        severity: 'info',
        message: `Very low utilization factor: ${(utilizationFactor * 100).toFixed(1)}%`,
        suggestion: 'Verify planned production time is correct for TEEP calculation',
      };
    }

    return null;
  },

  /**
   * Get all warnings for an OEE input
   */
  checkAllInputWarnings: (input: OeeInput): ValidationHint[] => {
    const warnings: ValidationHint[] = [];

    // Production warnings
    warnings.push(...MathHints.checkProductionWarnings(input.production));

    // Time warnings
    warnings.push(...MathHints.checkTimeWarnings(input.time_model));

    // Cycle time warnings
    const totalUnits: number = getInputValue(input.production.total_units);
    const runningTime: number = input.time_model.allocations
      .filter((a) => a.state === MachineState.Running)
      .reduce((sum, a) => sum + getInputValue(a.duration), 0);
    warnings.push(...MathHints.checkCycleTimeWarnings(input.cycle_time, totalUnits, runningTime));

    // Scrap rate
    const scrapHint = MathHints.checkScrapRate(input.production);
    if (scrapHint) warnings.push(scrapHint);

    // Downtime - FIX: Explicitly type totalDowntime as number
    const totalDowntime: number = input.time_model.allocations
      .filter((a) => a.state === MachineState.Stopped)
      .reduce((sum, a) => sum + getInputValue(a.duration), 0);
    const plannedTime: number = getInputValue(input.time_model.planned_production_time);
    const downtimeHint = MathHints.checkDowntimeExceedance(totalDowntime, plannedTime);
    if (downtimeHint) warnings.push(downtimeHint);

    // TEEP validity
    const teepHint = MathHints.checkTeepValidity(input.time_model);
    if (teepHint) warnings.push(teepHint);

    return warnings;
  },

  /**
   * Get all warnings for an OEE input with economic parameters
   */
  checkAllInputWarningsWithEconomics: (
    input: OeeInput,
    economicParams: EconomicParameters | null
  ): ValidationHint[] => {
    const warnings = MathHints.checkAllInputWarnings(input);

    // Economic plausibility
    if (economicParams) {
      warnings.push(
        ...MathHints.checkEconomicPlausibility(
          economicParams.unit_price,
          economicParams.material_cost
        )
      );
    }

    return warnings;
  },

  /**
   * Collect all hints for a complete input (legacy compatibility)
   */
  collectAllHints: (
    production: ProductionSummary,
    timeModel: TimeModel,
    cycleTime: CycleTimeModel,
    totalDowntime: number,
    economicParams?: {
      unit_price: [number, number, number];
      material_cost: [number, number, number];
    }
  ): ValidationHint[] => {
    const hints: ValidationHint[] = [];

    // Production consistency
    const productionHint = MathHints.checkProductionCountConsistency(production);
    if (productionHint) hints.push(productionHint);

    // Time consistency
    const timeHint = MathHints.checkTimeAllocationConsistency(timeModel);
    if (timeHint) hints.push(timeHint);

    // Cycle time plausibility
    const totalUnits: number = getInputValue(production.total_units);
    const runningTime: number = timeModel.allocations
      .filter((a) => a.state === MachineState.Running)
      .reduce((sum, a) => sum + getInputValue(a.duration), 0);
    hints.push(...MathHints.checkCycleTimePlausibility(cycleTime, totalUnits, runningTime));

    // Scrap rate
    const scrapHint = MathHints.checkScrapRate(production);
    if (scrapHint) hints.push(scrapHint);

    // Downtime
    const plannedTime: number = getInputValue(timeModel.planned_production_time);
    const downtimeHint = MathHints.checkDowntimeExceedance(totalDowntime, plannedTime);
    if (downtimeHint) hints.push(downtimeHint);

    // Zero production
    const zeroProductionHint = MathHints.checkZeroProduction(production);
    if (zeroProductionHint) hints.push(zeroProductionHint);

    // Economic plausibility
    if (economicParams) {
      hints.push(
        ...MathHints.checkEconomicPlausibility(economicParams.unit_price, economicParams.material_cost)
      );
    }

    // TEEP validity
    const teepHint = MathHints.checkTeepValidity(timeModel);
    if (teepHint) hints.push(teepHint);

    return hints;
  },
};

/**
 * Statistical helpers for data quality assessment
 */
export const StatisticalHelpers = {
  /**
   * Calculate coefficient of variation (CV)
   * Measures relative variability
   */
  coefficientOfVariation: (values: number[]): number => {
    if (values.length === 0) return 0;

    const mean = values.reduce((a, b) => a + b, 0) / values.length;
    if (mean === 0) return 0;

    const variance =
      values.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / values.length;
    const stdDev = Math.sqrt(variance);

    return stdDev / mean;
  },

  /**
   * Detect outliers using IQR method
   */
  detectOutliers: (values: number[]): number[] => {
    if (values.length < 4) return [];

    const sorted = [...values].sort((a, b) => a - b);
    const q1Index = Math.floor(sorted.length * 0.25);
    const q3Index = Math.floor(sorted.length * 0.75);

    const q1 = sorted[q1Index];
    const q3 = sorted[q3Index];
    const iqr = q3 - q1;

    const lowerBound = q1 - 1.5 * iqr;
    const upperBound = q3 + 1.5 * iqr;

    return values.filter((v) => v < lowerBound || v > upperBound);
  },

  /**
   * Check if data suggests trend (increasing/decreasing over time)
   */
  detectTrend: (values: number[]): 'increasing' | 'decreasing' | 'stable' => {
    if (values.length < 3) return 'stable';

    let increasing = 0;
    let decreasing = 0;

    for (let i = 1; i < values.length; i++) {
      if (values[i] > values[i - 1]) increasing++;
      if (values[i] < values[i - 1]) decreasing++;
    }

    const threshold = values.length * 0.6;

    if (increasing > threshold) return 'increasing';
    if (decreasing > threshold) return 'decreasing';
    return 'stable';
  },
};