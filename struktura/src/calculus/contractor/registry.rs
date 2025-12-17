use crate::calculus::contractor::{
    errors::{ContractingError, ContractingResult},
    models::*,
    traits::{CalculatorRegistry, ContractorCalculator},
};
use std::collections::HashMap;
use std::sync::Arc;

/// Thread-safe calculator registry with tactical precision
#[derive(Clone)]
pub struct ContractingRegistry {
    pub calculators: Arc<HashMap<String, Arc<dyn ContractorCalculator>>>,
    pub category_index: Arc<HashMap<CalculatorCategory, Vec<String>>>,
    pub tags_index: Arc<HashMap<String, Vec<String>>>,
}

impl ContractingRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            calculators: Arc::new(HashMap::new()),
            category_index: Arc::new(HashMap::new()),
            tags_index: Arc::new(HashMap::new()),
        }
    }

    /// Register a calculator in the arsenal
    pub fn register(&mut self, calculator: Arc<dyn ContractorCalculator>) {
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
    pub fn find(&self, id: &str) -> ContractingResult<Arc<dyn ContractorCalculator>> {
        self.calculators
            .get(id)
            .cloned()
            .ok_or_else(|| ContractingError::CalculatorNotFound(id.to_string()))
    }

    /// Get all calculators (for catalogue generation)
    pub fn all(&self) -> Vec<Arc<dyn ContractorCalculator>> {
        self.calculators.values().cloned().collect()
    }

    /// Get calculators by category
    pub fn by_category(&self, category: CalculatorCategory) -> Vec<Arc<dyn ContractorCalculator>> {
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
    pub fn search(&self, query: &str) -> Vec<Arc<dyn ContractorCalculator>> {
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
    pub fn catalogue(&self) -> ContractingCalculatorCatalogue {
        let categories = vec![
            ContractingCategoryInfo {
                id: "bidding".to_string(),
                name: "Bidding".to_string(),
                description: "Bid pricing, risk assessment, and contract estimation".to_string(),
                requires_certification: true,
                icon: Some("💰".to_string()),
            },
            ContractingCategoryInfo {
                id: "scheduling".to_string(),
                name: "Scheduling".to_string(),
                description: "Project timelines, critical path, resource leveling".to_string(),
                requires_certification: true,
                icon: Some("📅".to_string()),
            },
            ContractingCategoryInfo {
                id: "estimation".to_string(),
                name: "Estimation".to_string(),
                description: "Cost estimation, quantity takeoff, budgeting".to_string(),
                requires_certification: false,
                icon: Some("📊".to_string()),
            },
            ContractingCategoryInfo {
                id: "management".to_string(),
                name: "Management".to_string(),
                description: "Resource allocation, quality control, safety planning".to_string(),
                requires_certification: false,
                icon: Some("🛠️".to_string()),
            },
        ];

        let calculators: Vec<ContractingCalculatorMetadata> = self
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

            // Index by regulation codes
            for code in &calc.regulation_codes {
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

        ContractingCalculatorCatalogue {
            version: env!("CARGO_PKG_VERSION").to_string(),
            categories,
            calculators,
            disclaimer: "Contracting calculations require professional review. \
                        Results are preliminary and must be verified by a certified \
                        contractor before implementation."
                .to_string(),
            search_index: Some(SearchIndex { tags, keywords }),
        }
    }

    /// Get statistics about the registry
    pub fn stats(&self) -> RegistryStats {
        let mut by_category: HashMap<String, usize> = HashMap::new();
        let mut requires_certification_count = 0;
        let mut total_parameters = 0;

        for calc in self.all() {
            let metadata = calc.metadata();
            
            *by_category.entry(metadata.category.clone()).or_insert(0) += 1;
            
            if metadata.requires_certification_review {
                requires_certification_count += 1;
            }
            
            total_parameters += metadata.parameters.len();
        }

        RegistryStats {
            total_calculators: self.calculators.len(),
            by_category,
            requires_certification_count,
            total_parameters,
        }
    }
}

impl Default for ContractingRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Registry statistics for monitoring
#[derive(Debug, Clone)]
pub struct RegistryStats {
    pub total_calculators: usize,
    pub by_category: HashMap<String, usize>,
    pub requires_certification_count: usize,
    pub total_parameters: usize,
}

/// Builder for constructing registry with all calculators
pub struct RegistryBuilder {
    registry: ContractingRegistry,
}

impl RegistryBuilder {
    pub fn new() -> Self {
        Self {
            registry: ContractingRegistry::new(),
        }
    }

    /// Register a calculator
    pub fn with_calculator(mut self, calculator: Arc<dyn ContractorCalculator>) -> Self {
        self.registry.register(calculator);
        self
    }

    /// Register multiple calculators
    pub fn with_calculators(mut self, calculators: Vec<Arc<dyn ContractorCalculator>>) -> Self {
        for calc in calculators {
            self.registry.register(calc);
        }
        self
    }

    /// Build the final registry
    pub fn build(self) -> ContractingRegistry {
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
pub fn create_default_registry() -> ContractingRegistry {
    use crate::calculus::contractor::calculators;

    RegistryBuilder::new()
        // ========================================================================
        // BIDDING (6 calculators) - All require certification review
        // ========================================================================
        .with_calculator(Arc::new(calculators::bidding::BidPricingCalculator))
        .with_calculator(Arc::new(calculators::bidding::RiskAssessmentCalculator))
        .with_calculator(Arc::new(calculators::bidding::ContractEstimationCalculator))
        .with_calculator(Arc::new(calculators::bidding::ProfitMarginCalculator))
        .with_calculator(Arc::new(calculators::bidding::ContingencyPlanningCalculator))
        .with_calculator(Arc::new(calculators::bidding::BidBondCalculator))
        
        // ========================================================================
        // SCHEDULING (7 calculators) - All require certification review
        // ========================================================================
        .with_calculator(Arc::new(calculators::scheduling::CriticalPathCalculator))
        .with_calculator(Arc::new(calculators::scheduling::ResourceLevelingCalculator))
        .with_calculator(Arc::new(calculators::scheduling::GanttChartGenerator))
        .with_calculator(Arc::new(calculators::scheduling::DelayAnalysisCalculator))
        .with_calculator(Arc::new(calculators::scheduling::MilestoneTrackingCalculator))
        .with_calculator(Arc::new(calculators::scheduling::ScheduleOptimizationCalculator))
        .with_calculator(Arc::new(calculators::scheduling::TimeCostTradeoffCalculator))
        
        // ========================================================================
        // ESTIMATION (8 calculators) - No certification review required
        // ========================================================================
        .with_calculator(Arc::new(calculators::estimation::QuantityTakeoffCalculator))
        .with_calculator(Arc::new(calculators::estimation::CostBreakdownCalculator))
        .with_calculator(Arc::new(calculators::estimation::LaborCostEstimator))
        .with_calculator(Arc::new(calculators::estimation::MaterialCostEstimator))
        .with_calculator(Arc::new(calculators::estimation::EquipmentCostEstimator))
        .with_calculator(Arc::new(calculators::estimation::OverheadCalculator))
        .with_calculator(Arc::new(calculators::estimation::BudgetForecastCalculator))
        .with_calculator(Arc::new(calculators::estimation::ValueEngineeringCalculator))
        
        // ========================================================================
        // MANAGEMENT (8 calculators) - No certification review required
        // ========================================================================
        .with_calculator(Arc::new(calculators::management::ResourceAllocationCalculator))
        .with_calculator(Arc::new(calculators::management::QualityControlCalculator))
        .with_calculator(Arc::new(calculators::management::SafetyPlanningCalculator))
        .with_calculator(Arc::new(calculators::management::ChangeOrderCalculator))
        .with_calculator(Arc::new(calculators::management::ProgressTrackingCalculator))
        .with_calculator(Arc::new(calculators::management::CashFlowAnalysisCalculator))
        .with_calculator(Arc::new(calculators::management::SubcontractorEvaluationCalculator))
        .with_calculator(Arc::new(calculators::management::ProjectCloseoutCalculator))
        
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::contractor::traits::ContractorCalculator;
    use async_trait::async_trait;

    // Mock calculator for testing
    struct MockCalculator {
        id: String,
        category: CalculatorCategory,
    }

    #[async_trait]
    impl ContractorCalculator for MockCalculator {
        fn id(&self) -> &str {
            &self.id
        }

        fn name(&self) -> &str {
            "Mock Calculator"
        }

        fn category(&self) -> CalculatorCategory {
            self.category
        }

        fn metadata(&self) -> ContractingCalculatorMetadata {
            ContractingCalculatorMetadata {
                id: self.id.clone(),
                name: "Mock Calculator".to_string(),
                category: self.category.as_str().to_string(),
                description: "Test calculator".to_string(),
                regulation_codes: vec![],
                parameters: vec![],
                required_parameters: vec![],
                optional_parameters: vec![],
                typical_applications: vec![],
                requires_certification_review: false,
                complexity_level: None,
                calculation_time: None,
            }
        }

        fn validate(&self, _params: &ContractingParameters) -> ContractingResult<()> {
            Ok(())
        }

        async fn calculate(
            &self,
            _params: ContractingParameters,
        ) -> ContractingResult<ContractingCalculationResponse> {
            Ok(ContractingCalculationResponse {
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
        let mut registry = ContractingRegistry::new();
        
        let calc = Arc::new(MockCalculator {
            id: "test_calc".to_string(),
            category: CalculatorCategory::Bidding,
        });
        
        registry.register(calc);
        
        assert!(registry.find("test_calc").is_ok());
        assert!(registry.find("nonexistent").is_err());
    }

    #[test]
    fn test_registry_by_category() {
        let mut registry = ContractingRegistry::new();
        
        registry.register(Arc::new(MockCalculator {
            id: "bidding1".to_string(),
            category: CalculatorCategory::Bidding,
        }));
        
        registry.register(Arc::new(MockCalculator {
            id: "bidding2".to_string(),
            category: CalculatorCategory::Bidding,
        }));
        
        registry.register(Arc::new(MockCalculator {
            id: "scheduling1".to_string(),
            category: CalculatorCategory::Scheduling,
        }));
        
        let bidding_calcs = registry.by_category(CalculatorCategory::Bidding);
        assert_eq!(bidding_calcs.len(), 2);
        
        let scheduling_calcs = registry.by_category(CalculatorCategory::Scheduling);
        assert_eq!(scheduling_calcs.len(), 1);
    }

    #[test]
    fn test_registry_search() {
        let mut registry = ContractingRegistry::new();
        
        registry.register(Arc::new(MockCalculator {
            id: "bid_pricing".to_string(),
            category: CalculatorCategory::Bidding,
        }));
        
        let results = registry.search("bid");
        assert_eq!(results.len(), 1);
        
        let results = registry.search("xyz");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_registry_builder() {
        let registry = RegistryBuilder::new()
            .with_calculator(Arc::new(MockCalculator {
                id: "test1".to_string(),
                category: CalculatorCategory::Bidding,
            }))
            .with_calculator(Arc::new(MockCalculator {
                id: "test2".to_string(),
                category: CalculatorCategory::Estimation,
            }))
            .build();
        
        assert_eq!(registry.all().len(), 2);
    }
}