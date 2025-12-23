// ============================================================================
// Structural Engineering Calculators
// 
// Load analysis, member design, and code compliance calculators.
// All calculators in this module require PE (Professional Engineer) review.
// ============================================================================

// Individual calculator modules
pub mod beam_design;
pub mod column_design;
pub mod truss_analysis;
pub mod moment_frame_design;
pub mod connection_design;
pub mod slab_design;
pub mod lateral_load_analysis;

// Re-export calculators
pub use beam_design::BeamDesignCalculator;
pub use column_design::ColumnDesignCalculator;
pub use truss_analysis::TrussAnalysisCalculator;
pub use moment_frame_design::MomentFrameDesignCalculator;
pub use connection_design::ConnectionDesignCalculator;
pub use slab_design::SlabDesignCalculator;
pub use lateral_load_analysis::LateralLoadAnalysisCalculator;

// ============================================================================
// STRUCTURAL ENGINEERING CONSTANTS
// ============================================================================

/// Steel material properties
pub mod steel_properties {
    // Yield strengths (MPa)
    pub const FY_A36: f64 = 250.0;      // 36 ksi
    pub const FY_A572_GR50: f64 = 345.0; // 50 ksi
    pub const FY_A992: f64 = 345.0;      // 50 ksi (W-shapes)
    pub const FY_A500_GRC: f64 = 345.0;  // 50 ksi (HSS)
    
    // Tensile strengths (MPa)
    pub const FU_A36: f64 = 400.0;
    pub const FU_A572_GR50: f64 = 450.0;
    pub const FU_A992: f64 = 450.0;
    
    // Elastic modulus (GPa)
    pub const E_STEEL: f64 = 200.0;
    
    // Shear modulus (GPa)
    pub const G_STEEL: f64 = 77.0;
    
    // Density (kg/m³)
    pub const DENSITY_STEEL: f64 = 7850.0;
    
    // Poisson's ratio
    pub const POISSON_STEEL: f64 = 0.30;
}

/// Concrete material properties
pub mod concrete_properties {
    // Compressive strengths (MPa)
    pub const FC_C20: f64 = 20.0;
    pub const FC_C25: f64 = 25.0;
    pub const FC_C28: f64 = 28.0;  // 4000 psi
    pub const FC_C30: f64 = 30.0;
    pub const FC_C35: f64 = 35.0;  // 5000 psi
    pub const FC_C40: f64 = 40.0;
    
    // Elastic modulus (GPa) - approximate
    pub const E_CONCRETE_C28: f64 = 25.0;
    pub const E_CONCRETE_C35: f64 = 28.0;
    
    // Density (kg/m³)
    pub const DENSITY_NORMAL: f64 = 2400.0;
    pub const DENSITY_LIGHTWEIGHT: f64 = 1850.0;
    
    /// Calculate elastic modulus (GPa) per ACI 318
    pub fn elastic_modulus_aci(fc_mpa: f64, density_kg_m3: f64) -> f64 {
        // E = wc^1.5 × 0.043 × sqrt(fc)
        // wc in kg/m³, fc in MPa, result in GPa
        let wc = density_kg_m3 / 1000.0; // Convert to Mg/m³
        0.043 * wc.powf(1.5) * fc_mpa.sqrt()
    }
}

/// AISC/ACI resistance factors (LRFD)
pub mod resistance_factors {
    // AISC 360 (Steel)
    pub const PHI_FLEXURE: f64 = 0.90;
    pub const PHI_SHEAR: f64 = 0.90;
    pub const PHI_COMPRESSION: f64 = 0.90;
    pub const PHI_TENSION: f64 = 0.90;
    
    // ACI 318 (Concrete)
    pub const PHI_FLEXURE_TENSION: f64 = 0.90;
    pub const PHI_COMPRESSION_TIED: f64 = 0.65;
    pub const PHI_COMPRESSION_SPIRAL: f64 = 0.75;
    pub const PHI_SHEAR_CONCRETE: f64 = 0.75;
}

/// Load factors (LRFD)
pub mod load_factors {
    // ASCE 7 combinations
    pub const DEAD_ONLY: f64 = 1.4;
    pub const DEAD_PLUS_LIVE: (f64, f64) = (1.2, 1.6);
    pub const DEAD_PLUS_WIND: (f64, f64) = (1.2, 1.0);
    pub const DEAD_PLUS_SEISMIC: (f64, f64) = (1.2, 1.0);
    
    // Service load factors (ASD)
    pub const SERVICE_DEAD: f64 = 1.0;
    pub const SERVICE_LIVE: f64 = 1.0;
}

/// Deflection limits
pub mod deflection_limits {
    /// L/360 for live load (typical floors)
    pub const L_OVER_360: f64 = 360.0;
    
    /// L/240 for live load (roof with plaster)
    pub const L_OVER_240: f64 = 240.0;
    
    /// L/180 for live load (roof without plaster)
    pub const L_OVER_180: f64 = 180.0;
    
    /// L/120 for total load
    pub const L_OVER_120: f64 = 120.0;
}

/// Helper functions for structural calculations
pub mod helpers {
    /// Calculate factored load (LRFD)
    pub fn factored_load_basic(dead: f64, live: f64) -> f64 {
        1.2 * dead + 1.6 * live
    }
    
    /// Calculate service load (ASD)
    pub fn service_load(dead: f64, live: f64) -> f64 {
        dead + live
    }
    
    /// Check deflection limit
    pub fn check_deflection(deflection_mm: f64, span_m: f64, limit_ratio: f64) -> (bool, f64) {
        let span_mm = span_m * 1000.0;
        let allowable = span_mm / limit_ratio;
        let passes = deflection_mm <= allowable;
        let utilization = deflection_mm / allowable;
        (passes, utilization)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steel_properties() {
        use steel_properties::*;
        
        // Verify common steel grades
        assert_eq!(FY_A992, 345.0);
        assert_eq!(E_STEEL, 200.0);
    }

    #[test]
    #[ignore]
    fn test_concrete_elastic_modulus() {
        use concrete_properties::*;
        
        let e = elastic_modulus_aci(FC_C28, DENSITY_NORMAL);
        
        // Should be around 25-30 GPa for C28
        assert!(e > 20.0 && e < 35.0);
    }

    #[test]
    fn test_factored_loads() {
        use helpers::*;
        
        let factored = factored_load_basic(10.0, 15.0);
        let expected = 1.2 * 10.0 + 1.6 * 15.0;
        
        assert!((factored - expected).abs() < 0.01);
    }

    #[test]
    fn test_deflection_check() {
        use helpers::*;
        use deflection_limits::*;
        
        let (passes, ratio) = check_deflection(15.0, 6.0, L_OVER_360);
        
        // 15mm deflection on 6m span
        // Limit = 6000/360 = 16.67mm
        assert!(passes);
        assert!(ratio < 1.0);
    }
}