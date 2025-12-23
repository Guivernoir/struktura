# OEE Calculator Utilities

Utility functions for the OEE calculator component. Organized by category for easy discovery and maintainability.

## Files Overview

### `formatting.ts`

Display formatting for all types of values:

- Numbers (with localization)
- Percentages
- Durations (seconds → "1h 30m 45s")
- Currency (with proper symbols)
- Dates and times
- Large numbers with K/M/B suffixes
- Ranges and ratios

**Use when:** Preparing data for display in UI components.

**Example:**

```typescript
import { formatPercentage, formatDuration, formatCurrency } from "./utils";

formatPercentage(0.856); // "85.6%"
formatDuration(5400); // "1h 30m 0s"
formatCurrency(12500.5, "USD"); // "$12,500.50"
```

### `units.ts`

Unit conversions and rate calculations:

- Time conversions (seconds ↔ minutes ↔ hours ↔ days)
- Production rate conversions (units/sec ↔ units/min ↔ units/hour)
- Cycle time ↔ rate conversions
- Percentage conversions (decimal ↔ percentage ↔ basis points)
- Temperature, speed conversions (if needed)
- Unit validation utilities

**Use when:** Converting between units or validating unit consistency.

**Example:**

```typescript
import { TimeUnits, RateUnits, PercentageUnits } from "./utils";

TimeUnits.hoursToSeconds(2.5); // 9000
RateUnits.cycleTimeToRate(25.2); // 0.0397 units/sec
PercentageUnits.decimalToPercentage(0.85); // 85
```

### `mathHints.ts`

Mathematical validation and plausibility checks:

- Production count consistency
- Time allocation consistency
- Cycle time plausibility
- Scrap rate warnings
- Downtime exceedance checks
- Economic parameter validation
- TEEP validity checks
- Statistical helpers (CV, outlier detection, trend analysis)

**Use when:** Validating input data quality and providing user feedback.

**Example:**

```typescript
import { MathHints } from "./utils";

const hints = MathHints.collectAllHints(
  production,
  timeModel,
  cycleTime,
  totalDowntime,
  economicParams
);

hints.forEach((hint) => {
  console.log(`[${hint.severity}] ${hint.message}`);
});
```

### `guards.ts`

Type guards and runtime validation:

- InputValue type guards
- Number validation (valid, non-negative, positive, integer)
- Percentage validation (0-1 range)
- Date string validation (ISO 8601)
- Complete type guards for all model types
- Array, object, range, string guards
- Assertion helpers (throw on invalid)
- Safe parsing helpers (return null on failure)

**Use when:** Validating data at runtime, especially at API boundaries.

**Example:**

```typescript
import { isValidOeeInput, isNonNegativeNumber, Assertions } from "./utils";

if (isValidOeeInput(data)) {
  // TypeScript knows data is OeeInput
  processInput(data);
}

if (isNonNegativeNumber(value)) {
  // TypeScript knows value is number >= 0
  calculate(value);
}

const count = Assertions.assertNonNegative(input.count);
```

### `helpers.ts`

General utility functions:

- Deep clone and equality
- Debounce and throttle
- Async retry with backoff
- Array operations (groupBy, sum, average, median, unique, sort, partition)
- Number operations (clamp, lerp, mapRange, roundTo)
- Reason code formatting
- Loss tree traversal
- Safe division (percentage, ratio)
- Object operations (pick, omit, merge)
- Download and clipboard utilities
- InputValue helpers

**Use when:** Need common programming utilities not specific to OEE domain.

**Example:**

```typescript
import { debounce, groupBy, safePercentage, flattenLossTree } from "./utils";

const debouncedSearch = debounce(performSearch, 300);

const groupedByCategory = groupBy(items, (item) => item.category);

const scrapRate = safePercentage(scrapCount, totalCount);

const allNodes = flattenLossTree(lossTree.root);
```

## Import Patterns

### Named imports (recommended)

```typescript
import { formatPercentage, formatDuration } from "./utils";
import { TimeUnits, RateUnits } from "./utils";
import { MathHints } from "./utils";
```

### Category imports

```typescript
import * as Formatting from "./utils/formatting";
import * as Units from "./utils/units";
import * as Guards from "./utils/guards";
```

### Wildcard (discouraged - harder to tree-shake)

```typescript
import * as Utils from "./utils";
```

## Design Principles

### 1. Pure Functions

All utility functions are pure - same inputs always produce same outputs:

```typescript
formatPercentage(0.85); // Always "85.0%"
```

### 2. No Side Effects

Utilities don't modify input arguments or global state:

```typescript
const sorted = sortBy(array, (x) => x.value); // Returns new array
```

### 3. Type Safety

All functions are strongly typed with proper TypeScript annotations:

```typescript
function formatPercentage(value: number, decimals?: number): string;
```

### 4. Defensive Programming

Handle edge cases gracefully:

```typescript
safePercentage(10, 0); // Returns 0 instead of Infinity
average([]); // Returns 0 instead of NaN
```

### 5. Composability

Functions are designed to work well together:

```typescript
const formatted = formatPercentage(
  PercentageUnits.decimalToPercentage(safeRatio(good, total))
);
```

## Usage Patterns

### Formatting for Display

```typescript
import { formatPercentage, formatDuration, formatCurrency } from "./utils";

function MetricsDisplay({ metrics }: { metrics: CoreMetrics }) {
  return (
    <div>
      <div>OEE: {formatPercentage(metrics.oee.value)}</div>
      <div>Availability: {formatPercentage(metrics.availability.value)}</div>
    </div>
  );
}
```

### Unit Conversion

```typescript
import { TimeUnits, RateUnits } from "./utils";

const hoursWorked = TimeUnits.secondsToHours(totalSeconds);
const unitsPerHour = RateUnits.unitsPerSecondToHour(unitsPerSecond);
```

### Input Validation

```typescript
import { MathHints } from "./utils";

function validateInput(input: OeeInput) {
  const hints = MathHints.collectAllHints(
    input.production,
    input.time_model,
    input.cycle_time,
    calculateTotalDowntime(input.downtimes),
    economicParams
  );

  const errors = hints.filter((h) => h.severity === HintSeverity.Critical);
  const warnings = hints.filter((h) => h.severity === HintSeverity.Warning);

  return { errors, warnings };
}
```

### Type Guarding

```typescript
import { isValidOeeInput, isNonNegativeNumber } from "./utils";

function processData(data: unknown) {
  if (!isValidOeeInput(data)) {
    throw new Error("Invalid OEE input");
  }

  // TypeScript knows data is OeeInput here
  calculate(data);
}
```

### Array Operations

```typescript
import { groupBy, sortBy, unique } from "./utils";

const downtimesByReason = groupBy(downtimes, (dt) => dt.reason.path[0]);

const sortedByDuration = sortBy(
  downtimes,
  (dt) => InputValueHelpers.getValue(dt.duration),
  "desc"
);

const uniqueReasons = unique(downtimes.map((dt) => dt.reason.path[0]));
```

## Common Gotchas

### Duration is in Seconds

All duration values are in **seconds**, not milliseconds:

```typescript
// ✓ Correct
formatDuration(3600); // "1h 0m 0s"

// ✗ Wrong
formatDuration(Date.now()); // Way too large
```

### Percentage Range

Percentages in calculations use **0-1** range, not 0-100:

```typescript
// ✓ Correct
formatPercentage(0.85); // "85.0%"

// ✗ Wrong
formatPercentage(85); // "8500.0%"
```

### Safe Division

Always use safe division helpers when denominator might be zero:

```typescript
// ✓ Correct
const rate = safePercentage(good, total);

// ✗ Wrong (might produce Infinity or NaN)
const rate = (good / total) * 100;
```

### Immutability

Array utilities return new arrays - don't expect mutation:

```typescript
// ✓ Correct
const sorted = sortBy(array, (x) => x.value);

// ✗ Wrong (array is not modified)
sortBy(array, (x) => x.value);
console.log(array); // Still unsorted
```

## Performance Notes

### Debouncing

Use debounce for expensive operations triggered by user input:

```typescript
const debouncedValidate = debounce(validate, 300);
inputElement.addEventListener("input", debouncedValidate);
```

### Memoization

For expensive pure computations, consider memoization:

```typescript
const memoized = memoize(expensiveCalculation);
```

### Large Arrays

For very large arrays, consider chunking:

```typescript
const chunks = chunk(hugeArray, 1000);
chunks.forEach(processChunk);
```

## Testing

All utilities are pure functions and easy to test:

```typescript
import { formatPercentage, safePercentage } from "./utils";

describe("formatPercentage", () => {
  it("formats decimal to percentage", () => {
    expect(formatPercentage(0.856)).toBe("85.6%");
  });

  it("handles decimals parameter", () => {
    expect(formatPercentage(0.856, 2)).toBe("85.60%");
  });
});

describe("safePercentage", () => {
  it("returns percentage when denominator > 0", () => {
    expect(safePercentage(85, 100)).toBe(85);
  });

  it("returns 0 when denominator is 0", () => {
    expect(safePercentage(85, 0)).toBe(0);
  });
});
```

## Extension

To add new utilities:

1. Choose appropriate file based on category
2. Add function with JSDoc comment
3. Export from file
4. Add to index.ts if needed
5. Update this README with example
6. Write tests

Example:

```typescript
/**
 * Calculate weighted average
 */
export function weightedAverage(values: number[], weights: number[]): number {
  if (values.length !== weights.length) {
    throw new Error("Values and weights must have same length");
  }

  const totalWeight = sum(weights);
  if (totalWeight === 0) return 0;

  const weightedSum = values.reduce((acc, val, i) => acc + val * weights[i], 0);

  return weightedSum / totalWeight;
}
```

---

**Remember:** Utilities should be **simple**, **pure**, **typed**, and **well-tested**.
