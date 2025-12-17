use crate::calculus::beginner::{
    errors::{BeginnerError, BeginnerResult},
    models::*,
    traits::BeginnerCalculator,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Thread-safe calculator registry
#[derive(Clone)]
pub struct BeginnerRegistry {
    pub calculators: Arc<HashMap<String, Arc<dyn BeginnerCalculator>>>,
    pub category_index: Arc<HashMap<CalculatorCategory, Vec<String>>>,
    pub tags_index: Arc<HashMap<String, Vec<String>>>,
}

impl BeginnerRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            calculators: Arc::new(HashMap::new()),
            category_index: Arc::new(HashMap::new()),
            tags_index: Arc::new(HashMap::new()),
        }
    }

    /// Register a calculator
    pub fn register(&mut self, calculator: Arc<dyn BeginnerCalculator>) {
        let id = calculator.id().to_string();
        let category = calculator.category();

        let mut calculators = (*self.calculators).clone();
        let mut category_index = (*self.category_index).clone();

        calculators.insert(id.clone(), calculator);

        category_index
            .entry(category)
            .or_insert_with(Vec::new)
            .push(id.clone());

        self.calculators = Arc::new(calculators);
        self.category_index = Arc::new(category_index);
    }

    /// Find calculator by ID
    pub fn find(&self, id: &str) -> BeginnerResult<Arc<dyn BeginnerCalculator>> {
        self.calculators
            .get(id)
            .cloned()
            .ok_or_else(|| BeginnerError::CalculatorNotFound(id.to_string()))
    }

    /// Get all calculators
    pub fn all(&self) -> Vec<Arc<dyn BeginnerCalculator>> {
        self.calculators.values().cloned().collect()
    }

    /// Get calculators by category
    pub fn by_category(&self, category: CalculatorCategory) -> Vec<Arc<dyn BeginnerCalculator>> {
        self.category_index
            .get(&category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.calculators.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Search calculators by keyword
    pub fn search(&self, query: &str) -> Vec<Arc<dyn BeginnerCalculator>> {
        let query_lower = query.to_lowercase();
        
        self.calculators
            .values()
            .filter(|calc| {
                let metadata = calc.metadata();
                metadata.name.to_lowercase().contains(&query_lower)
                    || metadata.description.to_lowercase().contains(&query_lower)
                    || metadata.id.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect()
    }

    /// Generate complete API catalogue
    pub fn catalogue(&self) -> BeginnerCalculatorCatalogue {
        let categories = vec![
            BeginnerCategoryInfo {
                id: "garden".to_string(),
                name: "Garden & Landscaping".to_string(),
                description: "Planter boxes, raised beds, mulch, and garden materials".to_string(),
                icon: Some("🌱".to_string()),
            },
            BeginnerCategoryInfo {
                id: "interiors".to_string(),
                name: "Interior Construction".to_string(),
                description: "Walls, framing, drywall, and interior finishes".to_string(),
                icon: Some("🏠".to_string()),
            },
            BeginnerCategoryInfo {
                id: "outdoors".to_string(),
                name: "Outdoor Construction".to_string(),
                description: "Decks, patios, slabs, and exterior structures".to_string(),
                icon: Some("🌳".to_string()),
            },
            BeginnerCategoryInfo {
                id: "utilities".to_string(),
                name: "Utilities & Finishes".to_string(),
                description: "Paint, tile, flooring, and finishing materials".to_string(),
                icon: Some("🎨".to_string()),
            },
        ];

        let calculators: Vec<BeginnerCalculatorMetadata> = self
            .all()
            .iter()
            .map(|calc| calc.metadata())
            .collect();

        BeginnerCalculatorCatalogue {
            version: env!("CARGO_PKG_VERSION").to_string(),
            categories,
            calculators,
            disclaimer: "Calculations are estimates only. Consult professionals for accurate assessments.".to_string(),
        }
    }

    /// Get statistics about the registry
    pub fn stats(&self) -> RegistryStats {
        let mut by_category: HashMap<String, usize> = HashMap::new();
        let mut total_parameters = 0;

        for calc in self.all() {
            let metadata = calc.metadata();
            
            *by_category.entry(metadata.category.clone()).or_insert(0) += 1;
            
            total_parameters += metadata.parameters.len();
        }

        RegistryStats {
            total_calculators: self.calculators.len(),
            by_category,
            total_parameters,
        }
    }
}

/// Registry statistics
#[derive(Debug, Clone)]
pub struct RegistryStats {
    pub total_calculators: usize,
    pub by_category: HashMap<String, usize>,
    pub total_parameters: usize,
}

/// Builder for registry
pub struct RegistryBuilder {
    registry: BeginnerRegistry,
}

impl RegistryBuilder {
    pub fn new() -> Self {
        Self {
            registry: BeginnerRegistry::new(),
        }
    }

    pub fn with_calculator(mut self, calculator: Arc<dyn BeginnerCalculator>) -> Self {
        self.registry.register(calculator);
        self
    }

    pub fn build(self) -> BeginnerRegistry {
        self.registry
    }
}

/// Create default registry with all calculators
pub fn create_default_registry() -> BeginnerRegistry {
    use crate::calculus::beginner::calculators;

    RegistryBuilder::new()
        // Outdoors registry
        .with_calculator(Arc::new(calculators::outdoors::DeckCalculator))
        .with_calculator(Arc::new(calculators::outdoors::ConcreteSlabCalculator))
        .with_calculator(Arc::new(calculators::outdoors::PatioCalculator))
        .with_calculator(Arc::new(calculators::outdoors::FenceCalculator))
        .with_calculator(Arc::new(calculators::outdoors::RetainingWallCalculator))
        .with_calculator(Arc::new(calculators::outdoors::PergolaCalculator))
        .with_calculator(Arc::new(calculators::outdoors::ShedFoundationCalculator))
        .with_calculator(Arc::new(calculators::outdoors::DrivewayCalculator))

        // Garden registry
        .with_calculator(Arc::new(calculators::garden::PlanterBoxCalculator))
        .with_calculator(Arc::new(calculators::garden::MulchBedCalculator))
        .with_calculator(Arc::new(calculators::garden::RaisedGardenBedCalculator))
        .with_calculator(Arc::new(calculators::garden::CompostBinCalculator))
        .with_calculator(Arc::new(calculators::garden::GravelPathCalculator))
        .with_calculator(Arc::new(calculators::garden::SteppingStoneCalculator))
        .with_calculator(Arc::new(calculators::garden::DripIrrigationCalculator))
        .with_calculator(Arc::new(calculators::garden::SprinklerCoverageCalculator))
        .with_calculator(Arc::new(calculators::garden::LawnSeedCalculator))
        .with_calculator(Arc::new(calculators::garden::SodCalculator))
        .with_calculator(Arc::new(calculators::garden::SmallRetainingWallCalculator))

        // Interiors registry
        .with_calculator(Arc::new(calculators::interiors::WallFramingCalculator))
        .with_calculator(Arc::new(calculators::interiors::DrywallCountCalculator))
        .with_calculator(Arc::new(calculators::interiors::HardwoodFlooringCalculator))
        .with_calculator(Arc::new(calculators::interiors::LaminateFlooringCalculator))
        .with_calculator(Arc::new(calculators::interiors::InsulationCalculator))
        .with_calculator(Arc::new(calculators::interiors::DropCeilingCalculator))
        .with_calculator(Arc::new(calculators::interiors::DrywallCeilingCalculator))
        .with_calculator(Arc::new(calculators::interiors::BaseboardCalculator))
        .with_calculator(Arc::new(calculators::interiors::CrownMoldingCalculator))

        // Utilities registry
        .with_calculator(Arc::new(calculators::utilities::PaintCoverageCalculator))
        .with_calculator(Arc::new(calculators::utilities::TileCountCalculator))
        .with_calculator(Arc::new(calculators::utilities::WallpaperCalculator))
        .with_calculator(Arc::new(calculators::utilities::RecessedLightingCalculator))
        .with_calculator(Arc::new(calculators::utilities::TrackLightingCalculator))
        .with_calculator(Arc::new(calculators::utilities::HVACSizingCalculator))
        .with_calculator(Arc::new(calculators::utilities::PipeRunCalculator))
        .with_calculator(Arc::new(calculators::utilities::DrainLineCalculator))

        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::beginner::traits::BeginnerCalculator;
    use async_trait::async_trait;

    struct MockCalculator {
        id: String,
        category: CalculatorCategory,
    }

    #[async_trait]
    impl BeginnerCalculator for MockCalculator {
        fn id(&self) -> &str {
            &self.id
        }

        fn name(&self) -> &str {
            "Mock Calculator"
        }

        fn category(&self) -> CalculatorCategory {
            self.category
        }

        fn metadata(&self) -> BeginnerCalculatorMetadata {
            BeginnerCalculatorMetadata {
                id: self.id.clone(),
                name: "Mock Calculator".to_string(),
                category: self.category.as_str().to_string(),
                description: "Test calculator".to_string(),
                parameters: vec![],
                required_parameters: vec![],
                optional_parameters: vec![],
            }
        }

        fn validate(&self, _params: &BeginnerParameters) -> BeginnerResult<()> {
            Ok(())
        }

        async fn calculate(
            &self,
            _params: BeginnerParameters,
        ) -> BeginnerResult<BeginnerCalculationResponse> {
            Ok(BeginnerCalculationResponse {
                calculation_type: self.id.clone(),
                results: vec![],
                warnings: vec![],
            })
        }
    }

    #[test]
    fn test_registry_registration() {
        let mut registry = BeginnerRegistry::new();
        
        let calc = Arc::new(MockCalculator {
            id: "test_calc".to_string(),
            category: CalculatorCategory::Garden,
        });
        
        registry.register(calc);
        
        assert!(registry.find("test_calc").is_ok());
        assert!(registry.find("nonexistent").is_err());
    }

    #[test]
    fn test_registry_by_category() {
        let mut registry = BeginnerRegistry::new();
        
        registry.register(Arc::new(MockCalculator {
            id: "garden1".to_string(),
            category: CalculatorCategory::Garden,
        }));
        
        registry.register(Arc::new(MockCalculator {
            id: "garden2".to_string(),
            category: CalculatorCategory::Garden,
        }));
        
        registry.register(Arc::new(MockCalculator {
            id: "interiors1".to_string(),
            category: CalculatorCategory::Interiors,
        }));
        
        let garden_calcs = registry.by_category(CalculatorCategory::Garden);
        assert_eq!(garden_calcs.len(), 2);
        
        let interiors_calcs = registry.by_category(CalculatorCategory::Interiors);
        assert_eq!(interiors_calcs.len(), 1);
    }

    #[test]
    fn test_registry_search() {
        let mut registry = BeginnerRegistry::new();
        
        registry.register(Arc::new(MockCalculator {
            id: "deck".to_string(),
            category: CalculatorCategory::Outdoors,
        }));
        
        let results = registry.search("deck");
        assert_eq!(results.len(), 1);
        
        let results = registry.search("xyz");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_registry_builder() {
        let registry = RegistryBuilder::new()
            .with_calculator(Arc::new(MockCalculator {
                id: "test1".to_string(),
                category: CalculatorCategory::Garden,
            }))
            .with_calculator(Arc::new(MockCalculator {
                id: "test2".to_string(),
                category: CalculatorCategory::Utilities,
            }))
            .build();
        
        assert_eq!(registry.all().len(), 2);
    }
}