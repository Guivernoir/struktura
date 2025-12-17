// ============================================================================
// Garden & Landscaping Calculators Module
//
// A comprehensive regiment of garden and landscaping calculators.
// Each specialist cultivating their own plot for maximum growth potential.
//
// Structure:
// - planter_box.rs:     Raised bed construction and soil calculations
// - mulch_bed.rs:       Mulch coverage, fabric, and edging
// - raised_bed.rs:      Agricultural raised beds with drainage
// - compost_bin.rs:     Compost bin sizing and materials
// - garden_path.rs:     Gravel/stone pathways and edging
// - irrigation.rs:      Drip irrigation and sprinkler coverage
// - lawn.rs:            Lawn seeding, sod, and maintenance
// - retaining_wall.rs:  Small retaining walls and terracing
// ============================================================================

mod planter_box;
mod mulch_bed;
mod raised_bed;
mod compost_bin;
mod garden_path;
mod irrigation;
mod lawn;
mod retaining_wall;

// Strategic re-exports for external access
pub use planter_box::PlanterBoxCalculator;
pub use mulch_bed::MulchBedCalculator;
pub use raised_bed::RaisedGardenBedCalculator;
pub use compost_bin::CompostBinCalculator;
pub use garden_path::{GravelPathCalculator, SteppingStoneCalculator};
pub use irrigation::{DripIrrigationCalculator, SprinklerCoverageCalculator};
pub use lawn::{LawnSeedCalculator, SodCalculator};
pub use retaining_wall::SmallRetainingWallCalculator;

// Material constants shared across garden calculators
pub mod constants {
    // Soil and amendments (USD per m³)
    pub const TOPSOIL_COST_PER_M3: f64 = 45.0;
    pub const PREMIUM_SOIL_COST_PER_M3: f64 = 65.0;
    pub const COMPOST_COST_PER_M3: f64 = 55.0;
    pub const MULCH_COST_PER_M3: f64 = 38.0;
    pub const SAND_COST_PER_M3: f64 = 32.0;
    pub const GRAVEL_COST_PER_M3: f64 = 42.0;
    
    // Lumber costs (USD per m)
    pub const CEDAR_BOARD_COST_PER_M: f64 = 9.50;
    pub const TREATED_LUMBER_COST_PER_M: f64 = 5.75;
    pub const REDWOOD_BOARD_COST_PER_M: f64 = 12.50;
    
    // Landscaping materials
    pub const LANDSCAPE_FABRIC_COST_PER_M2: f64 = 1.85;
    pub const EDGING_COST_PER_M: f64 = 8.50;
    pub const GEOTEXTILE_COST_PER_M2: f64 = 2.40;
    
    // Irrigation components
    pub const DRIP_TUBING_COST_PER_M: f64 = 0.85;
    pub const EMITTER_COST_EACH: f64 = 0.45;
    pub const SPRINKLER_HEAD_COST: f64 = 12.50;
    
    // Standard dimensions
    pub const STANDARD_PLANTER_DEPTH: f64 = 0.40;     // 40cm optimal
    pub const MINIMUM_ROOT_DEPTH: f64 = 0.20;         // 20cm minimum
    pub const MULCH_LAYER_STANDARD: f64 = 0.08;       // 8cm typical
    pub const PATH_WIDTH_MINIMUM: f64 = 0.60;         // 60cm walkway
    
    // Waste factors
    pub const WASTE_FACTOR_SOIL: f64 = 0.10;
    pub const WASTE_FACTOR_GRAVEL: f64 = 0.15;
    pub const WASTE_FACTOR_LUMBER: f64 = 0.08;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_calculators_instantiate() {
        // Ensure all calculators can be created without panic
        let _ = PlanterBoxCalculator;
        let _ = MulchBedCalculator;
        let _ = RaisedGardenBedCalculator;
        let _ = CompostBinCalculator;
        let _ = GravelPathCalculator;
        let _ = SteppingStoneCalculator;
        let _ = DripIrrigationCalculator;
        let _ = SprinklerCoverageCalculator;
        let _ = LawnSeedCalculator;
        let _ = SodCalculator;
        let _ = SmallRetainingWallCalculator;
    }
}