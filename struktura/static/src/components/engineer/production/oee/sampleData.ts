/**
 * Sample Data Generator for OEE Calculator
 * 
 * Provides realistic test data for development and demonstration.
 * All types imported from models - no structure duplication.
 */

import type { OeeInput, EconomicParameters, ReasonCode } from './models';
import { MachineState, InputValueHelpers } from './models';

const { explicit } = InputValueHelpers;

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
    isFailure: true,
  };

  return {
    window: {
      start: shiftStart.toISOString(),
      end: shiftEnd.toISOString(),
    },
    machine: {
      machineId: "M-001",
      lineId: "Line-A",
      productId: "WIDGET-X",
      shiftId: "SHIFT-1",
    },
    timeModel: {
      plannedProductionTime: explicit(plannedTime),
      allocations: [
        {
          state: MachineState.Running,
          duration: explicit(runningTime),
          reason: undefined,
          notes: undefined,
        },
        {
          state: MachineState.Stopped,
          duration: explicit(downtimeTotal),
          reason: mechanicalFailure,
          notes: "Main spindle bearing replacement required",
        },
      ],
      allTime: explicit(86400), // 24 hours for TEEP calculation
    },
    production: {
      totalUnits: explicit(totalUnits),
      goodUnits: explicit(goodUnits),
      scrapUnits: explicit(scrapUnits),
      reworkedUnits: explicit(reworkUnits),
    },
    cycleTime: {
      idealCycleTime: explicit(idealCycleTime),
      averageCycleTime: explicit(26.5), // Slightly slower than ideal
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
      microStoppageThreshold: 30,        // 30 seconds
      smallStopThreshold: 300,           // 5 minutes
      speedLossThreshold: 0.05,          // 5%
      highScrapRateThreshold: 0.20,      // 20%
      lowUtilizationThreshold: 0.30,     // 30%
    },
  };
}

/**
 * Generate sample economic parameters
 */
export function generateSampleEconomicParams(): EconomicParameters {
  return {
    unitPrice: [45.0, 50.0, 55.0],              // Low, Central, High in USD
    marginalContribution: [20.0, 25.0, 30.0],   // Margin per unit
    materialCost: [15.0, 18.0, 22.0],           // Material cost per unit
    laborCostPerHour: [35.0, 40.0, 45.0],       // Labor rate
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
    timeModel: {
      ...base.timeModel,
      allocations: [
        {
          state: MachineState.Running,
          duration: explicit(27000), // 7.5 hours running
          reason: undefined,
          notes: undefined,
        },
        {
          state: MachineState.Stopped,
          duration: explicit(1800), // 30 minutes downtime
          reason: {
            path: ["Setup", "Product Changeover"],
            isFailure: false,
          },
          notes: "Planned changeover",
        },
      ],
    },
    production: {
      totalUnits: explicit(1050),
      goodUnits: explicit(1040),
      scrapUnits: explicit(8),
      reworkedUnits: explicit(2),
    },
    cycleTime: {
      idealCycleTime: explicit(25.2),
      averageCycleTime: explicit(25.7), // Very close to ideal
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
    timeModel: {
      ...base.timeModel,
      allocations: [
        {
          state: MachineState.Running,
          duration: explicit(21600), // 6 hours running
          reason: undefined,
          notes: undefined,
        },
        {
          state: MachineState.Stopped,
          duration: explicit(7200), // 2 hours downtime
          reason: {
            path: ["Mechanical", "Multiple Breakdowns"],
            isFailure: true,
          },
          notes: "Multiple equipment failures throughout shift",
        },
      ],
    },
    production: {
      totalUnits: explicit(850),
      goodUnits: explicit(720),
      scrapUnits: explicit(100),
      reworkedUnits: explicit(30),
    },
    cycleTime: {
      idealCycleTime: explicit(25.2),
      averageCycleTime: explicit(30.5), // Significantly slower
    },
    downtimes: {
      records: [
        {
          duration: explicit(4800),
          reason: {
            path: ["Mechanical", "Hydraulic Failure"],
            isFailure: true,
          },
          timestamp: base.window.start,
          notes: "Hydraulic system failure",
        },
        {
          duration: explicit(2400),
          reason: {
            path: ["Electrical", "Control Panel"],
            isFailure: true,
          },
          timestamp: base.window.start,
          notes: "Control panel malfunction",
        },
      ],
    },
  };
}

/**
 * Scenario descriptor for demo purposes
 */
export interface SampleScenario {
  name: string;
  description: string;
  input: () => OeeInput;
  economic: () => EconomicParameters;
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
} as const satisfies Record<string, SampleScenario>;

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