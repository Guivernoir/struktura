import PropTypes from "prop-types";
import Icon from "../Icon";
import ResultCard from "./ResultCard";
import ChartsView from "../engineer/ChartsView";
import { WarningSeverity } from "../../lib";

const ContractorResults = ({
  t,
  results,
  isLoading,
  error,
  warnings,
  structuredWarnings,
  recommendations,
  complianceNotes,
  analysis,
  theme,
  calculatorMetadata,
}) => {
  // Helper to safely access nested translations
  const getT = (path, fallback) => {
    return path.split(".").reduce((obj, k) => obj && obj[k], t) || fallback;
  };

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
    if (!structuredWarnings || structuredWarnings.length === 0) return null;

    const severityConfig = {
      [WarningSeverity.CRITICAL]: {
        icon: "AlertTriangle",
        bgClass: "bg-red-50 dark:bg-red-900/10",
        borderClass: "border-red-500",
        textClass: "text-red-900 dark:text-red-100",
        iconClass: "text-red-600 dark:text-red-500",
        label: "Critical Issue",
      },
      [WarningSeverity.HIGH]: {
        icon: "AlertCircle",
        bgClass: "bg-orange-50 dark:bg-orange-900/10",
        borderClass: "border-orange-500",
        textClass: "text-orange-900 dark:text-orange-100",
        iconClass: "text-orange-600 dark:text-orange-500",
        label: "High Priority",
      },
      [WarningSeverity.MEDIUM]: {
        icon: "Info",
        bgClass: "bg-yellow-50 dark:bg-yellow-900/10",
        borderClass: "border-yellow-500",
        textClass: "text-yellow-900 dark:text-yellow-100",
        iconClass: "text-yellow-600 dark:text-yellow-500",
        label: "Advisory",
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

    // Group by severity
    const grouped = structuredWarnings.reduce((acc, warning) => {
      const sev = warning.severity || WarningSeverity.LOW;
      if (!acc[sev]) acc[sev] = [];
      acc[sev].push(warning);
      return acc;
    }, {});

    return (
      <div className="space-y-3">
        {Object.entries(grouped).map(([severity, items]) => {
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
                  <ul className={`space-y-1.5 text-sm ${config.textClass}`}>
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
        <p className="font-medium">
          {getT(
            "contractor.results.calculating",
            "Processing contracting calculations..."
          )}
        </p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-6 text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 rounded-xl border border-red-200 dark:border-red-800 flex items-center gap-3">
        <Icon name="AlertCircle" size={24} className="flex-shrink-0" />
        <div className="flex-1">
          <h3 className="font-bold">
            {getT("contractor.results.error_title", "Calculation Error")}
          </h3>
          <p className="text-sm mt-1">{error}</p>
        </div>
      </div>
    );
  }

  if (!results || results.length === 0) {
    return (
      <div className="p-12 text-center text-charcoal-400 dark:text-steel-600 bg-sand-50/50 dark:bg-charcoal-900/50 rounded-2xl border border-dashed border-sand-300 dark:border-charcoal-700">
        <Icon
          name="Briefcase"
          size={48}
          className="inline-block mb-4 opacity-50"
        />
        <h3 className="text-lg font-semibold">
          {getT("contractor.results.ready_title", "Ready to Calculate")}
        </h3>
        <p className="text-sm mt-2">
          {getT(
            "contractor.results.ready_desc",
            "Input parameters to generate estimates."
          )}
        </p>
      </div>
    );
  }

  const criticalResults = results.filter((r) => r && r.is_critical === true);
  const standardResults = results.filter((r) => r && r.is_critical !== true);

  return (
    <div className="space-y-6">
      {/* Project Analysis Summary */}
      {analysis && (
        <div className="bg-gradient-to-br from-amber-50 to-orange-50 dark:from-amber-950/30 dark:to-orange-950/30 rounded-xl border-2 border-amber-300 dark:border-amber-700 p-6">
          <h3 className="text-sm font-bold text-amber-900 dark:text-amber-100 uppercase tracking-wide mb-4 flex items-center gap-2">
            <Icon name="TrendingUp" size={18} className="text-amber-600" />
            {getT("contractor.analysis.title", "Project Analysis")}
          </h3>
          <div className="grid grid-cols-2 gap-4">
            <div className="bg-white/60 dark:bg-charcoal-900/60 rounded-lg p-3">
              <div className="text-xs text-charcoal-500 dark:text-steel-500 mb-1">
                {getT("contractor.analysis.cost", "Total Cost")}
              </div>
              <div className="text-xl font-bold text-amber-700 dark:text-amber-300">
                ${analysis.total_cost?.toLocaleString()}
              </div>
            </div>
            <div className="bg-white/60 dark:bg-charcoal-900/60 rounded-lg p-3">
              <div className="text-xs text-charcoal-500 dark:text-steel-500 mb-1">
                {getT("contractor.analysis.duration", "Duration")}
              </div>
              <div className="text-xl font-bold text-amber-700 dark:text-amber-300">
                {analysis.total_duration} days
              </div>
            </div>
            <div className="bg-white/60 dark:bg-charcoal-900/60 rounded-lg p-3">
              <div className="text-xs text-charcoal-500 dark:text-steel-500 mb-1">
                {getT("contractor.analysis.risk", "Risk Level")}
              </div>
              <div className="text-xl font-bold text-amber-700 dark:text-amber-300">
                {(analysis.risk_level * 100).toFixed(0)}%
              </div>
            </div>
            <div className="bg-white/60 dark:bg-charcoal-900/60 rounded-lg p-3">
              <div className="text-xs text-charcoal-500 dark:text-steel-500 mb-1">
                {getT("contractor.analysis.compliance", "Compliance")}
              </div>
              <div className="text-xl font-bold text-amber-700 dark:text-amber-300">
                {(analysis.compliance_score * 100).toFixed(0)}%
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Critical Results */}
      {criticalResults.length > 0 && (
        <div className="p-4 bg-gradient-to-r from-red-50 to-orange-50 dark:from-red-950/30 dark:to-orange-950/30 rounded-xl border-2 border-red-300 dark:border-red-700">
          <div className="flex items-center gap-2 mb-3">
            <Icon
              name="ShieldAlert"
              size={20}
              className="text-red-600 dark:text-red-400"
            />
            <h3 className="text-sm font-bold text-red-900 dark:text-red-100 uppercase tracking-wide">
              {getT("contractor.results.critical", "Critical Parameters")}
            </h3>
          </div>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
            {criticalResults.map((result, index) => (
              <div
                key={index}
                className="p-3 bg-white/80 dark:bg-charcoal-900/80 rounded-lg border border-red-200 dark:border-red-800"
              >
                <div className="text-xs text-charcoal-500 dark:text-steel-500 mb-1 font-medium">
                  {result.label}
                </div>
                <div className="text-lg font-bold text-red-700 dark:text-red-400">
                  {result.formatted_value || `${result.value} ${result.unit}`}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Structured Warnings */}
      {renderStructuredWarnings()}

      {/* Legacy Warnings */}
      {!structuredWarnings && warnings && warnings.length > 0 && (
        <div className="p-4 bg-amber-50 dark:bg-amber-900/10 rounded-lg border-l-4 border-amber-500">
          <div className="flex items-start gap-3">
            <Icon
              name="AlertCircle"
              size={20}
              className="text-amber-600 dark:text-amber-500 mt-1"
            />
            <div className="flex-1">
              <h4 className="text-sm font-bold text-amber-900 dark:text-amber-100 uppercase tracking-wide mb-2">
                Notices
              </h4>
              <ul className="space-y-1 text-sm text-amber-800 dark:text-amber-200/80">
                {warnings.map((w, i) => (
                  <li key={i} className="flex items-start gap-2">
                    <span>•</span>
                    <span>{w}</span>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      )}

      {/* Recommendations */}
      {recommendations && recommendations.length > 0 && (
        <div className="p-4 bg-green-50 dark:bg-green-900/10 rounded-lg border-l-4 border-green-500">
          <div className="flex items-start gap-3">
            <Icon
              name="Lightbulb"
              size={20}
              className="text-green-600 dark:text-green-500 mt-1"
            />
            <div className="flex-1">
              <h4 className="text-sm font-bold text-green-900 dark:text-green-100 uppercase tracking-wide mb-2">
                Recommendations
              </h4>
              <ul className="space-y-1.5 text-sm text-green-800 dark:text-green-200/80">
                {recommendations.map((rec, i) => (
                  <li key={i} className="flex items-start gap-2">
                    <Icon
                      name="CheckCircle"
                      size={14}
                      className="mt-0.5 flex-shrink-0"
                    />
                    <span>{rec}</span>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      )}

      {/* Compliance Notes */}
      {complianceNotes && complianceNotes.length > 0 && (
        <div className="p-4 bg-blue-50 dark:bg-blue-900/10 rounded-lg border-l-4 border-blue-500">
          <div className="flex items-start gap-3">
            <Icon
              name="ClipboardList"
              size={20}
              className="text-blue-600 dark:text-blue-500 mt-1"
            />
            <div className="flex-1">
              <h4 className="text-sm font-bold text-blue-900 dark:text-blue-100 uppercase tracking-wide mb-2">
                Compliance Notes
              </h4>
              <ul className="space-y-1.5 text-sm text-blue-800 dark:text-blue-200/80">
                {complianceNotes.map((note, i) => (
                  <li key={i} className="flex items-start gap-2">
                    <span className="text-xs mt-0.5">•</span>
                    <span>{note}</span>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      )}

      {/* Standard Results */}
      {standardResults.length > 0 && (
        <div>
          <h3 className="text-sm font-semibold text-charcoal-700 dark:text-steel-300 mb-3 uppercase tracking-wide flex items-center gap-2">
            <Icon name="Calculator" size={16} />
            {getT("contractor.results.calculated", "Calculated Results")}
          </h3>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            {/* ... Results Mapping ... */}
            {standardResults.map((result, index) => (
              <ResultCard
                key={index}
                {...result}
                // Note: result.label usually comes from backend.
                // Ideally backend sends translation keys, or we display as is.
              />
            ))}
          </div>
        </div>
      )}

      {/* Visualization */}
      {standardResults.length > 0 && (
        <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-lg border border-sand-200 dark:border-charcoal-800 p-6">
          <h3 className="text-lg font-semibold text-charcoal-900 dark:text-white flex items-center gap-2 mb-4">
            <Icon name="BarChart2" size={20} className="text-amber-600" />
            Visual Analysis
          </h3>
          <ChartsView results={results} theme={theme} />
        </div>
      )}

      {/* Export Actions */}
      <div className="flex justify-between items-center pt-4 border-t border-sand-200 dark:border-charcoal-800">
        <div className="text-xs text-charcoal-400 dark:text-steel-600 flex items-center gap-2">
          {calculatorMetadata?.requires_certification_review && (
            <>
              <Icon name="Shield" size={14} className="text-orange-500" />
              <span>Professional certification review required</span>
            </>
          )}
        </div>
        <div className="flex gap-3">
          <button
            className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-charcoal-600 dark:text-steel-300 bg-sand-100 dark:bg-charcoal-800 hover:bg-sand-200 dark:hover:bg-charcoal-700 rounded-lg transition-colors"
            onClick={() => window.print()}
          >
            <Icon name="Printer" size={16} />
            {getT("contractor.actions.print", "Print")}
          </button>
          <button
            className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-amber-600 hover:bg-amber-700 rounded-lg transition-colors"
            onClick={() => {
              const csv = results
                .map((r) => `${r.label},${r.value},${r.unit}`)
                .join("\n");
              const blob = new Blob([csv], { type: "text/csv" });
              const url = URL.createObjectURL(blob);
              const a = document.createElement("a");
              a.href = url;
              a.download = `${
                calculatorMetadata?.name || "contractor"
              }_results.csv`;
              a.click();
            }}
          >
            <Icon name="Download" size={16} />
            {getT("contractor.actions.export", "Export")}
          </button>
        </div>
      </div>
    </div>
  );
};

ContractorResults.propTypes = {
  t: PropTypes.object.isRequired,
  results: PropTypes.array,
  isLoading: PropTypes.bool.isRequired,
  error: PropTypes.string,
  warnings: PropTypes.array,
  structuredWarnings: PropTypes.array,
  recommendations: PropTypes.array,
  complianceNotes: PropTypes.array,
  analysis: PropTypes.object,
  theme: PropTypes.string.isRequired,
  calculatorMetadata: PropTypes.object,
};

export default ContractorResults;
