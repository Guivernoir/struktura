import PropTypes from "prop-types";
import Icon from "../Icon";

const ResultCard = ({
  label,
  value,
  unit,
  formatted,
  tolerance,
  isCritical,
}) => {
  const displayValue = formatted || `${value} ${unit}`;

  return (
    <div
      className={`p-4 rounded-xl border transition-all duration-200 hover:shadow-md ${
        isCritical
          ? "bg-red-50 dark:bg-red-950/30 border-red-300 dark:border-red-700"
          : "bg-white dark:bg-charcoal-900 border-sand-200 dark:border-charcoal-800"
      }`}
    >
      <div className="flex items-start justify-between mb-2">
        <div className="text-xs font-medium text-charcoal-500 dark:text-steel-500 uppercase tracking-wide leading-tight">
          {label}
        </div>
        {isCritical && (
          <Icon
            name="AlertTriangle"
            size={14}
            className="text-red-600 dark:text-red-400 flex-shrink-0 ml-1"
          />
        )}
      </div>

      <div
        className={`text-2xl font-bold mb-1 ${
          isCritical
            ? "text-red-700 dark:text-red-300"
            : "text-charcoal-900 dark:text-white"
        }`}
      >
        {value.toLocaleString(undefined, { maximumFractionDigits: 3 })}
      </div>

      <div className="flex items-center justify-between">
        <div className="text-xs text-charcoal-400 dark:text-steel-600 font-medium">
          {unit}
        </div>
        {tolerance && (
          <div className="text-xs text-amber-600 dark:text-amber-400 font-mono flex items-center gap-1">
            <Icon name="TrendingUp" size={10} />±{tolerance}%
          </div>
        )}
      </div>
    </div>
  );
};

ResultCard.propTypes = {
  label: PropTypes.string.isRequired,
  value: PropTypes.number.isRequired,
  unit: PropTypes.string.isRequired,
  formatted: PropTypes.string,
  tolerance: PropTypes.string,
  isCritical: PropTypes.bool,
};

ResultCard.defaultProps = {
  formatted: null,
  tolerance: null,
  isCritical: false,
};

export default ResultCard;
