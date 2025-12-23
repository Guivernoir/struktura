# Consolidated Production Loss & OEE Engineering Calculator Framework

**Authored by: Senior CPO Partner**  
**Date: December 22, 2025**

As the senior CPO overseeing this initiative, I've undertaken a forensic dissection of the three provided documents—`revision.md`, `README.md`, and `structures.md`—treating them as a collective "corpse" on the necropsy table. This process involves slicing through layers of intent, structure, language, and logic to expose vital organs (strengths), pathologies (problems), and potential transplants (fixes). The goal is not mere critique but resurrection: synthesizing a single, cohesive Markdown document that encompasses **every aspect** from the originals while addressing flaws to create a healthier, more viable product blueprint.

This consolidated framework preserves the deterministic, assumption-driven ethos of the tool, refines its data model, enhances technical structures, and sharpens strategic positioning. It eliminates redundancies, resolves contradictions, and introduces fixes for sustainability. Let's begin with the dissection, then move to the rewritten, unified content.

---

## Forensic Dissection: Autopsying the Documents

Like a necropsy, I'll methodically carve through the "body" of these documents—examining anatomy (structure), physiology (logic and flow), pathology (problems), and etiology (root causes of issues). The originals total ~3,500 words; redundancies account for ~25% overlap (e.g., repeated emphasis on "deterministic" nature). Strengths are evident in their engineering rigor and liability-aware tone, but weaknesses lurk in inconsistencies and gaps.

### 1. Anatomical Overview (What Each Document Contributes)

- **revision.md**: Focuses on post-review analysis. Strengths: High-level strategic dissection (e.g., "Clean Room" approach, blind spots like "Double Entry" risk); technical refinements (Rust enums/structs for InputValue and LeverageImpact); CPO recommendations (UI interrogation, economic mapping). It ends with a next-steps prompt on outputs. This is the "brain" of the trio—strategic and forward-looking—but it's reactive, assuming prior context.
- **README.md**: The "heart" pumping core product definition. Strengths: Comprehensive outline from positioning to non-goals; features like Assumption Ledger, Loss Tree, Sensitivity Analysis; file structure tree. It's user-facing and motivational (e.g., "That's not a limitation. That's the feature."). However, it's aspirational without deep technical implementation details.
- **structures.md**: The "skeleton" providing data model rigor. Strengths: Mental model reset (from observational to analyst-curated); simplified Rust structs (e.g., OeeInput bundle); validation philosophy shift. It emphasizes gains like "cleaner math" and docs clarity. This is the most tactical, bridging to code.

**Synergies**: All reinforce the calculator's non-observational stance, assumption transparency, and engineering focus. Overlaps include loss decomposition, sensitivity, and economic translation.

### 2. Physiological Analysis (Logic, Flow, and Coherence)

- **Logic Strengths**: Consistent theme of "deterministic assumptions" avoids the "Observational Fallacy." Features like the Assumption Ledger build trust; mathematical partitioning of losses ensures auditability.
- **Flow Issues**: `revision.md` jumps from dissection to code snippets without smooth transitions. `README.md` is list-heavy but lacks integration with code (e.g., no direct mapping to structs). `structures.md` resets the model effectively but retrofits prior ideas, creating a timeline whiplash across docs.
- **Coherence Gaps**: Terms evolve inconsistently—e.g., "Aggressors" becomes "Loss Leverage Analysis," then "Theoretical Impact Analysis." Time modeling is mentioned variably (detailed in README, simplified in structures).

### 3. Pathological Findings (Problems Identified)

Here, I dissect flaws like tumors—some benign (redundancies), others malignant (contradictions risking product viability).

- **Problem 1: Structural Redundancy and Fragmentation**  
  Cause: Docs overlap on core concepts (e.g., Loss Tree in all three; time allocation in revision and structures). This fragments knowledge, risking developer confusion.  
  Impact: High technical debt in maintenance; users might miss holistic view.  
  **Suggested Fix**: Consolidate into modular sections (e.g., one "Data Model" absorbing all Rust structs). Introduce cross-references in the unified doc.

- **Problem 2: Ambiguities in Data Handling**  
  Cause: `revision.md` flags "Double Entry" risk (TimeAllocation vs. DowntimeRecord discrepancies) and cycle time overrides, but `structures.md` simplifies without resolving (e.g., no explicit hierarchy). `README.md` mentions "inferred vs. explicit" but doesn't enforce.  
  Impact: Potential runtime errors or inconsistent outputs, eroding trust.  
  **Suggested Fix**: Adopt `revision.md`'s InputValue enum universally. Add a "Resolution Hierarchy" rule: Explicit > Inferred > Default. Enforce in OeeInput validation.

- **Problem 3: Incomplete Output Modeling**  
  Cause: `revision.md` prompts for OeeResult/LossTree structs; `README.md` describes outputs descriptively; `structures.md` focuses on inputs. No unified output spec.  
  Impact: Engine is input-heavy but output-vague, hindering "traceability" promise.  
  **Suggested Fix**: Define output structs here (e.g., OeeResult with traceability fields). Link to Assumption Ledger for every metric.

- **Problem 4: Strategic Gaps in UX and Economics**  
  Cause: `revision.md` suggests "interrogating" data and refined economics (e.g., Marginal Contribution Loss); `README.md` has audience-specific reports; `structures.md` ignores UX. No integration.  
  Impact: Tool risks being engineer-only, missing executive buy-in. Economic translations could mislead without cost breakdowns.  
  **Suggested Fix**: Expand "Reporting Layers" with UX flows (e.g., validation flags). Define economic structs with additive mappings (fixed vs. variable costs).

- **Problem 5: Liability and Non-Goals Overemphasis**  
  Cause: All docs stress "not a System of Record," but repetition dilutes impact. `structures.md` removes behavioral validations, but `README.md` still implies some (e.g., "validation warnings").  
  Impact: Defensive tone might scare users; incomplete non-goals list misses edges like AI integration.  
  **Suggested Fix**: Centralize in one "Positioning & Non-Goals" section. Add positive framing: "Empowers engineers by decoupling from ops chaos."

- **Problem 6: Technical Debt in Code Snippets**  
  Cause: Rust structs are scattered (e.g., LeverageImpact in revision; OeeInput in structures). No versioning or error handling specs.  
  Impact: Implementation could diverge from docs.  
  **Suggested Fix**: Unify all structs in a "Technical Model" section. Add traits for serialization/validation.

- **Minor Pathologies**: Inconsistent terminology (e.g., "TEEP" in README but not elsewhere); file tree in README is outdated post-consolidation; lack of visuals (e.g., no diagrams for Loss Tree).

**Overall Etiology**: Docs evolved iteratively, leading to silos. Root cause: No single authoritative source. This consolidation fixes that.

### 4. Post-Mortem Prognosis

The "corpse" is viable for revival—core vitals (determinism, transparency) are strong. With fixes, this becomes a "senior-level" framework: lean (~20% shorter), coherent, and actionable. Risks mitigated: Overengineering avoided; alignment tightened.

Now, the resurrected body: the unified content.

---

## Positioning Statement

This tool is a **deterministic, assumption-driven engineering calculator** for production loss and OEE analysis. It computes outcomes based strictly on user-provided assumptions, positioning it as a "System of Reasoning" rather than a "System of Record." Outputs are deterministic, reproducible, and direct consequences of inputs. Accuracy scales linearly with input accuracy.

It decouples from real-world data quality, protecting against liability by stating: "This computes what must be true if assumptions are true." It targets analysts, supervisors, and engineers for post-hoc reasoning, scenario modeling, and CapEx justifications—not shop-floor operations or real-time monitoring.

**Non-Goals (Explicit Boundaries)**:

- Does not validate real-world data or observe machine behavior.
- Does not detect root causes, replace MES/historians, or handle scheduling/optimization.
- Excludes operator mistakes, immutable audits, or AI-driven inferences.
- No claims of causality or accounting truth—only mathematical partitions and leverage.

---

## 1. Input Model (Analyst-Curated Assumptions)

Inputs are assumed to be curated summaries, not raw logs. Assumptions are first-class, with sources (explicit/inferred/default) tracked.

### Core Structures (Rust-Defined)

```rust
pub enum InputValue<T> {
    Explicit(T),   // User-provided
    Inferred(T),   // Derived from others
    Default(T),    // System fallback
}

pub struct AnalysisWindow {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

pub struct MachineContext {
    pub machine_id: String,
    pub line_id: Option<String>,
    pub product_id: Option<String>,
}

pub enum MachineState {
    Running,
    Stopped,
    Setup,
    Starved,
    Blocked,
}

pub struct TimeAllocation {
    pub state: MachineState,
    pub duration: InputValue<Duration>,
    pub reason: Option<ReasonCode>,
}

pub struct ProductionSummary {
    pub total_units: InputValue<u32>,
    pub good_units: InputValue<u32>,
    pub scrap_units: InputValue<u32>,
    pub reworked_units: InputValue<u32>,
}

pub struct CycleTimeModel {
    pub ideal_cycle_time: InputValue<Duration>,
    pub average_cycle_time: Option<InputValue<Duration>>,
}

pub struct DowntimeRecord {
    pub duration: InputValue<Duration>,
    pub reason: ReasonCode,
}

pub struct ReasonCode {
    pub path: Vec<String>,  // Hierarchical, e.g., ["Mechanical", "Bearing Failure"]
}

pub struct OeeInput {
    pub window: AnalysisWindow,
    pub machine: MachineContext,
    pub planned_production_time: InputValue<Duration>,
    pub time_allocations: Vec<TimeAllocation>,
    pub production: ProductionSummary,
    pub cycle_time: CycleTimeModel,
    pub downtimes: Vec<DowntimeRecord>,
}
```

**Resolution Hierarchy**: Explicit > Inferred > Default. Handles ambiguities (e.g., average_cycle_time overrides calculated total_units / running_time if explicit).

**Mandatory Assumptions**: Planned time, shift structure, breaks, changeovers, micro-stoppage thresholds, production counts.

---

## 2. Assumption Ledger (Trust Builder)

A visible ledger tracks:

- Value sources.
- Thresholds/constraints.
- Warnings (e.g., discrepancies in time sums).

Highlighted for material impacts. Always accessible from results.

---

## 3. Validation Layer (Mathematical Integrity)

Focuses on logic:

- Sum(time_allocations) ≤ planned_time.
- good + scrap + rework = total_units.
- Ideal ≤ average cycle time (warning).
- Flags impossibilities (e.g., units exceeding ideal capacity).

Interrogates data via UI (e.g., "Physical Impossibility" alerts).

---

## 4. Calculation Engine (Pure Math)

### Core Metrics (ISO-Aligned)

- Availability, Performance, Quality, OEE (time-weighted).
- Formulas: Explicit, versioned, auditable.

### Extended Metrics

- TEEP, Utilization, MTBF/MTTR (modeled).
- Scrap rate, rework impact, speed/stop loss decomposition.  
  Each with required assumptions and sensitivity.

---

## 5. Loss Tree Decomposition

A mathematical partition:

- Availability: Failures, setups, waiting.
- Performance: Speed losses, micro-stoppages.
- Quality: Startup/steady-state scrap, rework.

No forensic claims—synthetic timelines for views (e.g., rolling OEE).

---

## 6. Loss Leverage Analysis

Theoretical impact:

```rust
pub struct LeverageImpact {
    pub loss_category: ReasonCode,
    pub oee_opportunity_points: f64,  // e.g., +4.2%
    pub throughput_gain: u32,         // e.g., +500 units
    pub sensitivity_score: f64,       // ±5% input swing
}
```

Outputs: Highest leverage, frequency traps, recoverability (user-defined).

---

## 7. Sensitivity & What-If Simulation

- Scenarios: Reduce setups, eliminate categories.
- Exposure: ±X% variations, stability flags.

---

## 8. System-Level & Temporal Modeling

Illustrative: Constraint candidates, starvation sensitivity.  
Synthetic: Startup vs. steady-state, shift variance. Labeled as "modeled."

---

## 9. Economic Translation

Additive mapping:

- Lost units → Marginal Contribution Loss (fixed costs).
- Scrap → Direct Material Waste (variable).
- Downtime → Opportunity cost.

Estimates only, user-valued.

```rust
pub struct EconomicImpact {
    pub marginal_loss: f64,
    pub material_waste: f64,
    pub opportunity_cost: f64,
}
```

---

## 10. Output Model (Traceable Results)

```rust
pub struct LossTreeNode {
    pub category: String,
    pub sub_nodes: Vec<LossTreeNode>,
    pub duration: Duration,
    pub impact: f64,
}

pub struct OeeResult {
    pub core_metrics: HashMap<String, f64>,  // e.g., {"OEE": 85.2}
    pub extended_metrics: HashMap<String, f64>,
    pub loss_tree: LossTreeNode,
    pub leverage_impacts: Vec<LeverageImpact>,
    pub economic: EconomicImpact,
    pub ledger: AssumptionLedger,  // Embedded traceability
    pub sensitivities: Vec<SensitivityDelta>,
}
```

Ensures "how was this calculated?" traceability.

---

## 11. Reporting Layers (Audience-Tailored)

- **Engineering**: Full trees, formulas, ledger, sensitivities.
- **Executive**: One-page summary, top leverages, economic ranges, "needle-movers."
- **Educational**: Step-by-step breakdowns, editable assumptions.

UX: Immediate feedback, interrogation flags.

---

## 12. Trust & Transparency

- Clickable explanations per metric.
- Input replay.
- No black boxes.

---

## 13. Project Structure & Next Steps

```
.
├── api.rs
├── assumptions
│   ├── counts.rs
│   ├── cycle.rs
│   ├── downtime.rs
│   ├── mod.rs
│   ├── thresholds.rs
│   └── time.rs
├── domain
│   ├── economics.rs
│   ├── extended.rs
│   ├── loss_tree.rs
│   ├── metrics.rs
│   └── mod.rs
├── engine
│   ├── decomposition.rs
│   ├── leverage.rs
│   ├── mod.rs
│   ├── oee.rs
│   ├── temporal_scrap.rs
│   ├── multi_machine.rs
│   └── sensitivity.rs
├── ledger
│   ├── assumption_tracking.rs
│   └── mod.rs
├── mod.rs
├── README.md
├── tests
│   ├── invalid_inputs.rs
│   ├── loss_tree.rs
│   ├── mod.rs
│   ├── oee_math.rs
│   └── sensitivity.rs
└── validation
├── logical.rs
├── mod.rs
├── ranges.rs
└── warnings.rs
```

**Next**: Implement OeeInput → OeeResult wiring; add what-if deltas; prototype validation UI.

This framework avoids overengineering, aligns with audience, and makes assumptions unavoidable. That's the feature.

## 14. Considerations

# 1. You’re still almost over-selling causality

We say “no root cause claims”… but then we give:

- Loss trees

- Aggressors

- Leverage analysis

- Constraint candidates

_To a careless reader, that sounds like causality._

Recommendation (non-optional):
Add one explicit rule, early and loud:

All loss categories are attribution buckets, not causal proofs.
They represent where impact accumulates, not why it exists.

Otherwise some middle manager will screenshot this and say:

“The calculator proved maintenance is the problem.”

That’s not a technical flaw—it’s a political one. Handle it upfront.

# 2. Economic translation is powerful… and dangerous

_Our economics layer is clean, but it’s walking on thin ice._

Risk points:

Marginal contribution assumptions

Fixed vs variable cost misclassification

Opportunity cost inflation

_We must force users to acknowledge uncertainty here._

Concrete fix:

Require confidence bands on every economic input

Default outputs to ranges, not point estimates

Label all economics as illustrative, not accounting-grade

Otherwise finance will nuke this on sight.

3. Validation scope needs one more hard line

We say validation is “logical only,” which is correct—but you still hint at things like:

“Physical impossibility”

“Ideal ≤ average cycle time”

That’s fine, but don’t drift further.

Bright-line rule to add:

Validation enforces internal mathematical coherence only, never external realism.

No heuristics. No “this seems unlikely.”
If we cross that line later, the whole positioning collapses.

4. Output model is good—but missing one executive weapon

Executives don’t care about trees. They care about ranked bets.

We already have the data—just formalize it.

Add one explicit output:

pub struct DecisionCandidate {
pub description: String,
pub expected_oee_gain: f64,
pub economic_range: (f64, f64),
pub sensitivity_risk: f64,
pub assumptions_required: Vec<String>,
}

Call it:

“Decision Hypotheses”

or “Engineering Bets”

This bridges engineering truth → business action without lying.

Structural sanity check 🧱

# Our directory structure passes the “6 months later” test.

# A few micro-optimizations:

assumptions/ vs ledger/ is clean—keep it

engine/ is well scoped

domain/ might grow fat—watch it

One suggestion:

Add versioning.rs or formula_registry.rs

Lock formulas by version early

Future you will thank present you.

### Strategic verdict

This plan is:

- Technically credible

- Professionally defensible

- Aligned with real engineering behavior

- Scalable without becoming a monster

The main risk left is not technical—it’s narrative misuse.
Solve that with explicit disclaimers and sharper language around attribution vs causality.

Do that, and this becomes:

A reference-grade engineering calculator, not another OEE toy.
