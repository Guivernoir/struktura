import PropTypes from "prop-types";
import Icon from "../Icon";
import ResultCard from "./ResultCard";
import ChartsView from "./ChartsView";
import { WarningSeverity } from "../../lib";

const EngineerResults = ({
  t,
  results,
  isLoading,
  error,
  warnings,
  structuredWarnings,
  recommendations,
  theme,
  calculatorMetadata,
}) => {
  const getLabel = (key) => {
    return (
      key
        .split(".")
        .reduce(
          (obj, prop) => (obj && obj[prop] !== undefined ? obj[prop] : key),
          t
        ) || key
    );
  };

  // Render structured warnings by severity
  const renderStructuredWarnings = () => {
    if (!structuredWarnings) return null;

    const severityConfig = {
      [WarningSeverity.CRITICAL]: {
        icon: "AlertTriangle",
        bgClass: "bg-red-50 dark:bg-red-900/10",
        borderClass: "border-red-500",
        textClass: "text-red-900 dark:text-red-100",
        iconClass: "text-red-600 dark:text-red-500",
        label: "Critical Safety Issue",
      },
      [WarningSeverity.HIGH]: {
        icon: "AlertCircle",
        bgClass: "bg-orange-50 dark:bg-orange-900/10",
        borderClass: "border-orange-500",
        textClass: "text-orange-900 dark:text-orange-100",
        iconClass: "text-orange-600 dark:text-orange-500",
        label: "High Priority Warning",
      },
      [WarningSeverity.MEDIUM]: {
        icon: "Info",
        bgClass: "bg-yellow-50 dark:bg-yellow-900/10",
        borderClass: "border-yellow-500",
        textClass: "text-yellow-900 dark:text-yellow-100",
        iconClass: "text-yellow-600 dark:text-yellow-500",
        label: "Advisory Notice",
      },
      [WarningSeverity.LOW]: {
        icon: "MessageSquare",
        bgClass: "bg-blue-50 dark:bg-blue-900/10",
        borderClass: "border-blue-500",
        textClass: "text-blue-900 dark:text-blue-100",
        iconClass: "text-blue-600 dark:text-blue-500",
        label: "Informational",
      },
    };

    return (
      <div className="space-y-3">
        {Object.entries(structuredWarnings).map(([severity, items]) => {
          if (!items || items.length === 0) return null;

          const config =
            severityConfig[severity] || severityConfig[WarningSeverity.LOW];

          return (
            <div
              key={severity}
              className={`p-4 ${config.bgClass} rounded-lg border-l-4 ${config.borderClass}`}
            >
              <div className="flex items-start gap-3">
                <Icon
                  name={config.icon}
                  size={20}
                  className={`${config.iconClass} mt-1 flex-shrink-0`}
                />
                <div className="flex-1">
                  <h4
                    className={`text-sm font-bold ${config.textClass} uppercase tracking-wide mb-2`}
                  >
                    {config.label}
                  </h4>
                  <ul
                    className={`space-y-1.5 text-sm ${config.textClass
                      .replace("900", "800")
                      .replace("100", "200/80")} font-mono`}
                  >
                    {items.map((warning, i) => (
                      <li key={i} className="flex items-start gap-2">
                        <span className="text-xs mt-0.5">•</span>
                        <div className="flex-1">
                          <span>{warning.message}</span>
                          {warning.affected_parameter && (
                            <span className="ml-2 text-xs px-1.5 py-0.5 bg-white/50 dark:bg-black/20 rounded">
                              {warning.affected_parameter}
                            </span>
                          )}
                        </div>
                      </li>
                    ))}
                  </ul>
                </div>
              </div>
            </div>
          );
        })}
      </div>
    );
  };

  if (isLoading) {
    return (
      <div className="p-12 text-center text-charcoal-500 dark:text-steel-500 bg-white dark:bg-charcoal-900 rounded-2xl border border-sand-200 dark:border-charcoal-800">
        <Icon
          name="Loader2"
          size={32}
          className="animate-spin inline-block mb-4"
        />
        <p className="font-medium">{getLabel("engineer.loading_analysis")}</p>
        <p className="text-xs mt-2 text-charcoal-400 dark:text-steel-600">
          Running structural analysis...
        </p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-6 text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 rounded-xl border border-red-200 dark:border-red-800 flex items-center gap-3">
        <Icon name="AlertCircle" size={24} className="flex-shrink-0" />
        <div className="flex-1">
          <h3 className="font-bold">Calculation Error</h3>
          <p className="text-sm mt-1">{error}</p>
        </div>
      </div>
    );
  }

  if (!results || results.length === 0) {
    return (
      <div className="p-12 text-center text-charcoal-400 dark:text-steel-600 bg-sand-50/50 dark:bg-charcoal-900/50 rounded-2xl border border-dashed border-sand-300 dark:border-charcoal-700">
        <Icon
          name="Activity"
          size={48}
          className="inline-block mb-4 opacity-50"
        />
        <h3 className="text-lg font-semibold">
          {getLabel("engineer.ready_to_calc")}
        </h3>
        <p className="text-sm mt-2">
          Input parameters and click Calculate to generate technical
          specifications.
        </p>

        {calculatorMetadata && calculatorMetadata.typical_applications && (
          <div className="mt-6 text-left max-w-md mx-auto">
            <h4 className="text-xs font-semibold text-charcoal-500 dark:text-steel-500 uppercase tracking-wider mb-2">
              Typical Applications:
            </h4>
            <ul className="space-y-1 text-xs text-charcoal-500 dark:text-steel-500">
              {calculatorMetadata.typical_applications.map((app, idx) => (
                <li key={idx} className="flex items-start gap-2">
                  <span className="text-indigo-500 mt-0.5">▸</span>
                  <span>{app}</span>
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>
    );
  }

  // Separate critical and non-critical results with defensive checks
  const criticalResults = results.filter((r) => r && r.isCritical === true);
  const standardResults = results.filter((r) => r && r.isCritical !== true);

  return (
    <div className="space-y-6">
      {/* Critical Results Banner */}
      {criticalResults.length > 0 && (
        <div className="p-4 bg-gradient-to-r from-red-50 to-orange-50 dark:from-red-950/30 dark:to-orange-950/30 rounded-xl border-2 border-red-300 dark:border-red-700">
          <div className="flex items-center gap-2 mb-3">
            <Icon
              name="ShieldAlert"
              size={20}
              className="text-red-600 dark:text-red-400"
            />
            <h3 className="text-sm font-bold text-red-900 dark:text-red-100 uppercase tracking-wide">
              Critical Design Parameters
            </h3>
          </div>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
            {criticalResults.map((result, index) => {
              // Defensive: ensure result has required fields
              if (!result || typeof result.value !== "number") {
                console.warn("Invalid critical result item:", result);
                return null;
              }
              return (
                <ResultCard
                  key={index}
                  label={result.label || "Unknown"}
                  value={result.value}
                  unit={result.unit || ""}
                  formatted={result.displayValue}
                  tolerance={result.tolerancePercent}
                  isCritical={true}
                />
              );
            })}
          </div>
        </div>
      )}

      {/* Structured Warnings */}
      {structuredWarnings && (
        <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-lg border border-sand-200 dark:border-charcoal-800 p-6 space-y-4">
          <h3 className="text-xl font-semibold text-charcoal-900 dark:text-white mb-2">
            Design & Compliance Analysis
          </h3>
          {renderStructuredWarnings()}
        </div>
      )}

      {/* Recommendations */}
      {recommendations && recommendations.length > 0 && (
        <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-lg border border-sand-200 dark:border-charcoal-800 p-6 space-y-4">
          <h3 className="text-xl font-semibold text-charcoal-900 dark:text-white">
            Recommendations
          </h3>
          <ul className="space-y-3 text-sm text-charcoal-700 dark:text-steel-300">
            {recommendations.map((rec, index) => (
              <li key={index} className="flex items-start gap-3">
                <Icon
                  name="CheckCircle"
                  size={20}
                  className="text-green-500 flex-shrink-0 mt-0.5"
                />
                <p>{rec}</p>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Standard Results Grid */}
      {standardResults.length > 0 && (
        <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-lg border border-sand-200 dark:border-charcoal-800 p-6">
          <h3
            // UPDATED: Standard results header text color to Red/Rose accent
            className="text-sm font-semibold text-red-600 dark:text-rose-400 mb-3 uppercase tracking-wide flex items-center gap-2"
          >
            <Icon name="Calculator" size={16} /> Calculated Results
          </h3>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            {standardResults.map((result, index) => {
              // Defensive: ensure result has required fields
              if (!result || typeof result.value !== "number") {
                console.warn("Invalid result item:", result);
                return null;
              }
              return (
                <ResultCard
                  key={index}
                  label={result.label || "Unknown"}
                  value={result.value}
                  unit={result.unit || ""}
                  formatted={result.displayValue}
                  tolerance={result.tolerancePercent}
                  isCritical={false}
                />
              );
            })}
          </div>

          {/* Visualization Dashboard */}
          <ChartsView results={results} theme={theme} />

          {/* Actions */}
          <div className="mt-8 pt-6 border-t border-sand-200 dark:border-charcoal-800 flex justify-end gap-3">
            <button
              className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-charcoal-800 dark:text-steel-200 bg-sand-100 dark:bg-charcoal-800 hover:bg-sand-200 dark:hover:bg-charcoal-700 rounded-lg transition-colors"
              onClick={() => window.print()}
            >
              <Icon name="Printer" size={16} />
              Print Spec
            </button>
            <button
              // UPDATED: Export button colors to Indigo accent
              className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-indigo-600 dark:bg-indigo-500 hover:bg-indigo-700 dark:hover:bg-indigo-600 rounded-lg transition-colors shadow-md shadow-indigo-600/30"
              onClick={() => {
                // Export to CSV
                const csv = results
                  .filter((r) => r && typeof r.value === "number")
                  .map(
                    (r) => `${r.label || "Unknown"},${r.value},${r.unit || ""}`
                  )
                  .join("\n");
                const blob = new Blob([csv], { type: "text/csv" });
                const url = URL.createObjectURL(blob);
                const a = document.createElement("a");
                a.href = url;
                a.download = `${
                  calculatorMetadata?.name || "calculation"
                }_results.csv`;
                a.click();
              }}
            >
              <Icon name="Download" size={16} />
              Export CSV
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

EngineerResults.propTypes = {
  t: PropTypes.object.isRequired,
  results: PropTypes.array,
  isLoading: PropTypes.bool.isRequired,
  error: PropTypes.string,
  warnings: PropTypes.array,
  structuredWarnings: PropTypes.object,
  recommendations: PropTypes.array,
  theme: PropTypes.string.isRequired,
  calculatorMetadata: PropTypes.object,
};

export default EngineerResults;
