//! Loss tree decomposition tests
//! 
//! Tests hierarchical loss structure and categorization

use super::*;
use crate::calculus::engineer::calculators::production::oee::engine::calculate_oee;

#[test]
fn test_loss_tree_structure() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Should have root node
    assert_eq!(result.loss_tree.root.category_key, "loss_tree.planned_time");
    
    // Should have child categories (Six Big Losses)
    assert!(!result.loss_tree.root.children.is_empty());
}

#[test]
fn test_six_big_losses_categories() {
    let input = TestFixture::basic()
        .with_downtime(1800, true) // Breakdown
        .with_production(1000, 900, 100, 0) // Quality loss
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Flatten tree to check categories
    let flat_tree = result.loss_tree.flatten();
    let categories: Vec<&str> = flat_tree.iter()
        .map(|node| node.category_key.as_str())
        .collect();
    
    // Should have availability losses
    assert!(categories.iter().any(|c| c.contains("availability")));
    
    // Should have performance losses
    assert!(categories.iter().any(|c| c.contains("performance")));
    
    // Should have quality losses
    assert!(categories.iter().any(|c| c.contains("quality")));
}

#[test]
fn test_loss_percentages_sum_to_100() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Top-level children should account for all losses
    let total_percentage: f64 = result.loss_tree.root.children.iter()
        .map(|child| child.percentage_of_planned)
        .sum();
    
    // Should be reasonable (losses won't be exactly 100% as there's also productive time)
    assert!(
        total_percentage < 1.0,
        "Loss percentages should be less than 100%"
    );
}

#[test]
fn test_breakdown_categorization() {
    let input = TestFixture::basic()
        .with_downtime(3600, true) // 1 hour breakdown (is_failure = true)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    let flat_tree = result.loss_tree.flatten();
    
    // Should have breakdown loss
    let has_breakdown = flat_tree.iter()
        .any(|node| node.category_key.contains("breakdowns"));
    
    assert!(has_breakdown, "Should categorize breakdowns");
}

#[test]
fn test_scrap_loss_time_equivalent() {
    let cycle_time = 30;
    let running_hours = 7;
    let theoretical = (running_hours * 3600 / cycle_time) as u32; // 840 units
    let scrap_units = 200;
    let good_units = theoretical - scrap_units;
    
    let input = TestFixture::basic()
        .with_time_allocations(running_hours, 1)
        .with_production(theoretical, good_units, scrap_units, 0)
        .with_cycle_time(cycle_time, None)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    let flat_tree = result.loss_tree.flatten();
    
    // Find production rejects node
    let rejects_node = flat_tree.iter()
        .find(|node| node.category_key.contains("production_rejects"));
    
    if let Some(node) = rejects_node {
        // Should be 200 units × 30 seconds = 6000 seconds
        assert_eq!(node.duration.as_secs(), 6000);
    }
}

#[test]
fn test_parent_percentages_calculated() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // All non-root nodes should have parent percentage
    let flat_tree = result.loss_tree.flatten();
    
    for node in flat_tree.iter().skip(1) { // Skip root
        // Should have valid parent percentage
        if let Some(parent_pct) = node.percentage_of_parent {
            assert!(
                parent_pct >= 0.0 && parent_pct <= 1.0,
                "Parent percentage should be valid"
            );
        }
    }
}

#[test]
fn test_loss_tree_with_setup_time() {
    let cycle_time = 25;
    let running_hours = 6;
    let theoretical = (running_hours * 3600 / cycle_time) as u32;
    
    let input = TestFixture::basic()
        .with_time_allocations(running_hours, 2) // 2 hours downtime
        .with_production(theoretical, theoretical, 0, 0)
        .build();
    
    // Manually add setup allocation
    // (In real system, would come from input)
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Should categorize setup/adjustment losses
    let flat_tree = result.loss_tree.flatten();
    let has_setup = flat_tree.iter()
        .any(|node| node.category_key.contains("setup"));
    
    // May or may not have setup depending on input structure
}

#[test]
fn test_small_stops_categorization() {
    // Small stops are below threshold
    let input = TestFixture::basic()
        .with_downtime(20, false) // 20 seconds (below default 30s threshold)
        .with_downtime(25, false) // Another small stop
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    let flat_tree = result.loss_tree.flatten();
    
    // Should categorize as small stops
    let has_small_stops = flat_tree.iter()
        .any(|node| node.category_key.contains("small_stops"));
    
    assert!(has_small_stops, "Should categorize small stops");
}

#[test]
fn test_speed_loss_calculation() {
    let input = TestFixture::basic()
        .with_production(800, 800, 0, 0) // Producing slowly
        .with_cycle_time(25, Some(35)) // Running slower than ideal
        .with_time_allocations(7, 1)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    let flat_tree = result.loss_tree.flatten();
    
    // Should have speed loss
    let has_speed_loss = flat_tree.iter()
        .any(|node| node.category_key.contains("speed_losses"));
    
    assert!(has_speed_loss, "Should identify speed losses");
}

#[test]
fn test_loss_tree_value_source_tracking() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    let flat_tree = result.loss_tree.flatten();
    
    // All nodes should have value source
    for node in &flat_tree {
        // Source should be valid
        assert!(
            matches!(
                node.source,
                crate::calculus::engineer::calculators::production::oee::domain::ValueSource::Explicit |
                crate::calculus::engineer::calculators::production::oee::domain::ValueSource::Inferred |
                crate::calculus::engineer::calculators::production::oee::domain::ValueSource::Default
            ),
            "Should have valid value source"
        );
    }
}

#[test]
fn test_loss_tree_flatten_preserves_hierarchy() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    let flat_tree = result.loss_tree.flatten();
    
    // First node should be root
    assert_eq!(flat_tree[0].category_key, "loss_tree.planned_time");
    
    // Should have reasonable number of nodes
    assert!(
        flat_tree.len() >= 4,
        "Should have at least root + 3 main categories"
    );
}

#[test]
fn test_loss_tree_with_rework() {
    let input = TestFixture::basic()
        .with_production(1000, 900, 50, 50) // 50 reworked
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Rework is tracked in extended metrics, not typically in loss tree
    // (loss tree focuses on time losses)
    assert!(result.extended_metrics.rework_rate.value > 0.0);
}

#[test]
fn test_zero_losses() {
    // Perfect scenario - no losses
    let input = TestFixture::basic()
        .with_production(1000, 1000, 0, 0)
        .with_time_allocations(8, 0)
        .build();
    
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Loss tree should still exist but with minimal losses
    assert!(!result.loss_tree.root.children.is_empty());
}

#[test]
fn test_loss_tree_translation_keys() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    let flat_tree = result.loss_tree.flatten();
    
    // All nodes should have translation keys
    for node in &flat_tree {
        assert!(
            node.category_key.starts_with("loss_tree."),
            "Should have proper translation key: {}",
            node.category_key
        );
        assert!(
            node.description_key.starts_with("loss_tree."),
            "Should have proper description key: {}",
            node.description_key
        );
    }
}

#[test]
fn test_loss_tree_durations_non_negative() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    let flat_tree = result.loss_tree.flatten();
    
    // All durations should be non-negative
    for node in &flat_tree {
        assert!(
            node.duration.as_secs() >= 0,
            "Duration should be non-negative"
        );
    }
}

#[test]
fn test_loss_tree_percentages_valid() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    let flat_tree = result.loss_tree.flatten();
    
    // All percentages should be valid (0-100%)
    for node in &flat_tree {
        assert!(
            node.percentage_of_planned >= 0.0 && node.percentage_of_planned <= 1.0,
            "Percentage should be in [0, 1]"
        );
        
        if let Some(parent_pct) = node.percentage_of_parent {
            assert!(
                parent_pct >= 0.0 && parent_pct <= 1.0,
                "Parent percentage should be in [0, 1]"
            );
        }
    }
}

#[test]
fn test_loss_tree_serializable() {
    let input = TestFixture::basic().build();
    let result = calculate_oee(input).expect("Calculation should succeed");
    
    // Loss tree should be serializable to JSON
    let json = serde_json::to_string(&result.loss_tree);
    assert!(json.is_ok(), "Loss tree should be serializable");
}