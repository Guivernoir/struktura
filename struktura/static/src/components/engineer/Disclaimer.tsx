/**
 * Engineering Calculator Disclaimer
 * 
 * The legally required "cover your ass" section.
 * But we make it look professional, because even warnings
 * should be aesthetically pleasing.
 * 
 * Remember: These calculations are tools, not replacements
 * for engineering judgment. If you're building something critical,
 * hire a professional. Your insurance company will thank you.
 */

import { useState } from "react";
import Icon from "./Icon";

// ============================================================================
// Type Definitions
// ============================================================================

interface DisclaimerProps {
  t: Record<string, any>;
}

// ============================================================================
// Main Component
// ============================================================================

const Disclaimer: React.FC<DisclaimerProps> = ({ t }) => {
  const [isExpanded, setIsExpanded] = useState(false);

  /**
   * Get translated label or fallback to key
   */
  const getLabel = (key: string): string => {
    return (
      key
        .split(".")
        .reduce(
          (obj, prop) => (obj && obj[prop] !== undefined ? obj[prop] : key),
          t as any
        ) || key
    );
  };

  return (
    <div className="bg-gradient-to-r from-amber-50 to-orange-50 dark:from-amber-950/20 dark:to-orange-950/20 border-2 border-amber-200 dark:border-amber-800 rounded-2xl overflow-hidden">
      {/* Header - Always Visible */}
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="w-full px-6 py-4 flex items-center justify-between gap-4 hover:bg-amber-100/50 dark:hover:bg-amber-900/20 transition-colors"
      >
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 bg-amber-200 dark:bg-amber-800 rounded-lg flex items-center justify-center flex-shrink-0">
            <Icon
              name="AlertTriangle"
              size={20}
              className="text-amber-900 dark:text-amber-200"
            />
          </div>
          <div className="text-left">
            <h3 className="font-semibold text-amber-900 dark:text-amber-200">
              {getLabel("engineer.disclaimer.title") || "Professional Engineering Disclaimer"}
            </h3>
            <p className="text-sm text-amber-700 dark:text-amber-300">
              {getLabel("engineer.disclaimer.subtitle") || "Important information about these calculations"}
            </p>
          </div>
        </div>
        <Icon
          name={isExpanded ? "ChevronUp" : "ChevronDown"}
          size={20}
          className="text-amber-700 dark:text-amber-300 flex-shrink-0"
        />
      </button>

      {/* Expandable Content */}
      {isExpanded && (
        <div className="px-6 pb-6 space-y-4 border-t border-amber-200 dark:border-amber-800 pt-4">
          {/* Main Warning */}
          <div className="space-y-2">
            <p className="text-sm text-amber-900 dark:text-amber-100 leading-relaxed">
              {getLabel("engineer.disclaimer.main") || 
                "These calculators are provided as educational and reference tools for preliminary calculations only. They are not a substitute for professional engineering judgment, detailed analysis, or adherence to applicable codes and standards."}
            </p>
          </div>

          {/* Key Points */}
          <div className="space-y-2">
            <h4 className="text-sm font-semibold text-amber-900 dark:text-amber-200 flex items-center gap-2">
              <Icon name="CheckCircle2" size={16} />
              <span>
                {getLabel("engineer.disclaimer.requirements_title") || "Professional Requirements"}
              </span>
            </h4>
            <ul className="space-y-2 text-sm text-amber-800 dark:text-amber-200">
              <li className="flex items-start gap-2">
                <Icon
                  name="Dot"
                  size={20}
                  className="text-amber-600 dark:text-amber-400 flex-shrink-0"
                />
                <span>
                  {getLabel("engineer.disclaimer.point_1") || 
                    "All critical structural and production calculations must be verified by a licensed professional engineer"}
                </span>
              </li>
              <li className="flex items-start gap-2">
                <Icon
                  name="Dot"
                  size={20}
                  className="text-amber-600 dark:text-amber-400 flex-shrink-0"
                />
                <span>
                  {getLabel("engineer.disclaimer.point_2") || 
                    "Results should be validated against applicable building codes, industry standards, and local regulations"}
                </span>
              </li>
              <li className="flex items-start gap-2">
                <Icon
                  name="Dot"
                  size={20}
                  className="text-amber-600 dark:text-amber-400 flex-shrink-0"
                />
                <span>
                  {getLabel("engineer.disclaimer.point_3") || 
                    "These tools make simplifying assumptions that may not apply to your specific situation"}
                </span>
              </li>
              <li className="flex items-start gap-2">
                <Icon
                  name="Dot"
                  size={20}
                  className="text-amber-600 dark:text-amber-400 flex-shrink-0"
                />
                <span>
                  {getLabel("engineer.disclaimer.point_4") || 
                    "Site-specific conditions, material properties, and load combinations require detailed engineering analysis"}
                </span>
              </li>
            </ul>
          </div>

          {/* Liability Section */}
          <div className="pt-3 border-t border-amber-200 dark:border-amber-800">
            <h4 className="text-sm font-semibold text-amber-900 dark:text-amber-200 flex items-center gap-2 mb-2">
              <Icon name="Shield" size={16} />
              <span>
                {getLabel("engineer.disclaimer.liability_title") || "Limitation of Liability"}
              </span>
            </h4>
            <p className="text-xs text-amber-800 dark:text-amber-300 leading-relaxed">
              {getLabel("engineer.disclaimer.liability") || 
                "The creators of these tools accept no liability for decisions made based on these calculations. Users assume full responsibility for verifying results and ensuring compliance with applicable standards. When in doubt, consult a qualified professional engineer."}
            </p>
          </div>

          {/* Best Practices */}
          <div className="pt-3 border-t border-amber-200 dark:border-amber-800">
            <h4 className="text-sm font-semibold text-amber-900 dark:text-amber-200 flex items-center gap-2 mb-2">
              <Icon name="BookOpen" size={16} />
              <span>
                {getLabel("engineer.disclaimer.best_practices_title") || "Recommended Practices"}
              </span>
            </h4>
            <ul className="space-y-1 text-xs text-amber-800 dark:text-amber-300">
              <li className="flex items-start gap-2">
                <span className="text-amber-600 dark:text-amber-400">→</span>
                <span>Use these calculators for preliminary estimates and feasibility studies</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-amber-600 dark:text-amber-400">→</span>
                <span>Cross-reference results with multiple sources and methods</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-amber-600 dark:text-amber-400">→</span>
                <span>Apply appropriate safety factors based on your specific application</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-amber-600 dark:text-amber-400">→</span>
                <span>Document all assumptions and limitations in your engineering reports</span>
              </li>
            </ul>
          </div>

          {/* Final Note */}
          <div className="pt-3 mt-3 border-t border-amber-200 dark:border-amber-800">
            <p className="text-xs text-amber-700 dark:text-amber-400 italic text-center">
              {getLabel("engineer.disclaimer.final") || 
                "Remember: Good engineering judgment cannot be automated. These are tools to assist, not replace, professional expertise."}
            </p>
          </div>
        </div>
      )}

      {/* Quick Reference When Collapsed */}
      {!isExpanded && (
        <div className="px-6 pb-4">
          <p className="text-xs text-amber-700 dark:text-amber-400 italic">
            Click to read important information about professional verification requirements
          </p>
        </div>
      )}
    </div>
  );
};

export default Disclaimer;