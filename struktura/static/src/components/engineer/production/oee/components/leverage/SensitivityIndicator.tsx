/**
 * SensitivityIndicator Component
 *
 * Visual indicator for stability/fragility signal.
 * Shows how sensitive the impact is to input variations.
 */

import React from "react";

export interface SensitivityIndicatorProps {
  score: number; // 0-1 range
  size?: "sm" | "md" | "lg";
  showLabel?: boolean;
}

const SensitivityIndicator: React.FC<SensitivityIndicatorProps> = ({
  score,
  size = "md",
  showLabel = true,
}) => {
  const percentage = score * 100;

  // Determine sensitivity level
  const getLevel = () => {
    if (percentage < 10)
      return {
        label: "Very Stable",
        color: "bg-green-500",
        textColor: "text-green-700 dark:text-green-400",
      };
    if (percentage < 25)
      return {
        label: "Stable",
        color: "bg-blue-500",
        textColor: "text-blue-700 dark:text-blue-400",
      };
    if (percentage < 50)
      return {
        label: "Moderate",
        color: "bg-yellow-500",
        textColor: "text-yellow-700 dark:text-yellow-400",
      };
    if (percentage < 75)
      return {
        label: "Sensitive",
        color: "bg-orange-500",
        textColor: "text-orange-700 dark:text-orange-400",
      };
    return {
      label: "Very Sensitive",
      color: "bg-red-500",
      textColor: "text-red-700 dark:text-red-400",
    };
  };

  const level = getLevel();

  const sizeClasses = {
    sm: "w-16 h-2",
    md: "w-24 h-3",
    lg: "w-32 h-4",
  };

  const textSizes = {
    sm: "text-xs",
    md: "text-sm",
    lg: "text-base",
  };

  return (
    <div className="text-right">
      {/* Progress Bar */}
      <div
        className={`${sizeClasses[size]} bg-charcoal-200 dark:bg-charcoal-700 rounded-full overflow-hidden`}
      >
        <div
          className={`h-full ${level.color} transition-all duration-300`}
          style={{ width: `${Math.min(percentage, 100)}%` }}
        />
      </div>

      {/* Label */}
      {showLabel && (
        <div className="mt-1 space-y-0.5">
          <div className={`${textSizes[size]} font-medium ${level.textColor}`}>
            {level.label}
          </div>
          <div className="text-xs text-charcoal-500 dark:text-charcoal-400">
            {percentage.toFixed(0)}% sensitivity
          </div>
        </div>
      )}
    </div>
  );
};

export default SensitivityIndicator;
