//! Production count assumptions
//! 
//! The core production numbers with source tracking.

use super::*;

/// Production count summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionSummary {
    pub total_units: InputValue<u32>,
    pub good_units: InputValue<u32>,
    pub scrap_units: InputValue<u32>,
    pub reworked_units: InputValue<u32>,
}

impl ProductionSummary {
    /// Validate internal consistency
    pub fn is_consistent(&self) -> bool {
        let total = *self.total_units.value();
        let good = *self.good_units.value();
        let scrap = *self.scrap_units.value();
        let rework = *self.reworked_units.value();
        
        good + scrap + rework == total
    }
    
    /// Get the discrepancy (if any)
    pub fn discrepancy(&self) -> i64 {
        let total = *self.total_units.value() as i64;
        let good = *self.good_units.value() as i64;
        let scrap = *self.scrap_units.value() as i64;
        let rework = *self.reworked_units.value() as i64;
        
        (good + scrap + rework) - total
    }
}

/// Count model builder for common patterns
pub struct CountModelBuilder {
    total: Option<InputValue<u32>>,
    good: Option<InputValue<u32>>,
    scrap: Option<InputValue<u32>>,
    rework: Option<InputValue<u32>>,
}

impl CountModelBuilder {
    pub fn new() -> Self {
        Self {
            total: None,
            good: None,
            scrap: None,
            rework: None,
        }
    }
    
    pub fn total(mut self, value: u32) -> Self {
        self.total = Some(InputValue::Explicit(value));
        self
    }
    
    pub fn good(mut self, value: u32) -> Self {
        self.good = Some(InputValue::Explicit(value));
        self
    }
    
    pub fn scrap(mut self, value: u32) -> Self {
        self.scrap = Some(InputValue::Explicit(value));
        self
    }
    
    pub fn rework(mut self, value: u32) -> Self {
        self.rework = Some(InputValue::Explicit(value));
        self
    }
    
    /// Build with inference where needed
    pub fn build(self) -> ProductionSummary {
        match (&self.total, &self.good, &self.scrap, &self.rework) {
            // All explicit - ideal case
            (Some(t), Some(g), Some(s), Some(r)) => ProductionSummary {
                total_units: t.clone(),
                good_units: g.clone(),
                scrap_units: s.clone(),
                reworked_units: r.clone(),
            },
            
            // Missing total - infer it
            (None, Some(g), Some(s), Some(r)) => {
                let total = *g.value() + *s.value() + *r.value();
                ProductionSummary {
                    total_units: InputValue::Inferred(total),
                    good_units: g.clone(),
                    scrap_units: s.clone(),
                    reworked_units: r.clone(),
                }
            }
            
            // Missing good - infer it
            (Some(t), None, Some(s), Some(r)) => {
                let good = (*t.value()).saturating_sub(*s.value() + *r.value());
                ProductionSummary {
                    total_units: t.clone(),
                    good_units: InputValue::Inferred(good),
                    scrap_units: s.clone(),
                    reworked_units: r.clone(),
                }
            }
            
            // Default to zeros for missing values
            _ => ProductionSummary {
                total_units: self.total.clone().unwrap_or(InputValue::Default(0)),
                good_units: self.good.clone().unwrap_or(InputValue::Default(0)),
                scrap_units: self.scrap.clone().unwrap_or(InputValue::Default(0)),
                reworked_units: self.rework.clone().unwrap_or(InputValue::Default(0)),
            },
        }
    }
}

impl Default for CountModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}

