# Engineering Calculator Unit Verification

## Critical Issue: Unit Conversion in Beam Design

### The Problem

Original code had incorrect unit conversion for moment capacity:

```rust
// WRONG - Division by 1000 gave wrong answer
let mp = section_modulus * (fy / 1000.0); // Claimed kN·m
let design_moment_capacity = PHI_FLEXURE * mp;
```

This resulted in utilization ratio of **17,036** instead of **0.02**!

### The Correct Calculation

#### Input Units

- **Section modulus (Z)**: m³
- **Yield strength (fy)**: MPa = N/mm² = MN/m²
- **Elastic modulus (E)**: GPa = 10³ MPa = 10⁶ kN/m²

#### Moment Capacity Calculation

**Step 1: Calculate plastic moment**

```
Mp = Z × fy
   = m³ × MPa
   = m³ × (MN/m²)
   = MN·m
```

**Step 2: Convert to kN·m**

```
Mp (kN·m) = Mp (MN·m) × 1000
```

**Step 3: Apply resistance factor**

```
φMn = φ × Mp × 1000
```

#### Verification Example

**Given:**

- Width (b) = 0.30 m
- Height (h) = 0.70 m
- fy = 345 MPa (A992 steel)
- φ = 0.90 (LRFD flexure)

**Calculate:**

1. Section modulus:

```
Z = b×h²/6 = 0.30 × 0.70² / 6 = 0.0245 m³
```

2. Plastic moment:

```
Mp = Z × fy
   = 0.0245 m³ × 345 MPa
   = 0.0245 m³ × 345 MN/m²
   = 8.4525 MN·m
   = 8,452.5 kN·m
```

3. Design capacity:

```
φMn = 0.90 × 8,452.5 kN·m = 7,607.25 kN·m ✓
```

**Applied moment** (6m span, 33.6 kN/m factored load):

```
M = wL²/8 = 33.6 × 6² / 8 = 151.2 kN·m
```

**Utilization:**

```
D/C = 151.2 / 7,607.25 = 0.0199 = 1.99% ✓
```

### Deflection Calculation Verification

#### Formula

```
δ = (5wL⁴)/(384EI)
```

#### Unit Analysis

```
w = kN/m
L = m
E = GPa = 10⁶ kN/m²
I = m⁴

δ = (kN/m × m⁴) / (10⁶ kN/m² × m⁴)
  = (kN × m³) / (10⁶ kN × m²)
  = m³ / (10⁶ × m²)
  = m / 10⁶ × 10⁶
  = m ✓
```

#### Verification Example

**Given:**

- Service load (w) = 20 kN/m
- Span (L) = 6.0 m
- E = 200 GPa = 200 × 10⁶ kN/m²
- I = (0.30 × 0.70³)/12 = 0.008575 m⁴

**Calculate:**

```
δ = (5 × 20 × 6⁴) / (384 × 200×10⁶ × 0.008575)
  = (5 × 20 × 1296) / (384 × 200×10⁶ × 0.008575)
  = 129,600 / (659,328,000)
  = 0.0001966 m
  = 0.197 mm ✓
```

**Allowable** (L/360):

```
δ_allow = 6000mm / 360 = 16.67 mm ✓
```

Deflection utilization = 0.197 / 16.67 = **1.18%** ✓

## Column Design Units

### Axial Capacity

**Given:**

- Area (A) = m²
- Concrete strength (f'c) = MPa = MN/m²

**Nominal capacity** (simplified):

```
Pn = 0.80 × A × f'c × 1000
   = 0.80 × m² × MN/m² × 1000
   = 0.80 × MN × 1000
   = kN ✓
```

### Slenderness Ratio

```
λ = KL/r

where:
K = effective length factor (dimensionless)
L = unbraced length (m)
r = radius of gyration (m)

Result: dimensionless ✓
```

## Heat Exchanger Units

### Heat Transfer Rate

**Given:**

- Mass flow (ṁ) = kg/s
- Specific heat (cp) = J/(kg·K) = kJ/(kg·K) / 1000
- Temperature difference (ΔT) = K or °C

**Calculate:**

```
Q = ṁ × cp × ΔT
  = kg/s × J/(kg·K) × K
  = J/s
  = W ✓

Convert to kW: Q(W) / 1000
```

### LMTD

```
LMTD = (ΔT₁ - ΔT₂) / ln(ΔT₁/ΔT₂)

All temperatures in K or °C (difference is same)
Result: K or °C ✓
```

### Heat Transfer Area

**Given:**

- Q = W
- U = W/(m²·K)
- LMTD = K

**Calculate:**

```
A = Q / (U × LMTD)
  = W / (W/(m²·K) × K)
  = W / (W/m²)
  = m² ✓
```

## Key Principles

1. **MPa is MN/m², not kN/m²**

   - 1 MPa = 1 N/mm² = 10⁶ N/m² = 1 MN/m² = 1000 kN/m²

2. **Always multiply section properties by stress**

   - Moment = Z × σ (m³ × MPa = MN·m)
   - Force = A × σ (m² × MPa = MN)

3. **Convert MN to kN at the end**

   - Multiply by 1000

4. **GPa for elastic modulus**

   - 1 GPa = 10³ MPa = 10⁶ kN/m²
   - Use: E × 10⁶ in deflection formulas

5. **Dimensionless ratios**
   - Utilization = Demand/Capacity (both same units)
   - Slenderness = Length/Length
   - Effectiveness = Temperature/Temperature

## Test Verification

With corrected units, the test beam (0.30m × 0.70m, 6m span) should have:

- **Capacity**: ~7,600 kN·m
- **Applied moment**: ~151 kN·m
- **Utilization**: ~2% ✓
- **Test should PASS** ✓
