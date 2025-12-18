use serde::{de, Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Location representation - where the operation is taking place
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub country_code: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl Location {
    pub fn new(country_code: impl Into<String>) -> Self {
        Self {
            country_code: country_code.into(),
            region: None,
            city: None,
            postal_code: None,
            latitude: None,
            longitude: None,
        }
    }
    
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }
    
    pub fn with_city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }
    
    pub fn with_coordinates(mut self, lat: f64, lon: f64) -> Self {
        self.latitude = Some(lat);
        self.longitude = Some(lon);
        self
    }
}

/// Material categories for civil engineering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MaterialCategory {
    // Structural
    Concrete,
    Rebar,
    Steel,
    Lumber,
    
    // Finishing
    Roofing,
    Decking,
    Flooring,
    
    // Foundation
    Gravel,
    Sand,
    BlockWork,
    
    // Hardware
    Fasteners,
    Anchors,
    Connectors,
    
    // Other
    Custom(String),
}

impl MaterialCategory {
    pub fn as_str(&self) -> &str {
        match self {
            MaterialCategory::Concrete => "concrete",
            MaterialCategory::Rebar => "rebar",
            MaterialCategory::Steel => "steel",
            MaterialCategory::Lumber => "lumber",
            MaterialCategory::Roofing => "roofing",
            MaterialCategory::Decking => "decking",
            MaterialCategory::Flooring => "flooring",
            MaterialCategory::Gravel => "gravel",
            MaterialCategory::Sand => "sand",
            MaterialCategory::BlockWork => "blockwork",
            MaterialCategory::Fasteners => "fasteners",
            MaterialCategory::Anchors => "anchors",
            MaterialCategory::Connectors => "connectors",
            MaterialCategory::Custom(s) => s,
        }
    }
}

/// Material identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MaterialId {
    pub category: MaterialCategory,
    pub code: String,          // e.g., "concrete_30mpa", "lumber_2x4"
    pub unit: String,          // e.g., "m3", "m", "unit", "kg"
    pub description: String,   // Human-readable description
}

impl MaterialId {
    pub fn new(
        category: MaterialCategory,
        code: impl Into<String>,
        unit: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            category,
            code: code.into(),
            unit: unit.into(),
            description: description.into(),
        }
    }
}

/// Currency representation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Currency {
    USD,
    BRL,
    EUR,
    GBP,
    CAD,
}

impl Currency {
    pub fn code(&self) -> &str {
        match self {
            Currency::USD => "USD",
            Currency::BRL => "BRL",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::CAD => "CAD",
        }
    }
    
    pub fn symbol(&self) -> &str {
        match self {
            Currency::USD => "$",
            Currency::BRL => "R$",
            Currency::EUR => "€",
            Currency::GBP => "£",
            Currency::CAD => "CA$",
        }
    }
}

/// Store information - where to acquire the supplies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreInfo {
    pub name: String,
    pub address: String,
    pub distance_km: Option<f64>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub maps_link: Option<String>,
}

/// Price information with store details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceInfo {
    pub material: MaterialId,
    pub price: f64,
    pub currency: Currency,
    pub store: StoreInfo,
    pub in_stock: bool,
    pub last_updated: DateTime<Utc>,
    pub notes: Option<String>,
}

/// Price request - what we need and where
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRequest {
    pub materials: Vec<MaterialId>,
    pub location: Location,
    pub max_distance_km: Option<f64>,  // Maximum store distance
    pub preferred_currency: Option<Currency>,
}

impl PriceRequest {
    pub fn new(location: Location) -> Self {
        Self {
            materials: Vec::new(),
            location,
            max_distance_km: Some(50.0), // Default: 50km radius
            preferred_currency: None,
        }
    }
    
    pub fn add_material(mut self, material: MaterialId) -> Self {
        self.materials.push(material);
        self
    }
    
    pub fn with_max_distance(mut self, km: f64) -> Self {
        self.max_distance_km = Some(km);
        self
    }
    
    pub fn with_currency(mut self, currency: Currency) -> Self {
        self.preferred_currency = Some(currency);
        self
    }
}

/// Price response - the intelligence report
#[derive(Debug, Clone, Serialize)]
pub struct PriceResponse {
    pub prices: Vec<PriceInfo>,
    pub unavailable: Vec<MaterialId>,
    pub warnings: Vec<String>,
    pub searched_at: DateTime<Utc>,
}

impl PriceResponse {
    pub fn new() -> Self {
        Self {
            prices: Vec::new(),
            unavailable: Vec::new(),
            warnings: Vec::new(),
            searched_at: Utc::now(),
        }
    }
    
    /// Get best price for a material
    pub fn best_price(&self, material: &MaterialId) -> Option<&PriceInfo> {
        self.prices
            .iter()
            .filter(|p| &p.material == material)
            .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
    }
    
    /// Get all prices for a material, sorted by price
    pub fn all_prices_for(&self, material: &MaterialId) -> Vec<&PriceInfo> {
        let mut prices: Vec<_> = self.prices
            .iter()
            .filter(|p| &p.material == material)
            .collect();
        prices.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        prices
    }
    
    /// Get all stores that have all materials
    pub fn stores_with_all_materials(&self) -> Vec<String> {
        use std::collections::HashMap;
        
        let mut store_materials: HashMap<String, Vec<String>> = HashMap::new();
        
        for price in &self.prices {
            store_materials
                .entry(price.store.name.clone())
                .or_default()
                .push(price.material.code.clone());
        }
        
        let required: std::collections::HashSet<_> = 
            self.prices.iter().map(|p| p.material.code.clone()).collect();
        
        store_materials
            .into_iter()
            .filter(|(_, materials)| {
                let has: std::collections::HashSet<_> = materials.iter().cloned().collect();
                required.is_subset(&has)
            })
            .map(|(store, _)| store)
            .collect()
    }
}

impl Default for PriceResponse {
    fn default() -> Self {
        Self::new()
    }
}