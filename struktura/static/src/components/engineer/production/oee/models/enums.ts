/**
 * Enums and constants for the OEE calculator
 * Mirrored from the Rust backend modules
 */

/**
 * Machine operational states
 * Matches enum MachineState in mod.rs
 */
export enum MachineState {
  Running = 'Running',
  Stopped = 'Stopped',
  Setup = 'Setup',
  Starved = 'Starved',
  Blocked = 'Blocked',
  Maintenance = 'Maintenance',
  Unknown = 'Unknown',
}

/**
 * Represents how a value was derived
 * Matches enum ValueSource in mod.rs
 */
export enum ValueSource {
  Explicit = 'Explicit', // User provided
  Inferred = 'Inferred', // Calculated from other inputs
  Default = 'Default',   // System default used
}

/**
 * Confidence level based on input quality
 * Matches enum Confidence in mod.rs
 */
export enum Confidence {
  High = 'High',    // All inputs explicit
  Medium = 'Medium',// Mix of explicit/inferred
  Low = 'Low',      // Significant defaults used
}

/**
 * Severity levels for validation issues
 * Matches enum Severity in validation module
 */
export enum ValidationSeverity {
  Fatal = 'Fatal',     // Blocks calculation
  Warning = 'Warning', // Suggests data quality issue
  Info = 'Info',       // Informational only
}

/**
 * Severity for business logic warnings in the ledger
 * Matches enum WarningSeverity in ledger module
 */
export enum LedgerWarningSeverity {
  High = 'High',
  Medium = 'Medium',
  Low = 'Low',
}

/**
 * Impact level of an assumption on results
 * Matches enum ImpactLevel in ledger module
 */
export enum ImpactLevel {
  Critical = 'Critical', // Changes significantly affect OEE
  High = 'High',         // Material impact
  Medium = 'Medium',     // Moderate impact
  Low = 'Low',           // Minor impact
  Info = 'Info',         // Informational only
}

/**
 * Helper to get translation key for machine state
 */
export const machineStateTranslationKey = (state: MachineState): string => {
  const keyMap: Record<MachineState, string> = {
    [MachineState.Running]: 'state.running',
    [MachineState.Stopped]: 'state.stopped',
    [MachineState.Setup]: 'state.setup',
    [MachineState.Starved]: 'state.starved',
    [MachineState.Blocked]: 'state.blocked',
    [MachineState.Maintenance]: 'state.maintenance',
    [MachineState.Unknown]: 'state.unknown',
  };
  return keyMap[state] || 'state.unknown';
};