// Time Conversion Library
// All conversions use precise, internationally standardized values
// Primary references: SI units, IAU standards, CODATA values

// ============================================================================
// FUNDAMENTAL TIME CONSTANTS (in seconds)
// ============================================================================

// Quantum scale time units
const PLANCK_TIME: f64 = 5.391247e-44; // CODATA 2022
const YOCTOSECOND: f64 = 1e-24;
const ZEPTOSECOND: f64 = 1e-21;
const ATTOSECOND: f64 = 1e-18;
const FEMTOSECOND: f64 = 1e-15;
const PICOSECOND: f64 = 1e-12;
const NANOSECOND: f64 = 1e-9;
const SHAKE: f64 = 1e-8; // Nuclear physics unit (10 ns)
const MICROSECOND: f64 = 1e-6;
const MILLISECOND: f64 = 1e-3;

// Standard time units
const MINUTE: f64 = 60.0;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;
const WEEK: f64 = 604800.0;
const FORTNIGHT: f64 = 1209600.0; // 14 days

// Year definitions (precise astronomical values)
const JULIAN_YEAR: f64 = 31557600.0; // Exactly 365.25 days (IAU definition)
const GREGORIAN_YEAR: f64 = 31556952.0; // Exactly 365.2425 days
const TROPICAL_YEAR: f64 = 31556925.445; // IUGG/IAU 2000 epoch
const SIDEREAL_YEAR: f64 = 31558149.7632; // J2000.0 epoch (365.25636 days)
const ANOMALISTIC_YEAR: f64 = 31558432.5504; // Perihelion to perihelion
const ECLIPSE_YEAR: f64 = 29947974.3328; // Node to node (346.620 days)
const GAUSSIAN_YEAR: f64 = 31558149.7635456; // Kepler's third law constant

// Month definitions
const SYNODIC_MONTH: f64 = 2551442.8896; // Lunar phase cycle (29.530589 days)
const SIDEREAL_MONTH: f64 = 2360591.5072; // 27.321661 days
const TROPICAL_MONTH: f64 = 2360584.6976; // 27.321582 days
const ANOMALISTIC_MONTH: f64 = 2380713.0368; // 27.554550 days
const DRACONIC_MONTH: f64 = 2351135.8368; // 27.212221 days (nodal)

// Day variations
const SIDEREAL_DAY: f64 = 86164.0905; // Earth rotation relative to stars
const SOLAR_DAY: f64 = 86400.0; // Mean solar day (24 hours)

// Special units
const JIFFY_PHYSICS: f64 = 3.3356409519815e-9; // Light travel time for 1 femtometer
const JIFFY_ELECTRONICS: f64 = 0.01666666667; // 1/60 second (AC cycle time)
const JIFFY_COMPUTING: f64 = 0.01; // 10 milliseconds (variable by system)
const SVEDBERG: f64 = 1e-13; // Used in sedimentation coefficients
const MOMENT: f64 = 90.0; // Medieval unit (1/40 of solar hour = 90 seconds)
const KE: f64 = 864.0; // Chinese traditional unit (1/100 day)

// Large time scales
const DECADE: f64 = 315576000.0; // 10 Julian years
const CENTURY: f64 = 3155760000.0; // 100 Julian years
const MILLENNIUM: f64 = 31557600000.0; // 1000 Julian years

// Astronomical long periods
const GALACTIC_YEAR: f64 = 7.25e15; // ~230 million years (solar orbit around galaxy)
const COSMOLOGICAL_DECADE: f64 = 3.1536e7; // Base-10 logarithmic time scale

// ============================================================================
// SECOND TO OTHER UNITS
// ============================================================================

/// Second to millisecond
pub async fn s_to_ms(num: f64) -> f64 {
    num * 1000.0
}

/// Second to microsecond
pub async fn s_to_us(num: f64) -> f64 {
    num * 1_000_000.0
}

/// Second to nanosecond
pub async fn s_to_ns(num: f64) -> f64 {
    num * 1_000_000_000.0
}

/// Second to picosecond
pub async fn s_to_ps(num: f64) -> f64 {
    num * 1e12
}

/// Second to femtosecond
pub async fn s_to_fs(num: f64) -> f64 {
    num * 1e15
}

/// Second to attosecond
pub async fn s_to_as(num: f64) -> f64 {
    num * 1e18
}

/// Second to zeptosecond
pub async fn s_to_zs(num: f64) -> f64 {
    num * 1e21
}

/// Second to yoctosecond
pub async fn s_to_ys(num: f64) -> f64 {
    num * 1e24
}

/// Second to shake (nuclear physics)
pub async fn s_to_shake(num: f64) -> f64 {
    num / SHAKE
}

/// Second to Planck time
pub async fn s_to_planck_time(num: f64) -> f64 {
    num / PLANCK_TIME
}

/// Second to minute
pub async fn s_to_min(num: f64) -> f64 {
    num / MINUTE
}

/// Second to hour
pub async fn s_to_h(num: f64) -> f64 {
    num / HOUR
}

/// Second to day
pub async fn s_to_day(num: f64) -> f64 {
    num / DAY
}

/// Second to week
pub async fn s_to_week(num: f64) -> f64 {
    num / WEEK
}

/// Second to fortnight
pub async fn s_to_fortnight(num: f64) -> f64 {
    num / FORTNIGHT
}

/// Second to Julian year
pub async fn s_to_julian_year(num: f64) -> f64 {
    num / JULIAN_YEAR
}

/// Second to Gregorian year
pub async fn s_to_gregorian_year(num: f64) -> f64 {
    num / GREGORIAN_YEAR
}

/// Second to tropical year
pub async fn s_to_tropical_year(num: f64) -> f64 {
    num / TROPICAL_YEAR
}

/// Second to sidereal year
pub async fn s_to_sidereal_year(num: f64) -> f64 {
    num / SIDEREAL_YEAR
}

/// Second to sidereal day
pub async fn s_to_sidereal_day(num: f64) -> f64 {
    num / SIDEREAL_DAY
}

/// Second to synodic month (lunar month)
pub async fn s_to_synodic_month(num: f64) -> f64 {
    num / SYNODIC_MONTH
}

/// Second to jiffy (physics - light travel time)
pub async fn s_to_jiffy_physics(num: f64) -> f64 {
    num / JIFFY_PHYSICS
}

/// Second to jiffy (electronics - 1/60 s)
pub async fn s_to_jiffy_electronics(num: f64) -> f64 {
    num / JIFFY_ELECTRONICS
}

/// Second to Svedberg
pub async fn s_to_svedberg(num: f64) -> f64 {
    num / SVEDBERG
}

// ============================================================================
// OTHER UNITS TO SECOND
// ============================================================================

/// Millisecond to second
pub async fn ms_to_s(num: f64) -> f64 {
    num / 1000.0
}

/// Microsecond to second
pub async fn us_to_s(num: f64) -> f64 {
    num / 1_000_000.0
}

/// Nanosecond to second
pub async fn ns_to_s(num: f64) -> f64 {
    num / 1_000_000_000.0
}

/// Picosecond to second
pub async fn ps_to_s(num: f64) -> f64 {
    num / 1e12
}

/// Femtosecond to second
pub async fn fs_to_s(num: f64) -> f64 {
    num / 1e15
}

/// Attosecond to second
pub async fn as_to_s(num: f64) -> f64 {
    num / 1e18
}

/// Zeptosecond to second
pub async fn zs_to_s(num: f64) -> f64 {
    num / 1e21
}

/// Yoctosecond to second
pub async fn ys_to_s(num: f64) -> f64 {
    num / 1e24
}

/// Shake to second
pub async fn shake_to_s(num: f64) -> f64 {
    num * SHAKE
}

/// Planck time to second
pub async fn planck_time_to_s(num: f64) -> f64 {
    num * PLANCK_TIME
}

/// Minute to second
pub async fn min_to_s(num: f64) -> f64 {
    num * MINUTE
}

/// Hour to second
pub async fn h_to_s(num: f64) -> f64 {
    num * HOUR
}

/// Day to second
pub async fn day_to_s(num: f64) -> f64 {
    num * DAY
}

/// Week to second
pub async fn week_to_s(num: f64) -> f64 {
    num * WEEK
}

/// Fortnight to second
pub async fn fortnight_to_s(num: f64) -> f64 {
    num * FORTNIGHT
}

/// Julian year to second
pub async fn julian_year_to_s(num: f64) -> f64 {
    num * JULIAN_YEAR
}

/// Gregorian year to second
pub async fn gregorian_year_to_s(num: f64) -> f64 {
    num * GREGORIAN_YEAR
}

/// Tropical year to second
pub async fn tropical_year_to_s(num: f64) -> f64 {
    num * TROPICAL_YEAR
}

/// Sidereal year to second
pub async fn sidereal_year_to_s(num: f64) -> f64 {
    num * SIDEREAL_YEAR
}

/// Anomalistic year to second
pub async fn anomalistic_year_to_s(num: f64) -> f64 {
    num * ANOMALISTIC_YEAR
}

/// Eclipse year to second
pub async fn eclipse_year_to_s(num: f64) -> f64 {
    num * ECLIPSE_YEAR
}

/// Gaussian year to second
pub async fn gaussian_year_to_s(num: f64) -> f64 {
    num * GAUSSIAN_YEAR
}

/// Sidereal day to second
pub async fn sidereal_day_to_s(num: f64) -> f64 {
    num * SIDEREAL_DAY
}

/// Synodic month to second
pub async fn synodic_month_to_s(num: f64) -> f64 {
    num * SYNODIC_MONTH
}

/// Sidereal month to second
pub async fn sidereal_month_to_s(num: f64) -> f64 {
    num * SIDEREAL_MONTH
}

/// Tropical month to second
pub async fn tropical_month_to_s(num: f64) -> f64 {
    num * TROPICAL_MONTH
}

/// Anomalistic month to second
pub async fn anomalistic_month_to_s(num: f64) -> f64 {
    num * ANOMALISTIC_MONTH
}

/// Draconic month to second
pub async fn draconic_month_to_s(num: f64) -> f64 {
    num * DRACONIC_MONTH
}

/// Jiffy (physics) to second
pub async fn jiffy_physics_to_s(num: f64) -> f64 {
    num * JIFFY_PHYSICS
}

/// Jiffy (electronics) to second
pub async fn jiffy_electronics_to_s(num: f64) -> f64 {
    num * JIFFY_ELECTRONICS
}

/// Jiffy (computing) to second
pub async fn jiffy_computing_to_s(num: f64) -> f64 {
    num * JIFFY_COMPUTING
}

/// Svedberg to second
pub async fn svedberg_to_s(num: f64) -> f64 {
    num * SVEDBERG
}

/// Moment to second
pub async fn moment_to_s(num: f64) -> f64 {
    num * MOMENT
}

/// Ke to second
pub async fn ke_to_s(num: f64) -> f64 {
    num * KE
}

/// Decade to second
pub async fn decade_to_s(num: f64) -> f64 {
    num * DECADE
}

/// Century to second
pub async fn century_to_s(num: f64) -> f64 {
    num * CENTURY
}

/// Millennium to second
pub async fn millennium_to_s(num: f64) -> f64 {
    num * MILLENNIUM
}

/// Galactic year to second
pub async fn galactic_year_to_s(num: f64) -> f64 {
    num * GALACTIC_YEAR
}

// ============================================================================
// MINUTE CONVERSIONS
// ============================================================================

pub async fn min_to_ms(num: f64) -> f64 {
    num * 60000.0
}

pub async fn min_to_h(num: f64) -> f64 {
    num / 60.0
}

pub async fn min_to_day(num: f64) -> f64 {
    num / 1440.0
}

pub async fn min_to_week(num: f64) -> f64 {
    num / 10080.0
}

pub async fn h_to_min(num: f64) -> f64 {
    num * 60.0
}

pub async fn day_to_min(num: f64) -> f64 {
    num * 1440.0
}

// ============================================================================
// HOUR CONVERSIONS
// ============================================================================

pub async fn h_to_ms(num: f64) -> f64 {
    num * 3_600_000.0
}

pub async fn h_to_day(num: f64) -> f64 {
    num / 24.0
}

pub async fn h_to_week(num: f64) -> f64 {
    num / 168.0
}

pub async fn day_to_h(num: f64) -> f64 {
    num * 24.0
}

pub async fn week_to_h(num: f64) -> f64 {
    num * 168.0
}

// ============================================================================
// DAY CONVERSIONS
// ============================================================================

pub async fn day_to_week(num: f64) -> f64 {
    num / 7.0
}

pub async fn day_to_fortnight(num: f64) -> f64 {
    num / 14.0
}

pub async fn day_to_julian_year(num: f64) -> f64 {
    num / 365.25
}

pub async fn day_to_gregorian_year(num: f64) -> f64 {
    num / 365.2425
}

pub async fn day_to_tropical_year(num: f64) -> f64 {
    num / 365.24219
}

pub async fn day_to_sidereal_year(num: f64) -> f64 {
    num / 365.25636
}

pub async fn week_to_day(num: f64) -> f64 {
    num * 7.0
}

pub async fn fortnight_to_day(num: f64) -> f64 {
    num * 14.0
}

// ============================================================================
// YEAR CONVERSIONS (INTERCONVERSION)
// ============================================================================

pub async fn julian_year_to_tropical_year(num: f64) -> f64 {
    num * (JULIAN_YEAR / TROPICAL_YEAR)
}

pub async fn julian_year_to_sidereal_year(num: f64) -> f64 {
    num * (JULIAN_YEAR / SIDEREAL_YEAR)
}

pub async fn julian_year_to_gregorian_year(num: f64) -> f64 {
    num * (JULIAN_YEAR / GREGORIAN_YEAR)
}

pub async fn tropical_year_to_julian_year(num: f64) -> f64 {
    num * (TROPICAL_YEAR / JULIAN_YEAR)
}

pub async fn tropical_year_to_sidereal_year(num: f64) -> f64 {
    num * (TROPICAL_YEAR / SIDEREAL_YEAR)
}

pub async fn sidereal_year_to_tropical_year(num: f64) -> f64 {
    num * (SIDEREAL_YEAR / TROPICAL_YEAR)
}

pub async fn sidereal_year_to_julian_year(num: f64) -> f64 {
    num * (SIDEREAL_YEAR / JULIAN_YEAR)
}

pub async fn gregorian_year_to_julian_year(num: f64) -> f64 {
    num * (GREGORIAN_YEAR / JULIAN_YEAR)
}

pub async fn gregorian_year_to_tropical_year(num: f64) -> f64 {
    num * (GREGORIAN_YEAR / TROPICAL_YEAR)
}

// ============================================================================
// YEAR TO COMMON UNITS
// ============================================================================

pub async fn julian_year_to_day(num: f64) -> f64 {
    num * 365.25
}

pub async fn julian_year_to_h(num: f64) -> f64 {
    num * 8766.0
}

pub async fn julian_year_to_min(num: f64) -> f64 {
    num * 525960.0
}

pub async fn tropical_year_to_day(num: f64) -> f64 {
    num * 365.24219
}

pub async fn sidereal_year_to_day(num: f64) -> f64 {
    num * 365.25636
}

pub async fn gregorian_year_to_day(num: f64) -> f64 {
    num * 365.2425
}

// ============================================================================
// MONTH CONVERSIONS
// ============================================================================

pub async fn synodic_month_to_day(num: f64) -> f64 {
    num * 29.530589
}

pub async fn sidereal_month_to_day(num: f64) -> f64 {
    num * 27.321661
}

pub async fn tropical_month_to_day(num: f64) -> f64 {
    num * 27.321582
}

pub async fn synodic_month_to_h(num: f64) -> f64 {
    num * 708.734136
}

pub async fn day_to_synodic_month(num: f64) -> f64 {
    num / 29.530589
}

pub async fn day_to_sidereal_month(num: f64) -> f64 {
    num / 27.321661
}

// ============================================================================
// QUANTUM SCALE CONVERSIONS
// ============================================================================

pub async fn planck_time_to_fs(num: f64) -> f64 {
    num * (PLANCK_TIME / FEMTOSECOND)
}

pub async fn fs_to_planck_time(num: f64) -> f64 {
    num * (FEMTOSECOND / PLANCK_TIME)
}

pub async fn fs_to_as(num: f64) -> f64 {
    num * 1000.0
}

pub async fn as_to_fs(num: f64) -> f64 {
    num / 1000.0
}

pub async fn as_to_zs(num: f64) -> f64 {
    num * 1000.0
}

pub async fn zs_to_as(num: f64) -> f64 {
    num / 1000.0
}

pub async fn ps_to_fs(num: f64) -> f64 {
    num * 1000.0
}

pub async fn fs_to_ps(num: f64) -> f64 {
    num / 1000.0
}

pub async fn ns_to_ps(num: f64) -> f64 {
    num * 1000.0
}

pub async fn ps_to_ns(num: f64) -> f64 {
    num / 1000.0
}

pub async fn us_to_ns(num: f64) -> f64 {
    num * 1000.0
}

pub async fn ns_to_us(num: f64) -> f64 {
    num / 1000.0
}

pub async fn ms_to_us(num: f64) -> f64 {
    num * 1000.0
}

pub async fn us_to_ms(num: f64) -> f64 {
    num / 1000.0
}

pub async fn shake_to_ns(num: f64) -> f64 {
    num * 10.0
}

pub async fn ns_to_shake(num: f64) -> f64 {
    num / 10.0
}

// ============================================================================
// SPECIAL UNIT CONVERSIONS
// ============================================================================

pub async fn sidereal_day_to_solar_day(num: f64) -> f64 {
    num * (SIDEREAL_DAY / SOLAR_DAY)
}

pub async fn solar_day_to_sidereal_day(num: f64) -> f64 {
    num * (SOLAR_DAY / SIDEREAL_DAY)
}

pub async fn jiffy_physics_to_fs(num: f64) -> f64 {
    num * (JIFFY_PHYSICS / FEMTOSECOND)
}

pub async fn jiffy_electronics_to_ms(num: f64) -> f64 {
    num * (JIFFY_ELECTRONICS / MILLISECOND)
}

pub async fn moment_to_min(num: f64) -> f64 {
    num * 1.5
}

pub async fn ke_to_min(num: f64) -> f64 {
    num * 14.4
}

// ============================================================================
// LARGE SCALE CONVERSIONS
// ============================================================================

pub async fn decade_to_year(num: f64) -> f64 {
    num * 10.0
}

pub async fn century_to_year(num: f64) -> f64 {
    num * 100.0
}

pub async fn millennium_to_year(num: f64) -> f64 {
    num * 1000.0
}

pub async fn year_to_decade(num: f64) -> f64 {
    num / 10.0
}

pub async fn year_to_century(num: f64) -> f64 {
    num / 100.0
}

pub async fn year_to_millennium(num: f64) -> f64 {
    num / 1000.0
}

pub async fn galactic_year_to_julian_year(num: f64) -> f64 {
    num * (GALACTIC_YEAR / JULIAN_YEAR)
}

pub async fn julian_year_to_galactic_year(num: f64) -> f64 {
    num * (JULIAN_YEAR / GALACTIC_YEAR)
}