// Physical Constants Library
// Based on CODATA 2022 recommended values
// Source: physics.nist.gov/constants
//
// Note: Some constants are exact by definition since the 2019 SI redefinition
// These are marked with (exact) in comments

// ============================================================================
// UNIVERSAL CONSTANTS
// ============================================================================

/// Speed of light in vacuum (exact by SI definition)
/// c = 299,792,458 m/s
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Newtonian constant of gravitation
/// G = 6.67430(15) × 10⁻¹¹ m³/(kg·s²)
/// Relative uncertainty: 2.2 × 10⁻⁵
pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;

/// Planck constant (exact by SI definition)
/// h = 6.62607015 × 10⁻³⁴ J·s
pub const PLANCK_CONSTANT: f64 = 6.62607015e-34;

/// Reduced Planck constant (ℏ = h/2π) (exact)
/// ℏ = 1.054571817... × 10⁻³⁴ J·s
pub const REDUCED_PLANCK_CONSTANT: f64 = 1.054571817e-34;

/// Magnetic constant (vacuum permeability)
/// μ₀ = 1.25663706212(19) × 10⁻⁶ N/A²
pub const MAGNETIC_CONSTANT: f64 = 1.25663706212e-6;

/// Electric constant (vacuum permittivity)
/// ε₀ = 8.8541878128(13) × 10⁻¹² F/m
pub const ELECTRIC_CONSTANT: f64 = 8.8541878128e-12;

/// Characteristic impedance of vacuum
/// Z₀ = 376.730313668(57) Ω
pub const IMPEDANCE_OF_VACUUM: f64 = 376.730313668;

// ============================================================================
// ELECTROMAGNETIC CONSTANTS
// ============================================================================

/// Elementary charge (exact by SI definition)
/// e = 1.602176634 × 10⁻¹⁹ C
pub const ELEMENTARY_CHARGE: f64 = 1.602176634e-19;

/// Magnetic flux quantum
/// Φ₀ = 2.067833848... × 10⁻¹⁵ Wb
pub const MAGNETIC_FLUX_QUANTUM: f64 = 2.067833848e-15;

/// Conductance quantum
/// G₀ = 7.748091729... × 10⁻⁵ S
pub const CONDUCTANCE_QUANTUM: f64 = 7.748091729e-5;

/// Josephson constant
/// Kⱼ = 483597.8484... × 10⁹ Hz/V
pub const JOSEPHSON_CONSTANT: f64 = 483_597_848_400_000.0;

/// Von Klitzing constant
/// Rₖ = 25812.80745... Ω
pub const VON_KLITZING_CONSTANT: f64 = 25_812.80745;

/// Bohr magneton
/// μB = 9.2740100783(28) × 10⁻²⁴ J/T
pub const BOHR_MAGNETON: f64 = 9.2740100783e-24;

/// Nuclear magneton
/// μN = 5.0507837461(15) × 10⁻²⁷ J/T
pub const NUCLEAR_MAGNETON: f64 = 5.0507837461e-27;

// ============================================================================
// ATOMIC AND NUCLEAR CONSTANTS
// ============================================================================

/// Fine-structure constant
/// α = 7.2973525643(11) × 10⁻³
pub const FINE_STRUCTURE_CONSTANT: f64 = 7.2973525643e-3;

/// Inverse fine-structure constant
/// α⁻¹ = 137.035999177(21)
pub const INVERSE_FINE_STRUCTURE: f64 = 137.035999177;

/// Rydberg constant
/// R∞ = 10973731.568157(12) m⁻¹
pub const RYDBERG_CONSTANT: f64 = 10_973_731.568157;

/// Bohr radius
/// a₀ = 5.29177210544(82) × 10⁻¹¹ m
pub const BOHR_RADIUS: f64 = 5.29177210544e-11;

/// Classical electron radius
/// rₑ = 2.8179403205(13) × 10⁻¹⁵ m
pub const CLASSICAL_ELECTRON_RADIUS: f64 = 2.8179403205e-15;

/// Compton wavelength of electron
/// λC = 2.42631023538(16) × 10⁻¹² m
pub const COMPTON_WAVELENGTH_ELECTRON: f64 = 2.42631023538e-12;

/// Compton wavelength of proton
/// λC,p = 1.32140985538(40) × 10⁻¹⁵ m
pub const COMPTON_WAVELENGTH_PROTON: f64 = 1.32140985538e-15;

/// Compton wavelength of neutron
/// λC,n = 1.31959090379(72) × 10⁻¹⁵ m
pub const COMPTON_WAVELENGTH_NEUTRON: f64 = 1.31959090379e-15;

/// Proton charge radius
/// rₚ = 8.4074(51) × 10⁻¹⁶ m
pub const PROTON_CHARGE_RADIUS: f64 = 8.4074e-16;

// ============================================================================
// PARTICLE MASSES
// ============================================================================

/// Electron mass
/// mₑ = 9.1093837139(28) × 10⁻³¹ kg
pub const ELECTRON_MASS: f64 = 9.1093837139e-31;

/// Proton mass
/// mₚ = 1.67262192595(52) × 10⁻²⁷ kg
pub const PROTON_MASS: f64 = 1.67262192595e-27;

/// Neutron mass
/// mₙ = 1.67492750056(85) × 10⁻²⁷ kg
pub const NEUTRON_MASS: f64 = 1.67492750056e-27;

/// Deuteron mass
/// mₐ = 3.3435837768(10) × 10⁻²⁷ kg
pub const DEUTERON_MASS: f64 = 3.3435837768e-27;

/// Triton mass
/// mₜ = 5.0073567512(15) × 10⁻²⁷ kg
pub const TRITON_MASS: f64 = 5.0073567512e-27;

/// Helion mass (He-3 nucleus)
/// mₕ = 5.0064127862(15) × 10⁻²⁷ kg
pub const HELION_MASS: f64 = 5.0064127862e-27;

/// Alpha particle mass (He-4 nucleus)
/// mα = 6.6446573450(21) × 10⁻²⁷ kg
pub const ALPHA_PARTICLE_MASS: f64 = 6.6446573450e-27;

/// Muon mass
/// mμ = 1.883531627(42) × 10⁻²⁸ kg
pub const MUON_MASS: f64 = 1.883531627e-28;

/// Tau mass
/// mτ = 3.16754(21) × 10⁻²⁷ kg
pub const TAU_MASS: f64 = 3.16754e-27;

// ============================================================================
// MASS RATIOS
// ============================================================================

/// Proton-electron mass ratio
/// mₚ/mₑ = 1836.152673426(32)
pub const PROTON_ELECTRON_MASS_RATIO: f64 = 1836.152673426;

/// Neutron-electron mass ratio
/// mₙ/mₑ = 1838.683661399(49)
pub const NEUTRON_ELECTRON_MASS_RATIO: f64 = 1838.683661399;

/// Deuteron-electron mass ratio
/// mₐ/mₑ = 3670.483014120(73)
pub const DEUTERON_ELECTRON_MASS_RATIO: f64 = 3670.483014120;

/// Alpha particle-electron mass ratio
/// mα/mₑ = 7294.299542389(18)
pub const ALPHA_ELECTRON_MASS_RATIO: f64 = 7294.299542389;

// ============================================================================
// PHYSICO-CHEMICAL CONSTANTS
// ============================================================================

/// Avogadro constant (exact by SI definition)
/// Nₐ = 6.02214076 × 10²³ mol⁻¹
pub const AVOGADRO_CONSTANT: f64 = 6.02214076e23;

/// Boltzmann constant (exact by SI definition)
/// k = 1.380649 × 10⁻²³ J/K
pub const BOLTZMANN_CONSTANT: f64 = 1.380649e-23;

/// Molar gas constant
/// R = 8.314462618... J/(mol·K)
pub const MOLAR_GAS_CONSTANT: f64 = 8.314462618;

/// Faraday constant
/// F = 96485.33212... C/mol
pub const FARADAY_CONSTANT: f64 = 96_485.33212;

/// Molar volume of ideal gas (273.15 K, 100 kPa)
/// Vₘ = 22.71095464... × 10⁻³ m³/mol
pub const MOLAR_VOLUME_IDEAL_GAS_100KPA: f64 = 0.02271095464;

/// Molar volume of ideal gas (273.15 K, 101.325 kPa)
/// Vₘ = 22.41396954... × 10⁻³ m³/mol
pub const MOLAR_VOLUME_IDEAL_GAS_101325PA: f64 = 0.02241396954;

/// Loschmidt constant (273.15 K, 101.325 kPa)
/// n₀ = 2.686780111... × 10²⁵ m⁻³
pub const LOSCHMIDT_CONSTANT: f64 = 2.686780111e25;

/// Stefan-Boltzmann constant
/// σ = 5.670374419... × 10⁻⁸ W/(m²·K⁴)
pub const STEFAN_BOLTZMANN_CONSTANT: f64 = 5.670374419e-8;

/// First radiation constant
/// c₁ = 3.741771852... × 10⁻¹⁶ W·m²
pub const FIRST_RADIATION_CONSTANT: f64 = 3.741771852e-16;

/// Second radiation constant
/// c₂ = 1.438776877... × 10⁻² m·K
pub const SECOND_RADIATION_CONSTANT: f64 = 1.438776877e-2;

/// Wien wavelength displacement law constant
/// b = 2.897771955... × 10⁻³ m·K
pub const WIEN_WAVELENGTH_DISPLACEMENT: f64 = 2.897771955e-3;

/// Wien frequency displacement law constant
/// b' = 5.878925757... × 10¹⁰ Hz/K
pub const WIEN_FREQUENCY_DISPLACEMENT: f64 = 5.878925757e10;

// ============================================================================
// ATOMIC MASS CONSTANTS
// ============================================================================

/// Atomic mass constant (unified atomic mass unit)
/// mᵤ = 1.66053906892(52) × 10⁻²⁷ kg
pub const ATOMIC_MASS_CONSTANT: f64 = 1.66053906892e-27;

/// Molar mass constant
/// Mᵤ = 0.99999999965(30) × 10⁻³ kg/mol
pub const MOLAR_MASS_CONSTANT: f64 = 0.99999999965e-3;

/// Molar mass of carbon-12
/// M(¹²C) = 11.9999999958(36) × 10⁻³ kg/mol
pub const MOLAR_MASS_CARBON_12: f64 = 11.9999999958e-3;

// ============================================================================
// ENERGY CONVERSION FACTORS
// ============================================================================

/// Electron volt to joule conversion (exact)
/// 1 eV = 1.602176634 × 10⁻¹⁹ J
pub const ELECTRON_VOLT: f64 = 1.602176634e-19;

/// Hartree energy
/// Eₕ = 4.3597447222060(48) × 10⁻¹⁸ J
pub const HARTREE_ENERGY: f64 = 4.3597447222060e-18;

/// Energy equivalent of electron mass
/// mₑc² = 8.1871057880(25) × 10⁻¹⁴ J
pub const ELECTRON_MASS_ENERGY_EQUIVALENT: f64 = 8.1871057880e-14;

/// Energy equivalent of proton mass
/// mₚc² = 1.50327761598(46) × 10⁻¹⁰ J
pub const PROTON_MASS_ENERGY_EQUIVALENT: f64 = 1.50327761598e-10;

/// Energy equivalent of neutron mass
/// mₙc² = 1.50534976813(77) × 10⁻¹⁰ J
pub const NEUTRON_MASS_ENERGY_EQUIVALENT: f64 = 1.50534976813e-10;

/// Energy equivalent of atomic mass unit
/// 1 u·c² = 1.49241808796(45) × 10⁻¹⁰ J
pub const ATOMIC_MASS_UNIT_ENERGY_EQUIVALENT: f64 = 1.49241808796e-10;

// ============================================================================
// NATURAL UNITS (PLANCK UNITS)
// ============================================================================

/// Planck length
/// lₚ = √(ℏG/c³) = 1.616255(18) × 10⁻³⁵ m
pub const PLANCK_LENGTH: f64 = 1.616255e-35;

/// Planck mass
/// mₚ = √(ℏc/G) = 2.176434(24) × 10⁻⁸ kg
pub const PLANCK_MASS: f64 = 2.176434e-8;

/// Planck time
/// tₚ = √(ℏG/c⁵) = 5.391247(60) × 10⁻⁴⁴ s
pub const PLANCK_TIME: f64 = 5.391247e-44;

/// Planck temperature
/// Tₚ = √(ℏc⁵/Gk²) = 1.416784(16) × 10³² K
pub const PLANCK_TEMPERATURE: f64 = 1.416784e32;

// ============================================================================
// MAGNETIC MOMENTS
// ============================================================================

/// Electron magnetic moment
/// μₑ = -9.2847646917(29) × 10⁻²⁴ J/T
pub const ELECTRON_MAGNETIC_MOMENT: f64 = -9.2847646917e-24;

/// Proton magnetic moment
/// μₚ = 1.41060679545(60) × 10⁻²⁶ J/T
pub const PROTON_MAGNETIC_MOMENT: f64 = 1.41060679545e-26;

/// Neutron magnetic moment
/// μₙ = -9.6623653(23) × 10⁻²⁷ J/T
pub const NEUTRON_MAGNETIC_MOMENT: f64 = -9.6623653e-27;

/// Muon magnetic moment
/// μμ = -4.49044830(10) × 10⁻²⁶ J/T
pub const MUON_MAGNETIC_MOMENT: f64 = -4.49044830e-26;

// ============================================================================
// G-FACTORS AND ANOMALIES
// ============================================================================

/// Electron g-factor
/// gₑ = -2.00231930436092(36)
pub const ELECTRON_G_FACTOR: f64 = -2.00231930436092;

/// Muon g-factor
/// gμ = -2.00233184123(82)
pub const MUON_G_FACTOR: f64 = -2.00233184123;

/// Proton g-factor
/// gₚ = 5.5856946893(16)
pub const PROTON_G_FACTOR: f64 = 5.5856946893;

/// Electron magnetic moment anomaly
/// aₑ = 1.15965218128(18) × 10⁻³
pub const ELECTRON_MAGNETIC_MOMENT_ANOMALY: f64 = 1.15965218128e-3;

/// Muon magnetic moment anomaly
/// aμ = 1.16592059(22) × 10⁻³
pub const MUON_MAGNETIC_MOMENT_ANOMALY: f64 = 1.16592059e-3;

// ============================================================================
// WEAK INTERACTION CONSTANTS
// ============================================================================

/// Fermi coupling constant
/// Gᶠ/(ℏc)³ = 1.1663788(7) × 10⁻⁵ GeV⁻²
pub const FERMI_COUPLING_CONSTANT: f64 = 1.1663788e-5;

/// Weak mixing angle (sin²θW)
/// sin²θW = 0.23121(4)
pub const WEAK_MIXING_ANGLE: f64 = 0.23121;

// ============================================================================
// STANDARD GRAVITY AND ATMOSPHERIC CONSTANTS
// ============================================================================

/// Standard acceleration of gravity (exact by definition)
/// gₙ = 9.80665 m/s²
pub const STANDARD_GRAVITY: f64 = 9.80665;

/// Standard atmosphere (exact by definition)
/// 1 atm = 101325 Pa
pub const STANDARD_ATMOSPHERE: f64 = 101_325.0;

// ============================================================================
// ASTRONOMICAL CONSTANTS
// ============================================================================

/// Astronomical unit (exact by IAU 2012 definition)
/// 1 AU = 149597870700 m
pub const ASTRONOMICAL_UNIT: f64 = 149_597_870_700.0;

/// Parsec (derived from AU)
/// 1 pc = 3.0856775814913673 × 10¹⁶ m
pub const PARSEC: f64 = 3.0856775814913673e16;

/// Light year (derived from c)
/// 1 ly = 9.4607304725808 × 10¹⁵ m
pub const LIGHT_YEAR: f64 = 9.4607304725808e15;

/// Solar mass (IAU 2015 nominal value)
/// M☉ = 1.98847 × 10³⁰ kg
pub const SOLAR_MASS: f64 = 1.98847e30;

/// Solar radius (IAU 2015 nominal value)
/// R☉ = 6.957 × 10⁸ m
pub const SOLAR_RADIUS: f64 = 6.957e8;

/// Solar luminosity (IAU 2015 nominal value)
/// L☉ = 3.828 × 10²⁶ W
pub const SOLAR_LUMINOSITY: f64 = 3.828e26;

/// Earth mass
/// M⊕ = 5.9722 × 10²⁴ kg
pub const EARTH_MASS: f64 = 5.9722e24;

/// Earth equatorial radius (IUGG value)
/// R⊕ = 6378137 m
pub const EARTH_EQUATORIAL_RADIUS: f64 = 6_378_137.0;

/// Earth polar radius (IUGG value)
/// R⊕,p = 6356752 m
pub const EARTH_POLAR_RADIUS: f64 = 6_356_752.0;

// ============================================================================
// CONVERSION FACTORS (DIMENSIONLESS)
// ============================================================================

/// Degrees to radians
pub const DEGREES_TO_RADIANS: f64 = 0.017453292519943295;

/// Radians to degrees
pub const RADIANS_TO_DEGREES: f64 = 57.29577951308232;

/// Pi (π)
pub const PI: f64 = std::f64::consts::PI;

/// Euler's number (e)
pub const E: f64 = std::f64::consts::E;

/// Golden ratio (φ)
pub const GOLDEN_RATIO: f64 = 1.618033988749895;

// ============================================================================
// HELPER FUNCTIONS FOR ASYNC API COMPATIBILITY
// ============================================================================

/// Get speed of light in vacuum
pub async fn speed_of_light() -> f64 {
    SPEED_OF_LIGHT
}

/// Get gravitational constant
pub async fn gravitational_constant() -> f64 {
    GRAVITATIONAL_CONSTANT
}

/// Get Planck constant
pub async fn planck_constant() -> f64 {
    PLANCK_CONSTANT
}

/// Get reduced Planck constant (ℏ)
pub async fn reduced_planck_constant() -> f64 {
    REDUCED_PLANCK_CONSTANT
}

/// Get elementary charge
pub async fn elementary_charge() -> f64 {
    ELEMENTARY_CHARGE
}

/// Get Avogadro constant
pub async fn avogadro_constant() -> f64 {
    AVOGADRO_CONSTANT
}

/// Get Boltzmann constant
pub async fn boltzmann_constant() -> f64 {
    BOLTZMANN_CONSTANT
}

/// Get molar gas constant
pub async fn molar_gas_constant() -> f64 {
    MOLAR_GAS_CONSTANT
}

/// Get electron mass
pub async fn electron_mass() -> f64 {
    ELECTRON_MASS
}

/// Get proton mass
pub async fn proton_mass() -> f64 {
    PROTON_MASS
}

/// Get neutron mass
pub async fn neutron_mass() -> f64 {
    NEUTRON_MASS
}

/// Get fine-structure constant
pub async fn fine_structure_constant() -> f64 {
    FINE_STRUCTURE_CONSTANT
}

/// Get Rydberg constant
pub async fn rydberg_constant() -> f64 {
    RYDBERG_CONSTANT
}

/// Get standard gravity
pub async fn standard_gravity() -> f64 {
    STANDARD_GRAVITY
}

/// Get standard atmosphere pressure
pub async fn standard_atmosphere() -> f64 {
    STANDARD_ATMOSPHERE
}

/// Get astronomical unit
pub async fn astronomical_unit() -> f64 {
    ASTRONOMICAL_UNIT
}

/// Get light year
pub async fn light_year() -> f64 {
    LIGHT_YEAR
}

/// Get parsec
pub async fn parsec() -> f64 {
    PARSEC
}

/// Get Stefan-Boltzmann constant
pub async fn stefan_boltzmann_constant() -> f64 {
    STEFAN_BOLTZMANN_CONSTANT
}

/// Get Faraday constant
pub async fn faraday_constant() -> f64 {
    FARADAY_CONSTANT
}

/// Get electron volt in joules
pub async fn electron_volt() -> f64 {
    ELECTRON_VOLT
}