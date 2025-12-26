/**
 * AssumptionSourceBadge Component
 *
 * Visual indicator for the source of an assumption value.
 * Shows whether the value is Explicit (user), Inferred (calculated), or Default (system).
 */

import React from "react";

export interface AssumptionSourceBadgeProps {
  source: "explicit" | "inferred" | "default";
  showLabel?: boolean;
  size?: "sm" | "md";
}

const AssumptionSourceBadge: React.FC<AssumptionSourceBadgeProps> = ({
  source,
  showLabel = true,
  size = "sm",
}) => {
  const config = {
    explicit: {
      label: "Explicit",
      icon: "✓",
      className: "source-pill--explicit",
      description: "User provided",
    },
    inferred: {
      label: "Inferred",
      icon: "⚙",
      className: "source-pill--inferred",
      description: "Calculated from other inputs",
    },
    default: {
      label: "Default",
      icon: "○",
      className: "source-pill--default",
      description: "System default",
    },
  };

  const { label, icon, className, description } = config[source];
  const sizeClass = size === "sm" ? "text-xs px-2 py-1" : "text-sm px-3 py-1.5";

  return (
    <span
      className={`source-pill ${className} ${sizeClass} inline-flex items-center gap-1`}
      title={description}
    >
      <span>{icon}</span>
      {showLabel && <span>{label}</span>}
    </span>
  );
};

export default AssumptionSourceBadge;
