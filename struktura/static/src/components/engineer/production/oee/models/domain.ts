/**
 * OEE Domain Layer - Data Structures
 * 
 * Where assumptions become insights. Every calculation traceable,
 * every metric explainable. Attribution only - no causality claims.
 */

/**
 * Represents how a value was derived
 */
export type ValueSource = 'explicit' | 'inferred' | 'default';

/**
 * Confidence level based on input quality
 */
export type Confidence = 'high' | 'medium' | 'low';

/**
 * A calculated metric with full traceability
 */
export interface TrackedMetric {
  nameKey: string;
  value: number;
  unitKey: string;
  formulaKey: string;
  formulaParams: Record<string, number>;
  confidence: Confidence;
}

/**
 * A time-based loss allocation
 */
export interface LossAllocation {
  categoryKey: string;
  duration: number; // seconds
  percentage: number;
  subAllocations: LossAllocation[];
}

/**
 * Economic impact with uncertainty bounds
 */
export interface EconomicImpact {
  descriptionKey: string;
  lowEstimate: number;
  centralEstimate: number;
  highEstimate: number;
  currency: string;
  assumptions: string[];
}

/**
 * Core OEE metrics bundle
 */
export interface CoreMetrics {
  availability: TrackedMetric;
  performance: TrackedMetric;
  quality: TrackedMetric;
  oee: TrackedMetric;
}

/**
 * Extended metrics beyond core OEE
 */
export interface ExtendedMetrics {
  teep?: TrackedMetric;
  utilization: TrackedMetric;
  mtbf?: TrackedMetric;
  mttr?: TrackedMetric;
  scrapRate: TrackedMetric;
  reworkRate: TrackedMetric;
  netOperatingTime: TrackedMetric;
}

/**
 * A node in the loss tree (hierarchical structure)
 */
export interface LossTreeNode {
  categoryKey: string;
  descriptionKey: string;
  duration: number; // seconds
  percentageOfPlanned: number;
  percentageOfParent?: number;
  children: LossTreeNode[];
  source: ValueSource;
}

/**
 * Complete loss tree structure
 */
export interface LossTree {
  root: LossTreeNode;
  plannedTime: number; // seconds
}

/**
 * Economic parameters with uncertainty
 */
export interface EconomicParameters {
  /** (low, central, high) */
  unitPrice: [number, number, number];
  /** (low, central, high) */
  marginalContribution: [number, number, number];
  /** (low, central, high) */
  materialCost: [number, number, number];
  /** (low, central, high) */
  laborCostPerHour: [number, number, number];
  currency: string;
}

/**
 * Complete economic analysis
 */
export interface EconomicAnalysis {
  throughputLoss: EconomicImpact;
  materialWaste: EconomicImpact;
  reworkCost: EconomicImpact;
  opportunityCost: EconomicImpact;
  totalImpact: EconomicImpact;
}