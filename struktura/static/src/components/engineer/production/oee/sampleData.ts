/**
 * Sample Data Generator for OEE Calculator
 * 
 * Provides realistic test data for development and demonstration.
 * Place this in: struktura/static/src/components/engineer/production/oee/sampleData.ts
 */

import type { OeeInput, EconomicParameters } from './models';
import { MachineState } from './models/enums';
import { explicit, type ReasonCode } from './models/input';

/**
 * Generate a sample OEE input with realistic production data
 */
export function generateSampleInput(): OeeInput {
  const now = new Date();
  const shiftStart = new Date(now);
  shiftStart.setHours(8, 0, 0, 0);
  
  const shiftEnd = new Date(shiftStart);
  shiftEnd.setHours(16, 0, 0, 0);

  // 8-hour shift = 28800 seconds
  const plannedTime = 28800;
  
  // Time allocations
  const runningTime = 25200; // 7 hours running
  const downtimeTotal = 3600; // 1 hour downtime

  // Production counts
  const totalUnits = 1000;
  const goodUnits = 950;
  const scrapUnits = 30;
  const reworkUnits = 20;

  // Ideal cycle time: 25.2 seconds per unit
  const idealCycleTime = 25.2;

  // Downtime reason codes
  const mechanicalFailure: ReasonCode = {
    path: ["Mechanical", "Bearing Failure"],
    is_failure: true,
  };

  return {
    window: {
      start: shiftStart.toISOString(),
      end: shiftEnd.toISOString(),
    },
    machine: {
      machine_id: "M-001",
      line_id: "Line-A",
      product_id: "WIDGET-X",
      shift_id: "SHIFT-1",
    },
    time_model: {
      planned_production_time: explicit(plannedTime),
      allocations: [
        {
          state: MachineState.Running,
          duration: explicit(runningTime),
          reason: null,
          notes: null,
        },
        {
          state: MachineState.Stopped,
          duration: explicit(downtimeTotal),
          reason: mechanicalFailure,
          notes: "Main spindle bearing replacement required",
        },
      ],
      all_time: explicit(86400), // 24 hours for TEEP calculation
    },
    production: {
      total_units: explicit(totalUnits),
      good_units: explicit(goodUnits),
      scrap_units: explicit(scrapUnits),
      reworked_units: explicit(reworkUnits),
    },
    cycle_time: {
      ideal_cycle_time: explicit(idealCycleTime),
      average_cycle_time: explicit(26.5), // Slightly slower than ideal
    },
    downtimes: {
      records: [
        {
          duration: explicit(3600),
          reason: mechanicalFailure,
          timestamp: new Date(shiftStart.getTime() + 14400000).toISOString(), // 4 hours into shift
          notes: "Main spindle bearing replacement required",
        },
      ],
    },
    thresholds: {
      micro_stoppage_threshold: 30,        // 30 seconds
      small_stop_threshold: 300,           // 5 minutes
      speed_loss_threshold: 0.05,          // 5%
      high_scrap_rate_threshold: 0.20,     // 20%
      low_utilization_threshold: 0.30,     // 30%
    },
  };
}

/**
 * Generate sample economic parameters
 */
export function generateSampleEconomicParams(): EconomicParameters {
  return {
    unit_price: [45.0, 50.0, 55.0],              // Low, Central, High in USD
    marginal_contribution: [20.0, 25.0, 30.0],   // Margin per unit
    material_cost: [15.0, 18.0, 22.0],           // Material cost per unit
    labor_cost_per_hour: [35.0, 40.0, 45.0],     // Labor rate
    currency: "USD",
  };
}

/**
 * Generate high-performance scenario (world-class OEE)
 */
export function generateHighPerformanceInput(): OeeInput {
  const base = generateSampleInput();
  
  return {
    ...base,
    time_model: {
      ...base.time_model,
      allocations: [
        {
          state: MachineState.Running,
          duration: explicit(27000), // 7.5 hours running
          reason: null,
          notes: null,
        },
        {
          state: MachineState.Stopped,
          duration: explicit(1800), // 30 minutes downtime
          reason: {
            path: ["Setup", "Product Changeover"],
            is_failure: false,
          },
          notes: "Planned changeover",
        },
      ],
    },
    production: {
      total_units: explicit(1050),
      good_units: explicit(1040),
      scrap_units: explicit(8),
      reworked_units: explicit(2),
    },
    cycle_time: {
      ideal_cycle_time: explicit(25.2),
      average_cycle_time: explicit(25.7), // Very close to ideal
    },
  };
}

/**
 * Generate problematic scenario (needs improvement)
 */
export function generateProblematicInput(): OeeInput {
  const base = generateSampleInput();
  
  return {
    ...base,
    time_model: {
      ...base.time_model,
      allocations: [
        {
          state: MachineState.Running,
          duration: explicit(21600), // 6 hours running
          reason: null,
          notes: null,
        },
        {
          state: MachineState.Stopped,
          duration: explicit(7200), // 2 hours downtime
          reason: {
            path: ["Mechanical", "Multiple Breakdowns"],
            is_failure: true,
          },
          notes: "Multiple equipment failures throughout shift",
        },
      ],
    },
    production: {
      total_units: explicit(850),
      good_units: explicit(720),
      scrap_units: explicit(100),
      reworked_units: explicit(30),
    },
    cycle_time: {
      ideal_cycle_time: explicit(25.2),
      average_cycle_time: explicit(30.5), // Significantly slower
    },
    downtimes: {
      records: [
        {
          duration: explicit(4800),
          reason: {
            path: ["Mechanical", "Hydraulic Failure"],
            is_failure: true,
          },
          timestamp: base.window.start,
          notes: "Hydraulic system failure",
        },
        {
          duration: explicit(2400),
          reason: {
            path: ["Electrical", "Control Panel"],
            is_failure: true,
          },
          timestamp: base.window.start,
          notes: "Control panel malfunction",
        },
      ],
    },
  };
}

/**
 * Scenario library for demos and testing
 */
export const SAMPLE_SCENARIOS = {
  standard: {
    name: "Standard Shift",
    description: "Typical 8-hour shift with moderate performance",
    input: generateSampleInput,
    economic: generateSampleEconomicParams,
  },
  worldClass: {
    name: "World-Class Performance",
    description: "High availability, performance, and quality (OEE > 85%)",
    input: generateHighPerformanceInput,
    economic: generateSampleEconomicParams,
  },
  problematic: {
    name: "Problematic Shift",
    description: "Multiple issues requiring attention (OEE < 60%)",
    input: generateProblematicInput,
    economic: generateSampleEconomicParams,
  },
};

/**
 * Get a scenario by name
 */
export function getScenario(scenarioName: keyof typeof SAMPLE_SCENARIOS) {
  const scenario = SAMPLE_SCENARIOS[scenarioName];
  return {
    input: scenario.input(),
    economic: scenario.economic(),
  };
}