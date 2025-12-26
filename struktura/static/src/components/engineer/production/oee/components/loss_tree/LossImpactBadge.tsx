/**
 * LossImpactBadge Component
 *
 * Displays OEE points / time / throughput delta for a loss node.
 * Shows duration and percentage context in a compact badge format.
 */

import React from "react";
import { formatDuration } from "../../utils";

export interface LossImpactBadgeProps {
  duration: number;
  percentageOfPlanned: number;
  percentageOfParent: number | null;
  showParentPercentage?: boolean;
}

const LossImpactBadge: React.FC<LossImpactBadgeProps> = ({
  duration,
  percentageOfPlanned,
  percentageOfParent,
  showParentPercentage = true,
}) => {
  return (
    <div className="flex items-center gap-3">
      {/* Duration */}
      <div className="text-right">
        <div className="duration text-sm font-semibold text-charcoal-900 dark:text-charcoal-100">
          {formatDuration(duration, { compact: true, maxParts: 2 })}
        </div>
        <div className="text-xs text-charcoal-500 dark:text-charcoal-400">
          Duration
        </div>
      </div>

      {/* Percentage of Planned */}
      <div className="text-right">
        <div className="percentage text-sm font-semibold text-steel-600 dark:text-steel-400">
          {(percentageOfPlanned * 100).toFixed(1)}%
        </div>
        <div className="text-xs text-charcoal-500 dark:text-charcoal-400">
          of Planned
        </div>
      </div>

      {/* Percentage of Parent (if applicable) */}
      {showParentPercentage && percentageOfParent !== null && (
        <div className="text-right">
          <div className="percentage text-sm font-semibold text-charcoal-600 dark:text-charcoal-300">
            {(percentageOfParent * 100).toFixed(1)}%
          </div>
          <div className="text-xs text-charcoal-500 dark:text-charcoal-400">
            of Parent
          </div>
        </div>
      )}
    </div>
  );
};

export default LossImpactBadge;
