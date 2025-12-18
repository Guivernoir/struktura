use crate::pricing::{errors::*, models::*, traits::*};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Central coordination for price providers - Now with 100% more DuckDuckGo
/// 
/// Mission Control has been upgraded. No more API keys to lose, no more rate limits to hit.
/// Just good old-fashioned web reconnaissance with a side of plausible deniability.
pub struct PricingEngine {
    providers: Arc<RwLock<Vec<Arc<dyn PriceProvider>>>>,
    converter: Option<Arc<dyn CurrencyConverter>>,
}

impl PricingEngine {
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(Vec::new())),
            converter: None,
        }
    }
    
    pub async fn register_provider(&self, provider: Arc<dyn PriceProvider>) {
        let mut providers = self.providers.write().await;
        providers.push(provider);
    }
    
    pub fn with_converter(mut self, converter: Arc<dyn CurrencyConverter>) -> Self {
        self.converter = Some(converter);
        self
    }
    
    async fn find_providers(&self, location: &Location) -> Vec<Arc<dyn PriceProvider>> {
        let providers = self.providers.read().await;
        providers
            .iter()
            .filter(|p| p.supports_location(location))
            .cloned()
            .collect()
    }
    
    pub async fn fetch_prices(&self, request: &PriceRequest) -> PricingResult<PriceResponse> {
        let providers = self.find_providers(&request.location).await;
        
        if providers.is_empty() {
            return Err(PricingError::UnsupportedLocation(
                request.location.country_code.clone()
            ));
        }
        
        let mut combined = PriceResponse::new();
        
        // Deploy all reconnaissance units in parallel
        let mut tasks = Vec::new();
        for provider in providers {
            let req = request.clone();
            tasks.push(async move {
                (provider.name().to_string(), provider.fetch_prices(&req).await)
            });
        }
        
        let results = futures::future::join_all(tasks).await;
        
        // Compile intelligence reports
        for (provider_name, result) in results {
            match result {
                Ok(response) => {
                    combined.prices.extend(response.prices);
                    combined.unavailable.extend(response.unavailable);
                    combined.warnings.extend(response.warnings);
                }
                Err(e) => {
                    combined.warnings.push(format!(
                        "Provider '{}' encountered difficulties: {}", 
                        provider_name, e
                    ));
                }
            }
        }
        
        // Eliminate duplicate intelligence
        combined.unavailable.sort_by(|a, b| a.code.cmp(&b.code));
        combined.unavailable.dedup_by(|a, b| a.code == b.code);
        
        // Currency conversion operations
        if let Some(target_currency) = request.preferred_currency {
            if let Some(ref converter) = self.converter {
                for price in &mut combined.prices {
                    if price.currency != target_currency {
                        match converter.convert(price.price, price.currency, target_currency).await {
                            Ok(converted) => {
                                price.price = converted;
                                price.currency = target_currency;
                            }
                            Err(e) => {
                                combined.warnings.push(format!(
                                    "Currency conversion failed for {}: {}",
                                    price.material.code, e
                                ));
                            }
                        }
                    }
                }
            }
        }
        
        Ok(combined)
    }
    
    pub async fn list_providers(&self) -> Vec<String> {
        let providers = self.providers.read().await;
        providers.iter().map(|p| p.name().to_string()).collect()
    }
}

impl Default for PricingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize pricing engine with DuckDuckGo and static fallback
/// 
/// "The new standard operating procedure: Free, fast, and untraceable."
pub async fn init_pricing_engine() -> PricingResult<PricingEngine> {
    let engine = PricingEngine::new();
    
    // Primary reconnaissance: DuckDuckGo
    // No API keys. No rate limits. No corporate tracking.
    let ddg = Arc::new(DuckDuckGoProvider::new());
    engine.register_provider(ddg).await;
    
    // Emergency fallback: Static data
    // For when the network is down or you're working behind enemy lines
    let static_provider = Arc::new(StaticProvider::new());
    engine.register_provider(static_provider).await;
    
    // Currency converter for cross-border operations
    let converter = Arc::new(crate::pricing::converter::SimpleCurrencyConverter::new());
    let engine = engine.with_converter(converter);
    
    Ok(engine)
}