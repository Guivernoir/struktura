# OEE Calculator Models

TypeScript type definitions that **exactly mirror** the Rust backend structures.

## Overview

This folder contains all type definitions for the OEE calculator component:

- **`enums.ts`** - All enum types (MachineState, Confidence, ValueSource, etc.)
- **`assumptions.ts`** - Input value types and assumption structures
- **`input.ts`** - Complete input structure (`OeeInput`)
- **`output.ts`** - Complete output structure (`OeeResult`)
- **`index.ts`** - Convenient re-exports

## Philosophy

These types are designed with **traceability** as the core principle:

1. Every value has a **source** (Explicit/Inferred/Default)
2. Every metric is **traceable** to its inputs
3. Every assumption is **auditable** via the ledger

## Key Types

### InputValue<T>

The foundation of trust. Every input knows how it was obtained:

```typescript
type InputValue<T> =
  | { type: "Explicit"; value: T } // User provided
  | { type: "Inferred"; value: T } // Calculated from other inputs
  | { type: "Default"; value: T }; // System default

// Usage
import { InputValueHelpers } from "./models";

const explicitValue = InputValueHelpers.explicit(100);
const inferredValue = InputValueHelpers.inferred(95);
const defaultValue = InputValueHelpers.default(0);

// Get the actual value
const value = InputValueHelpers.getValue(explicitValue); // 100

// Check the source
if (InputValueHelpers.isExplicit(explicitValue)) {
  console.log("User provided this value");
}
```

### OeeInput

The complete input structure sent to the backend:

```typescript
const input: OeeInput = {
  window: {
    start: "2024-01-01T08:00:00Z",
    end: "2024-01-01T16:00:00Z",
  },
  machine: {
    machine_id: "M-001",
    line_id: "Line-A",
  },
  time_model: {
    planned_production_time: InputValueHelpers.explicit(28800), // 8 hours in seconds
    allocations: [
      {
        state: MachineState.Running,
        duration: InputValueHelpers.explicit(25200), // 7 hours
      },
      {
        state: MachineState.Stopped,
        duration: InputValueHelpers.explicit(3600), // 1 hour
        reason: {
          path: ["Mechanical", "Bearing Failure"],
          is_failure: true,
        },
      },
    ],
  },
  production: {
    total_units: InputValueHelpers.explicit(1000),
    good_units: InputValueHelpers.explicit(950),
    scrap_units: InputValueHelpers.explicit(30),
    reworked_units: InputValueHelpers.explicit(20),
  },
  cycle_time: {
    ideal_cycle_time: InputValueHelpers.explicit(25.2), // seconds per unit
  },
  downtimes: {
    records: [
      {
        duration: InputValueHelpers.explicit(3600),
        reason: {
          path: ["Mechanical", "Bearing Failure"],
          is_failure: true,
        },
        notes: "Main spindle bearing replacement required",
      },
    ],
  },
  thresholds: DEFAULT_THRESHOLDS,
};
```

### OeeResult

The complete output structure returned by the backend:

```typescript
const result: OeeResult = {
  core_metrics: {
    availability: {
      name_key: "metrics.availability",
      value: 0.875,
      unit_key: "units.percentage",
      formula_key: "formulas.availability",
      formula_params: {
        /* ... */
      },
      confidence: Confidence.High,
    },
    performance: {
      /* ... */
    },
    quality: {
      /* ... */
    },
    oee: {
      /* ... */
    },
  },
  extended_metrics: {
    /* ... */
  },
  loss_tree: {
    /* ... */
  },
  ledger: {
    analysis_timestamp: "2024-01-01T16:00:00Z",
    assumptions: [
      {
        assumption_key: "input.planned_time",
        description_key: "assumptions.planned_time_desc",
        value: 28800,
        source: "explicit",
        timestamp: "2024-01-01T16:00:00Z",
        impact: ImpactLevel.Critical,
        related_assumptions: [],
      },
      // ... more assumptions
    ],
    warnings: [
      /* ... */
    ],
    thresholds: [
      /* ... */
    ],
    source_statistics: {
      explicit_count: 8,
      inferred_count: 2,
      default_count: 0,
      total_count: 10,
      explicit_percentage: 80.0,
      inferred_percentage: 20.0,
      default_percentage: 0.0,
    },
    metadata: {},
  },
  validation: {
    is_valid: true,
    errors: [],
    warnings: ["High scrap rate detected: 3.0%"],
  },
};

// Usage
import { formatPercentage } from "./models";

console.log(formatPercentage(result.core_metrics.oee)); // "78.5%"
```

## Helper Functions

### InputValueHelpers

Utilities for working with `InputValue<T>`:

```typescript
InputValueHelpers.explicit(value); // Create explicit value
InputValueHelpers.inferred(value); // Create inferred value
InputValueHelpers.default(value); // Create default value
InputValueHelpers.getValue(input); // Extract value
InputValueHelpers.isExplicit(input); // Check if explicit
InputValueHelpers.sourceType(input); // Get source as string
InputValueHelpers.map(input, fn); // Transform value
```

### Formatting Helpers

```typescript
formatPercentage(metric, decimals?)   // Format as percentage
formatDuration(seconds)                // Format duration (e.g., "2h 30m 15s")
formatCurrency(value, currency)        // Format monetary value
getOeeColorClass(value)                // Get Tailwind color class for OEE
getConfidenceColorClass(confidence)    // Get badge color for confidence
```

### Constants

```typescript
DEFAULT_THRESHOLDS; // Industry standard thresholds
STRICT_THRESHOLDS; // Aggressive thresholds
LENIENT_THRESHOLDS; // Relaxed thresholds
```

## Type Safety

All types include type guards for runtime validation:

```typescript
import { isValidOeeInput, isValidOeeResult } from "./models";

if (isValidOeeInput(data)) {
  // TypeScript knows data is OeeInput
  sendToBackend(data);
}

if (isValidOeeResult(response)) {
  // TypeScript knows response is OeeResult
  displayResults(response);
}
```

## Duration Handling

**Important:** All durations are in **seconds** (not milliseconds).

```typescript
// Rust: Duration::from_secs(3600)
// TypeScript: 3600

const oneHour = 3600; // seconds
const oneDay = 24 * 60 * 60; // 86400 seconds
```

## Datetime Handling

All datetimes are **ISO 8601 strings**:

```typescript
// Good
const timestamp = "2024-01-01T16:00:00Z";

// Bad
const timestamp = new Date(); // Use .toISOString() instead
```

## Design Principles

1. **Exact mirror of Rust types** - No creative interpretations
2. **Explicit typing everywhere** - No `any`, minimal `unknown`
3. **Discriminated unions** - Type-safe enums as union types
4. **Immutable by convention** - No mutations expected
5. **Self-documenting** - JSDoc comments on everything

## Common Patterns

### Creating Input Values

```typescript
// Explicit (user provided)
const units = InputValueHelpers.explicit(1000);

// Inferred (calculated)
const avgCycleTime = InputValueHelpers.inferred(totalTime / totalUnits);

// Default (fallback)
const scrap = InputValueHelpers.default(0);
```

### Checking Confidence

```typescript
const metric = result.core_metrics.oee;

switch (metric.confidence) {
  case Confidence.High:
    // All inputs were explicit
    break;
  case Confidence.Medium:
    // Mix of explicit/inferred
    break;
  case Confidence.Low:
    // Significant defaults used
    break;
}
```

### Navigating the Loss Tree

```typescript
const tree = result.loss_tree;

function traverseTree(node: LossTreeNode, depth: number = 0) {
  console.log(`${"  ".repeat(depth)}${node.category_key}: ${node.duration}s`);

  for (const child of node.children) {
    traverseTree(child, depth + 1);
  }
}

traverseTree(tree.root);
```

## Notes

- These types are **read-only** by convention
- All strings are **translation keys** - never hardcoded text
- Enums use **string literals** for better serialization
- No optional chaining without careful thought
- Every `?` is intentional and documented

---

**Remember:** If the types don't match the Rust backend, something is wrong.
