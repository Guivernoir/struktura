/**
 * Mathematical validation hints and helpers
 * 
 * Provides human-readable explanations for mathematical issues,
 * plausibility checks, and consistency validation.
 * 
 * These are HINTS, not errors - they suggest when something might be wrong
 * but never block computation.
 */

import type { ProductionSummary, TimeModel, CycleTimeModel } from '../models';
import { InputValueHelpers } from '../models';

/**
 * Validation hint severity
 */
export enum HintSeverity {
  Info = 'info',
  Warning = 'warning',
  Critical = 'critical',
}

/**
 * Validation hint structure
 */
export interface ValidationHint {
  severity: HintSeverity;
  category: string;
  message: string;
  suggestion?: string;
  affectedFields?: string[];
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
    const total = InputValueHelpers.getValue(production.total_units);
    const good = InputValueHelpers.getValue(production.good_units);
    const scrap = InputValueHelpers.getValue(production.scrap_units);
    const rework = InputValueHelpers.getValue(production.reworked_units);

    const sum = good + scrap + rework;
    const diff = Math.abs(sum - total);

    if (diff > 0) {
      return {
        severity: HintSeverity.Critical,
        category: 'production_counts',
        message: `Production counts don't reconcile: ${good} + ${scrap} + ${rework} = ${sum}, but total is ${total}`,
        suggestion: `Adjust counts so they sum to ${total}, or update total to ${sum}`,
        affectedFields: ['total_units', 'good_units', 'scrap_units', 'reworked_units'],
        explanation: 'Good units + Scrap + Rework must equal Total units for mathematical consistency.',
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
    const planned = InputValueHelpers.getValue(timeModel.planned_production_time);
    const allocated = timeModel.allocations.reduce(
      (sum, alloc) => sum + InputValueHelpers.getValue(alloc.duration),
      0
    );

    const diff = Math.abs(allocated - planned);
    const percentDiff = planned > 0 ? (diff / planned) * 100 : 0;

    if (diff > tolerance * planned) {
      return {
        severity: HintSeverity.Warning,
        category: 'time_allocation',
        message: `Time allocations (${allocated}s) don't match planned time (${planned}s). Difference: ${diff.toFixed(0)}s (${percentDiff.toFixed(1)}%)`,
        suggestion: 'Review time allocations or adjust planned production time',
        affectedFields: ['planned_production_time', 'allocations'],
        explanation: 'Sum of all time allocations should equal planned production time.',
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
    const ideal = InputValueHelpers.getValue(cycleTime.ideal_cycle_time);

    // Check if ideal cycle time is realistic (0.1s to 1 hour)
    if (ideal < 0.1 || ideal > 3600) {
      hints.push({
        severity: ideal < 0.1 ? HintSeverity.Critical : HintSeverity.Warning,
        category: 'cycle_time_plausibility',
        message: `Ideal cycle time (${ideal}s) seems ${ideal < 0.1 ? 'too fast' : 'too slow'} to be realistic`,
        suggestion: ideal < 0.1 
          ? 'Check if unit is correct - should be seconds per unit'
          : 'Verify cycle time - values over 1 hour are unusual',
        affectedFields: ['ideal_cycle_time'],
      });
    }

    // Check if average cycle time makes sense given production
    if (totalUnits > 0 && runningTime > 0) {
      const impliedCycleTime = runningTime / totalUnits;
      const average = cycleTime.average_cycle_time
        ? InputValueHelpers.getValue(cycleTime.average_cycle_time)
        : null;

      if (average && Math.abs(impliedCycleTime - average) > average * 0.2) {
        hints.push({
          severity: HintSeverity.Warning,
          category: 'cycle_time_consistency',
          message: `Average cycle time (${average.toFixed(2)}s) differs significantly from calculated (${impliedCycleTime.toFixed(2)}s)`,
          suggestion: 'Verify running time, unit counts, or average cycle time',
          affectedFields: ['average_cycle_time', 'total_units', 'running_time'],
          explanation: 'Running time / Total units should approximately equal average cycle time.',
        });
      }

      // Check if ideal vs actual is reasonable
      if (impliedCycleTime < ideal * 0.8) {
        hints.push({
          severity: HintSeverity.Warning,
          category: 'performance_implausible',
          message: `Actual cycle time (${impliedCycleTime.toFixed(2)}s) is faster than ideal (${ideal}s)`,
          suggestion: 'This suggests performance >120% which is typically impossible. Check ideal cycle time or counts.',
          affectedFields: ['ideal_cycle_time', 'total_units', 'running_time'],
        });
      }
    }

    return hints;
  },

  /**
   * Check if scrap rate is concerning
   */
  checkScrapRate: (
    production: ProductionSummary,
    warningThreshold: number = 0.05,
    criticalThreshold: number = 0.15
  ): ValidationHint | null => {
    const total = InputValueHelpers.getValue(production.total_units);
    const scrap = InputValueHelpers.getValue(production.scrap_units);

    if (total === 0) return null;

    const scrapRate = scrap / total;

    if (scrapRate >= criticalThreshold) {
      return {
        severity: HintSeverity.Critical,
        category: 'quality_concern',
        message: `Scrap rate is very high: ${(scrapRate * 100).toFixed(1)}% (${scrap}/${total})`,
        suggestion: 'This suggests a serious quality issue. Verify counts or investigate root cause.',
        affectedFields: ['scrap_units', 'total_units'],
      };
    }

    if (scrapRate >= warningThreshold) {
      return {
        severity: HintSeverity.Warning,
        category: 'quality_concern',
        message: `Scrap rate is elevated: ${(scrapRate * 100).toFixed(1)}% (${scrap}/${total})`,
        suggestion: 'Consider investigating quality issues.',
        affectedFields: ['scrap_units', 'total_units'],
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
        severity: HintSeverity.Critical,
        category: 'time_consistency',
        message: `Total downtime (${totalDowntime}s) exceeds planned production time (${plannedTime}s)`,
        suggestion: 'Review downtime records or adjust planned production time',
        affectedFields: ['downtimes', 'planned_production_time'],
        explanation: 'Downtime cannot be greater than the total available time.',
      };
    }

    if (totalDowntime > plannedTime * 0.8) {
      return {
        severity: HintSeverity.Warning,
        category: 'availability_concern',
        message: `Downtime is very high: ${(totalDowntime / plannedTime * 100).toFixed(1)}% of planned time`,
        suggestion: 'This will result in very low availability (<20%). Verify downtime records.',
        affectedFields: ['downtimes'],
      };
    }

    return null;
  },

  /**
   * Check if zero production occurred
   */
  checkZeroProduction: (
    totalUnits: number
  ): ValidationHint | null => {
    if (totalUnits === 0) {
      return {
        severity: HintSeverity.Warning,
        category: 'production_concern',
        message: 'No units produced during analysis period',
        suggestion: 'Verify this is intentional (e.g., all downtime) or check data completeness',
        affectedFields: ['total_units'],
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
        severity: HintSeverity.Warning,
        category: 'economic_plausibility',
        message: `Material cost (${materialCostCentral}) exceeds unit price (${unitPriceCentral})`,
        suggestion: 'This implies negative margins. Verify economic parameters.',
        affectedFields: ['unit_price', 'material_cost'],
      });
    }

    // Check uncertainty ranges
    const [lowPrice, centralPrice, highPrice] = unitPrice;
    const priceSpread = (highPrice - lowPrice) / centralPrice;
    
    if (priceSpread > 0.5) {
      hints.push({
        severity: HintSeverity.Info,
        category: 'economic_uncertainty',
        message: `Unit price uncertainty is high: ±${(priceSpread * 50).toFixed(0)}%`,
        suggestion: 'High uncertainty will produce wide economic impact ranges.',
        affectedFields: ['unit_price'],
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

    const allTime = InputValueHelpers.getValue(timeModel.all_time);
    const plannedTime = InputValueHelpers.getValue(timeModel.planned_production_time);

    if (plannedTime > allTime) {
      return {
        severity: HintSeverity.Critical,
        category: 'teep_configuration',
        message: `Planned time (${plannedTime}s) cannot exceed all time (${allTime}s)`,
        suggestion: 'All time should be the 24/7 calendar time for the period',
        affectedFields: ['planned_production_time', 'all_time'],
        explanation: 'TEEP requires all_time >= planned_production_time',
      };
    }

    const utilizationFactor = plannedTime / allTime;
    if (utilizationFactor < 0.2) {
      return {
        severity: HintSeverity.Info,
        category: 'teep_configuration',
        message: `Very low utilization factor: ${(utilizationFactor * 100).toFixed(1)}%`,
        suggestion: 'Verify planned production time is correct for TEEP calculation',
        affectedFields: ['planned_production_time', 'all_time'],
      };
    }

    return null;
  },

  /**
   * Collect all hints for a complete input
   */
  collectAllHints: (
    production: ProductionSummary,
    timeModel: TimeModel,
    cycleTime: CycleTimeModel,
    totalDowntime: number,
    economicParams?: { unit_price: [number, number, number]; material_cost: [number, number, number] }
  ): ValidationHint[] => {
    const hints: ValidationHint[] = [];

    // Production consistency
    const productionHint = MathHints.checkProductionCountConsistency(production);
    if (productionHint) hints.push(productionHint);

    // Time consistency
    const timeHint = MathHints.checkTimeAllocationConsistency(timeModel);
    if (timeHint) hints.push(timeHint);

    // Cycle time plausibility
    const totalUnits = InputValueHelpers.getValue(production.total_units);
    const runningTime = timeModel.allocations
      .filter(a => a.state === 'Running')
      .reduce((sum, a) => sum + InputValueHelpers.getValue(a.duration), 0);
    hints.push(...MathHints.checkCycleTimePlausibility(cycleTime, totalUnits, runningTime));

    // Scrap rate
    const scrapHint = MathHints.checkScrapRate(production);
    if (scrapHint) hints.push(scrapHint);

    // Downtime
    const plannedTime = InputValueHelpers.getValue(timeModel.planned_production_time);
    const downtimeHint = MathHints.checkDowntimeExceedance(totalDowntime, plannedTime);
    if (downtimeHint) hints.push(downtimeHint);

    // Zero production
    const zeroProductionHint = MathHints.checkZeroProduction(totalUnits);
    if (zeroProductionHint) hints.push(zeroProductionHint);

    // Economic plausibility
    if (economicParams) {
      hints.push(...MathHints.checkEconomicPlausibility(
        economicParams.unit_price,
        economicParams.material_cost
      ));
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
    
    const variance = values.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / values.length;
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
    
    return values.filter(v => v < lowerBound || v > upperBound);
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