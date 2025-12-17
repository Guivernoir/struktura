import PropTypes from "prop-types";
import Icon from "../Icon";

const BeginnerResults = ({
  t,
  results,
  isLoading,
  error,
  warnings,
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

  if (isLoading) {
    return (
      <div className="p-12 text-center text-charcoal-500 dark:text-steel-500 bg-white dark:bg-charcoal-900 rounded-2xl border border-sand-200 dark:border-charcoal-800">
        <Icon
          name="Loader2"
          size={32}
          className="animate-spin inline-block mb-4"
        />
        <p className="font-medium">{getLabel("beginner.loading")}</p>
        <p className="text-xs mt-2 text-charcoal-400 dark:text-steel-600">
          Calculating your project needs...
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
          name="Hammer"
          size={48}
          className="inline-block mb-4 opacity-50"
        />
        <h3 className="text-lg font-semibold">
          {getLabel("beginner.ready_to_calc")}
        </h3>
        <p className="text-sm mt-2">
          Select a project and enter dimensions to get started!
        </p>

        {calculatorMetadata && calculatorMetadata.typical_applications && (
          <div className="mt-6 text-left max-w-md mx-auto">
            <h4 className="text-xs font-semibold text-charcoal-500 dark:text-steel-500 uppercase tracking-wider mb-2">
              Perfect for:
            </h4>
            <ul className="space-y-1 text-xs text-charcoal-500 dark:text-steel-500">
              {calculatorMetadata.typical_applications.map((app, idx) => (
                <li key={idx} className="flex items-start gap-2">
                  <span className="text-green-500 mt-0.5">▸</span>
                  <span>{app}</span>
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Success Banner */}
      <div className="bg-gradient-to-br from-green-50 to-emerald-50 dark:from-green-950/30 dark:to-emerald-950/30 rounded-xl border-2 border-green-300 dark:border-green-700 p-6">
        <div className="flex items-center gap-3 mb-4">
          <div className="flex-shrink-0 w-10 h-10 bg-green-600 dark:bg-green-500 rounded-full flex items-center justify-center">
            <Icon name="CheckCircle" size={20} className="text-white" />
          </div>
          <div>
            <h3 className="text-lg font-bold text-green-900 dark:text-green-100">
              {getLabel("beginner.results.success") || "Your Project Needs"}
            </h3>
            <p className="text-sm text-green-700 dark:text-green-300">
              Here's what you'll need for this project
            </p>
          </div>
        </div>

        {/* Results Grid */}
        <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
          {results.map((result, index) => (
            <div
              key={index}
              className="p-4 bg-white/80 dark:bg-charcoal-900/80 rounded-lg border border-green-200 dark:border-green-800"
            >
              <div className="text-xs text-charcoal-500 dark:text-steel-500 mb-1 font-medium uppercase tracking-wide">
                {result.label}
              </div>
              <div className="text-2xl font-bold text-green-700 dark:text-green-300">
                {typeof result.value === "number"
                  ? result.value.toLocaleString(undefined, {
                      maximumFractionDigits: 2,
                    })
                  : result.value}
              </div>
              <div className="text-xs text-charcoal-400 dark:text-steel-600 mt-1">
                {result.unit}
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Warnings */}
      {warnings && warnings.length > 0 && (
        <div className="p-4 bg-amber-50 dark:bg-amber-900/10 rounded-lg border-l-4 border-amber-500">
          <div className="flex items-start gap-3">
            <Icon
              name="AlertTriangle"
              size={20}
              className="text-amber-600 dark:text-amber-500 mt-1 flex-shrink-0"
            />
            <div className="flex-1">
              <h4 className="text-sm font-bold text-amber-900 dark:text-amber-100 uppercase tracking-wide mb-2">
                {getLabel("beginner.results.important_notes") ||
                  "Important Considerations"}
              </h4>
              <ul className="space-y-1.5 text-sm text-amber-800 dark:text-amber-200/80">
                {warnings.map((warning, i) => (
                  <li key={i} className="flex items-start gap-2">
                    <span className="text-xs mt-0.5">•</span>
                    <span>{warning}</span>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      )}

      {/* Shopping List Card */}
      <div className="bg-white dark:bg-charcoal-900 rounded-2xl shadow-lg border border-sand-200 dark:border-charcoal-800 p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-charcoal-900 dark:text-white flex items-center gap-2">
            <Icon name="ShoppingCart" size={20} className="text-green-600" />
            {getLabel("beginner.results.shopping_list") || "Shopping List"}
          </h3>
        </div>

        <div className="space-y-3">
          {results.map((result, index) => (
            <div
              key={index}
              className="flex items-center justify-between p-3 bg-sand-50 dark:bg-charcoal-800 rounded-lg hover:bg-sand-100 dark:hover:bg-charcoal-700 transition"
            >
              <div className="flex items-center gap-3">
                <div className="w-8 h-8 bg-green-100 dark:bg-green-900/30 rounded-full flex items-center justify-center flex-shrink-0">
                  <span className="text-green-600 dark:text-green-400 font-bold text-sm">
                    {index + 1}
                  </span>
                </div>
                <div>
                  <div className="font-medium text-charcoal-900 dark:text-white">
                    {result.label}
                  </div>
                  <div className="text-xs text-charcoal-500 dark:text-steel-500">
                    Round up when purchasing
                  </div>
                </div>
              </div>
              <div className="text-right">
                <div className="font-bold text-charcoal-900 dark:text-white">
                  {typeof result.value === "number"
                    ? result.value.toLocaleString(undefined, {
                        maximumFractionDigits: 2,
                      })
                    : result.value}
                </div>
                <div className="text-xs text-charcoal-500 dark:text-steel-500">
                  {result.unit}
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* Action Buttons */}
        <div className="mt-6 flex gap-3">
          <button
            className="flex-1 flex items-center justify-center gap-2 px-4 py-3 text-sm font-medium text-charcoal-600 dark:text-steel-300 bg-sand-100 dark:bg-charcoal-800 hover:bg-sand-200 dark:hover:bg-charcoal-700 rounded-lg transition-colors"
            onClick={() => window.print()}
          >
            <Icon name="Printer" size={16} />
            Print List
          </button>
          <button
            className="flex-1 flex items-center justify-center gap-2 px-4 py-3 text-sm font-medium text-white bg-green-600 hover:bg-green-700 rounded-lg transition-colors"
            onClick={() => {
              const text = results
                .map((r) => `${r.label}: ${r.value} ${r.unit}`)
                .join("\n");
              navigator.clipboard.writeText(text);
              // You could add a toast notification here
            }}
          >
            <Icon name="Copy" size={16} />
            Copy List
          </button>
        </div>
      </div>

      {/* Next Steps Card */}
      <div className="bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-blue-950/30 dark:to-indigo-950/30 rounded-xl p-6 border border-blue-200 dark:border-blue-800">
        <h4 className="text-sm font-bold text-blue-900 dark:text-blue-100 uppercase tracking-wide mb-3 flex items-center gap-2">
          <Icon name="Compass" size={16} className="text-blue-600" />
          {getLabel("beginner.results.next_steps") || "Next Steps"}
        </h4>
        <ol className="space-y-2 text-sm text-blue-800 dark:text-blue-200">
          <li className="flex items-start gap-3">
            <span className="flex-shrink-0 w-6 h-6 bg-blue-600 dark:bg-blue-500 text-white rounded-full flex items-center justify-center text-xs font-bold">
              1
            </span>
            <span>Purchase materials from your shopping list</span>
          </li>
          <li className="flex items-start gap-3">
            <span className="flex-shrink-0 w-6 h-6 bg-blue-600 dark:bg-blue-500 text-white rounded-full flex items-center justify-center text-xs font-bold">
              2
            </span>
            <span>Check local building codes and get permits if needed</span>
          </li>
          <li className="flex items-start gap-3">
            <span className="flex-shrink-0 w-6 h-6 bg-blue-600 dark:bg-blue-500 text-white rounded-full flex items-center justify-center text-xs font-bold">
              3
            </span>
            <span>Gather tools and safety equipment before starting</span>
          </li>
        </ol>
      </div>
    </div>
  );
};

BeginnerResults.propTypes = {
  t: PropTypes.object.isRequired,
  results: PropTypes.array,
  isLoading: PropTypes.bool.isRequired,
  error: PropTypes.string,
  warnings: PropTypes.array,
  theme: PropTypes.string.isRequired,
  calculatorMetadata: PropTypes.object,
};

export default BeginnerResults;
