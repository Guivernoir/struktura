// ============================================================================
// Interior Construction Calculators Module
//
// A well-organized battalion of interior construction calculators.
// Each calculator deployed in its own file for maximum tactical flexibility.
//
// Structure:
// - wall_framing.rs:    Stud, plate, and structural framing calculations
// - drywall.rs:         Drywall sheets, compound, tape, finishing materials
// - flooring.rs:        Hardwood, laminate, vinyl flooring estimates
// - insulation.rs:      Thermal insulation materials and R-values
// - ceiling.rs:         Drop ceiling, drywall ceiling, material counts
// - trim.rs:            Baseboards, crown molding, door/window casing
// ============================================================================

mod wall_framing;
mod drywall;
mod flooring;
mod insulation;
mod ceiling;
mod trim;

// Strategic re-exports for external access
pub use wall_framing::WallFramingCalculator;
pub use drywall::DrywallCountCalculator;
pub use flooring::{HardwoodFlooringCalculator, LaminateFlooringCalculator};
pub use insulation::InsulationCalculator;
pub use ceiling::{DropCeilingCalculator, DrywallCeilingCalculator};
pub use trim::{BaseboardCalculator, CrownMoldingCalculator};

// Material constants shared across calculators
pub mod constants {
    /// Standard US/Canada stud spacing (16 inches on center)
    pub const STUD_SPACING: f64 = 0.406;
    
    /// Standard stud height (8 feet)
    pub const STUD_HEIGHT_STANDARD: f64 = 2.44;
    
    /// Standard drywall sheet dimensions
    pub const DRYWALL_SHEET_WIDTH: f64 = 1.22;   // 4 feet
    pub const DRYWALL_SHEET_HEIGHT: f64 = 2.44;  // 8 feet
    pub const DRYWALL_SHEET_AREA: f64 = DRYWALL_SHEET_WIDTH * DRYWALL_SHEET_HEIGHT;
    
    /// Standard door opening width
    pub const STANDARD_DOOR_WIDTH: f64 = 0.91; // 36 inches
    
    /// Standard window opening width
    pub const STANDARD_WINDOW_WIDTH: f64 = 1.22; // 4 feet
    
    /// Typical ceiling height (residential)
    pub const TYPICAL_CEILING_HEIGHT: f64 = 2.44; // 8 feet
    
    /// Waste factors for various materials
    pub const WASTE_FACTOR_DRYWALL: f64 = 0.15;
    pub const WASTE_FACTOR_FLOORING: f64 = 0.10;
    pub const WASTE_FACTOR_TRIM: f64 = 0.08;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_calculators_instantiate() {
        // Ensure all calculators can be created without panic
        let _ = WallFramingCalculator;
        let _ = DrywallCountCalculator;
        let _ = HardwoodFlooringCalculator;
        let _ = LaminateFlooringCalculator;
        let _ = InsulationCalculator;
        let _ = DropCeilingCalculator;
        let _ = DrywallCeilingCalculator;
        let _ = BaseboardCalculator;
        let _ = CrownMoldingCalculator;
    }
}