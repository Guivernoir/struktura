// ============================================================================
// Mechanical Engineering Calculators
// 
// Thermodynamics, fluid mechanics, HVAC, and pump system calculators.
// These calculators typically do not require PE review unless for critical
// applications (pressure vessels, life safety systems).
// ============================================================================

// Individual calculator modules
pub mod heat_exchanger;
pub mod pump_sizing;
pub mod piping_pressure_drop;
pub mod hvac_load_calculation;
pub mod refrigeration_cycle;
pub mod compressor_sizing;
pub mod valve_sizing;
pub mod thermal_expansion;

// Re-export calculators
pub use heat_exchanger::HeatExchangerCalculator;
pub use pump_sizing::PumpSizingCalculator;
pub use piping_pressure_drop::PipingPressureDropCalculator;
pub use hvac_load_calculation::HVACLoadCalculationCalculator;
pub use refrigeration_cycle::RefrigerationCycleCalculator;
pub use compressor_sizing::CompressorSizingCalculator;
pub use valve_sizing::ValveSizingCalculator;
pub use thermal_expansion::ThermalExpansionCalculator;

// ============================================================================
// MECHANICAL ENGINEERING CONSTANTS
// ============================================================================

/// Fluid properties at standard conditions
pub mod fluid_properties {
    // Water properties at 20°C
    pub const WATER_DENSITY: f64 = 1000.0;        // kg/m³
    pub const WATER_VISCOSITY: f64 = 0.001;       // Pa·s
    pub const WATER_SPECIFIC_HEAT: f64 = 4186.0;  // J/(kg·K)
    pub const WATER_THERMAL_COND: f64 = 0.598;    // W/(m·K)
    
    // Air properties at 20°C, 1 atm
    pub const AIR_DENSITY: f64 = 1.204;           // kg/m³
    pub const AIR_VISCOSITY: f64 = 1.825e-5;      // Pa·s
    pub const AIR_SPECIFIC_HEAT: f64 = 1005.0;    // J/(kg·K)
    pub const AIR_THERMAL_COND: f64 = 0.0257;     // W/(m·K)
    
    // Steam properties (saturated at 100°C)
    pub const STEAM_DENSITY_100C: f64 = 0.598;    // kg/m³
    pub const STEAM_LATENT_HEAT: f64 = 2257e3;    // J/kg
}

/// Physical constants
pub mod constants {
    pub const GRAVITY: f64 = 9.81;                 // m/s²
    pub const GAS_CONSTANT: f64 = 8.314;           // J/(mol·K)
    pub const STEFAN_BOLTZMANN: f64 = 5.67e-8;     // W/(m²·K⁴)
    pub const ATMOSPHERIC_PRESSURE: f64 = 101.325; // kPa
}

/// Heat exchanger typical values
pub mod heat_exchanger_values {
    // Overall heat transfer coefficients (W/(m²·K))
    pub const U_WATER_WATER: f64 = 850.0;
    pub const U_WATER_OIL: f64 = 350.0;
    pub const U_WATER_AIR: f64 = 60.0;
    pub const U_STEAM_WATER: f64 = 2500.0;
    pub const U_GAS_GAS: f64 = 35.0;
    
    // Fouling factors (m²·K/W)
    pub const FOULING_DISTILLED_WATER: f64 = 0.0001;
    pub const FOULING_CITY_WATER: f64 = 0.0002;
    pub const FOULING_COOLING_TOWER: f64 = 0.0003;
    pub const FOULING_SEAWATER: f64 = 0.0001;
}

/// Pump and piping constants
pub mod pump_hydraulics {
    // Standard gravity for hydraulic calculations
    pub const G: f64 = 9.81; // m/s²
    
    // Typical pump efficiencies (%)
    pub const EFF_SMALL_CENTRIFUGAL: f64 = 60.0;
    pub const EFF_MEDIUM_CENTRIFUGAL: f64 = 75.0;
    pub const EFF_LARGE_CENTRIFUGAL: f64 = 85.0;
    
    // Motor efficiencies (%)
    pub const EFF_MOTOR_SMALL: f64 = 85.0;
    pub const EFF_MOTOR_MEDIUM: f64 = 90.0;
    pub const EFF_MOTOR_LARGE: f64 = 95.0;
    
    // Friction factors (smooth pipes)
    pub const FRICTION_LAMINAR: f64 = 64.0; // f = 64/Re
    pub const FRICTION_TURBULENT_SMOOTH: f64 = 0.02;
}

/// Helper functions for mechanical calculations
pub mod helpers {
    use super::constants::GRAVITY;
    
    /// Calculate Reynolds number
    pub fn reynolds_number(velocity: f64, diameter: f64, density: f64, viscosity: f64) -> f64 {
        (density * velocity * diameter) / viscosity
    }
    
    /// Calculate friction factor (Haaland approximation for turbulent flow)
    pub fn friction_factor_turbulent(reynolds: f64, roughness: f64, diameter: f64) -> f64 {
        let rel_rough = roughness / diameter;
        let term1 = (rel_rough / 3.7).powf(1.11);
        let term2 = 6.9 / reynolds;
        let f_inv = -1.8 * (term1 + term2).log10();
        1.0 / f_inv.powi(2)
    }
    
    /// Calculate pressure drop in pipe (Darcy-Weisbach)
    pub fn pressure_drop_pipe(
        friction_factor: f64,
        length: f64,
        diameter: f64,
        velocity: f64,
        density: f64,
    ) -> f64 {
        // Δp = f × (L/D) × (ρv²/2)
        friction_factor * (length / diameter) * (density * velocity.powi(2) / 2.0)
    }
    
    /// Calculate hydraulic power (kW)
    pub fn hydraulic_power_kw(flow_m3s: f64, head_m: f64, density: f64) -> f64 {
        (density * GRAVITY * flow_m3s * head_m) / 1000.0
    }
    
    /// Calculate LMTD (Log Mean Temperature Difference)
    pub fn lmtd_counterflow(t_hot_in: f64, t_hot_out: f64, t_cold_in: f64, t_cold_out: f64) -> f64 {
        let dt1 = t_hot_in - t_cold_out;
        let dt2 = t_hot_out - t_cold_in;
        
        if (dt1 - dt2).abs() < 0.01 {
            // If differences are nearly equal, use arithmetic mean
            (dt1 + dt2) / 2.0
        } else {
            // Use log mean
            (dt1 - dt2) / (dt1 / dt2).ln()
        }
    }
    
    /// Calculate heat transfer rate (kW)
    pub fn heat_transfer_kw(mass_flow: f64, specific_heat: f64, temp_diff: f64) -> f64 {
        (mass_flow * specific_heat * temp_diff) / 1000.0
    }
    
    /// Calculate NTU (Number of Transfer Units)
    pub fn ntu(ua: f64, c_min: f64) -> f64 {
        ua / c_min
    }
    
    /// Calculate effectiveness from NTU (counterflow, C_ratio = 1)
    pub fn effectiveness_from_ntu_counterflow(ntu: f64, c_ratio: f64) -> f64 {
        if (c_ratio - 1.0).abs() < 0.01 {
            // C_ratio ≈ 1
            ntu / (ntu + 1.0)
        } else {
            // C_ratio ≠ 1
            let exp_term = (-ntu * (1.0 - c_ratio)).exp();
            (1.0 - exp_term) / (1.0 - c_ratio * exp_term)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reynolds_number() {
        use helpers::*;
        use fluid_properties::*;
        
        // Water at 1 m/s in 0.1m diameter pipe
        let re = reynolds_number(1.0, 0.1, WATER_DENSITY, WATER_VISCOSITY);
        
        // Should be 100,000 (turbulent)
        assert!((re - 100_000.0).abs() < 1000.0);
    }

    #[test]
    fn test_hydraulic_power() {
        use helpers::*;
        use fluid_properties::*;
        
        // 0.1 m³/s at 20m head
        let power = hydraulic_power_kw(0.1, 20.0, WATER_DENSITY);
        
        // P = ρ × g × Q × H = 1000 × 9.81 × 0.1 × 20 = 19.62 kW
        assert!((power - 19.62).abs() < 0.1);
    }

    #[test]
    fn test_lmtd() {
        use helpers::*;
        
        // Counterflow heat exchanger
        let lmtd = lmtd_counterflow(80.0, 50.0, 20.0, 40.0);
        
        // dt1 = 80-40 = 40, dt2 = 50-20 = 30
        // LMTD = (40-30)/ln(40/30) = 34.76
        assert!((lmtd - 34.76).abs() < 0.1);
    }

    #[test]
    fn test_pressure_drop() {
        use helpers::*;
        use fluid_properties::*;
        
        // 100m pipe, 0.1m diameter, 2 m/s velocity
        let dp = pressure_drop_pipe(0.02, 100.0, 0.1, 2.0, WATER_DENSITY);
        
        // Should be reasonable (few kPa)
        assert!(dp > 0.0 && dp < 100_000.0); // Less than 100 kPa
    }
}