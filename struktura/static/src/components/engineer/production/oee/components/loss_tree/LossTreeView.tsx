/**
 * LossTreeView Component
 *
 * Tree container for hierarchical loss partitions.
 * Shows duration & impact per node, percent-of-total context.
 *
 * CRITICAL: Use "allocated to", never "caused by" language.
 */

import React, { useState } from "react";
import LossNode from "./LossNode";
import type { LossTree } from "../../models";
import { formatDuration } from "../../utils";

export interface LossTreeViewProps {
  lossTree: LossTree;
  expandAll?: boolean;
}

const LossTreeView: React.FC<LossTreeViewProps> = ({
  lossTree,
  expandAll = false,
}) => {
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(
    expandAll ? new Set(["root"]) : new Set()
  );

  const toggleNode = (nodeId: string) => {
    setExpandedNodes((prev) => {
      const next = new Set(prev);
      if (next.has(nodeId)) {
        next.delete(nodeId);
      } else {
        next.add(nodeId);
      }
      return next;
    });
  };

  const expandAllNodes = () => {
    const allNodeIds = new Set<string>();
    const traverse = (node: any, path: string = "") => {
      const nodeId = path + node.category_key;
      allNodeIds.add(nodeId);
      node.children?.forEach((child: any, idx: number) =>
        traverse(child, `${nodeId}-${idx}-`)
      );
    };
    traverse(lossTree.root);
    setExpandedNodes(allNodeIds);
  };

  const collapseAllNodes = () => {
    setExpandedNodes(new Set());
  };

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h3 className="section-subheading mb-1">Loss Attribution Tree</h3>
          <p className="text-sm text-charcoal-600 dark:text-charcoal-400 italic">
            These categories represent where impact accumulates, not why it
            exists.
          </p>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={expandAllNodes}
            className="text-xs px-3 py-1.5 border border-steel-400 dark:border-steel-600 
                     text-steel-600 dark:text-steel-400 rounded hover:bg-steel-50 
                     dark:hover:bg-steel-900/20 transition-colors"
          >
            Expand All
          </button>
          <button
            onClick={collapseAllNodes}
            className="text-xs px-3 py-1.5 border border-steel-400 dark:border-steel-600 
                     text-steel-600 dark:text-steel-400 rounded hover:bg-steel-50 
                     dark:hover:bg-steel-900/20 transition-colors"
          >
            Collapse All
          </button>
        </div>
      </div>

      {/* Summary Stats */}
      <div className="bg-sand-50 dark:bg-charcoal-800 rounded-lg p-4">
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide">
              Total Planned Time
            </div>
            <div className="text-lg font-semibold text-charcoal-900 dark:text-charcoal-100 mt-1">
              {formatDuration(lossTree.planned_time, {
                compact: true,
                maxParts: 2,
              })}
            </div>
          </div>
          <div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide">
              Root Duration
            </div>
            <div className="text-lg font-semibold text-charcoal-900 dark:text-charcoal-100 mt-1">
              {formatDuration(lossTree.root.duration, {
                compact: true,
                maxParts: 2,
              })}
            </div>
          </div>
          <div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide">
              Loss Percentage
            </div>
            <div className="text-lg font-semibold text-charcoal-900 dark:text-charcoal-100 mt-1">
              {(lossTree.root.percentage_of_planned * 100).toFixed(1)}%
            </div>
          </div>
          <div>
            <div className="text-xs text-charcoal-500 dark:text-charcoal-400 uppercase tracking-wide">
              Child Categories
            </div>
            <div className="text-lg font-semibold text-charcoal-900 dark:text-charcoal-100 mt-1">
              {lossTree.root.children?.length || 0}
            </div>
          </div>
        </div>
      </div>

      {/* Tree */}
      <div className="loss-tree">
        <LossNode
          node={lossTree.root}
          isExpanded={expandedNodes.has("root")}
          onToggle={() => toggleNode("root")}
          path="root"
          expandedNodes={expandedNodes}
          onNodeToggle={toggleNode}
        />
      </div>
    </div>
  );
};

export default LossTreeView;
