// ============================================================================
// Utilities & Finishes Calculators Module
//
// A well-organized regiment of finishing and utility calculators.
// Each specialist deployed in its own tactical position.
//
// Structure:
// - paint.rs:       Paint coverage, primers, room painting estimates
// - tile.rs:        Floor/wall tile materials and installation
// - wallpaper.rs:   Wallpaper rolls, adhesive, and application
// - lighting.rs:    Recessed lighting layout and electrical
// - hvac.rs:        Basic HVAC sizing and duct material estimates
// - plumbing.rs:    Pipe materials for basic installations
// ============================================================================

mod paint;
mod tile;
mod wallpaper;
mod lighting;
mod hvac;
mod plumbing;

// Strategic re-exports for external access
pub use paint::PaintCoverageCalculator;
pub use tile::TileCountCalculator;
pub use wallpaper::WallpaperCalculator;
pub use lighting::{RecessedLightingCalculator, TrackLightingCalculator};
pub use hvac::HVACSizingCalculator;
pub use plumbing::{PipeRunCalculator, DrainLineCalculator};

// Material constants shared across calculators
pub mod constants {
    /// Standard room openings
    pub const STANDARD_DOOR_AREA: f64 = 1.82;  // 0.91m x 2.0m
    pub const STANDARD_WINDOW_AREA: f64 = 1.2; // Typical window
    
    /// Electrical spacing standards
    pub const OUTLET_SPACING_MAX: f64 = 3.66;  // 12 feet max per code
    pub const SWITCH_HEIGHT: f64 = 1.22;       // 48 inches standard
    
    /// Plumbing standards
    pub const PIPE_SUPPORT_SPACING: f64 = 1.22; // Support every 4 feet
    pub const VENT_PIPE_MIN_DIAMETER: f64 = 0.0381; // 1.5 inches
    
    /// Waste factors
    pub const WASTE_FACTOR_PAINT: f64 = 0.05;
    pub const WASTE_FACTOR_TILE: f64 = 0.12;
    pub const WASTE_FACTOR_WALLPAPER: f64 = 0.15;
    pub const WASTE_FACTOR_PIPE: f64 = 0.10;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_calculators_instantiate() {
        // Ensure all calculators can be created without panic
        let _ = PaintCoverageCalculator;
        let _ = TileCountCalculator;
        let _ = WallpaperCalculator;
        let _ = RecessedLightingCalculator;
        let _ = TrackLightingCalculator;
        let _ = HVACSizingCalculator;
        let _ = PipeRunCalculator;
        let _ = DrainLineCalculator;
    }
}