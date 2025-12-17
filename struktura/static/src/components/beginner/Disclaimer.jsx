import PropTypes from 'prop-types';

const Disclaimer = ({ t }) => {
  const getLabel = (key) => {
    return key.split('.').reduce((obj, prop) => 
      (obj && obj[prop] !== undefined) ? obj[prop] : key, 
      t
    ) || key;
  };

  return (
    <div className="mt-12 p-4 bg-sand-50 dark:bg-charcoal-900 rounded-xl border border-sand-200 dark:border-charcoal-800">
      <p className="text-xs text-center text-charcoal-500 dark:text-steel-500">
        <strong>{getLabel('disclaimer.title')}:</strong> {getLabel('disclaimer.text')}
      </p>
    </div>
  );
};

Disclaimer.propTypes = {
  t: PropTypes.object.isRequired,
};

export default Disclaimer;