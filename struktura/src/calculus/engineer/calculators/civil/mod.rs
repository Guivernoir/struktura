// ============================================================================
// Civil Engineering Calculators
// 
// Geotechnical, pavement, infrastructure, and foundation design calculators.
// All calculators in this module require PE (Professional Engineer) review.
// ============================================================================

// Individual calculator modules
pub mod retaining_wall;
pub mod pavement_design;
pub mod foundation_design;
pub mod slope_stability;
pub mod settlement_analysis;
pub mod soil_bearing_capacity;

// Re-export calculators
pub use retaining_wall::RetainingWallCalculator;
pub use pavement_design::PavementDesignCalculator;
pub use foundation_design::FoundationDesignCalculator;
pub use slope_stability::SlopeStabilityCalculator;
pub use settlement_analysis::SettlementAnalysisCalculator;
pub use soil_bearing_capacity::SoilBearingCapacityCalculator;

// ============================================================================
// CIVIL ENGINEERING CONSTANTS
// ============================================================================

/// Common soil unit weights (kN/m³)
pub mod soil_properties {
    pub const UNIT_WEIGHT_SANDY: f64 = 18.0;
    pub const UNIT_WEIGHT_CLAY: f64 = 16.5;
    pub const UNIT_WEIGHT_GRAVEL: f64 = 19.0;
    pub const UNIT_WEIGHT_SILT: f64 = 17.0;
    pub const UNIT_WEIGHT_SATURATED_SAND: f64 = 20.0;
}

/// Common concrete properties
pub mod concrete_properties {
    pub const UNIT_WEIGHT_CONCRETE: f64 = 24.0; // kN/m³
    pub const ELASTIC_MODULUS_C25: f64 = 30.0; // GPa
    pub const ELASTIC_MODULUS_C30: f64 = 32.0; // GPa
    pub const ELASTIC_MODULUS_C40: f64 = 35.0; // GPa
}

/// Rankine earth pressure coefficients
pub mod earth_pressure {
    /// Active earth pressure coefficient for φ=30°
    pub const RANKINE_ACTIVE_30: f64 = 0.33;
    /// Passive earth pressure coefficient for φ=30°
    pub const RANKINE_PASSIVE_30: f64 = 3.0;
    
    /// Calculate Rankine active coefficient
    pub fn rankine_active(friction_angle_degrees: f64) -> f64 {
        let phi = friction_angle_degrees.to_radians();
        (1.0 - phi.sin()) / (1.0 + phi.sin())
    }
    
    /// Calculate Rankine passive coefficient
    pub fn rankine_passive(friction_angle_degrees: f64) -> f64 {
        let phi = friction_angle_degrees.to_radians();
        (1.0 + phi.sin()) / (1.0 - phi.sin())
    }
}

/// Pavement design constants
pub mod pavement {
    /// ESAL (Equivalent Single Axle Load) traffic categories
    pub const ESAL_LIGHT_TRAFFIC: f64 = 50_000.0;
    pub const ESAL_MEDIUM_TRAFFIC: f64 = 500_000.0;
    pub const ESAL_HEAVY_TRAFFIC: f64 = 2_000_000.0;
    pub const ESAL_VERY_HEAVY_TRAFFIC: f64 = 5_000_000.0;
    
    /// Pavement layer coefficients (AASHTO)
    pub const ASPHALT_LAYER_COEFF: f64 = 0.44;
    pub const BASE_LAYER_COEFF: f64 = 0.14;
    pub const SUBBASE_LAYER_COEFF: f64 = 0.11;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rankine_coefficients() {
        use earth_pressure::*;
        
        // Test for 30 degrees
        let ka = rankine_active(30.0);
        let kp = rankine_passive(30.0);
        
        assert!((ka - 0.33).abs() < 0.01);
        assert!((kp - 3.0).abs() < 0.1);
    }

    #[test]
    fn test_constants_validity() {
        use soil_properties::*;
        
        // Soil unit weights should be reasonable
        assert!(UNIT_WEIGHT_SANDY > 15.0 && UNIT_WEIGHT_SANDY < 25.0);
        assert!(UNIT_WEIGHT_CLAY > 15.0 && UNIT_WEIGHT_CLAY < 25.0);
    }
}