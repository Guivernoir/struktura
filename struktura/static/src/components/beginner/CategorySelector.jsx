import PropTypes from "prop-types";
import Icon from "../Icon";

// Icon mapping for beginner categories
const CATEGORY_ICONS = {
  garden: "Seedling",
  interiors: "Home",
  outdoors: "TreePine",
  utilities: "Wrench",
};

const BeginnerCategorySelector = ({
  t,
  categories,
  selectedCategory,
  setSelectedCategory,
}) => {
  const getLabel = (categoryId) => {
    const key = `beginner.categories.${categoryId}`;
    const translated = t[key];
    return translated && translated !== key ? translated : null;
  };

  if (!categories.length) {
    return (
      <div className="flex overflow-x-auto pb-2 space-x-2">
        <div className="flex-shrink-0 px-6 py-2.5 rounded-full bg-green-100 dark:bg-green-900/30 w-36 animate-pulse" />
        <div className="flex-shrink-0 px-6 py-2.5 rounded-full bg-green-100 dark:bg-green-900/30 w-40 animate-pulse" />
        <div className="flex-shrink-0 px-6 py-2.5 rounded-full bg-green-100 dark:bg-green-900/30 w-32 animate-pulse" />
      </div>
    );
  }

  return (
    <div className="flex overflow-x-auto pb-2 space-x-3 scrollbar-hide">
      {categories.map((category) => {
        const displayName = getLabel(category.id) || category.name;

        return (
          <button
            key={category.id}
            onClick={() => setSelectedCategory(category.id)}
            className={`relative flex-shrink-0 flex items-center gap-2.5 px-5 py-3 rounded-full text-sm font-semibold transition-all duration-200 whitespace-nowrap
              ${
                selectedCategory === category.id
                  ? "bg-gradient-to-r from-green-600 to-emerald-600 text-white shadow-xl shadow-green-600/30"
                  : "bg-gradient-to-r from-gray-100 to-gray-200 dark:from-charcoal-800 dark:to-charcoal-700 text-charcoal-800 dark:text-steel-200 hover:shadow-md"
              }`}
            title={category.description}
            aria-pressed={selectedCategory === category.id}
          >
            <Icon name={CATEGORY_ICONS[category.id] || "Hammer"} size={20} />
            <span>{displayName}</span>
          </button>
        );
      })}
    </div>
  );
};

BeginnerCategorySelector.propTypes = {
  t: PropTypes.object.isRequired,
  categories: PropTypes.array.isRequired,
  selectedCategory: PropTypes.string.isRequired,
  setSelectedCategory: PropTypes.func.isRequired,
};

export default BeginnerCategorySelector;
