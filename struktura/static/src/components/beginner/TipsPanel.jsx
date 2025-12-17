import PropTypes from 'prop-types';
import Icon from '../Icon';

const TipsPanel = ({ t }) => {
  const getLabel = (key) => {
    return key.split('.').reduce((obj, prop) => 
      (obj && obj[prop] !== undefined) ? obj[prop] : key, 
      t
    ) || key;
  };

  const tips = [
    getLabel('beginner.tips.local_codes'),
    getLabel('beginner.tips.seasonal_pricing'),
    getLabel('beginner.tips.material_waste'),
  ];

  return (
    <div className="p-6 bg-white dark:bg-charcoal-900 rounded-2xl shadow-lg border border-sand-200 dark:border-charcoal-800">
      <h3 className="text-lg font-semibold mb-4 text-charcoal-900 dark:text-white flex items-center gap-2">
        <Icon name="Lightbulb" size={20} className="text-sand-600 dark:text-sand-400" />
        {getLabel('beginner.tips.title')}
      </h3>
      <ul className="space-y-2 text-charcoal-600 dark:text-steel-400">
        {tips.map((tip, index) => (
          <li key={index} className="flex gap-2">
            <span className="text-blue-500 font-bold">•</span>
            <span>{tip}</span>
          </li>
        ))}
      </ul>
    </div>
  );
};

TipsPanel.propTypes = {
  t: PropTypes.object.isRequired,
};

export default TipsPanel;