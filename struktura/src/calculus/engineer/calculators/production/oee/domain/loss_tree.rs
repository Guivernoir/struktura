//! Loss tree decomposition
//! 
//! Mathematical partitioning of losses into categories.
//! This is attribution, NOT causality.
//! 
//! Think of it as a budget breakdown, not a forensic investigation.

use super::*;

/// A node in the loss tree (hierarchical structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LossTreeNode {
    /// Category name (translation key)
    pub category_key: String,
    /// Human-readable description (translation key)
    pub description_key: String,
    /// Time attributed to this category
    pub duration: Duration,
    /// Percentage of total planned time
    pub percentage_of_planned: f64,
    /// Percentage of parent category (if any)
    pub percentage_of_parent: Option<f64>,
    /// Child categories
    pub children: Vec<LossTreeNode>,
    /// How this was determined
    pub source: ValueSource,
}

impl LossTreeNode {
    pub fn new(
        category_key: &str,
        description_key: &str,
        duration: Duration,
        planned_time: Duration,
        source: ValueSource,
    ) -> Self {
        let percentage_of_planned = if planned_time.as_secs() > 0 {
            duration.as_secs_f64() / planned_time.as_secs_f64()
        } else {
            0.0
        };
        
        Self {
            category_key: category_key.to_string(),
            description_key: description_key.to_string(),
            duration,
            percentage_of_planned,
            percentage_of_parent: None,
            children: Vec::new(),
            source,
        }
    }
    
    pub fn add_child(&mut self, child: LossTreeNode) {
        self.children.push(child);
    }
    
    /// Recalculate parent percentages after tree construction
    pub fn calculate_parent_percentages(&mut self) {
        let parent_duration = self.duration;
        
        for child in &mut self.children {
            child.percentage_of_parent = Some(
                if parent_duration.as_secs() > 0 {
                    child.duration.as_secs_f64() / parent_duration.as_secs_f64()
                } else {
                    0.0
                }
            );
            
            // Recurse
            child.calculate_parent_percentages();
        }
    }
}

/// Complete loss tree structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LossTree {
    /// Root node (typically "Planned Time")
    pub root: LossTreeNode,
    /// Total planned time (for reference)
    pub planned_time: Duration,
}

impl LossTree {
    pub fn new(planned_time: Duration) -> Self {
        let root = LossTreeNode::new(
            "loss_tree.planned_time",
            "loss_tree.planned_time_desc",
            planned_time,
            planned_time,
            ValueSource::Explicit,
        );
        
        Self {
            root,
            planned_time,
        }
    }
    
    /// Build standard six big losses structure
    pub fn build_six_big_losses(
        planned_time: Duration,
        breakdowns: Duration,
        setup_adjustments: Duration,
        small_stops: Duration,
        speed_losses: Duration,
        startup_rejects: Duration,
        production_rejects: Duration,
    ) -> Self {
        let mut tree = Self::new(planned_time);
        
        // Availability losses
        let availability_loss = breakdowns + setup_adjustments;
        let mut availability_node = LossTreeNode::new(
            "loss_tree.availability_losses",
            "loss_tree.availability_losses_desc",
            availability_loss,
            planned_time,
            ValueSource::Inferred,
        );
        
        availability_node.add_child(LossTreeNode::new(
            "loss_tree.breakdowns",
            "loss_tree.breakdowns_desc",
            breakdowns,
            planned_time,
            ValueSource::Explicit,
        ));
        
        availability_node.add_child(LossTreeNode::new(
            "loss_tree.setup_adjustments",
            "loss_tree.setup_adjustments_desc",
            setup_adjustments,
            planned_time,
            ValueSource::Explicit,
        ));
        
        // Performance losses
        let performance_loss = small_stops + speed_losses;
        let mut performance_node = LossTreeNode::new(
            "loss_tree.performance_losses",
            "loss_tree.performance_losses_desc",
            performance_loss,
            planned_time,
            ValueSource::Inferred,
        );
        
        performance_node.add_child(LossTreeNode::new(
            "loss_tree.small_stops",
            "loss_tree.small_stops_desc",
            small_stops,
            planned_time,
            ValueSource::Explicit,
        ));
        
        performance_node.add_child(LossTreeNode::new(
            "loss_tree.speed_losses",
            "loss_tree.speed_losses_desc",
            speed_losses,
            planned_time,
            ValueSource::Explicit,
        ));
        
        // Quality losses
        let quality_loss = startup_rejects + production_rejects;
        let mut quality_node = LossTreeNode::new(
            "loss_tree.quality_losses",
            "loss_tree.quality_losses_desc",
            quality_loss,
            planned_time,
            ValueSource::Inferred,
        );
        
        quality_node.add_child(LossTreeNode::new(
            "loss_tree.startup_rejects",
            "loss_tree.startup_rejects_desc",
            startup_rejects,
            planned_time,
            ValueSource::Explicit,
        ));
        
        quality_node.add_child(LossTreeNode::new(
            "loss_tree.production_rejects",
            "loss_tree.production_rejects_desc",
            production_rejects,
            planned_time,
            ValueSource::Explicit,
        ));
        
        // Add to root
        availability_node.calculate_parent_percentages();
        performance_node.calculate_parent_percentages();
        quality_node.calculate_parent_percentages();
        
        tree.root.add_child(availability_node);
        tree.root.add_child(performance_node);
        tree.root.add_child(quality_node);
        
        tree.root.calculate_parent_percentages();
        tree
    }
    
    /// Flatten tree for reporting (depth-first traversal)
    pub fn flatten(&self) -> Vec<LossTreeNode> {
        let mut result = Vec::new();
        self.flatten_recursive(&self.root, &mut result);
        result
    }
    
    fn flatten_recursive(&self, node: &LossTreeNode, result: &mut Vec<LossTreeNode>) {
        result.push(node.clone());
        for child in &node.children {
            self.flatten_recursive(child, result);
        }
    }
}
