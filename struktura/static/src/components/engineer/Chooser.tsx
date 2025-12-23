/**
 * Calculator Chooser Component
 * 
 * A visual selector for engineering calculators.
 * Filters by category, shows complexity, looks professional.
 * 
 * Design philosophy: Users should see all options immediately,
 * not hunt through dropdowns like it's 2005.
 */

import { useState, useMemo } from "react";
import { CalculatorId, CalculatorDefinition } from "../../pages/EngineerCalculator";
import Icon from "./Icon";

// ============================================================================
// Type Definitions
// ============================================================================

interface CalculatorChooserProps {
  calculators: CalculatorDefinition[];
  onSelect: (id: CalculatorId) => void;
  getLabel: (key: string) => string;
}

// ============================================================================
// Category Configuration - The Organizational Schema
// ============================================================================

const CATEGORIES = [
  { id: 'all', nameKey: 'engineer.categories.all', icon: 'Grid3x3' },
  { id: 'production', nameKey: 'engineer.categories.production', icon: 'Activity' },
  { id: 'structural', nameKey: 'engineer.categories.structural', icon: 'Columns' },
  { id: 'materials', nameKey: 'engineer.categories.materials', icon: 'Box' },
  { id: 'geotechnical', nameKey: 'engineer.categories.geotechnical', icon: 'Mountain' },
  { id: 'mechanical', nameKey: 'engineer.categories.mechanical', icon: 'Wind' },
] as const;

/**
 * Complexity badge configuration
 * Color-coded because humans are visual creatures
 */
const COMPLEXITY_STYLES = {
  basic: {
    bg: 'bg-green-100 dark:bg-green-950/50',
    text: 'text-green-800 dark:text-green-300',
    border: 'border-green-200 dark:border-green-800',
    label: 'Basic',
  },
  intermediate: {
    bg: 'bg-amber-100 dark:bg-amber-950/50',
    text: 'text-amber-800 dark:text-amber-300',
    border: 'border-amber-200 dark:border-amber-800',
    label: 'Intermediate',
  },
  advanced: {
    bg: 'bg-red-100 dark:bg-red-950/50',
    text: 'text-red-800 dark:text-red-300',
    border: 'border-red-200 dark:border-red-800',
    label: 'Advanced',
  },
} as const;

// ============================================================================
// Main Component
// ============================================================================

const CalculatorChooser: React.FC<CalculatorChooserProps> = ({
  calculators,
  onSelect,
  getLabel,
}) => {
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const [searchQuery, setSearchQuery] = useState<string>('');

  /**
   * Filter calculators by category and search query
   * Memoized because we're not savages who re-filter on every render
   */
  const filteredCalculators = useMemo(() => {
    let filtered = calculators;

    // Category filter
    if (selectedCategory !== 'all') {
      filtered = filtered.filter(calc => calc.category === selectedCategory);
    }

    // Search filter
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(calc =>
        getLabel(calc.nameKey).toLowerCase().includes(query) ||
        getLabel(calc.descriptionKey).toLowerCase().includes(query)
      );
    }

    return filtered;
  }, [calculators, selectedCategory, searchQuery, getLabel]);

  return (
    <div className="space-y-8">
      {/* Search Bar - Because Finding Things Should Be Easy */}
      <div className="max-w-2xl mx-auto">
        <div className="relative">
          <Icon
            name="Search"
            size={20}
            className="absolute left-4 top-1/2 -translate-y-1/2 text-charcoal-400 dark:text-steel-500"
          />
          <input
            type="text"
            placeholder="Search calculators..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-12 pr-4 py-3 bg-white dark:bg-charcoal-900 border border-sand-200 dark:border-charcoal-800 rounded-xl text-charcoal-900 dark:text-white placeholder-charcoal-400 dark:placeholder-steel-500 focus:outline-none focus:ring-2 focus:ring-red-500 focus:border-transparent transition-all"
          />
        </div>
      </div>

      {/* Category Tabs - Visual Filtering */}
      <div className="flex flex-wrap gap-2 justify-center">
        {CATEGORIES.map(category => (
          <button
            key={category.id}
            onClick={() => setSelectedCategory(category.id)}
            className={`
              inline-flex items-center gap-2 px-4 py-2 rounded-lg font-medium text-sm transition-all
              ${selectedCategory === category.id
                ? 'bg-red-600 text-white shadow-lg shadow-red-500/30'
                : 'bg-white dark:bg-charcoal-900 text-charcoal-700 dark:text-steel-300 border border-sand-200 dark:border-charcoal-800 hover:border-red-300 dark:hover:border-red-700'
              }
            `}
          >
            <Icon
              name={category.icon}
              size={16}
            />
            <span>{getLabel(category.nameKey)}</span>
          </button>
        ))}
      </div>

      {/* Results Count - Situational Awareness */}
      {searchQuery.trim() && (
        <div className="text-center text-sm text-charcoal-600 dark:text-steel-400">
          Found {filteredCalculators.length} calculator{filteredCalculators.length !== 1 ? 's' : ''}
        </div>
      )}

      {/* Calculator Grid - The Main Event */}
      {filteredCalculators.length > 0 ? (
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredCalculators.map(calculator => {
            const complexityStyle = COMPLEXITY_STYLES[calculator.complexity];
            
            return (
              <button
                key={calculator.id}
                onClick={() => onSelect(calculator.id)}
                className="group relative bg-white dark:bg-charcoal-900 rounded-xl border-2 border-sand-200 dark:border-charcoal-800 hover:border-red-500 dark:hover:border-red-500 p-6 text-left transition-all hover:shadow-xl hover:shadow-red-500/10 hover:-translate-y-1"
              >
                {/* Icon Badge */}
                <div className="mb-4">
                  <div className="w-12 h-12 bg-gradient-to-br from-red-500 to-rose-600 rounded-lg flex items-center justify-center group-hover:scale-110 transition-transform">
                    <Icon
                      name={calculator.icon}
                      size={24}
                      className="text-white"
                    />
                  </div>
                </div>

                {/* Calculator Name */}
                <h3 className="text-lg font-bold text-charcoal-900 dark:text-white mb-2 group-hover:text-red-600 dark:group-hover:text-red-400 transition-colors">
                  {getLabel(calculator.nameKey)}
                </h3>

                {/* Description */}
                <p className="text-sm text-charcoal-600 dark:text-steel-400 mb-4 line-clamp-2">
                  {getLabel(calculator.descriptionKey)}
                </p>

                {/* Complexity Badge */}
                <div className="flex items-center gap-2">
                  <span className={`
                    inline-flex items-center px-2 py-1 rounded text-xs font-medium border
                    ${complexityStyle.bg} ${complexityStyle.text} ${complexityStyle.border}
                  `}>
                    {complexityStyle.label}
                  </span>
                </div>

                {/* Hover Arrow */}
                <div className="absolute top-6 right-6 opacity-0 group-hover:opacity-100 transition-opacity">
                  <Icon
                    name="ArrowRight"
                    size={20}
                    className="text-red-600 dark:text-red-400"
                  />
                </div>
              </button>
            );
          })}
        </div>
      ) : (
        // Empty State - When The Search Fails
        <div className="text-center py-12">
          <div className="w-16 h-16 mx-auto mb-4 bg-sand-100 dark:bg-charcoal-800 rounded-full flex items-center justify-center">
            <Icon
              name="Search"
              size={32}
              className="text-charcoal-400 dark:text-steel-500"
            />
          </div>
          <h3 className="text-xl font-semibold text-charcoal-900 dark:text-white mb-2">
            No calculators found
          </h3>
          <p className="text-charcoal-600 dark:text-steel-400">
            Try adjusting your search or category filter
          </p>
        </div>
      )}
    </div>
  );
};

export default CalculatorChooser;