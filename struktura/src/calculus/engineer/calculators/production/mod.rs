// ============================================================================
// Production Engineering Calculators
// 
// Manufacturing systems, line balancing, material handling, and process
// optimization calculators. These typically do not require PE review.
// ============================================================================

// Individual calculator modules
pub mod conveyor_belt;
pub mod line_balancing;
pub mod oee_calculation;
pub mod inventory_optimization;
pub mod capacity_planning;
pub mod process_capability;
pub mod work_sampling;
pub mod facility_layout;

// Re-export calculators
pub use conveyor_belt::ConveyorBeltCalculator;
pub use line_balancing::ProductionLineBalancingCalculator;
pub use oee_calculation::OEECalculator;
pub use inventory_optimization::InventoryOptimizationCalculator;
pub use capacity_planning::CapacityPlanningCalculator;
pub use process_capability::ProcessCapabilityCalculator;
pub use work_sampling::WorkSamplingCalculator;
pub use facility_layout::FacilityLayoutCalculator;

// ============================================================================
// PRODUCTION ENGINEERING CONSTANTS
// ============================================================================

/// Material handling constants (CEMA standards)
pub mod material_handling {
    // Belt speed ranges (m/s)
    pub const BELT_SPEED_MIN: f64 = 0.5;
    pub const BELT_SPEED_TYPICAL: f64 = 1.5;
    pub const BELT_SPEED_MAX: f64 = 4.0;
    
    // Surcharge angles for granular materials (degrees)
    pub const SURCHARGE_ANGLE_FINE: f64 = 10.0;
    pub const SURCHARGE_ANGLE_TYPICAL: f64 = 20.0;
    pub const SURCHARGE_ANGLE_COARSE: f64 = 30.0;
    
    // Friction coefficients
    pub const FRICTION_BELT_IDLER: f64 = 0.02;
    pub const FRICTION_CONCRETE_SOIL: f64 = 0.5;
    pub const FRICTION_STEEL_SOIL: f64 = 0.4;
    
    // Material bulk densities (kg/m³)
    pub const DENSITY_COAL: f64 = 800.0;
    pub const DENSITY_SAND: f64 = 1600.0;
    pub const DENSITY_GRAVEL: f64 = 1750.0;
    pub const DENSITY_LIMESTONE: f64 = 1500.0;
    pub const DENSITY_IRON_ORE: f64 = 2400.0;
}

/// Lean manufacturing and line balancing
pub mod lean_manufacturing {
    // Target efficiency levels (%)
    pub const TARGET_LINE_EFFICIENCY: f64 = 85.0;
    pub const MINIMUM_ACCEPTABLE_EFFICIENCY: f64 = 75.0;
    pub const WORLD_CLASS_EFFICIENCY: f64 = 95.0;
    
    // Balance delay thresholds (%)
    pub const ACCEPTABLE_BALANCE_DELAY: f64 = 15.0;
    pub const TARGET_BALANCE_DELAY: f64 = 10.0;
    
    // OEE (Overall Equipment Effectiveness) benchmarks (%)
    pub const OEE_WORLD_CLASS: f64 = 85.0;
    pub const OEE_GOOD: f64 = 65.0;
    pub const OEE_ACCEPTABLE: f64 = 50.0;
    
    // Labor cost estimates (USD/hour)
    pub const LABOR_RATE_LOW: f64 = 15.0;
    pub const LABOR_RATE_MEDIUM: f64 = 25.0;
    pub const LABOR_RATE_HIGH: f64 = 40.0;
}

/// Process capability indices
pub mod process_capability_indices {
    // Capability index thresholds
    pub const CPK_WORLD_CLASS: f64 = 2.0;
    pub const CPK_ADEQUATE: f64 = 1.33;
    pub const CPK_MINIMUM: f64 = 1.0;
    
    // Sigma levels
    pub const THREE_SIGMA_PPM: f64 = 66_807.0;
    pub const FOUR_SIGMA_PPM: f64 = 6_210.0;
    pub const FIVE_SIGMA_PPM: f64 = 233.0;
    pub const SIX_SIGMA_PPM: f64 = 3.4;
}

/// Helper functions for production calculations
pub mod helpers {
    /// Calculate takt time (available time / demand)
    pub fn takt_time(available_time_minutes: f64, demand_units: f64) -> f64 {
        available_time_minutes / demand_units
    }
    
    /// Calculate cycle time (bottleneck station time)
    pub fn cycle_time(station_times: &[f64]) -> f64 {
        station_times.iter().copied().fold(f64::MIN, f64::max)
    }
    
    /// Calculate line efficiency
    pub fn line_efficiency(
        total_task_time: f64,
        num_stations: usize,
        cycle_time: f64,
    ) -> f64 {
        (total_task_time / (num_stations as f64 * cycle_time)) * 100.0
    }
    
    /// Calculate balance delay (idle time percentage)
    pub fn balance_delay(line_efficiency: f64) -> f64 {
        100.0 - line_efficiency
    }
    
    /// Calculate smoothness index (lower is better)
    pub fn smoothness_index(
        station_times: &[f64],
        cycle_time: f64,
    ) -> f64 {
        let n = station_times.len() as f64;
        let sum_squared_diff: f64 = station_times
            .iter()
            .map(|&t| (cycle_time - t).powi(2))
            .sum();
        
        (sum_squared_diff / n).sqrt() * 100.0 / cycle_time
    }
    
    /// Calculate theoretical minimum workstations
    pub fn min_workstations(total_task_time: f64, takt_time: f64) -> usize {
        (total_task_time / takt_time).ceil() as usize
    }
    
    /// Calculate OEE (Overall Equipment Effectiveness)
    pub fn oee(
        availability: f64,  // %
        performance: f64,   // %
        quality: f64,       // %
    ) -> f64 {
        (availability / 100.0) * (performance / 100.0) * (quality / 100.0) * 100.0
    }
    
    /// Calculate Economic Order Quantity (EOQ)
    pub fn eoq(
        annual_demand: f64,
        ordering_cost: f64,
        holding_cost_per_unit: f64,
    ) -> f64 {
        ((2.0 * annual_demand * ordering_cost) / holding_cost_per_unit).sqrt()
    }
    
    /// Calculate Reorder Point (ROP)
    pub fn reorder_point(
        daily_demand: f64,
        lead_time_days: f64,
        safety_stock: f64,
    ) -> f64 {
        (daily_demand * lead_time_days) + safety_stock
    }
    
    /// Calculate Process Capability Index (Cpk)
    pub fn cpk(
        mean: f64,
        std_dev: f64,
        lower_spec: f64,
        upper_spec: f64,
    ) -> f64 {
        let cpu = (upper_spec - mean) / (3.0 * std_dev);
        let cpl = (mean - lower_spec) / (3.0 * std_dev);
        cpu.min(cpl)
    }
    
    /// Calculate belt capacity (CEMA formula)
    pub fn belt_capacity_volumetric(
        belt_width: f64,
        belt_speed: f64,
        surcharge_angle: f64,
    ) -> f64 {
        // Simplified formula: Q = A × v × 3600
        // where A = cross-sectional area
        let surcharge_factor = 1.0 + (surcharge_angle / 90.0) * 0.5;
        let area = belt_width.powi(2) * 0.08 * surcharge_factor;
        area * belt_speed * 3600.0 // m³/h
    }
    
    /// Calculate required motor power for conveyor
    pub fn conveyor_motor_power(
        belt_length: f64,
        belt_speed: f64,
        material_load_kg_m: f64,
        inclination_degrees: f64,
    ) -> f64 {
        let gravity = 9.81;
        let friction = 0.02;
        
        let effective_tension = material_load_kg_m * gravity * belt_length *
            (friction + inclination_degrees.to_radians().sin());
        
        let power_kw = (effective_tension * belt_speed) / 1000.0;
        power_kw * 1.15 // Add 15% safety factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_takt_time() {
        use helpers::*;
        
        // 480 minutes available, 100 units demand
        let takt = takt_time(480.0, 100.0);
        assert_eq!(takt, 4.8); // 4.8 minutes per unit
    }

    #[test]
    fn test_line_efficiency() {
        use helpers::*;
        
        // 12 minutes total task time, 5 stations, 3 minutes cycle time
        let eff = line_efficiency(12.0, 5, 3.0);
        assert_eq!(eff, 80.0); // 80% efficiency
    }

    #[test]
    fn test_min_workstations() {
        use helpers::*;
        
        let min_stations = min_workstations(12.0, 3.0);
        assert_eq!(min_stations, 4); // Ceiling of 12/3 = 4
    }

    #[test]
    fn test_oee() {
        use helpers::*;
        
        // 90% availability, 95% performance, 98% quality
        let oee_value = oee(90.0, 95.0, 98.0);
        assert!((oee_value - 83.79).abs() < 0.1);
    }

    #[test]
    fn test_eoq() {
        use helpers::*;
        
        // 10,000 annual demand, $50 ordering cost, $2 holding cost
        let eoq_qty = eoq(10_000.0, 50.0, 2.0);
        
        // EOQ = sqrt(2×10000×50/2) = 707.1
        assert!((eoq_qty - 707.1).abs() < 1.0);
    }

    #[test]
    fn test_cpk() {
        use helpers::*;
        
        // Mean = 10, StdDev = 1, LSL = 5, USL = 15
        let cpk_value = cpk(10.0, 1.0, 5.0, 15.0);
        
        // Cpk = min((15-10)/(3×1), (10-5)/(3×1)) = min(1.67, 1.67) = 1.67
        assert!((cpk_value - 1.67).abs() < 0.01);
    }

    #[test]
    fn test_belt_capacity() {
        use helpers::*;
        
        let capacity = belt_capacity_volumetric(0.8, 1.5, 20.0);
        
        // Should be positive and reasonable
        assert!(capacity > 0.0);
        assert!(capacity < 1000.0); // Less than 1000 m³/h for 0.8m belt
    }

    #[test]
    fn test_conveyor_power() {
        use helpers::*;
        
        let power = conveyor_motor_power(50.0, 1.5, 100.0, 0.0);
        
        // Should be positive
        assert!(power > 0.0);
        // Should be reasonable for these parameters
        assert!(power < 50.0); // Less than 50 kW
    }
}