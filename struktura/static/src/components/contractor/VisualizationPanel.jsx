import PropTypes from "prop-types";
import ThreeDView from "../ThreeDView";

const VisualizationPanel = ({ t, formData, theme }) => {
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

  const dimFor3D = {
    width: formData.dimensions?.width || 1,
    length: formData.dimensions?.length || 1,
    height: formData.dimensions?.height || 0.1,
    material: formData.material?.material_type || "wood",
  };

  // Note: Parent component (ContractorCalculator) now handles the availability check.
  // This component assumes it is only rendered when a view is available.

  return (
    <div className="flex flex-col h-full p-4 bg-white dark:bg-charcoal-900 rounded-2xl shadow-xl border border-sand-200 dark:border-charcoal-800">
      <div className="mb-4">
        <h2 className="text-xl font-semibold text-charcoal-900 dark:text-white">
          {getLabel("beginner.visualization.title") || "Visualization"}
        </h2>
        <p className="text-sm text-charcoal-500 dark:text-steel-400">
          Rotate and zoom to inspect dimensions.
        </p>
      </div>

      <div className="flex-grow w-full relative min-h-[300px]">
        <ThreeDView dimensions={dimFor3D} theme={theme} />
      </div>
    </div>
  );
};

VisualizationPanel.propTypes = {
  t: PropTypes.object.isRequired,
  formData: PropTypes.object.isRequired, // Updated to object, since hook provides formData with nested dimensions/material
  theme: PropTypes.string.isRequired,
};

export default VisualizationPanel;
