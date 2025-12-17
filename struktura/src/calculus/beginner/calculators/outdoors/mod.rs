//
// Outdoor Construction Calculators Module
// 
// Strategic organization: Each calculator deployed in its own file,
// mod.rs serves as the command center for re-exports.
//
// Architecture resembles a well-planned construction site:
// - Each calculator is a specialized crew with a specific mission
// - mod.rs is the site manager coordinating all operations
// - Clean separation of concerns, no cross-contamination

pub mod deck;
pub mod concrete_slab;
pub mod patio;
pub mod fence;
pub mod retaining_wall;
pub mod pergola;
pub mod shed_foundation;
pub mod driveway;

// Re-export all calculators for convenient access
pub use deck::DeckCalculator;
pub use concrete_slab::ConcreteSlabCalculator;
pub use patio::PatioCalculator;
pub use fence::FenceCalculator;
pub use retaining_wall::RetainingWallCalculator;
pub use pergola::PergolaCalculator;
pub use shed_foundation::ShedFoundationCalculator;
pub use driveway::DrivewayCalculator;

// Module-level constants for shared outdoor construction parameters
pub(crate) mod constants {
    // Structural lumber pricing (treated for outdoor use)
    pub const TREATED_4X4_COST_PER_M: f64 = 12.50;
    pub const TREATED_2X6_COST_PER_M: f64 = 8.75;
    pub const TREATED_2X4_COST_PER_M: f64 = 6.25;
    
    // Concrete and masonry
    pub const CONCRETE_COST_PER_M3: f64 = 145.0;
    pub const CONCRETE_WASTE_FACTOR: f64 = 1.08;
    pub const REBAR_COST_PER_KG: f64 = 1.85;
    pub const REBAR_DENSITY_KG_PER_M3: f64 = 95.0;
    
    // Base materials
    pub const GRAVEL_BASE_THICKNESS: f64 = 0.10;
    pub const GRAVEL_COST_PER_M3: f64 = 45.0;
    pub const SAND_COST_PER_M3: f64 = 38.0;
    
    // Fasteners and hardware
    pub const HARDWARE_COST_PER_M2: f64 = 3.20;
    pub const POST_ANCHOR_COST: f64 = 8.50;
    pub const JOIST_HANGER_COST: f64 = 2.75;
    
    // Labor rates (rough estimates)
    pub const SKILLED_LABOR_RATE: f64 = 65.0;
    pub const GENERAL_LABOR_RATE: f64 = 45.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::beginner::traits::BeginnerCalculator;
    
    #[test]
    fn test_all_calculators_have_unique_ids() {
        let calculators: Vec<Box<dyn BeginnerCalculator>> = vec![
            Box::new(DeckCalculator),
            Box::new(ConcreteSlabCalculator),
            Box::new(PatioCalculator),
            Box::new(FenceCalculator),
            Box::new(RetainingWallCalculator),
            Box::new(PergolaCalculator),
            Box::new(ShedFoundationCalculator),
            Box::new(DrivewayCalculator),
        ];
        
        let ids: Vec<&str> = calculators.iter().map(|c| c.id()).collect();
        let unique_ids: std::collections::HashSet<&str> = ids.iter().copied().collect();
        
        assert_eq!(
            ids.len(),
            unique_ids.len(),
            "Duplicate calculator IDs detected - battlefield confusion imminent"
        );
    }
    
    #[test]
    fn test_all_calculators_are_outdoors_category() {
        use crate::calculus::beginner::models::CalculatorCategory;
        
        let calculators: Vec<Box<dyn BeginnerCalculator>> = vec![
            Box::new(DeckCalculator),
            Box::new(ConcreteSlabCalculator),
            Box::new(PatioCalculator),
            Box::new(FenceCalculator),
            Box::new(RetainingWallCalculator),
            Box::new(PergolaCalculator),
            Box::new(ShedFoundationCalculator),
            Box::new(DrivewayCalculator),
        ];
        
        for calc in calculators {
            assert_eq!(
                calc.category(),
                CalculatorCategory::Outdoors,
                "Calculator {} has infiltrated the wrong category",
                calc.id()
            );
        }
    }
}