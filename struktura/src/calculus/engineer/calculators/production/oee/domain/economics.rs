//! Economic translation of losses
//! 
//! CRITICAL: All outputs are ESTIMATES with uncertainty bounds.
//! This is NOT accounting-grade data.
//! 
//! Per Section 14.2: "Require confidence bands on every economic input"

use super::*;

/// Economic parameters with uncertainty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicParameters {
    /// Unit price (low, central, high)
    pub unit_price: (f64, f64, f64),
    /// Marginal contribution per unit (low, central, high)
    pub marginal_contribution: (f64, f64, f64),
    /// Material cost per unit (low, central, high)
    pub material_cost: (f64, f64, f64),
    /// Labor cost per hour (low, central, high)
    pub labor_cost_per_hour: (f64, f64, f64),
    /// Currency code
    pub currency: String,
}

impl EconomicParameters {
    /// Create with point estimates (converts to ranges with ±10%)
    pub fn from_point_estimates(
        unit_price: f64,
        marginal_contribution: f64,
        material_cost: f64,
        labor_cost_per_hour: f64,
        currency: &str,
    ) -> Self {
        let spread = 0.10; // ±10% default uncertainty
        
        Self {
            unit_price: (
                unit_price * (1.0 - spread),
                unit_price,
                unit_price * (1.0 + spread),
            ),
            marginal_contribution: (
                marginal_contribution * (1.0 - spread),
                marginal_contribution,
                marginal_contribution * (1.0 + spread),
            ),
            material_cost: (
                material_cost * (1.0 - spread),
                material_cost,
                material_cost * (1.0 + spread),
            ),
            labor_cost_per_hour: (
                labor_cost_per_hour * (1.0 - spread),
                labor_cost_per_hour,
                labor_cost_per_hour * (1.0 + spread),
            ),
            currency: currency.to_string(),
        }
    }
}

/// Complete economic analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicAnalysis {
    /// Lost throughput impact (marginal contribution)
    pub throughput_loss: EconomicImpact,
    /// Material waste from scrap
    pub material_waste: EconomicImpact,
    /// Rework cost (material + labor)
    pub rework_cost: EconomicImpact,
    /// Opportunity cost of downtime
    pub opportunity_cost: EconomicImpact,
    /// Total impact (sum of above)
    pub total_impact: EconomicImpact,
}

/// Calculate marginal contribution loss from reduced throughput
pub fn calculate_throughput_loss(
    lost_units: u32,
    params: &EconomicParameters,
) -> EconomicImpact {
    let (low, central, high) = params.marginal_contribution;
    
    EconomicImpact {
        description_key: "economics.throughput_loss".to_string(),
        low_estimate: (lost_units as f64) * low,
        central_estimate: (lost_units as f64) * central,
        high_estimate: (lost_units as f64) * high,
        currency: params.currency.clone(),
        assumptions: vec![
            "economics.assumptions.marginal_contribution".to_string(),
            "economics.assumptions.lost_units_calculated".to_string(),
        ],
    }
}

/// Calculate direct material waste from scrap
pub fn calculate_material_waste(
    scrap_units: u32,
    params: &EconomicParameters,
) -> EconomicImpact {
    let (low, central, high) = params.material_cost;
    
    EconomicImpact {
        description_key: "economics.material_waste".to_string(),
        low_estimate: (scrap_units as f64) * low,
        central_estimate: (scrap_units as f64) * central,
        high_estimate: (scrap_units as f64) * high,
        currency: params.currency.clone(),
        assumptions: vec![
            "economics.assumptions.material_cost_per_unit".to_string(),
            "economics.assumptions.scrap_is_total_loss".to_string(),
        ],
    }
}

/// Calculate rework cost (material + labor)
pub fn calculate_rework_cost(
    rework_units: u32,
    avg_rework_time_hours: f64,
    params: &EconomicParameters,
) -> EconomicImpact {
    let (mat_low, mat_central, mat_high) = params.material_cost;
    let (labor_low, labor_central, labor_high) = params.labor_cost_per_hour;
    
    // Material cost (assume 50% additional material usage)
    let material_factor = 0.5;
    let material_low = (rework_units as f64) * mat_low * material_factor;
    let material_central = (rework_units as f64) * mat_central * material_factor;
    let material_high = (rework_units as f64) * mat_high * material_factor;
    
    // Labor cost
    let total_rework_hours = (rework_units as f64) * avg_rework_time_hours;
    let labor_cost_low = total_rework_hours * labor_low;
    let labor_cost_central = total_rework_hours * labor_central;
    let labor_cost_high = total_rework_hours * labor_high;
    
    EconomicImpact {
        description_key: "economics.rework_cost".to_string(),
        low_estimate: material_low + labor_cost_low,
        central_estimate: material_central + labor_cost_central,
        high_estimate: material_high + labor_cost_high,
        currency: params.currency.clone(),
        assumptions: vec![
            "economics.assumptions.rework_material_factor".to_string(),
            "economics.assumptions.rework_time_estimate".to_string(),
            "economics.assumptions.labor_cost_per_hour".to_string(),
        ],
    }
}

/// Calculate opportunity cost of downtime
pub fn calculate_opportunity_cost(
    downtime_hours: f64,
    theoretical_units_per_hour: f64,
    params: &EconomicParameters,
) -> EconomicImpact {
    let (low, central, high) = params.marginal_contribution;
    
    let lost_units = downtime_hours * theoretical_units_per_hour;
    
    EconomicImpact {
        description_key: "economics.opportunity_cost".to_string(),
        low_estimate: lost_units * low,
        central_estimate: lost_units * central,
        high_estimate: lost_units * high,
        currency: params.currency.clone(),
        assumptions: vec![
            "economics.assumptions.marginal_contribution".to_string(),
            "economics.assumptions.theoretical_capacity".to_string(),
            "economics.assumptions.demand_exists".to_string(),
        ],
    }
}

/// Sum economic impacts
pub fn sum_economic_impacts(impacts: &[EconomicImpact]) -> EconomicImpact {
    let low_sum: f64 = impacts.iter().map(|i| i.low_estimate).sum();
    let central_sum: f64 = impacts.iter().map(|i| i.central_estimate).sum();
    let high_sum: f64 = impacts.iter().map(|i| i.high_estimate).sum();
    
    let currency = impacts.first()
        .map(|i| i.currency.clone())
        .unwrap_or_else(|| "USD".to_string());
    
    let mut all_assumptions: Vec<String> = impacts
        .iter()
        .flat_map(|i| i.assumptions.iter().cloned())
        .collect();
    all_assumptions.sort();
    all_assumptions.dedup();
    
    EconomicImpact {
        description_key: "economics.total_impact".to_string(),
        low_estimate: low_sum,
        central_estimate: central_sum,
        high_estimate: high_sum,
        currency,
        assumptions: all_assumptions,
    }
}

/// Perform complete economic analysis
pub fn analyze_economics(
    lost_units: u32,
    scrap_units: u32,
    rework_units: u32,
    downtime_hours: f64,
    theoretical_units_per_hour: f64,
    avg_rework_time_hours: f64,
    params: &EconomicParameters,
) -> EconomicAnalysis {
    let throughput_loss = calculate_throughput_loss(lost_units, params);
    let material_waste = calculate_material_waste(scrap_units, params);
    let rework_cost = calculate_rework_cost(rework_units, avg_rework_time_hours, params);
    let opportunity_cost = calculate_opportunity_cost(downtime_hours, theoretical_units_per_hour, params);
    
    let total_impact = sum_economic_impacts(&[
        throughput_loss.clone(),
        material_waste.clone(),
        rework_cost.clone(),
        opportunity_cost.clone(),
    ]);
    
    EconomicAnalysis {
        throughput_loss,
        material_waste,
        rework_cost,
        opportunity_cost,
        total_impact,
    }
}