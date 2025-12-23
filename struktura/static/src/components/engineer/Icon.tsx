/**
 * Icon Component
 * 
 * A type-safe wrapper around lucide-react icons.
 * Because importing icons one by one is tedious,
 * and magic strings without autocomplete are barbaric.
 * 
 * Usage:
 *   <Icon name="Calculator" size={24} className="text-red-600" />
 * 
 * Note: Only includes icons actually used in engineer components.
 * Extend as needed, but don't import the entire library.
 * Bundle size is a virtue.
 */

import {
  Activity,
  AlertCircle,
  AlertTriangle,
  ArrowRight,
  BookOpen,
  Box,
  BoxSelect,
  Calculator,
  CheckCircle2,
  ChevronDown,
  ChevronUp,
  Columns,
  Dot,
  FileText,
  Grid3x3,
  Loader2,
  Mountain,
  Search,
  Shield,
  Wind,
  type LucideIcon,
} from 'lucide-react';

// ============================================================================
// Icon Registry - The Single Source of Truth
// ============================================================================

/**
 * Map of icon names to their lucide-react components.
 * Add new icons here as needed, maintain alphabetical order
 * because we're civilized people.
 */
const ICON_REGISTRY = {
  Activity,
  AlertCircle,
  AlertTriangle,
  ArrowRight,
  BookOpen,
  Box,
  BoxSelect,
  Calculator,
  CheckCircle2,
  ChevronDown,
  ChevronUp,
  Columns,
  Dot,
  FileText,
  Grid3x3,
  Loader2,
  Mountain,
  Search,
  Shield,
  Wind,
} as const;

// ============================================================================
// Type Definitions - Autocomplete Is A Human Right
// ============================================================================

/**
 * Valid icon names - derived from registry keys
 * TypeScript will yell at you if you typo an icon name
 */
export type IconName = keyof typeof ICON_REGISTRY;

/**
 * Icon component props
 */
export interface IconProps {
  /** Icon name from the registry */
  name: IconName;
  /** Size in pixels (both width and height) */
  size?: number;
  /** Additional CSS classes */
  className?: string;
  /** Stroke width (lucide default is 2) */
  strokeWidth?: number;
  /** Aria label for accessibility */
  'aria-label'?: string;
}

// ============================================================================
// Icon Component - Simple But Effective
// ============================================================================

/**
 * Icon component with type-safe name selection
 * 
 * @example
 * <Icon name="Calculator" size={24} />
 * <Icon name="AlertCircle" size={16} className="text-red-600" />
 * <Icon name="Loader2" size={20} className="animate-spin" />
 */
const Icon: React.FC<IconProps> = ({
  name,
  size = 24,
  className = '',
  strokeWidth = 2,
  'aria-label': ariaLabel,
}) => {
  const IconComponent = ICON_REGISTRY[name] as LucideIcon;

  // Defensive programming - should never happen with TypeScript
  if (!IconComponent) {
    console.warn(`Icon "${name}" not found in registry`);
    return null;
  }

  return (
    <IconComponent
      size={size}
      className={className}
      strokeWidth={strokeWidth}
      aria-label={ariaLabel || name}
      aria-hidden={!ariaLabel}
    />
  );
};

export default Icon;

// ============================================================================
// Convenience Exports
// ============================================================================

/**
 * Export all available icon names for documentation/testing
 */
export const AVAILABLE_ICONS = Object.keys(ICON_REGISTRY) as IconName[];

/**
 * Check if an icon name is valid
 * Useful for dynamic icon selection
 */
export function isValidIconName(name: string): name is IconName {
  return name in ICON_REGISTRY;
}

/**
 * Get icon component directly if needed
 * (Escape hatch for advanced use cases)
 */
export function getIconComponent(name: IconName): LucideIcon {
  return ICON_REGISTRY[name] as LucideIcon;
}