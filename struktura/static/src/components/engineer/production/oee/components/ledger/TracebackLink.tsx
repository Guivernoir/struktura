/**
 * TracebackLink Component
 *
 * Cross-navigation glue between assumptions and affected results.
 * Provides clickable links to related assumptions.
 */

import React from "react";

export interface TracebackLinkProps {
  assumptionKey: string;
  onClick?: (key: string) => void;
}

const TracebackLink: React.FC<TracebackLinkProps> = ({
  assumptionKey,
  onClick,
}) => {
  const handleClick = () => {
    if (onClick) {
      onClick(assumptionKey);
    } else {
      // Default behavior: scroll to assumption if on same page
      const element = document.querySelector(
        `[data-assumption-key="${assumptionKey}"]`
      );
      if (element) {
        element.scrollIntoView({ behavior: "smooth", block: "center" });
        // Highlight briefly
        element.classList.add("animate-highlight-assumption");
        setTimeout(() => {
          element.classList.remove("animate-highlight-assumption");
        }, 1000);
      }
    }
  };

  return (
    <button
      onClick={handleClick}
      className="inline-flex items-center gap-1 px-2 py-1 text-xs font-mono 
               bg-steel-100 dark:bg-steel-900/20 text-steel-700 dark:text-steel-400 
               rounded hover:bg-steel-200 dark:hover:bg-steel-800/30 transition-colors
               border border-steel-300 dark:border-steel-700"
      title={`Navigate to ${assumptionKey}`}
    >
      <svg
        className="w-3 h-3"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M13 7l5 5m0 0l-5 5m5-5H6"
        />
      </svg>
      <span className="truncate max-w-[200px]">{assumptionKey}</span>
    </button>
  );
};

export default TracebackLink;
