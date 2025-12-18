import PropTypes from "prop-types";
import ThreeDView from "../ThreeDView";

const ContractorVisualizationPanel = ({ t, formData, theme }) => {
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
    width: formData.dimensions?.width || 10,
    length: formData.dimensions?.length || 10,
    height: formData.dimensions?.height || 3,
    material: formData.material?.material_type?.toLowerCase() || "concrete",
  };

  return (
    <div className="flex flex-col h-full p-4 bg-white dark:bg-charcoal-900 rounded-2xl shadow-xl border border-sand-200 dark:border-charcoal-800">
      <div className="mb-4">
        <h2 className="text-xl font-semibold text-charcoal-900 dark:text-white">
          {getLabel("contractor.visualization.panel_title") ||
            "Project Visualization"}
        </h2>
        <p className="text-sm text-charcoal-500 dark:text-steel-400">
          {t?.engineer?.visualization?.preview_subtitle ||
            "3D preview of project dimensions"}
        </p>
      </div>

      <div className="flex-grow w-full relative min-h-[300px]">
        <ThreeDView dimensions={dimFor3D} theme={theme} />
      </div>

      {/* Project Stats */}
      <div className="mt-4 pt-4 border-t border-sand-200 dark:border-charcoal-800 grid grid-cols-3 gap-3">
        <div className="text-center">
          <div className="text-xs text-charcoal-500 dark:text-steel-500 mb-1">
            {t?.engineer?.visualization?.area || "Area"}
          </div>
          <div className="text-sm font-bold text-charcoal-900 dark:text-white">
            {(dimFor3D.width * dimFor3D.length).toFixed(1)} m²
          </div>
        </div>
        <div className="text-center">
          <div className="text-xs text-charcoal-500 dark:text-steel-500 mb-1">
            {t?.engineer?.visualization?.volume || "Volume"}
          </div>
          <div className="text-sm font-bold text-charcoal-900 dark:text-white">
            {(dimFor3D.width * dimFor3D.length * dimFor3D.height).toFixed(1)} m³
          </div>
        </div>
        <div className="text-center">
          <div className="text-xs text-charcoal-500 dark:text-steel-500 mb-1">
            {t?.engineer?.visualization?.material || "Material"}
          </div>
          <div className="text-sm font-bold text-charcoal-900 dark:text-white capitalize">
            {dimFor3D.material}
          </div>
        </div>
      </div>
    </div>
  );
};

ContractorVisualizationPanel.propTypes = {
  t: PropTypes.object.isRequired,
  formData: PropTypes.object.isRequired,
  theme: PropTypes.string.isRequired,
};

export default ContractorVisualizationPanel;
