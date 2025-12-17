// Mass/Weight Conversion Library
// All conversions use precise, internationally standardized values
// Primary reference: NIST, BIPM, and international measurement standards

// ============================================================================
// METRIC SYSTEM TO KILOGRAM CONVERSIONS
// ============================================================================

/// Gram to kilogram
pub async fn g_to_kg(num: f64) -> f64 {
    num / 1000.0
}

/// Milligram to kilogram
pub async fn mg_to_kg(num: f64) -> f64 {
    num / 1_000_000.0
}

/// Microgram to kilogram
pub async fn micro_g_to_kg(num: f64) -> f64 {
    num / 1_000_000_000.0
}

/// Nanogram to kilogram
pub async fn nano_g_to_kg(num: f64) -> f64 {
    num / 1_000_000_000_000.0
}

/// Picogram to kilogram
pub async fn pico_g_to_kg(num: f64) -> f64 {
    num / 1_000_000_000_000_000.0
}

/// Metric ton (tonne) to kilogram (exact)
pub async fn tonne_to_kg(num: f64) -> f64 {
    num * 1000.0
}

/// Megagram to kilogram (exact, same as tonne)
pub async fn megagram_to_kg(num: f64) -> f64 {
    num * 1000.0
}

/// Quintal to kilogram (exact)
pub async fn quintal_to_kg(num: f64) -> f64 {
    num * 100.0
}

/// Decigram to kilogram
pub async fn dg_to_kg(num: f64) -> f64 {
    num / 10_000.0
}

/// Centigram to kilogram
pub async fn cg_to_kg(num: f64) -> f64 {
    num / 100_000.0
}

/// Decagram to kilogram
pub async fn dag_to_kg(num: f64) -> f64 {
    num / 100.0
}

/// Hectogram to kilogram
pub async fn hg_to_kg(num: f64) -> f64 {
    num / 10.0
}

// ============================================================================
// AVOIRDUPOIS SYSTEM TO KILOGRAM CONVERSIONS
// ============================================================================

/// Pound (avoirdupois) to kilogram (exact by international agreement 1959)
/// 1 lb = 0.45359237 kg exactly
pub async fn lb_to_kg(num: f64) -> f64 {
    num * 0.45359237
}

/// Ounce (avoirdupois) to kilogram (exact)
/// 1 oz = 1/16 lb = 0.028349523125 kg
pub async fn oz_to_kg(num: f64) -> f64 {
    num * 0.028349523125
}

/// Short ton (US ton) to kilogram (exact)
/// 1 short ton = 2000 lb = 907.18474 kg
pub async fn short_ton_to_kg(num: f64) -> f64 {
    num * 907.18474
}

/// Long ton (Imperial ton, UK ton) to kilogram (exact)
/// 1 long ton = 2240 lb = 1016.0469088 kg
pub async fn long_ton_to_kg(num: f64) -> f64 {
    num * 1016.0469088
}

/// Dram (avoirdupois) to kilogram (exact)
/// 1 dram = 1/16 oz = 1/256 lb
pub async fn dram_to_kg(num: f64) -> f64 {
    num * 0.0017718451953125
}

/// Grain to kilogram (exact - fundamental to all traditional weight systems)
/// 1 grain = 64.79891 mg = 0.00006479891 kg exactly
pub async fn grain_to_kg(num: f64) -> f64 {
    num * 0.00006479891
}

/// Stone to kilogram (exact)
/// 1 stone = 14 lb = 6.35029318 kg
pub async fn stone_to_kg(num: f64) -> f64 {
    num * 6.35029318
}

/// Quarter (UK) to kilogram (exact)
/// 1 quarter = 28 lb = 12.70058636 kg
pub async fn quarter_to_kg(num: f64) -> f64 {
    num * 12.70058636
}

/// Hundredweight (short, US) to kilogram (exact)
/// 1 cwt (US) = 100 lb = 45.359237 kg
pub async fn cwt_short_to_kg(num: f64) -> f64 {
    num * 45.359237
}

/// Hundredweight (long, UK) to kilogram (exact)
/// 1 cwt (UK) = 112 lb = 50.80234544 kg
pub async fn cwt_long_to_kg(num: f64) -> f64 {
    num * 50.80234544
}

// ============================================================================
// TROY SYSTEM TO KILOGRAM CONVERSIONS
// ============================================================================

/// Troy ounce to kilogram (exact)
/// 1 oz t = 480 grains = 31.1034768 g = 0.0311034768 kg
pub async fn troy_oz_to_kg(num: f64) -> f64 {
    num * 0.0311034768
}

/// Troy pound to kilogram (exact)
/// 1 lb t = 12 troy oz = 0.3732417216 kg
pub async fn troy_lb_to_kg(num: f64) -> f64 {
    num * 0.3732417216
}

/// Pennyweight to kilogram (exact)
/// 1 dwt = 1/20 troy oz = 24 grains = 1.55517384 g
pub async fn pennyweight_to_kg(num: f64) -> f64 {
    num * 0.00155517384
}

// ============================================================================
// PRECIOUS METALS & GEMS
// ============================================================================

/// Carat (metric) to kilogram (exact by definition)
/// 1 carat = 200 mg = 0.0002 kg
pub async fn carat_to_kg(num: f64) -> f64 {
    num * 0.0002
}

/// Point (metric, for diamonds) to kilogram
/// 1 point = 0.01 carat = 2 mg
pub async fn point_to_kg(num: f64) -> f64 {
    num * 0.000002
}

/// Momme (Japanese, for pearls) to kilogram
/// 1 momme = 3.75 g (traditional Japanese unit)
pub async fn momme_to_kg(num: f64) -> f64 {
    num * 0.00375
}

// ============================================================================
// ATOMIC AND SCIENTIFIC UNITS
// ============================================================================

/// Atomic mass unit (unified) to kilogram (CODATA 2022)
/// 1 u = 1.66053906892(52) × 10⁻²⁷ kg
pub async fn amu_to_kg(num: f64) -> f64 {
    num * 1.66053906892e-27
}

/// Dalton to kilogram (same as amu)
/// 1 Da = 1 u
pub async fn dalton_to_kg(num: f64) -> f64 {
    num * 1.66053906892e-27
}

/// Electron mass to kilogram (CODATA 2022)
pub async fn electron_mass_to_kg(num: f64) -> f64 {
    num * 9.1093837139e-31
}

/// Proton mass to kilogram (CODATA 2022)
pub async fn proton_mass_to_kg(num: f64) -> f64 {
    num * 1.67262192595e-27
}

/// Neutron mass to kilogram (CODATA 2022)
pub async fn neutron_mass_to_kg(num: f64) -> f64 {
    num * 1.67492750056e-27
}

// ============================================================================
// ENGINEERING UNITS
// ============================================================================

/// Slug to kilogram (exact)
/// 1 slug = 1 lbf·s²/ft = 14.5939029372064 kg
pub async fn slug_to_kg(num: f64) -> f64 {
    num * 14.593903
}

/// Kilogram-force second squared per meter (technical mass unit) to kilogram
/// 1 TMU = 9.80665 kg (equal to 1 kgf·s²/m)
pub async fn tmu_to_kg(num: f64) -> f64 {
    num * 9.80665
}

// ============================================================================
// HISTORICAL AND REGIONAL UNITS
// ============================================================================

/// Scruple (apothecary) to kilogram (exact)
/// 1 scruple = 20 grains = 1.2959782 g
pub async fn scruple_to_kg(num: f64) -> f64 {
    num * 0.0012959782
}

/// Drachm (apothecary) to kilogram (exact)
/// 1 drachm = 3 scruples = 60 grains = 3.8879346 g
pub async fn drachm_to_kg(num: f64) -> f64 {
    num * 0.0038879346
}

/// Arroba (Spanish) to kilogram
/// 1 arroba ≈ 11.5 kg (varies by region, using common value)
pub async fn arroba_to_kg(num: f64) -> f64 {
    num * 11.5
}

/// Catty (East Asian) to kilogram
/// 1 catty (斤) ≈ 0.60478982 kg (Chinese standard)
pub async fn catty_to_kg(num: f64) -> f64 {
    num * 0.60478982
}

/// Tael (East Asian) to kilogram
/// 1 tael (两) ≈ 37.7994 g (Chinese standard)
pub async fn tael_to_kg(num: f64) -> f64 {
    num * 0.0377994
}

/// Picul to kilogram
/// 1 picul ≈ 60.478982 kg (Southeast Asian, equals 100 catties)
pub async fn picul_to_kg(num: f64) -> f64 {
    num * 60.478982
}

/// Maund (Indian) to kilogram
/// 1 maund ≈ 37.3242 kg (varies by region)
pub async fn maund_to_kg(num: f64) -> f64 {
    num * 37.3242
}

/// Seer (Indian) to kilogram
/// 1 seer ≈ 0.933105 kg
pub async fn seer_to_kg(num: f64) -> f64 {
    num * 0.933105
}

/// Tola (Indian) to kilogram
/// 1 tola = 11.6638038 g (standardized in India)
pub async fn tola_to_kg(num: f64) -> f64 {
    num * 0.0116638038
}

// ============================================================================
// KILOGRAM TO METRIC SYSTEM CONVERSIONS
// ============================================================================

pub async fn kg_to_g(num: f64) -> f64 {
    num * 1000.0
}

pub async fn kg_to_mg(num: f64) -> f64 {
    num * 1_000_000.0
}

pub async fn kg_to_micro_g(num: f64) -> f64 {
    num * 1_000_000_000.0
}

pub async fn kg_to_nano_g(num: f64) -> f64 {
    num * 1_000_000_000_000.0
}

pub async fn kg_to_tonne(num: f64) -> f64 {
    num / 1000.0
}

pub async fn kg_to_quintal(num: f64) -> f64 {
    num / 100.0
}

pub async fn kg_to_dg(num: f64) -> f64 {
    num * 10_000.0
}

pub async fn kg_to_cg(num: f64) -> f64 {
    num * 100_000.0
}

pub async fn kg_to_dag(num: f64) -> f64 {
    num * 100.0
}

pub async fn kg_to_hg(num: f64) -> f64 {
    num * 10.0
}

// ============================================================================
// KILOGRAM TO AVOIRDUPOIS SYSTEM CONVERSIONS
// ============================================================================

pub async fn kg_to_lb(num: f64) -> f64 {
    num / 0.45359237
}

pub async fn kg_to_oz(num: f64) -> f64 {
    num / 0.028349523125
}

pub async fn kg_to_short_ton(num: f64) -> f64 {
    num / 907.18474
}

pub async fn kg_to_long_ton(num: f64) -> f64 {
    num / 1016.0469088
}

pub async fn kg_to_dram(num: f64) -> f64 {
    num / 0.0017718451953125
}

pub async fn kg_to_grain(num: f64) -> f64 {
    num / 0.00006479891
}

pub async fn kg_to_stone(num: f64) -> f64 {
    num / 6.35029318
}

pub async fn kg_to_cwt_short(num: f64) -> f64 {
    num / 45.359237
}

pub async fn kg_to_cwt_long(num: f64) -> f64 {
    num / 50.80234544
}

// ============================================================================
// KILOGRAM TO TROY SYSTEM CONVERSIONS
// ============================================================================

pub async fn kg_to_troy_oz(num: f64) -> f64 {
    num / 0.0311034768
}

pub async fn kg_to_troy_lb(num: f64) -> f64 {
    num / 0.3732417216
}

pub async fn kg_to_pennyweight(num: f64) -> f64 {
    num / 0.00155517384
}

// ============================================================================
// KILOGRAM TO PRECIOUS METALS & GEMS
// ============================================================================

pub async fn kg_to_carat(num: f64) -> f64 {
    num / 0.0002
}

pub async fn kg_to_point(num: f64) -> f64 {
    num / 0.000002
}

pub async fn kg_to_momme(num: f64) -> f64 {
    num / 0.00375
}

// ============================================================================
// KILOGRAM TO ENGINEERING UNITS
// ============================================================================

pub async fn kg_to_slug(num: f64) -> f64 {
    num / 14.593903
}

pub async fn kg_to_tmu(num: f64) -> f64 {
    num / 9.80665
}

// ============================================================================
// KILOGRAM TO ATOMIC UNITS
// ============================================================================

pub async fn kg_to_amu(num: f64) -> f64 {
    num / 1.66053906892e-27
}

pub async fn kg_to_dalton(num: f64) -> f64 {
    num / 1.66053906892e-27
}

// ============================================================================
// GRAM CONVERSIONS (COMMONLY USED)
// ============================================================================

pub async fn g_to_mg(num: f64) -> f64 {
    num * 1000.0
}

pub async fn g_to_micro_g(num: f64) -> f64 {
    num * 1_000_000.0
}

pub async fn g_to_oz(num: f64) -> f64 {
    num / 28.349523125
}

pub async fn g_to_lb(num: f64) -> f64 {
    num / 453.59237
}

pub async fn g_to_troy_oz(num: f64) -> f64 {
    num / 31.1034768
}

pub async fn g_to_carat(num: f64) -> f64 {
    num / 0.2
}

pub async fn g_to_grain(num: f64) -> f64 {
    num / 0.06479891
}

pub async fn g_to_tonne(num: f64) -> f64 {
    num / 1_000_000.0
}

pub async fn mg_to_g(num: f64) -> f64 {
    num / 1000.0
}

pub async fn mg_to_micro_g(num: f64) -> f64 {
    num * 1000.0
}

pub async fn mg_to_carat(num: f64) -> f64 {
    num / 200.0
}

pub async fn mg_to_grain(num: f64) -> f64 {
    num / 64.79891
}

// ============================================================================
// POUND CONVERSIONS (COMMONLY USED)
// ============================================================================

pub async fn lb_to_oz(num: f64) -> f64 {
    num * 16.0
}

pub async fn lb_to_g(num: f64) -> f64 {
    num * 453.59237
}

pub async fn lb_to_grain(num: f64) -> f64 {
    num * 7000.0
}

pub async fn lb_to_short_ton(num: f64) -> f64 {
    num / 2000.0
}

pub async fn lb_to_long_ton(num: f64) -> f64 {
    num / 2240.0
}

pub async fn lb_to_troy_oz(num: f64) -> f64 {
    num * 14.583333333
}

pub async fn lb_to_troy_lb(num: f64) -> f64 {
    num * 1.2152777778
}

pub async fn oz_to_lb(num: f64) -> f64 {
    num / 16.0
}

pub async fn oz_to_g(num: f64) -> f64 {
    num * 28.349523125
}

pub async fn oz_to_grain(num: f64) -> f64 {
    num * 437.5
}

pub async fn oz_to_troy_oz(num: f64) -> f64 {
    num * 0.9114583333
}

pub async fn oz_to_dram(num: f64) -> f64 {
    num * 16.0
}

// ============================================================================
// TONNE/TON CONVERSIONS
// ============================================================================

pub async fn tonne_to_g(num: f64) -> f64 {
    num * 1_000_000.0
}

pub async fn tonne_to_lb(num: f64) -> f64 {
    num * 2204.62262185
}

pub async fn tonne_to_short_ton(num: f64) -> f64 {
    num * 1.10231131092
}

pub async fn tonne_to_long_ton(num: f64) -> f64 {
    num * 0.984206527611
}

pub async fn short_ton_to_tonne(num: f64) -> f64 {
    num * 0.90718474
}

pub async fn short_ton_to_lb(num: f64) -> f64 {
    num * 2000.0
}

pub async fn short_ton_to_long_ton(num: f64) -> f64 {
    num * 0.892857142857
}

pub async fn long_ton_to_tonne(num: f64) -> f64 {
    num * 1.0160469088
}

pub async fn long_ton_to_lb(num: f64) -> f64 {
    num * 2240.0
}

pub async fn long_ton_to_short_ton(num: f64) -> f64 {
    num * 1.12
}

// ============================================================================
// TROY SYSTEM INTERNAL CONVERSIONS
// ============================================================================

pub async fn troy_oz_to_g(num: f64) -> f64 {
    num * 31.1034768
}

pub async fn troy_oz_to_grain(num: f64) -> f64 {
    num * 480.0
}

pub async fn troy_oz_to_pennyweight(num: f64) -> f64 {
    num * 20.0
}

pub async fn troy_oz_to_oz(num: f64) -> f64 {
    num * 1.09714285714
}

pub async fn troy_oz_to_lb(num: f64) -> f64 {
    num / 14.583333333
}

pub async fn troy_lb_to_troy_oz(num: f64) -> f64 {
    num * 12.0
}

pub async fn troy_lb_to_g(num: f64) -> f64 {
    num * 373.2417216
}

pub async fn troy_lb_to_lb(num: f64) -> f64 {
    num * 0.822857142857
}

pub async fn pennyweight_to_g(num: f64) -> f64 {
    num * 1.55517384
}

pub async fn pennyweight_to_grain(num: f64) -> f64 {
    num * 24.0
}

pub async fn pennyweight_to_troy_oz(num: f64) -> f64 {
    num / 20.0
}

pub async fn grain_to_g(num: f64) -> f64 {
    num * 0.06479891
}

pub async fn grain_to_mg(num: f64) -> f64 {
    num * 64.79891
}

pub async fn grain_to_troy_oz(num: f64) -> f64 {
    num / 480.0
}

pub async fn grain_to_oz(num: f64) -> f64 {
    num / 437.5
}

pub async fn grain_to_lb(num: f64) -> f64 {
    num / 7000.0
}

pub async fn grain_to_pennyweight(num: f64) -> f64 {
    num / 24.0
}

// ============================================================================
// CARAT CONVERSIONS
// ============================================================================

pub async fn carat_to_g(num: f64) -> f64 {
    num * 0.2
}

pub async fn carat_to_mg(num: f64) -> f64 {
    num * 200.0
}

pub async fn carat_to_oz(num: f64) -> f64 {
    num * 0.00705479239
}

pub async fn carat_to_grain(num: f64) -> f64 {
    num * 3.08647167
}

pub async fn carat_to_point(num: f64) -> f64 {
    num * 100.0
}

pub async fn point_to_carat(num: f64) -> f64 {
    num / 100.0
}

pub async fn point_to_mg(num: f64) -> f64 {
    num * 2.0
}

// ============================================================================
// SLUG CONVERSIONS (ENGINEERING)
// ============================================================================

pub async fn slug_to_lb(num: f64) -> f64 {
    num * 32.17404855643
}

pub async fn slug_to_g(num: f64) -> f64 {
    num * 14593.903
}

pub async fn slug_to_oz(num: f64) -> f64 {
    num * 514.7847769
}

pub async fn lb_to_slug(num: f64) -> f64 {
    num / 32.17404855643
}

// ============================================================================
// STONE CONVERSIONS (UK)
// ============================================================================

pub async fn stone_to_lb(num: f64) -> f64 {
    num * 14.0
}

pub async fn stone_to_g(num: f64) -> f64 {
    num * 6350.29318
}

pub async fn stone_to_oz(num: f64) -> f64 {
    num * 224.0
}

pub async fn lb_to_stone(num: f64) -> f64 {
    num / 14.0
}

// ============================================================================
// ATOMIC MASS CONVERSIONS
// ============================================================================

pub async fn amu_to_g(num: f64) -> f64 {
    num * 1.66053906892e-24
}

pub async fn amu_to_mg(num: f64) -> f64 {
    num * 1.66053906892e-21
}

pub async fn amu_to_dalton(num: f64) -> f64 {
    num // They are identical
}

pub async fn dalton_to_amu(num: f64) -> f64 {
    num // They are identical
}

pub async fn dalton_to_g(num: f64) -> f64 {
    num * 1.66053906892e-24
}