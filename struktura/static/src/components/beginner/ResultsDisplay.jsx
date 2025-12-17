import PropTypes from 'prop-types';
import Icon from '../Icon';
import ResultCard from './ResultCard';

const ResultsDisplay = ({ t, results, isLoading, error, warnings }) => {
  const getLabel = (key) => {
    return key.split('.').reduce((obj, prop) => 
      (obj && obj[prop] !== undefined) ? obj[prop] : key, 
      t
    ) || key;
  };

  if (isLoading) {
    return (
      <div className="space-y-4">
        <div className="p-6 text-center text-charcoal-500 dark:text-steel-500">
          <Icon name="Loader2" size={24} className="animate-spin inline-block mr-2" />
          {getLabel('beginner.loading')}
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="space-y-4">
        <div className="p-6 text-center text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 rounded-xl border border-red-200 dark:border-red-800">
          <Icon name="AlertCircle" size={24} className="inline-block mb-2" />
          <p>{error}</p>
        </div>
      </div>
    );
  }

  if (!results || results.length === 0) {
    return (
      <div className="space-y-4">
        <div className="p-6 text-center text-charcoal-500 dark:text-steel-500 bg-sand-50 dark:bg-charcoal-900 rounded-xl border border-sand-200 dark:border-charcoal-700">
          <Icon name="Sigma" size={24} className="inline-block mb-2" />
          <p>{getLabel('beginner.no_results')}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {/* Warnings Panel (if any) */}
      {warnings && warnings.length > 0 && (
        <div className="p-4 bg-amber-50 dark:bg-amber-900/20 rounded-xl border border-amber-200 dark:border-amber-800">
          <div className="flex items-start gap-3">
            <Icon name="AlertTriangle" size={20} className="text-amber-600 dark:text-amber-400 flex-shrink-0 mt-0.5" />
            <div className="flex-1">
              <h3 className="font-semibold text-amber-900 dark:text-amber-200 mb-2">
                Important Considerations
              </h3>
              <ul className="space-y-1 text-sm text-amber-800 dark:text-amber-300">
                {warnings.map((warning, idx) => (
                  <li key={idx} className="flex gap-2">
                    <span>•</span>
                    <span>{warning}</span>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      )}

      {/* Results Grid */}
      <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
        {results.map((result, index) => (
          <ResultCard 
            key={index} 
            label={result.label} 
            value={result.value} 
            unit={result.unit} 
          />
        ))}
      </div>
    </div>
  );
};

ResultsDisplay.propTypes = {
  t: PropTypes.object.isRequired,
  results: PropTypes.array,
  isLoading: PropTypes.bool.isRequired,
  error: PropTypes.string,
  warnings: PropTypes.array,
};

export default ResultsDisplay;