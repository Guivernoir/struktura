use crate::calculus::engineer::{
    errors::{EngineeringError, EngineeringResult},
    models::*,
    traits::{CalculatorRegistry, EngineerCalculator},
};
use std::collections::HashMap;
use std::sync::Arc;

/// Thread-safe calculator registry with tactical precision
#[derive(Clone)]
pub struct EngineeringRegistry {
    pub calculators: Arc<HashMap<String, Arc<dyn EngineerCalculator>>>,
    pub category_index: Arc<HashMap<CalculatorCategory, Vec<String>>>,
    pub tags_index: Arc<HashMap<String, Vec<String>>>,
}

impl EngineeringRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            calculators: Arc::new(HashMap::new()),
            category_index: Arc::new(HashMap::new()),
            tags_index: Arc::new(HashMap::new()),
        }
    }

    /// Register a calculator in the arsenal
    pub fn register(&mut self, calculator: Arc<dyn EngineerCalculator>) {
        let id = calculator.id().to_string();
        let category = calculator.category();

        // Clone the Arc pointers for mutation
        let mut calculators = (*self.calculators).clone();
        let mut category_index = (*self.category_index).clone();

        // Register calculator
        calculators.insert(id.clone(), calculator);

        // Update category index
        category_index
            .entry(category)
            .or_insert_with(Vec::new)
            .push(id.clone());

        // Update Arc pointers
        self.calculators = Arc::new(calculators);
        self.category_index = Arc::new(category_index);
    }

    /// Find calculator by ID with surgical precision
    pub fn find(&self, id: &str) -> EngineeringResult<Arc<dyn EngineerCalculator>> {
        self.calculators
            .get(id)
            .cloned()
            .ok_or_else(|| EngineeringError::CalculatorNotFound(id.to_string()))
    }

    /// Get all calculators (for catalogue generation)
    pub fn all(&self) -> Vec<Arc<dyn EngineerCalculator>> {
        self.calculators.values().cloned().collect()
    }

    /// Get calculators by category
    pub fn by_category(&self, category: CalculatorCategory) -> Vec<Arc<dyn EngineerCalculator>> {
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
    pub fn search(&self, query: &str) -> Vec<Arc<dyn EngineerCalculator>> {
        let query_lower = query.to_lowercase();
        
        self.calculators
            .values()
            .filter(|calc| {
                let metadata = calc.metadata();
                metadata.name.to_lowercase().contains(&query_lower)
                    || metadata.description.to_lowercase().contains(&query_lower)
                    || metadata.id.to_lowercase().contains(&query_lower)
                    || metadata.typical_applications.iter()
                        .any(|app| app.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect()
    }

    /// Generate complete API catalogue
    pub fn catalogue(&self) -> EngineeringCalculatorCatalogue {
        let categories = vec![
            EngineeringCategoryInfo {
                id: "civil".to_string(),
                name: "Civil Engineering".to_string(),
                description: "Geotechnical, pavement, and infrastructure design".to_string(),
                requires_pe: true,
                icon: Some("🏗️".to_string()),
            },
            EngineeringCategoryInfo {
                id: "structural".to_string(),
                name: "Structural Engineering".to_string(),
                description: "Load analysis, beam design, column design, code compliance".to_string(),
                requires_pe: true,
                icon: Some("🏛️".to_string()),
            },
            EngineeringCategoryInfo {
                id: "mechanical".to_string(),
                name: "Mechanical Engineering".to_string(),
                description: "Thermodynamics, fluid mechanics, HVAC, pump systems".to_string(),
                requires_pe: false,
                icon: Some("⚙️".to_string()),
            },
            EngineeringCategoryInfo {
                id: "production".to_string(),
                name: "Production Engineering".to_string(),
                description: "Manufacturing systems, line balancing, material handling".to_string(),
                requires_pe: false,
                icon: Some("🏭".to_string()),
            },
        ];

        let calculators: Vec<EngineeringCalculatorMetadata> = self
            .all()
            .iter()
            .map(|calc| calc.metadata())
            .collect();

        // Build search index
        let mut tags: HashMap<String, Vec<String>> = HashMap::new();
        let mut keywords: HashMap<String, Vec<String>> = HashMap::new();

        for calc in &calculators {
            // Index by category
            tags.entry(calc.category.clone())
                .or_insert_with(Vec::new)
                .push(calc.id.clone());

            // Index by design codes
            for code in &calc.design_codes {
                tags.entry(code.clone())
                    .or_insert_with(Vec::new)
                    .push(calc.id.clone());
            }

            // Index by keywords from name and description
            let words: Vec<String> = calc.name
                .split_whitespace()
                .chain(calc.description.split_whitespace())
                .map(|w| w.to_lowercase())
                .filter(|w| w.len() > 3) // Skip short words
                .collect();

            for word in words {
                keywords
                    .entry(word)
                    .or_insert_with(Vec::new)
                    .push(calc.id.clone());
            }
        }

        EngineeringCalculatorCatalogue {
            version: env!("CARGO_PKG_VERSION").to_string(),
            categories,
            calculators,
            disclaimer: "Engineering calculations require professional review. \
                        Results are preliminary and must be verified by a licensed \
                        Professional Engineer before construction or implementation."
                .to_string(),
            search_index: Some(SearchIndex { tags, keywords }),
        }
    }

    /// Get statistics about the registry
    pub fn stats(&self) -> RegistryStats {
        let mut by_category: HashMap<String, usize> = HashMap::new();
        let mut requires_pe_count = 0;
        let mut total_parameters = 0;

        for calc in self.all() {
            let metadata = calc.metadata();
            
            *by_category.entry(metadata.category.clone()).or_insert(0) += 1;
            
            if metadata.requires_pe_review {
                requires_pe_count += 1;
            }
            
            total_parameters += metadata.parameters.len();
        }

        RegistryStats {
            total_calculators: self.calculators.len(),
            by_category,
            requires_pe_count,
            total_parameters,
        }
    }
}

impl Default for EngineeringRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Registry statistics for monitoring
#[derive(Debug, Clone)]
pub struct RegistryStats {
    pub total_calculators: usize,
    pub by_category: HashMap<String, usize>,
    pub requires_pe_count: usize,
    pub total_parameters: usize,
}

/// Builder for constructing registry with all calculators
pub struct RegistryBuilder {
    registry: EngineeringRegistry,
}

impl RegistryBuilder {
    pub fn new() -> Self {
        Self {
            registry: EngineeringRegistry::new(),
        }
    }

    /// Register a calculator
    pub fn with_calculator(mut self, calculator: Arc<dyn EngineerCalculator>) -> Self {
        self.registry.register(calculator);
        self
    }

    /// Register multiple calculators
    pub fn with_calculators(mut self, calculators: Vec<Arc<dyn EngineerCalculator>>) -> Self {
        for calc in calculators {
            self.registry.register(calc);
        }
        self
    }

    /// Build the final registry
    pub fn build(self) -> EngineeringRegistry {
        self.registry
    }
}

impl Default for RegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Create registry with all available calculators
/// This is the main entry point for initializing the registry
pub fn create_default_registry() -> EngineeringRegistry {
    use crate::calculus::engineer::calculators;

    RegistryBuilder::new()
        // ========================================================================
        // CIVIL ENGINEERING (6 calculators) - All require PE review
        // ========================================================================
        .with_calculator(Arc::new(calculators::civil::RetainingWallCalculator))
        .with_calculator(Arc::new(calculators::civil::PavementDesignCalculator))
        .with_calculator(Arc::new(calculators::civil::FoundationDesignCalculator))
        .with_calculator(Arc::new(calculators::civil::SlopeStabilityCalculator))
        .with_calculator(Arc::new(calculators::civil::SettlementAnalysisCalculator))
        .with_calculator(Arc::new(calculators::civil::SoilBearingCapacityCalculator))
        
        // ========================================================================
        // STRUCTURAL ENGINEERING (7 calculators) - All require PE review
        // ========================================================================
        .with_calculator(Arc::new(calculators::structural::BeamDesignCalculator))
        .with_calculator(Arc::new(calculators::structural::ColumnDesignCalculator))
        .with_calculator(Arc::new(calculators::structural::TrussAnalysisCalculator))
        .with_calculator(Arc::new(calculators::structural::MomentFrameDesignCalculator))
        .with_calculator(Arc::new(calculators::structural::ConnectionDesignCalculator))
        .with_calculator(Arc::new(calculators::structural::SlabDesignCalculator))
        .with_calculator(Arc::new(calculators::structural::LateralLoadAnalysisCalculator))
        
        // ========================================================================
        // MECHANICAL ENGINEERING (8 calculators) - No PE review required
        // ========================================================================
        .with_calculator(Arc::new(calculators::mechanical::HeatExchangerCalculator))
        .with_calculator(Arc::new(calculators::mechanical::PumpSizingCalculator))
        .with_calculator(Arc::new(calculators::mechanical::PipingPressureDropCalculator))
        .with_calculator(Arc::new(calculators::mechanical::HVACLoadCalculationCalculator))
        .with_calculator(Arc::new(calculators::mechanical::RefrigerationCycleCalculator))
        .with_calculator(Arc::new(calculators::mechanical::CompressorSizingCalculator))
        .with_calculator(Arc::new(calculators::mechanical::ValveSizingCalculator))
        .with_calculator(Arc::new(calculators::mechanical::ThermalExpansionCalculator))
        
        // ========================================================================
        // PRODUCTION ENGINEERING (8 calculators) - No PE review required
        // ========================================================================
        .with_calculator(Arc::new(calculators::production::ConveyorBeltCalculator))
        .with_calculator(Arc::new(calculators::production::ProductionLineBalancingCalculator))
        .with_calculator(Arc::new(calculators::production::OEECalculator))
        .with_calculator(Arc::new(calculators::production::InventoryOptimizationCalculator))
        .with_calculator(Arc::new(calculators::production::CapacityPlanningCalculator))
        .with_calculator(Arc::new(calculators::production::ProcessCapabilityCalculator))
        .with_calculator(Arc::new(calculators::production::WorkSamplingCalculator))
        .with_calculator(Arc::new(calculators::production::FacilityLayoutCalculator))
        
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::engineer::traits::EngineerCalculator;
    use async_trait::async_trait;

    // Mock calculator for testing
    struct MockCalculator {
        id: String,
        category: CalculatorCategory,
    }

    #[async_trait]
    impl EngineerCalculator for MockCalculator {
        fn id(&self) -> &str {
            &self.id
        }

        fn name(&self) -> &str {
            "Mock Calculator"
        }

        fn category(&self) -> CalculatorCategory {
            self.category
        }

        fn metadata(&self) -> EngineeringCalculatorMetadata {
            EngineeringCalculatorMetadata {
                id: self.id.clone(),
                name: "Mock Calculator".to_string(),
                category: self.category.as_str().to_string(),
                description: "Test calculator".to_string(),
                design_codes: vec![],
                parameters: vec![],
                required_parameters: vec![],
                optional_parameters: vec![],
                typical_applications: vec![],
                requires_pe_review: false,
                complexity_level: None,
                calculation_time: None,
            }
        }

        fn validate(&self, _params: &EngineeringParameters) -> EngineeringResult<()> {
            Ok(())
        }

        async fn calculate(
            &self,
            _params: EngineeringParameters,
        ) -> EngineeringResult<EngineeringCalculationResponse> {
            Ok(EngineeringCalculationResponse {
                calculation_type: self.id.clone(),
                results: vec![],
                analysis: None,
                warnings: vec![],
                structured_warnings: None,
                recommendations: vec![],
                compliance_notes: vec![],
                calculation_metadata: None,
            })
        }
    }

    #[test]
    fn test_registry_registration() {
        let mut registry = EngineeringRegistry::new();
        
        let calc = Arc::new(MockCalculator {
            id: "test_calc".to_string(),
            category: CalculatorCategory::Civil,
        });
        
        registry.register(calc);
        
        assert!(registry.find("test_calc").is_ok());
        assert!(registry.find("nonexistent").is_err());
    }

    #[test]
    fn test_registry_by_category() {
        let mut registry = EngineeringRegistry::new();
        
        registry.register(Arc::new(MockCalculator {
            id: "civil1".to_string(),
            category: CalculatorCategory::Civil,
        }));
        
        registry.register(Arc::new(MockCalculator {
            id: "civil2".to_string(),
            category: CalculatorCategory::Civil,
        }));
        
        registry.register(Arc::new(MockCalculator {
            id: "structural1".to_string(),
            category: CalculatorCategory::Structural,
        }));
        
        let civil_calcs = registry.by_category(CalculatorCategory::Civil);
        assert_eq!(civil_calcs.len(), 2);
        
        let structural_calcs = registry.by_category(CalculatorCategory::Structural);
        assert_eq!(structural_calcs.len(), 1);
    }

    #[test]
    fn test_registry_search() {
        let mut registry = EngineeringRegistry::new();
        
        registry.register(Arc::new(MockCalculator {
            id: "beam_design".to_string(),
            category: CalculatorCategory::Structural,
        }));
        
        let results = registry.search("beam");
        assert_eq!(results.len(), 1);
        
        let results = registry.search("xyz");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_registry_builder() {
        let registry = RegistryBuilder::new()
            .with_calculator(Arc::new(MockCalculator {
                id: "test1".to_string(),
                category: CalculatorCategory::Civil,
            }))
            .with_calculator(Arc::new(MockCalculator {
                id: "test2".to_string(),
                category: CalculatorCategory::Mechanical,
            }))
            .build();
        
        assert_eq!(registry.all().len(), 2);
    }
}