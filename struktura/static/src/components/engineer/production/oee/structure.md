# Component Folder Contract — OEE

This is not “organization for convenience”.
This is **organizational governance**.

---

## 1. `components/assumptions/`

**Purpose:**
Declare, interrogate, and surface _assumptions_ — nothing else.

### ✅ Must include

- Structured input fields
- Source indicators (Explicit / Inferred / Default)
- Contextual warnings (“this assumption materially affects X”)
- Grouping logic (time, counts, cycle, downtime)

### ❌ Must NOT include

- Any calculation logic
- Any derived metrics
- Any API calls
- Any cross-assumption reconciliation

### Typical contents

- `AssumptionPanel.tsx`
  Container per assumption domain (Time, Counts, Cycle…)
- `AssumptionField.tsx`
  A **single** assumption input + metadata
- `AssumptionSourceBadge.tsx`
  Visual source indicator
- `AssumptionWarnings.tsx`
  Non-blocking warnings tied to _this assumption only_

**Hard rule:**
An assumption component must remain valid even if the calculation engine is removed entirely.

If it “knows” what OEE is, it’s doing too much.

---

## 2. `components/validation/`

**Purpose:**
Expose **mathematical integrity issues**, not “user mistakes”.

### ✅ Must include

- Logical contradictions
- Physical impossibilities
- Reconciliation warnings
- Severity levels (info / warning / critical)

### ❌ Must NOT include

- UI validation like “required field”
- Business logic
- Suggestions on what the user _should_ do

### Typical contents

- `ValidationBanner.tsx`
  Global, always-visible status surface
- `ValidationItem.tsx`
  One issue = one explanation

**Tone rule:**
Never say “fix this”.
Always say “this implies X, which conflicts with Y”.

This preserves user agency and legal distance.

---

## 3. `components/results/`

**Purpose:**
Present **computed outputs** and nothing upstream of them.

### ✅ Must include

- Final metrics
- Units & formatting
- Direct formula inspection
- Drill-down entry points

### ❌ Must NOT include

- Raw assumptions
- Editable inputs
- Economic interpretation
- Trend narratives

### Typical contents

- `OeeSummary.tsx`
  A/P/Q/OEE, period-scoped
- `MetricCard.tsx`
  Single metric + unit + confidence context
- `FormulaPopover.tsx`
  Literal math + substituted values

**Hard rule:**
A result component must render correctly given a frozen `OeeResult`.

No backreferences to input state.

---

## 4. `components/loss-tree/`

**Purpose:**
Explain **where losses mathematically reside**, not what caused them.

### ✅ Must include

- Hierarchical loss partitions
- Duration & impact per node
- Percent-of-total context

### ❌ Must NOT include

- Timelines pretending to be real
- Event-level narratives
- Root-cause language

### Typical contents

- `LossTreeView.tsx`
  Tree container
- `LossNode.tsx`
  Recursive node
- `LossImpactBadge.tsx`
  OEE points / time / throughput delta

**Language rule:**
Use “allocated to”, never “caused by”.

That distinction matters legally and intellectually.

---

## 5. `components/leverage/`

**Purpose:**
Show **theoretical improvement potential**, not recommendations.

### ✅ Must include

- Delta-based impact ranking
- Sensitivity exposure
- Recoverability hints (if user-defined)

### ❌ Must NOT include

- Prescriptive actions (“fix this first”)
- Operational advice
- ROI claims without ranges

### Typical contents

- `LeverageTable.tsx`
  Ranked theoretical impacts
- `LeverageRow.tsx`
  One leverage hypothesis
- `SensitivityIndicator.tsx`
  Stability / fragility signal

**Hard rule:**
Leverage components talk in **“if X were reduced”**, never “should”.

---

## 6. `components/ledger/`

**Purpose:**
Provide **receipts**. This is your trust anchor.

### ✅ Must include

- Complete assumption list
- Source classification
- Downstream dependency mapping
- Navigation back to affected results

### ❌ Must NOT include

- Editable inputs
- Calculations
- Summaries

### Typical contents

- `AssumptionLedger.tsx`
  Full ledger view
- `LedgerEntry.tsx`
  One assumption, one impact map
- `TracebackLink.tsx`
  Cross-navigation glue

If this folder is weak, the entire product is weak. Period.

---

## 7. `components/economics/`

**Purpose:**
Translate technical loss into **financial ranges**, not accounting truth.

### ✅ Must include

- Marginal loss
- Material waste
- Opportunity cost
- Ranges & uncertainty

### ❌ Must NOT include

- Accounting logic
- Financial validation
- Single-point “total loss” claims

### Typical contents

- `EconomicImpactPanel.tsx`
- `EconomicRangeBar.tsx`

**Disclosure rule:**
Every economic number must scream “estimate”.

If it looks precise, you’ve crossed a line.

---

## 8. `views/`

**Purpose:**
Layout orchestration only.

### ✅ Must include

- Composition of components
- Navigation inside the OEE component
- Responsiveness logic

### ❌ Must NOT include

- Business logic
- State mutation
- API calls

Think of views as **stage managers**, not actors.

---

## 9. `models/`

**Purpose:**
Frontend contracts mirroring backend domain.

### ✅ Must include

- Type definitions only
- Enums
- DTOs

### ❌ Must NOT include

- Functions
- Defaults
- UI helpers

If logic sneaks in here, refactor immediately.

---

## 10. `utils/`

**Purpose:**
Pure, boring helpers.

### ✅ Must include

- Unit formatting
- Numeric guards
- Display helpers

### ❌ Must NOT include

- Business rules
- Domain logic
- Side effects

If it wouldn’t survive a standalone unit test, it doesn’t belong here.

---

## 11. `OeeEngine.tsx`

**Purpose:**
Orchestration and nothing else.

### Responsibilities

- Hold local OEE state
- Call `useOee`
- Coordinate views
- Emit final results upward

### Must NOT

- Render detailed UI
- Contain formulas
- Know domain rules

If this file grows teeth, something upstream failed.

---

## Final governance rule (this matters)

> **If a folder can’t explain its existence in one sentence, it’s wrong.**

Your current structure is solid **because** it enforces cognitive boundaries.
Respect those boundaries and this component will age well.

Next logical step, if you want to stay disciplined:

- define **one concrete AssumptionPanel contract**
- and write tests for _traceability_, not math

That’s how you keep this from becoming another “smart-looking calculator” that nobody trusts.
