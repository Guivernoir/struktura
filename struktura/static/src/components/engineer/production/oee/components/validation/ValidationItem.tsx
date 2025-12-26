/**
 * ValidationItem Component
 *
 * Single validation issue with explanation.
 * One issue = one explanation.
 * Never suggests what the user "should" do, only explains conflicts.
 */

import React from "react";
import type { ValidationIssue } from "../../models";

export interface ValidationItemProps {
  issue: ValidationIssue;
}

const ValidationItem: React.FC<ValidationItemProps> = ({ issue }) => {
  const severityConfig = {
    Fatal: {
      bgColor: "bg-red-100 dark:bg-red-900/20",
      borderColor: "border-red-200 dark:border-red-800",
      textColor: "text-red-800 dark:text-red-400",
      label: "Mathematical Inconsistency",
    },
    Warning: {
      bgColor: "bg-yellow-100 dark:bg-yellow-900/20",
      borderColor: "border-yellow-200 dark:border-yellow-800",
      textColor: "text-yellow-800 dark:text-yellow-400",
      label: "Plausibility Warning",
    },
    Info: {
      bgColor: "bg-blue-100 dark:bg-blue-900/20",
      borderColor: "border-blue-200 dark:border-blue-800",
      textColor: "text-blue-800 dark:text-blue-400",
      label: "Note",
    },
  };

  const config = severityConfig[issue.severity];

  // Format message from message_key and params
  const formatMessage = () => {
    let message = issue.message_key;
    if (issue.params && typeof issue.params === "object") {
      Object.entries(issue.params).forEach(([key, value]) => {
        message = message.replace(`{${key}}`, String(value));
      });
    }
    return message;
  };

  return (
    <div
      className={`${config.bgColor} border ${config.borderColor} rounded-md p-3`}
    >
      <div className="flex items-start gap-2">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-1">
            <span
              className={`text-xs font-semibold ${config.textColor} uppercase tracking-wide`}
            >
              {config.label}
            </span>
            {issue.code && (
              <code className="text-xs px-1.5 py-0.5 bg-white dark:bg-charcoal-800 rounded font-mono">
                {issue.code}
              </code>
            )}
          </div>

          <p className={`text-sm ${config.textColor} font-medium`}>
            {formatMessage()}
          </p>

          {issue.field_path && (
            <p className="text-xs text-charcoal-600 dark:text-charcoal-400 mt-1">
              Affects: <code className="font-mono">{issue.field_path}</code>
            </p>
          )}
        </div>
      </div>
    </div>
  );
};

export default ValidationItem;
