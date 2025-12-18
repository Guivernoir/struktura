use crate::pricing::{errors::*, models::Currency, traits::CurrencyConverter};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Simple currency converter with static rates
/// 
/// For tactical operations. Real rates would require API integration.
pub struct SimpleCurrencyConverter {
    rates: RwLock<HashMap<(Currency, Currency), f64>>,
}

impl SimpleCurrencyConverter {
    pub fn new() -> Self {
        let mut rates = HashMap::new();
        
        // Static rates (approximate, Dec 2024)
        rates.insert((Currency::USD, Currency::BRL), 4.95);
        rates.insert((Currency::USD, Currency::EUR), 0.93);
        rates.insert((Currency::USD, Currency::GBP), 0.79);
        rates.insert((Currency::USD, Currency::CAD), 1.35);
        
        rates.insert((Currency::BRL, Currency::USD), 0.202);
        rates.insert((Currency::EUR, Currency::USD), 1.075);
        rates.insert((Currency::GBP, Currency::USD), 1.266);
        rates.insert((Currency::CAD, Currency::USD), 0.741);
        
        // Cross rates (triangulated through USD)
        rates.insert((Currency::BRL, Currency::EUR), 0.188);
        rates.insert((Currency::EUR, Currency::BRL), 5.32);
        
        Self {
            rates: RwLock::new(rates),
        }
    }
    
    pub async fn update_rate(&self, from: Currency, to: Currency, rate: f64) -> PricingResult<()> {
        if rate <= 0.0 {
            return Err(PricingError::ConfigError("Rate must be positive".to_string()));
        }
        
        let mut rates = self.rates.write().await;
        rates.insert((from, to), rate);
        rates.insert((to, from), 1.0 / rate);
        
        Ok(())
    }
}

impl Default for SimpleCurrencyConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CurrencyConverter for SimpleCurrencyConverter {
    async fn convert(&self, amount: f64, from: Currency, to: Currency) -> PricingResult<f64> {
        if from == to {
            return Ok(amount);
        }
        
        let rate = self.get_rate(from, to).await?;
        Ok(amount * rate)
    }
    
    async fn get_rate(&self, from: Currency, to: Currency) -> PricingResult<f64> {
        if from == to {
            return Ok(1.0);
        }
        
        let rates = self.rates.read().await;
        
        rates.get(&(from, to))
            .copied()
            .ok_or_else(|| PricingError::ApiError(
                format!("No rate available for {} to {}", from.code(), to.code())
            ))
    }
}

#[cfg(test)]
mod test_pricing_convertion {
    use super::*;
    
    #[tokio::test]
    async fn test_conversion() {
        let converter = SimpleCurrencyConverter::new();
        
        let result = converter.convert(100.0, Currency::USD, Currency::BRL).await.unwrap();
        assert!(result > 400.0 && result < 600.0);
        
        let same = converter.convert(100.0, Currency::USD, Currency::USD).await.unwrap();
        assert_eq!(same, 100.0);
    }
}