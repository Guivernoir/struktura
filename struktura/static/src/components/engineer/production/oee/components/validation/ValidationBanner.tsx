/**
 * ValidationBanner Component
 *
 * Global, always-visible validation status surface.
 * Shows mathematical integrity issues, not user mistakes.
 *
 * Tone: Never say "fix this", always say "this implies X, which conflicts with Y"
 */

import React, { useState } from "react";
import ValidationItem from "./ValidationItem";
import type { ValidationResult } from "../../models";

export interface ValidationBannerProps {
  validation: ValidationResult;
  expanded?: boolean;
}

const ValidationBanner: React.FC<ValidationBannerProps> = ({
  validation,
  expanded: defaultExpanded = false,
}) => {
  const [isExpanded, setIsExpanded] = useState(defaultExpanded);

  const errors = validation.issues.filter((i) => i.severity === "Fatal");
  const warnings = validation.issues.filter((i) => i.severity === "Warning");
  const info = validation.issues.filter((i) => i.severity === "Info");
  const totalIssues = errors.length + warnings.length + info.length;

  if (validation.is_valid && totalIssues === 0) {
    return (
      <div className="bg-green-50 dark:bg-green-900/20 border-l-4 border-green-400 p-4 rounded-md animate-slide-down">
        <div className="flex items-center">
          <svg
            className="w-5 h-5 text-green-600 dark:text-green-400 mr-3"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fillRule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
              clipRule="evenodd"
            />
          </svg>
          <span className="text-sm font-medium text-green-800 dark:text-green-400">
            All mathematical integrity checks passed
          </span>
        </div>
      </div>
    );
  }

  const severity =
    errors.length > 0 ? "error" : warnings.length > 0 ? "warning" : "info";

  const bgColor = {
    error: "bg-red-50 dark:bg-red-900/20 border-red-400",
    warning: "bg-yellow-50 dark:bg-yellow-900/20 border-yellow-400",
    info: "bg-blue-50 dark:bg-blue-900/20 border-blue-400",
  }[severity];

  const textColor = {
    error: "text-red-800 dark:text-red-400",
    warning: "text-yellow-800 dark:text-yellow-400",
    info: "text-blue-800 dark:text-blue-400",
  }[severity];

  const iconColor = {
    error: "text-red-600 dark:text-red-400",
    warning: "text-yellow-600 dark:text-yellow-400",
    info: "text-blue-600 dark:text-blue-400",
  }[severity];

  return (
    <div
      className={`${bgColor} border-l-4 rounded-md overflow-hidden animate-slide-down`}
    >
      <div
        className="p-4 cursor-pointer hover:bg-opacity-75 transition-colors"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <div className="flex items-start justify-between">
          <div className="flex items-start gap-3">
            <svg
              className={`w-5 h-5 ${iconColor} flex-shrink-0 mt-0.5`}
              fill="currentColor"
              viewBox="0 0 20 20"
            >
              {severity === "error" && (
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                  clipRule="evenodd"
                />
              )}
              {severity === "warning" && (
                <path
                  fillRule="evenodd"
                  d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                  clipRule="evenodd"
                />
              )}
              {severity === "info" && (
                <path
                  fillRule="evenodd"
                  d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                  clipRule="evenodd"
                />
              )}
            </svg>
            <div>
              <p className={`text-sm font-semibold ${textColor}`}>
                {totalIssues} Issue{totalIssues !== 1 ? "s" : ""} Detected
                {errors.length > 0 &&
                  ` (${errors.length} Inconsistenc${
                    errors.length !== 1 ? "ies" : "y"
                  })`}
              </p>
              <p className="text-xs mt-1 text-charcoal-600 dark:text-charcoal-400">
                {!validation.is_valid
                  ? "These issues suggest data quality concerns but will not block calculation"
                  : "Informational notices about your inputs"}
                {" • "}
                <span className="font-medium">
                  Click to {isExpanded ? "collapse" : "expand"}
                </span>
              </p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            {/* Issue count badge */}
            <span
              className={`inline-flex items-center justify-center px-2.5 py-1 rounded-full text-xs font-bold ${
                severity === "error"
                  ? "bg-red-200 dark:bg-red-900 text-red-900 dark:text-red-200"
                  : severity === "warning"
                  ? "bg-yellow-200 dark:bg-yellow-900 text-yellow-900 dark:text-yellow-200"
                  : "bg-blue-200 dark:bg-blue-900 text-blue-900 dark:text-blue-200"
              }`}
            >
              {totalIssues}
            </span>
            <svg
              className={`w-5 h-5 ${iconColor} transition-transform ${
                isExpanded ? "rotate-180" : ""
              }`}
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M19 9l-7 7-7-7"
              />
            </svg>
          </div>
        </div>
      </div>

      {isExpanded && (
        <div className="px-4 pb-4 space-y-2 animate-slide-down">
          {errors.length > 0 && (
            <div className="space-y-2">
              <h4 className="text-xs font-semibold text-charcoal-700 dark:text-charcoal-300 uppercase tracking-wide">
                Mathematical Inconsistencies ({errors.length})
              </h4>
              {errors.map((issue, idx) => (
                <ValidationItem key={`error-${idx}`} issue={issue} />
              ))}
            </div>
          )}
          {warnings.length > 0 && (
            <div className="space-y-2 mt-3">
              <h4 className="text-xs font-semibold text-charcoal-700 dark:text-charcoal-300 uppercase tracking-wide">
                Warnings ({warnings.length})
              </h4>
              {warnings.map((issue, idx) => (
                <ValidationItem key={`warning-${idx}`} issue={issue} />
              ))}
            </div>
          )}
          {info.length > 0 && (
            <div className="space-y-2 mt-3">
              <h4 className="text-xs font-semibold text-charcoal-700 dark:text-charcoal-300 uppercase tracking-wide">
                Informational Notes ({info.length})
              </h4>
              {info.map((issue, idx) => (
                <ValidationItem key={`info-${idx}`} issue={issue} />
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default ValidationBanner;
