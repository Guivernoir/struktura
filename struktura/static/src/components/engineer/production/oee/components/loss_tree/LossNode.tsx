/**
 * LossNode Component
 *
 * Recursive node component for the loss tree.
 * Displays category, duration, and percentage context.
 * Uses "allocated to" language, never "caused by".
 */

import React from "react";
import LossImpactBadge from "./LossImpactBadge";
import type { LossTreeNode as LossTreeNodeType } from "../../models";
import { formatDuration } from "../../utils";

export interface LossNodeProps {
  node: LossTreeNodeType;
  isExpanded: boolean;
  onToggle: () => void;
  path: string;
  expandedNodes: Set<string>;
  onNodeToggle: (nodeId: string) => void;
  level?: number;
}

const LossNode: React.FC<LossNodeProps> = ({
  node,
  isExpanded,
  onToggle,
  path,
  expandedNodes,
  onNodeToggle,
  level = 0,
}) => {
  const hasChildren = node.children && node.children.length > 0;

  // Determine category type for styling
  const getCategoryType = () => {
    const key = node.category_key.toLowerCase();
    if (key.includes("availability")) return "availability";
    if (key.includes("performance")) return "performance";
    if (key.includes("quality")) return "quality";
    return "default";
  };

  const categoryType = getCategoryType();

  const categoryColors = {
    availability: "loss-tree-node--availability",
    performance: "loss-tree-node--performance",
    quality: "loss-tree-node--quality",
    default: "",
  };

  return (
    <div className={level > 0 ? "loss-tree-children" : ""}>
      {/* Node */}
      <div
        className={`loss-tree-node ${
          hasChildren ? "loss-tree-node--expandable" : ""
        } ${categoryColors[categoryType]}`}
        onClick={hasChildren ? onToggle : undefined}
        style={{ cursor: hasChildren ? "pointer" : "default" }}
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 flex-1 min-w-0">
            {/* Expand/Collapse Icon */}
            {hasChildren && (
              <svg
                className={`w-4 h-4 text-charcoal-600 dark:text-charcoal-400 flex-shrink-0 transition-transform ${
                  isExpanded ? "rotate-90" : ""
                }`}
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 5l7 7-7 7"
                />
              </svg>
            )}
            {!hasChildren && <div className="w-4" />}

            {/* Category Label */}
            <div className="flex-1 min-w-0">
              <div className="category-label truncate">{node.category_key}</div>
              {node.description_key && (
                <div className="text-xs text-charcoal-500 dark:text-charcoal-400 mt-0.5 truncate">
                  {node.description_key}
                </div>
              )}
            </div>
          </div>

          {/* Metrics */}
          <div className="flex items-center gap-4 ml-4">
            <LossImpactBadge
              duration={node.duration}
              percentageOfPlanned={node.percentage_of_planned}
              percentageOfParent={node.percentage_of_parent}
            />
          </div>
        </div>
      </div>

      {/* Children */}
      {hasChildren && isExpanded && (
        <div className="mt-2 animate-expand-loss">
          {node.children.map((child, idx) => {
            const childPath = `${path}-${idx}-${child.category_key}`;
            return (
              <LossNode
                key={childPath}
                node={child}
                isExpanded={expandedNodes.has(childPath)}
                onToggle={() => onNodeToggle(childPath)}
                path={childPath}
                expandedNodes={expandedNodes}
                onNodeToggle={onNodeToggle}
                level={level + 1}
              />
            );
          })}
        </div>
      )}
    </div>
  );
};

export default LossNode;
