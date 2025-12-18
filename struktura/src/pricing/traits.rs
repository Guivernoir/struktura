use crate::pricing::{errors::PricingResult, models::*};
use async_trait::async_trait;

/// Price provider trait - the reconnaissance interface
/// 
/// Each provider scouts different territory (APIs, databases, web scraping).
/// The mission: find materials and their prices at nearby stores.
#[async_trait]
pub trait PriceProvider: Send + Sync {
    /// Provider identification
    fn name(&self) -> &str;
    
    /// Check if this provider can operate in the given location
    fn supports_location(&self, location: &Location) -> bool;
    
    /// Fetch prices from nearby stores
    /// 
    /// This is where the provider does its reconnaissance work.
    /// Returns prices with store information.
    async fn fetch_prices(&self, request: &PriceRequest) -> PricingResult<PriceResponse>;
    
    /// Check if provider is operational
    async fn health_check(&self) -> PricingResult<bool> {
        Ok(true)
    }
    
    /// Supported material categories
    fn supported_categories(&self) -> Vec<MaterialCategory> {
        vec![] // Empty = supports all
    }
}

/// Currency converter for cross-border operations
#[async_trait]
pub trait CurrencyConverter: Send + Sync {
    /// Convert amount between currencies
    async fn convert(&self, amount: f64, from: Currency, to: Currency) -> PricingResult<f64>;
    
    /// Get current exchange rate
    async fn get_rate(&self, from: Currency, to: Currency) -> PricingResult<f64>;
}