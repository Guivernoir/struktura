# OEE Engineering Calculator Component

> **Deterministic. Assumption-Driven. Analyst-First.**

This component implements a **production loss & OEE engineering calculator**, designed explicitly for **analysts, supervisors, and production engineers** performing **post-hoc reasoning, scenario analysis, and decision support**.

It is **not** an operational system, **not** a data ingestion tool, and **not** a real-time monitoring interface.

If you are looking for live dashboards, MES features, or automated root-cause detection, you are in the wrong room.

---

## What This Component Is

This OEE component is a **pure calculation and reasoning surface**.

It takes **user-curated assumptions** as input and produces **deterministic outputs** based strictly on those assumptions.

> **If the inputs are wrong, the outputs will be wrong — and that is intentional.**

The component answers one question only:

> _“What must be true **if** these assumptions are true?”_

---

## What This Component Is Not (Non-Goals)

To prevent misuse and scope creep, the following are **explicit non-goals**:

- ❌ No real-time data ingestion
- ❌ No machine connectivity
- ❌ No operator-facing workflows
- ❌ No automatic validation against reality
- ❌ No root-cause detection or AI inference
- ❌ No optimization or scheduling logic
- ❌ No claims of causality or accounting truth

This component **does not observe reality**.
It **models implications**.

---

## Target Users

This component is built for:

- Production engineers
- Industrial / manufacturing analysts
- Supervisors preparing:

  - loss analysis
  - improvement scenarios
  - CapEx justification
  - post-mortems
  - training material

It is **deliberately hostile** to casual usage.
High friction = high trust.

---

## Core Design Principles

### 1. Assumptions Are First-Class Citizens

Every number has:

- a source (explicit / inferred / default)
- a lineage
- a downstream impact

There are **no silent defaults**.

---

### 2. Deterministic by Construction

- Same inputs → same outputs
- No time dependency
- No hidden state
- No stochastic behavior

This enables:

- reproducibility
- auditability
- explainability

---

### 3. Traceability Over Convenience

Every result can answer:

- _“How was this calculated?”_
- _“Which assumptions does this depend on?”_

If a metric cannot be traced, it does not belong.

---

### 4. Calculator, Not System of Record

This component:

- does **not** reconcile conflicting data
- does **not** decide which input is “correct”
- does **not** validate real-world plausibility beyond math

That responsibility belongs to the user.

---

## Technology Stack

- **TypeScript / TSX**
- **TailwindCSS**
- **PostCSS**
- Framework-agnostic UI logic (mounted by the engineer page)
- Backend calculation engine implemented in **Rust**

This component communicates exclusively through its local API client.

---

## Component Scope

This README covers **only the OEE component**, located at:

```
src/components/engineer/production/oee
```

It does **not** document:

- the engineer page
- global routing
- authentication
- cross-calculator orchestration

---

## High-Level Architecture (Frontend)

The component is structured into three always-present layers:

1. **Assumption Builder**

   - Structured, explicit input
   - Source tagging
   - Visibility of defaults

2. **Computation Surface**

   - OEE metrics
   - Loss tree decomposition
   - Leverage & sensitivity analysis

3. **Traceability Layer**

   - Assumption ledger
   - Formula inspection
   - Input → output mapping

The user never “leaves” the component — they **drill**.

---

## Folder Structure (Component-Only)

```
oee/
├── OeeEngine.tsx          // Root orchestration component
├── api.ts                 // OEE-specific API client
├── useOee.ts              // Local hook (state + orchestration)
│
├── models/                // Frontend mirrors of domain structs
│   ├── input.ts
│   ├── output.ts
│   ├── assumptions.ts
│   └── enums.ts
│
├── components/
│   ├── assumptions/       // Assumption declaration & interrogation
│   ├── validation/        // Mathematical consistency warnings
│   ├── results/           // Core OEE metrics & formulas
│   ├── loss-tree/         // Loss decomposition (partitioned)
│   ├── leverage/          // Impact & sensitivity analysis
│   ├── ledger/            // Assumption traceability
│   └── economics/         // Economic translation
│
├── views/                 // High-level internal layouts
│   ├── AssumptionsView.tsx
│   ├── ResultsView.tsx
│   └── TraceabilityView.tsx
│
├── utils/
│   ├── formatting.ts
│   ├── units.ts
│   ├── mathHints.ts
│   └── guards.ts
│
└── styles/
    └── oee.css
```

---

## Input Philosophy

Inputs are **curated summaries**, not raw event streams.

Examples:

- Planned production time for a period
- Aggregated downtime by category
- Total units, scrap, rework
- Ideal and/or average cycle time

The component **assumes the user has already done the messy work**.

---

## Validation Philosophy

Validation exists to protect **mathematical integrity**, not reality:

- Time partitions must not exceed planned time
- Unit counts must reconcile
- Physical impossibilities are flagged

Validation:

- never blocks computation
- never auto-corrects
- always explains _why_ something is questionable

---

## Outputs

The component produces:

- Core OEE metrics (Availability, Performance, Quality, OEE)
- Extended indicators (TEEP, utilization, modeled MTBF/MTTR)
- Loss tree decomposition
- Theoretical leverage analysis
- Sensitivity exposure
- Economic impact estimates
- A complete assumption ledger

All outputs are **traceable back to inputs**.

---

## Economic Outputs (Important Disclaimer)

Economic figures are:

- **estimates**
- **user-parameterized**
- **non-accounting**

They exist to support:

- prioritization
- scenario comparison
- executive communication

They are **not** financial statements.

---

## Trust & Transparency Features

- Formula inspection per metric
- Click-through assumption lineage
- Explicit labeling of inferred/default values
- Deterministic recomputation

No black boxes. No magic.

---

## Integration Contract

The engineer page:

- mounts the component
- provides context (if needed)
- receives computed results

The OEE component:

- owns its internal state
- exposes no global side effects
- can be mounted, unmounted, or reset safely

---

## Final Note (Read This Twice)

This component is intentionally **not ergonomic**.

If users:

- have to think before entering numbers
- feel uneasy about defaults
- are forced to confront assumptions

Then the component is doing its job.

If it ever feels like a dashboard, something went wrong.

---
