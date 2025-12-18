use thiserror::Error;
use std::fmt::Display;

#[derive(Error, Debug, Display)]
pub enum PricingError {
    #[error("Location not supported: {0}")]
    UnsupportedLocation(String),
    
    #[error("Material '{0}' not found in any nearby stores")]
    MaterialNotFound(String),
    
    #[error("Provider '{0}' failed: {1}")]
    ProviderFailed(String, String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Invalid coordinates: {0}")]
    InvalidCoordinates(String),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("No stores found within {0}km radius")]
    NoStoresInRadius(f64),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl PricingError {
    /// Check if error warrants a retry
    /// Some battles are worth fighting again.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            PricingError::NetworkError(_) | PricingError::ApiError(_)
        )
    }
}

pub type PricingResult<T> = Result<T, PricingError>;