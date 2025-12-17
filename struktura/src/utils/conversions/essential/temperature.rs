// Temperature Conversion Library
// 
// CRITICAL NOTE: Temperature conversions use offset formulas, NOT simple multiplication
// Temperature differences (ΔT) use different formulas than absolute temperatures
// 
// All conversions use precise, internationally standardized formulas
// Primary references: SI units, NIST, BIPM standards

// ============================================================================
// REFERENCE POINTS FOR TEMPERATURE SCALES
// ============================================================================

// Celsius: Water freezes at 0°C, boils at 100°C
// Fahrenheit: Water freezes at 32°F, boils at 212°F
// Kelvin: Absolute zero at 0K, water freezes at 273.15K
// Rankine: Absolute zero at 0°R, water freezes at 491.67°R
// Réaumur: Water freezes at 0°Ré, boils at 80°Ré
// Delisle: Water boils at 0°De, freezes at 150°De (inverted scale)
// Newton: Water freezes at 0°N, boils at 33°N
// Rømer: Water freezes at 7.5°Rø, boils at 60°Rø

// ============================================================================
// ABSOLUTE TEMPERATURE CONSTANTS
// ============================================================================

/// Absolute zero in Celsius (exact by definition since 2019 SI redefinition)
const ABSOLUTE_ZERO_C: f64 = -273.15;

/// Absolute zero in Fahrenheit (exact)
const ABSOLUTE_ZERO_F: f64 = -459.67;

/// Water triple point in Kelvin (exact by old definition, now derived)
const WATER_TRIPLE_POINT_K: f64 = 273.16;

/// Standard reference temperature (0°C = 273.15 K exactly)
const ZERO_CELSIUS_IN_KELVIN: f64 = 273.15;

/// Fahrenheit offset from Rankine
const FAHRENHEIT_OFFSET: f64 = 459.67;

// ============================================================================
// CELSIUS CONVERSIONS
// ============================================================================

/// Celsius to Fahrenheit
/// Formula: °F = (°C × 9/5) + 32
pub async fn c_to_f(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

/// Celsius to Kelvin
/// Formula: K = °C + 273.15 (exact)
pub async fn c_to_k(celsius: f64) -> f64 {
    celsius + ZERO_CELSIUS_IN_KELVIN
}

/// Celsius to Rankine
/// Formula: °R = (°C + 273.15) × 9/5
pub async fn c_to_r(celsius: f64) -> f64 {
    (celsius + ZERO_CELSIUS_IN_KELVIN) * 9.0 / 5.0
}

/// Celsius to Réaumur
/// Formula: °Ré = °C × 4/5
pub async fn c_to_re(celsius: f64) -> f64 {
    celsius * 4.0 / 5.0
}

/// Celsius to Delisle
/// Formula: °De = (100 - °C) × 3/2
pub async fn c_to_de(celsius: f64) -> f64 {
    (100.0 - celsius) * 3.0 / 2.0
}

/// Celsius to Newton
/// Formula: °N = °C × 33/100
pub async fn c_to_n(celsius: f64) -> f64 {
    celsius * 33.0 / 100.0
}

/// Celsius to Rømer
/// Formula: °Rø = °C × 21/40 + 7.5
pub async fn c_to_ro(celsius: f64) -> f64 {
    celsius * 21.0 / 40.0 + 7.5
}

// ============================================================================
// FAHRENHEIT CONVERSIONS
// ============================================================================

/// Fahrenheit to Celsius
/// Formula: °C = (°F - 32) × 5/9
pub async fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// Fahrenheit to Kelvin
/// Formula: K = (°F + 459.67) × 5/9
pub async fn f_to_k(fahrenheit: f64) -> f64 {
    (fahrenheit + FAHRENHEIT_OFFSET) * 5.0 / 9.0
}

/// Fahrenheit to Rankine
/// Formula: °R = °F + 459.67 (exact)
pub async fn f_to_r(fahrenheit: f64) -> f64 {
    fahrenheit + FAHRENHEIT_OFFSET
}

/// Fahrenheit to Réaumur
/// Formula: °Ré = (°F - 32) × 4/9
pub async fn f_to_re(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 4.0 / 9.0
}

/// Fahrenheit to Delisle
/// Formula: °De = (212 - °F) × 5/6
pub async fn f_to_de(fahrenheit: f64) -> f64 {
    (212.0 - fahrenheit) * 5.0 / 6.0
}

/// Fahrenheit to Newton
/// Formula: °N = (°F - 32) × 11/60
pub async fn f_to_n(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 11.0 / 60.0
}

/// Fahrenheit to Rømer
/// Formula: °Rø = (°F - 32) × 7/24 + 7.5
pub async fn f_to_ro(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 7.0 / 24.0 + 7.5
}

// ============================================================================
// KELVIN CONVERSIONS
// ============================================================================

/// Kelvin to Celsius
/// Formula: °C = K - 273.15 (exact)
pub async fn k_to_c(kelvin: f64) -> f64 {
    kelvin - ZERO_CELSIUS_IN_KELVIN
}

/// Kelvin to Fahrenheit
/// Formula: °F = K × 9/5 - 459.67
pub async fn k_to_f(kelvin: f64) -> f64 {
    kelvin * 9.0 / 5.0 - FAHRENHEIT_OFFSET
}

/// Kelvin to Rankine
/// Formula: °R = K × 9/5 (exact)
pub async fn k_to_r(kelvin: f64) -> f64 {
    kelvin * 9.0 / 5.0
}

/// Kelvin to Réaumur
/// Formula: °Ré = (K - 273.15) × 4/5
pub async fn k_to_re(kelvin: f64) -> f64 {
    (kelvin - ZERO_CELSIUS_IN_KELVIN) * 4.0 / 5.0
}

/// Kelvin to Delisle
/// Formula: °De = (373.15 - K) × 3/2
pub async fn k_to_de(kelvin: f64) -> f64 {
    (373.15 - kelvin) * 3.0 / 2.0
}

/// Kelvin to Newton
/// Formula: °N = (K - 273.15) × 33/100
pub async fn k_to_n(kelvin: f64) -> f64 {
    (kelvin - ZERO_CELSIUS_IN_KELVIN) * 33.0 / 100.0
}

/// Kelvin to Rømer
/// Formula: °Rø = (K - 273.15) × 21/40 + 7.5
pub async fn k_to_ro(kelvin: f64) -> f64 {
    (kelvin - ZERO_CELSIUS_IN_KELVIN) * 21.0 / 40.0 + 7.5
}

// ============================================================================
// RANKINE CONVERSIONS
// ============================================================================

/// Rankine to Celsius
/// Formula: °C = (°R - 491.67) × 5/9
pub async fn r_to_c(rankine: f64) -> f64 {
    (rankine - 491.67) * 5.0 / 9.0
}

/// Rankine to Fahrenheit
/// Formula: °F = °R - 459.67 (exact)
pub async fn r_to_f(rankine: f64) -> f64 {
    rankine - FAHRENHEIT_OFFSET
}

/// Rankine to Kelvin
/// Formula: K = °R × 5/9 (exact)
pub async fn r_to_k(rankine: f64) -> f64 {
    rankine * 5.0 / 9.0
}

/// Rankine to Réaumur
/// Formula: °Ré = (°R - 491.67) × 4/9
pub async fn r_to_re(rankine: f64) -> f64 {
    (rankine - 491.67) * 4.0 / 9.0
}

/// Rankine to Delisle
/// Formula: °De = (671.67 - °R) × 5/6
pub async fn r_to_de(rankine: f64) -> f64 {
    (671.67 - rankine) * 5.0 / 6.0
}

/// Rankine to Newton
/// Formula: °N = (°R - 491.67) × 11/60
pub async fn r_to_n(rankine: f64) -> f64 {
    (rankine - 491.67) * 11.0 / 60.0
}

/// Rankine to Rømer
/// Formula: °Rø = (°R - 491.67) × 7/24 + 7.5
pub async fn r_to_ro(rankine: f64) -> f64 {
    (rankine - 491.67) * 7.0 / 24.0 + 7.5
}

// ============================================================================
// RÉAUMUR CONVERSIONS
// ============================================================================

/// Réaumur to Celsius
/// Formula: °C = °Ré × 5/4
pub async fn re_to_c(reaumur: f64) -> f64 {
    reaumur * 5.0 / 4.0
}

/// Réaumur to Fahrenheit
/// Formula: °F = °Ré × 9/4 + 32
pub async fn re_to_f(reaumur: f64) -> f64 {
    reaumur * 9.0 / 4.0 + 32.0
}

/// Réaumur to Kelvin
/// Formula: K = °Ré × 5/4 + 273.15
pub async fn re_to_k(reaumur: f64) -> f64 {
    reaumur * 5.0 / 4.0 + ZERO_CELSIUS_IN_KELVIN
}

/// Réaumur to Rankine
/// Formula: °R = °Ré × 9/4 + 491.67
pub async fn re_to_r(reaumur: f64) -> f64 {
    reaumur * 9.0 / 4.0 + 491.67
}

/// Réaumur to Delisle
/// Formula: °De = (80 - °Ré) × 15/8
pub async fn re_to_de(reaumur: f64) -> f64 {
    (80.0 - reaumur) * 15.0 / 8.0
}

/// Réaumur to Newton
/// Formula: °N = °Ré × 33/80
pub async fn re_to_n(reaumur: f64) -> f64 {
    reaumur * 33.0 / 80.0
}

/// Réaumur to Rømer
/// Formula: °Rø = °Ré × 21/32 + 7.5
pub async fn re_to_ro(reaumur: f64) -> f64 {
    reaumur * 21.0 / 32.0 + 7.5
}

// ============================================================================
// DELISLE CONVERSIONS
// ============================================================================

/// Delisle to Celsius
/// Formula: °C = (150 - °De) × 2/3
pub async fn de_to_c(delisle: f64) -> f64 {
    (150.0 - delisle) * 2.0 / 3.0
}

/// Delisle to Fahrenheit
/// Formula: °F = 212 - °De × 6/5
pub async fn de_to_f(delisle: f64) -> f64 {
    212.0 - delisle * 6.0 / 5.0
}

/// Delisle to Kelvin
/// Formula: K = 373.15 - °De × 2/3
pub async fn de_to_k(delisle: f64) -> f64 {
    373.15 - delisle * 2.0 / 3.0
}

/// Delisle to Rankine
/// Formula: °R = 671.67 - °De × 6/5
pub async fn de_to_r(delisle: f64) -> f64 {
    671.67 - delisle * 6.0 / 5.0
}

/// Delisle to Réaumur
/// Formula: °Ré = (80 - °De) × 8/15
pub async fn de_to_re(delisle: f64) -> f64 {
    (80.0 - delisle) * 8.0 / 15.0
}

/// Delisle to Newton
/// Formula: °N = (33 - °De) × 11/50
pub async fn de_to_n(delisle: f64) -> f64 {
    (33.0 - delisle) * 11.0 / 50.0
}

/// Delisle to Rømer
/// Formula: °Rø = 60 - °De × 7/20
pub async fn de_to_ro(delisle: f64) -> f64 {
    60.0 - delisle * 7.0 / 20.0
}

// ============================================================================
// NEWTON CONVERSIONS
// ============================================================================

/// Newton to Celsius
/// Formula: °C = °N × 100/33
pub async fn n_to_c(newton: f64) -> f64 {
    newton * 100.0 / 33.0
}

/// Newton to Fahrenheit
/// Formula: °F = °N × 60/11 + 32
pub async fn n_to_f(newton: f64) -> f64 {
    newton * 60.0 / 11.0 + 32.0
}

/// Newton to Kelvin
/// Formula: K = °N × 100/33 + 273.15
pub async fn n_to_k(newton: f64) -> f64 {
    newton * 100.0 / 33.0 + ZERO_CELSIUS_IN_KELVIN
}

/// Newton to Rankine
/// Formula: °R = °N × 60/11 + 491.67
pub async fn n_to_r(newton: f64) -> f64 {
    newton * 60.0 / 11.0 + 491.67
}

/// Newton to Réaumur
/// Formula: °Ré = °N × 80/33
pub async fn n_to_re(newton: f64) -> f64 {
    newton * 80.0 / 33.0
}

/// Newton to Delisle
/// Formula: °De = (33 - °N) × 50/11
pub async fn n_to_de(newton: f64) -> f64 {
    (33.0 - newton) * 50.0 / 11.0
}

/// Newton to Rømer
/// Formula: °Rø = °N × 35/22 + 7.5
pub async fn n_to_ro(newton: f64) -> f64 {
    newton * 35.0 / 22.0 + 7.5
}

// ============================================================================
// RØMER CONVERSIONS
// ============================================================================

/// Rømer to Celsius
/// Formula: °C = (°Rø - 7.5) × 40/21
pub async fn ro_to_c(romer: f64) -> f64 {
    (romer - 7.5) * 40.0 / 21.0
}

/// Rømer to Fahrenheit
/// Formula: °F = (°Rø - 7.5) × 24/7 + 32
pub async fn ro_to_f(romer: f64) -> f64 {
    (romer - 7.5) * 24.0 / 7.0 + 32.0
}

/// Rømer to Kelvin
/// Formula: K = (°Rø - 7.5) × 40/21 + 273.15
pub async fn ro_to_k(romer: f64) -> f64 {
    (romer - 7.5) * 40.0 / 21.0 + ZERO_CELSIUS_IN_KELVIN
}

/// Rømer to Rankine
/// Formula: °R = (°Rø - 7.5) × 24/7 + 491.67
pub async fn ro_to_r(romer: f64) -> f64 {
    (romer - 7.5) * 24.0 / 7.0 + 491.67
}

/// Rømer to Réaumur
/// Formula: °Ré = (°Rø - 7.5) × 32/21
pub async fn ro_to_re(romer: f64) -> f64 {
    (romer - 7.5) * 32.0 / 21.0
}

/// Rømer to Delisle
/// Formula: °De = (60 - °Rø) × 20/7
pub async fn ro_to_de(romer: f64) -> f64 {
    (60.0 - romer) * 20.0 / 7.0
}

/// Rømer to Newton
/// Formula: °N = (°Rø - 7.5) × 22/35
pub async fn ro_to_n(romer: f64) -> f64 {
    (romer - 7.5) * 22.0 / 35.0
}

// ============================================================================
// TEMPERATURE DIFFERENCE (DELTA) CONVERSIONS
// ============================================================================
// Note: Delta temperature conversions do NOT use offsets, only scale factors

/// Celsius delta to Fahrenheit delta
/// Formula: Δ°F = Δ°C × 9/5
pub async fn delta_c_to_delta_f(delta_celsius: f64) -> f64 {
    delta_celsius * 9.0 / 5.0
}

/// Celsius delta to Kelvin delta
/// Formula: ΔK = Δ°C (1:1 ratio, exact)
pub async fn delta_c_to_delta_k(delta_celsius: f64) -> f64 {
    delta_celsius
}

/// Celsius delta to Rankine delta
/// Formula: Δ°R = Δ°C × 9/5
pub async fn delta_c_to_delta_r(delta_celsius: f64) -> f64 {
    delta_celsius * 9.0 / 5.0
}

/// Celsius delta to Réaumur delta
/// Formula: Δ°Ré = Δ°C × 4/5
pub async fn delta_c_to_delta_re(delta_celsius: f64) -> f64 {
    delta_celsius * 4.0 / 5.0
}

/// Fahrenheit delta to Celsius delta
/// Formula: Δ°C = Δ°F × 5/9
pub async fn delta_f_to_delta_c(delta_fahrenheit: f64) -> f64 {
    delta_fahrenheit * 5.0 / 9.0
}

/// Fahrenheit delta to Kelvin delta
/// Formula: ΔK = Δ°F × 5/9
pub async fn delta_f_to_delta_k(delta_fahrenheit: f64) -> f64 {
    delta_fahrenheit * 5.0 / 9.0
}

/// Fahrenheit delta to Rankine delta
/// Formula: Δ°R = Δ°F (1:1 ratio, exact)
pub async fn delta_f_to_delta_r(delta_fahrenheit: f64) -> f64 {
    delta_fahrenheit
}

/// Kelvin delta to Celsius delta
/// Formula: Δ°C = ΔK (1:1 ratio, exact)
pub async fn delta_k_to_delta_c(delta_kelvin: f64) -> f64 {
    delta_kelvin
}

/// Kelvin delta to Fahrenheit delta
/// Formula: Δ°F = ΔK × 9/5
pub async fn delta_k_to_delta_f(delta_kelvin: f64) -> f64 {
    delta_kelvin * 9.0 / 5.0
}

/// Kelvin delta to Rankine delta
/// Formula: Δ°R = ΔK × 9/5
pub async fn delta_k_to_delta_r(delta_kelvin: f64) -> f64 {
    delta_kelvin * 9.0 / 5.0
}

/// Rankine delta to Fahrenheit delta
/// Formula: Δ°F = Δ°R (1:1 ratio, exact)
pub async fn delta_r_to_delta_f(delta_rankine: f64) -> f64 {
    delta_rankine
}

/// Rankine delta to Celsius delta
/// Formula: Δ°C = Δ°R × 5/9
pub async fn delta_r_to_delta_c(delta_rankine: f64) -> f64 {
    delta_rankine * 5.0 / 9.0
}

/// Rankine delta to Kelvin delta
/// Formula: ΔK = Δ°R × 5/9
pub async fn delta_r_to_delta_k(delta_rankine: f64) -> f64 {
    delta_rankine * 5.0 / 9.0
}

// ============================================================================
// UTILITY FUNCTIONS FOR ABSOLUTE ZERO CHECKING
// ============================================================================

/// Check if Celsius temperature is physically valid (≥ absolute zero)
pub async fn is_valid_celsius(celsius: f64) -> bool {
    celsius >= ABSOLUTE_ZERO_C
}

/// Check if Fahrenheit temperature is physically valid (≥ absolute zero)
pub async fn is_valid_fahrenheit(fahrenheit: f64) -> bool {
    fahrenheit >= ABSOLUTE_ZERO_F
}

/// Check if Kelvin temperature is physically valid (≥ 0)
pub async fn is_valid_kelvin(kelvin: f64) -> bool {
    kelvin >= 0.0
}

/// Check if Rankine temperature is physically valid (≥ 0)
pub async fn is_valid_rankine(rankine: f64) -> bool {
    rankine >= 0.0
}

/// Get absolute zero in Celsius
pub async fn absolute_zero_celsius() -> f64 {
    ABSOLUTE_ZERO_C
}

/// Get absolute zero in Fahrenheit
pub async fn absolute_zero_fahrenheit() -> f64 {
    ABSOLUTE_ZERO_F
}

/// Get absolute zero in Kelvin
pub async fn absolute_zero_kelvin() -> f64 {
    0.0
}

/// Get absolute zero in Rankine
pub async fn absolute_zero_rankine() -> f64 {
    0.0
}