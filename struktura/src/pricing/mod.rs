pub mod errors;
pub mod models;
pub mod traits;
pub mod providers;
pub mod converter;
pub mod registry;

// Re-exports for tactical convenience
pub use errors::{PricingError, PricingResult};
pub use models::*;
pub use traits::*;
pub use registry::{PricingEngine, init_pricing_engine};

/// Quick price lookup helper
pub fn quick_lookup(
    country: &str,
    materials: Vec<(MaterialCategory, &str, &str, &str)>,
) -> PriceRequest {
    let location = Location::new(country);
    let mut request = PriceRequest::new(location);
    
    for (category, code, unit, desc) in materials {
        request = request.add_material(MaterialId::new(category, code, unit, desc));
    }
    
    request
}

#[cfg(test)]
mod test_pricing_module {
    use super::*;
    
    #[tokio::test]
    async fn test_quick_lookup() {
        let engine = init_pricing_engine().await.unwrap();
        
        let request = quick_lookup(
            "BR",
            vec![
                (MaterialCategory::Concrete, "concrete_30mpa", "m3", "Concrete 30MPa"),
                (MaterialCategory::Lumber, "lumber_2x4_3m", "unit", "2x4 Lumber 3m"),
            ],
        );
        
        let response = engine.fetch_prices(&request).await.unwrap();
        assert!(!response.prices.is_empty());
    }
}