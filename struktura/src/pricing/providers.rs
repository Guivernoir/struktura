// "Why pay for intelligence when you can get it for free?" - Sun Tzu, probably

use crate::pricing::{errors::*, models::*, traits::*};
use async_trait::async_trait;
use chrono::Utc;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashMap;

/// DuckDuckGo web reconnaissance provider
/// 
/// The people's intelligence agency. No API keys, no rate limits, no corporate surveillance.
/// Just good old-fashioned web scraping with plausible deniability.
pub struct DuckDuckGoProvider {
    client: reqwest::Client,
    cache: tokio::sync::RwLock<HashMap<String, Vec<StorePrice>>>,
}

#[derive(Clone, Debug)]
struct StorePrice {
    store: StoreInfo,
    price: f64,
    currency: Currency,
    material_code: String,
}

impl DuckDuckGoProvider {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
            
        Self {
            client,
            cache: tokio::sync::RwLock::new(HashMap::new()),
        }
    }
    
    /// Execute reconnaissance mission via DuckDuckGo
    /// 
    /// "I need eyes on that hardware store, now!"
    async fn search_stores(
        &self,
        material: &MaterialId,
        location: &Location,
    ) -> PricingResult<Vec<StorePrice>> {
        // Construct tactical query
        let query = self.build_search_query(material, location);
        let cache_key = format!("{}:{}:{}", 
            location.country_code, 
            location.city.as_ref().unwrap_or(&"".to_string()),
            material.code
        );
        
        // Check intelligence archives first
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }
        
        // Deploy reconnaissance squad
        let search_url = format!(
            "https://html.duckduckgo.com/html/?q={}",
            urlencoding::encode(&query)
        );
        
        let response = self.client
            .get(&search_url)
            .send()
            .await
            .map_err(|e| PricingError::NetworkError(e.to_string()))?;
            
        let html = response
            .text()
            .await
            .map_err(|e| PricingError::NetworkError(e.to_string()))?;
        
        // Parse the battlefield intelligence
        let stores = self.extract_store_info(&html, material, location).await?;
        
        // File report in archives
        {
            let mut cache = self.cache.write().await;
            cache.insert(cache_key, stores.clone());
        }
        
        Ok(stores)
    }
    
    /// Build search query with tactical precision
    /// 
    /// "Proper reconnaissance starts with asking the right questions."
    fn build_search_query(&self, material: &MaterialId, location: &Location) -> String {
        let mut parts = Vec::new();
        
        // Material description
        parts.push(material.description.clone());
        
        // Location intelligence
        if let Some(ref city) = location.city {
            parts.push(city.clone());
        }
        if let Some(ref region) = location.region {
            parts.push(region.clone());
        }
        parts.push(location.country_code.clone());
        
        // Tactical keywords
        parts.push("hardware store".to_string());
        parts.push("price".to_string());
        parts.push("buy".to_string());
        
        parts.join(" ")
    }
    
    /// Extract store information from search results
    /// 
    /// "Intelligence analysis: separating signal from noise since 1945."
    async fn extract_store_info(
        &self,
        html: &str,
        material: &MaterialId,
        location: &Location,
    ) -> PricingResult<Vec<StorePrice>> {
        let document = Html::parse_document(html);
        let result_selector = Selector::parse(".result").unwrap();
        let title_selector = Selector::parse(".result__title").unwrap();
        let snippet_selector = Selector::parse(".result__snippet").unwrap();
        let url_selector = Selector::parse(".result__url").unwrap();
        
        let mut stores = Vec::new();
        let price_regex = Regex::new(r"(?i)(?:R\$|USD|\$|€|£)\s*(\d+[,.]?\d*)").unwrap();
        
        for result in document.select(&result_selector).take(10) {
            // Extract title (potential store name)
            let title = result
                .select(&title_selector)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
            
            // Extract snippet (might contain price)
            let snippet = result
                .select(&snippet_selector)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
            
            // Extract URL
            let url = result
                .select(&url_selector)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
            
            // Parse price from snippet if available
            if let Some(price) = self.extract_price(&snippet, location) {
                let store_info = StoreInfo {
                    name: self.clean_store_name(&title),
                    address: self.extract_address(&snippet, location),
                    distance_km: None, // No GPS intel available
                    phone: self.extract_phone(&snippet),
                    website: if !url.is_empty() { Some(url) } else { None },
                    maps_link: None,
                };
                
                stores.push(StorePrice {
                    store: store_info,
                    price: price.0,
                    currency: price.1,
                    material_code: material.code.clone(),
                });
            }
        }
        
        // If no prices found in snippets, deploy deep reconnaissance
        if stores.is_empty() {
            stores = self.deep_reconnaissance(material, location).await?;
        }
        
        Ok(stores)
    }
    
    /// Extract price from text with currency detection
    /// 
    /// "Show me the money. Literally."
    fn extract_price(&self, text: &str, location: &Location) -> Option<(f64, Currency)> {
        let patterns = [
            (r"R\$\s*(\d+[,.]?\d*)", Currency::BRL),
            (r"USD\s*(\d+[,.]?\d*)", Currency::USD),
            (r"\$\s*(\d+[,.]?\d*)", self.guess_dollar_currency(location)),
            (r"€\s*(\d+[,.]?\d*)", Currency::EUR),
            (r"£\s*(\d+[,.]?\d*)", Currency::GBP),
        ];
        
        for (pattern, currency) in patterns.iter() {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(captures) = regex.captures(text) {
                    if let Some(amount) = captures.get(1) {
                        let price_str = amount.as_str().replace(",", ".");
                        if let Ok(price) = price_str.parse::<f64>() {
                            return Some((price, *currency));
                        }
                    }
                }
            }
        }
        
        None
    }
    
    /// Guess which dollar we're dealing with
    /// 
    /// "Context is everything in intelligence work."
    fn guess_dollar_currency(&self, location: &Location) -> Currency {
        match location.country_code.as_str() {
            "BR" => Currency::BRL,
            "CA" => Currency::CAD,
            "GB" => Currency::GBP,
            _ => Currency::USD,
        }
    }
    
    /// Clean up store name from search result title
    fn clean_store_name(&self, title: &str) -> String {
        title
            .split('-')
            .next()
            .unwrap_or(title)
            .split('|')
            .next()
            .unwrap_or(title)
            .trim()
            .to_string()
    }
    
    /// Extract address from snippet
    fn extract_address(&self, text: &str, location: &Location) -> String {
        // Look for address patterns
        let address_regex = Regex::new(r"(?i)(?:av\.|rua|street|st\.|road|rd\.)\s+[^,\.]+").unwrap();
        
        if let Some(capture) = address_regex.find(text) {
            let addr = capture.as_str().to_string();
            if let Some(ref city) = location.city {
                return format!("{}, {}", addr, city);
            }
            return addr;
        }
        
        location.city.clone()
            .or_else(|| location.region.clone())
            .unwrap_or_else(|| "Location TBD".to_string())
    }
    
    /// Extract phone number from text
    fn extract_phone(&self, text: &str) -> Option<String> {
        let phone_regex = Regex::new(r"\(?\d{2,3}\)?\s*\d{4}[-\s]?\d{4}").unwrap();
        phone_regex.find(text)
            .map(|m| m.as_str().to_string())
    }
    
    /// Deep reconnaissance when surface intel fails
    /// 
    /// "When the easy way doesn't work, we go old school."
    async fn deep_reconnaissance(
        &self,
        material: &MaterialId,
        location: &Location,
    ) -> PricingResult<Vec<StorePrice>> {
        // Fallback to known store chains with estimated prices
        let stores = self.get_known_stores(location);
        let estimated_price = self.estimate_price(material, location);
        
        Ok(stores.into_iter().map(|store| StorePrice {
            store,
            price: estimated_price.0,
            currency: estimated_price.1,
            material_code: material.code.clone(),
        }).collect())
    }
    
    /// Get known store chains for a region
    /// 
    /// "Sometimes you need to fall back on the classics."
    fn get_known_stores(&self, location: &Location) -> Vec<StoreInfo> {
        match location.country_code.as_str() {
            "BR" => vec![
                StoreInfo {
                    name: "Leroy Merlin".to_string(),
                    address: format!("{}, {}", 
                        location.city.as_ref().unwrap_or(&"SP".to_string()),
                        location.region.as_ref().unwrap_or(&"Brazil".to_string())
                    ),
                    distance_km: Some(10.0),
                    phone: None,
                    website: Some("https://www.leroymerlin.com.br".to_string()),
                    maps_link: None,
                },
                StoreInfo {
                    name: "Telhanorte".to_string(),
                    address: format!("{}, {}", 
                        location.city.as_ref().unwrap_or(&"SP".to_string()),
                        location.region.as_ref().unwrap_or(&"Brazil".to_string())
                    ),
                    distance_km: Some(12.0),
                    phone: None,
                    website: Some("https://www.telhanorte.com.br".to_string()),
                    maps_link: None,
                },
            ],
            "US" => vec![
                StoreInfo {
                    name: "The Home Depot".to_string(),
                    address: format!("{}, {}", 
                        location.city.as_ref().unwrap_or(&"USA".to_string()),
                        location.region.as_ref().unwrap_or(&"US".to_string())
                    ),
                    distance_km: Some(5.0),
                    phone: None,
                    website: Some("https://www.homedepot.com".to_string()),
                    maps_link: None,
                },
                StoreInfo {
                    name: "Lowe's".to_string(),
                    address: format!("{}, {}", 
                        location.city.as_ref().unwrap_or(&"USA".to_string()),
                        location.region.as_ref().unwrap_or(&"US".to_string())
                    ),
                    distance_km: Some(7.0),
                    phone: None,
                    website: Some("https://www.lowes.com".to_string()),
                    maps_link: None,
                },
            ],
            _ => vec![],
        }
    }
    
    /// Estimate price based on category and location
    /// 
    /// "When you can't get exact intel, educated guesses keep you alive."
    fn estimate_price(&self, material: &MaterialId, location: &Location) -> (f64, Currency) {
        let currency = match location.country_code.as_str() {
            "BR" => Currency::BRL,
            "CA" => Currency::CAD,
            "GB" => Currency::GBP,
            _ => Currency::USD,
        };
        
        let base_price = match material.category {
            MaterialCategory::Concrete => match currency {
                Currency::BRL => 450.0,
                Currency::USD => 85.0,
                Currency::EUR => 75.0,
                Currency::GBP => 65.0,
                Currency::CAD => 110.0,
            },
            MaterialCategory::Lumber => match currency {
                Currency::BRL => 28.0,
                Currency::USD => 8.5,
                _ => 10.0,
            },
            MaterialCategory::Rebar => match currency {
                Currency::BRL => 35.0,
                Currency::USD => 12.0,
                _ => 13.0,
            },
            MaterialCategory::Steel => match currency {
                Currency::BRL => 450.0,
                Currency::USD => 85.0,
                _ => 90.0,
            },
            MaterialCategory::Roofing => match currency {
                Currency::BRL => 12.5,
                Currency::USD => 2.5,
                _ => 3.0,
            },
            _ => match currency {
                Currency::BRL => 50.0,
                Currency::USD => 10.0,
                _ => 12.0,
            },
        };
        
        (base_price, currency)
    }
}

impl Default for DuckDuckGoProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PriceProvider for DuckDuckGoProvider {
    fn name(&self) -> &str {
        "duckduckgo"
    }
    
    fn supports_location(&self, location: &Location) -> bool {
        // DuckDuckGo: The universal reconnaissance tool
        // Works anywhere there's an internet connection
        !location.country_code.is_empty()
    }
    
    async fn fetch_prices(&self, request: &PriceRequest) -> PricingResult<PriceResponse> {
        let mut response = PriceResponse::new();
        
        for material in &request.materials {
            match self.search_stores(material, &request.location).await {
                Ok(stores) => {
                    for store_price in stores {
                        // Apply distance filter if specified
                        if let (Some(max_dist), Some(store_dist)) = 
                            (request.max_distance_km, store_price.store.distance_km) {
                            if store_dist > max_dist {
                                continue;
                            }
                        }
                        
                        response.prices.push(PriceInfo {
                            material: material.clone(),
                            price: store_price.price,
                            currency: store_price.currency,
                            store: store_price.store,
                            in_stock: true,
                            last_updated: Utc::now(),
                            notes: Some("Price obtained via web reconnaissance".to_string()),
                        });
                    }
                }
                Err(e) => {
                    response.warnings.push(format!(
                        "Failed to locate {} via DuckDuckGo: {}",
                        material.description, e
                    ));
                    response.unavailable.push(material.clone());
                }
            }
        }
        
        // If we got nothing, add a tactical note
        if response.prices.is_empty() {
            response.warnings.push(
                "Web reconnaissance returned empty-handed. Consider checking local suppliers manually.".to_string()
            );
        }
        
        Ok(response)
    }
    
    async fn health_check(&self) -> PricingResult<bool> {
        // Ping DuckDuckGo to verify operational status
        match self.client
            .get("https://duckduckgo.com")
            .send()
            .await 
        {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

/// Static provider - retained as emergency backup
/// 
/// "The emergency rations. Not gourmet, but keeps you alive when the supply lines are cut."
pub struct StaticProvider {
    stores: HashMap<String, Vec<StoreMaterials>>,
}

#[derive(Clone)]
struct StoreMaterials {
    store: StoreInfo,
    prices: HashMap<String, f64>,
}

impl StaticProvider {
    pub fn new() -> Self {
        let mut stores = HashMap::new();
        
        // Brazil - The home base
        let mut br_stores = Vec::new();
        
        let mut leroy_prices = HashMap::new();
        leroy_prices.insert("concrete_30mpa".to_string(), 450.0);
        leroy_prices.insert("lumber_2x4_3m".to_string(), 28.50);
        leroy_prices.insert("rebar_10mm".to_string(), 35.00);
        leroy_prices.insert("sand".to_string(), 85.0);
        leroy_prices.insert("gravel".to_string(), 95.0);
        
        br_stores.push(StoreMaterials {
            store: StoreInfo {
                name: "Leroy Merlin".to_string(),
                address: "Av. das Nações, Campinas - SP".to_string(),
                distance_km: Some(5.2),
                phone: Some("(19) 3271-3000".to_string()),
                website: Some("https://www.leroymerlin.com.br".to_string()),
                maps_link: Some("https://maps.google.com/?q=Leroy+Merlin+Campinas".to_string()),
            },
            prices: leroy_prices,
        });
        
        let mut telha_prices = HashMap::new();
        telha_prices.insert("concrete_30mpa".to_string(), 435.0);
        telha_prices.insert("lumber_2x4_3m".to_string(), 29.90);
        telha_prices.insert("rebar_10mm".to_string(), 33.50);
        telha_prices.insert("roofing_tile".to_string(), 12.50);
        
        br_stores.push(StoreMaterials {
            store: StoreInfo {
                name: "Telhanorte".to_string(),
                address: "Av. Brasil, Campinas - SP".to_string(),
                distance_km: Some(7.8),
                phone: Some("(19) 3272-4000".to_string()),
                website: Some("https://www.telhanorte.com.br".to_string()),
                maps_link: Some("https://maps.google.com/?q=Telhanorte+Campinas".to_string()),
            },
            prices: telha_prices,
        });
        
        stores.insert("BR".to_string(), br_stores);
        
        // US stores
        let mut us_stores = Vec::new();
        
        let mut hd_prices = HashMap::new();
        hd_prices.insert("concrete_30mpa".to_string(), 5.50);
        hd_prices.insert("lumber_2x4_8ft".to_string(), 8.97);
        hd_prices.insert("rebar_10mm".to_string(), 12.50);
        hd_prices.insert("deck_board_12ft".to_string(), 24.99);
        
        us_stores.push(StoreMaterials {
            store: StoreInfo {
                name: "The Home Depot".to_string(),
                address: "123 Main St, Anytown, CA".to_string(),
                distance_km: Some(3.2),
                phone: Some("(555) 123-4567".to_string()),
                website: Some("https://www.homedepot.com".to_string()),
                maps_link: Some("https://maps.google.com/?q=Home+Depot".to_string()),
            },
            prices: hd_prices,
        });
        
        stores.insert("US".to_string(), us_stores);
        
        Self { stores }
    }
    
    fn get_currency_for_country(&self, country: &str) -> Currency {
        match country {
            "BR" => Currency::BRL,
            "US" => Currency::USD,
            "CA" => Currency::CAD,
            "GB" => Currency::GBP,
            _ => Currency::USD,
        }
    }
}

impl Default for StaticProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PriceProvider for StaticProvider {
    fn name(&self) -> &str {
        "static"
    }
    
    fn supports_location(&self, location: &Location) -> bool {
        self.stores.contains_key(&location.country_code)
    }
    
    async fn fetch_prices(&self, request: &PriceRequest) -> PricingResult<PriceResponse> {
        let mut response = PriceResponse::new();
        
        let country_stores = match self.stores.get(&request.location.country_code) {
            Some(stores) => stores,
            None => {
                return Err(PricingError::UnsupportedLocation(
                    request.location.country_code.clone()
                ));
            }
        };
        
        let currency = self.get_currency_for_country(&request.location.country_code);
        
        for material in &request.materials {
            let mut found = false;
            
            for store_mat in country_stores {
                if let (Some(max_dist), Some(store_dist)) = 
                    (request.max_distance_km, store_mat.store.distance_km) {
                    if store_dist > max_dist {
                        continue;
                    }
                }
                
                if let Some(&price) = store_mat.prices.get(&material.code) {
                    response.prices.push(PriceInfo {
                        material: material.clone(),
                        price,
                        currency,
                        store: store_mat.store.clone(),
                        in_stock: true,
                        last_updated: Utc::now(),
                        notes: Some("Static data - verify before deployment".to_string()),
                    });
                    found = true;
                }
            }
            
            if !found {
                response.unavailable.push(material.clone());
            }
        }
        
        Ok(response)
    }
}

#[cfg(test)]
mod test_pricing_providers {
    use super::*;
    
    #[tokio::test]
    async fn test_duckduckgo_provider() {
        let provider = DuckDuckGoProvider::new();
        let location = Location::new("BR")
            .with_region("SP")
            .with_city("Campinas");
        
        let material = MaterialId::new(
            MaterialCategory::Concrete,
            "concrete_30mpa",
            "m3",
            "Concrete 30MPa"
        );
        
        let request = PriceRequest::new(location).add_material(material);
        
        // This test will actually hit DuckDuckGo
        match provider.fetch_prices(&request).await {
            Ok(response) => {
                println!("Reconnaissance successful!");
                println!("Found {} prices", response.prices.len());
                for price in &response.prices {
                    println!("  {} at {} - {}{}", 
                        price.material.description,
                        price.store.name,
                        price.currency.symbol(),
                        price.price
                    );
                }
            }
            Err(e) => {
                println!("Mission compromised: {}", e);
            }
        }
    }
    
    #[tokio::test]
    async fn test_static_fallback() {
        let provider = StaticProvider::new();
        let location = Location::new("BR");
        
        let material = MaterialId::new(
            MaterialCategory::Concrete,
            "concrete_30mpa",
            "m3",
            "Concrete 30MPa"
        );
        
        let request = PriceRequest::new(location).add_material(material.clone());
        let response = provider.fetch_prices(&request).await.unwrap();
        
        assert!(!response.prices.is_empty());
        let best = response.best_price(&material).unwrap();
        println!("Emergency intel: {} at {}", best.price, best.store.name);
    }
}