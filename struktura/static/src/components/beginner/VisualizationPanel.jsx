import PropTypes from 'prop-types';
import ThreeDView from '../ThreeDView';

const VisualizationPanel = ({ t, formData, theme }) => {
  const getLabel = (key) => {
    return key.split('.').reduce((obj, prop) => 
      (obj && obj[prop] !== undefined) ? obj[prop] : key, 
      t
    ) || key;
  };

  // Safe formatting of dimensions
  const dimFor3D = {
    width: formData.width || 1,
    length: formData.length || 1,
    height: formData.height || 0.1,
    material: formData.material || 'wood',
  };

  // Define which calculator types support 3D
  const supportedTypes = ['deck', 'concrete_slab', 'planter_box', 'wall_framing', 'mulch_bed'];
  const is3DViewable = supportedTypes.includes(formData.type);

  return (
    <div className="flex flex-col h-full p-4 bg-white dark:bg-charcoal-900 rounded-2xl shadow-xl border border-sand-200 dark:border-charcoal-800">
      <div className="mb-4">
        <h2 className="text-xl font-semibold text-charcoal-900 dark:text-white">
            {getLabel('beginner.visualization.title') || 'Visualization'}
        </h2>
        <p className="text-sm text-charcoal-500 dark:text-steel-400">
            {is3DViewable 
                ? "Rotate and zoom to inspect dimensions." 
                : "3D preview not available for this item."}
        </p>
      </div>

      <div className="flex-grow w-full relative">
        {is3DViewable ? (
            <ThreeDView dimensions={dimFor3D} theme={theme} />
        ) : (
            <div className="w-full h-full min-h-[300px] flex items-center justify-center rounded-2xl bg-sand-50 dark:bg-charcoal-950/50 text-charcoal-400 dark:text-steel-600 border border-dashed border-sand-300 dark:border-charcoal-700">
                <div className="text-center p-6">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-12 h-12 mx-auto mb-3 opacity-50">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M21 7.5l-9-5.25L3 7.5m18 0l-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9" />
                    </svg>
                    <p>{getLabel('beginner.visualization.not_available') || "No visualization available"}</p>
                </div>
            </div>
        )}
      </div>
    </div>
  );
};

VisualizationPanel.propTypes = {
  t: PropTypes.object.isRequired,
  formData: PropTypes.shape({
    type: PropTypes.string,
    width: PropTypes.oneOfType([PropTypes.string, PropTypes.number]),
    length: PropTypes.oneOfType([PropTypes.string, PropTypes.number]),
    height: PropTypes.oneOfType([PropTypes.string, PropTypes.number]),
    material: PropTypes.string,
  }).isRequired,
  theme: PropTypes.string.isRequired,
};

export default VisualizationPanel;