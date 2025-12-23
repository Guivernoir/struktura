# OEE Calculator Styles

CSS files that complement Tailwind utility classes for the OEE calculator component.

## Files Overview

- **`variables.css`** - CSS custom properties (design tokens)
- **`typography.css`** - Text styling and typography utilities
- **`animations.css`** - Keyframe animations and transitions
- **`oee.css`** - OEE-specific component styles
- **`index.css`** - Main entry point (imports all other files)

## Installation

Import the styles in your main component:

```typescript
// In OeeEngine.tsx or your root component
import "./styles/index.css";
```

Or import individual files as needed:

```typescript
import "./styles/variables.css";
import "./styles/oee.css";
```

## Design System

### Color Palette

The design uses three main color families from `tailwind.config.ts`:

**Sand (Warm Neutrals)**

- Light backgrounds and subtle elements
- Values: `--color-sand-50` through `--color-sand-900`

**Steel (Cool Neutrals)**

- Interactive elements and accents
- Values: `--color-steel-50` through `--color-steel-900`

**Charcoal (Dark Neutrals)**

- Text and primary UI elements
- Values: `--color-charcoal-50` through `--color-charcoal-950`

### OEE-Specific Semantic Colors

```css
/* OEE Value Ranges */
--color-oee-world-class: #10b981; /* >= 85% */
--color-oee-acceptable: #f59e0b; /* 60-84% */
--color-oee-needs-improvement: #ef4444; /* < 60% */

/* Metric Colors */
--color-availability: #3b82f6; /* Blue */
--color-performance: #8b5cf6; /* Purple */
--color-quality: #10b981; /* Green */

/* Confidence Levels */
--color-confidence-high: #10b981;
--color-confidence-medium: #f59e0b;
--color-confidence-low: #ef4444;

/* Input Source Types */
--color-source-explicit: #3b82f6; /* User provided */
--color-source-inferred: #8b5cf6; /* Calculated */
--color-source-default: #6b7280; /* System default */
```

### Typography

**Fonts:**

- Body: Inter (via `--font-sans`)
- Headings: Space Grotesk (via `--font-display`)
- Code/Monospace: System monospace stack

**Special Classes:**

```css
.metric-value        /* Large, prominent numbers */
/* Large, prominent numbers */
.metric-label        /* Uppercase metric labels */
.percentage          /* Tabular numbers for percentages */
.duration            /* Monospace duration display */
.formula; /* Formula display with background */
```

### Spacing

Uses a consistent spacing scale:

```css
--spacing-xs:  0.25rem  /* 4px */
--spacing-sm:  0.5rem   /* 8px */
--spacing-md:  1rem     /* 16px */
--spacing-lg:  1.5rem   /* 24px */
--spacing-xl:  2rem     /* 32px */
--spacing-2xl: 3rem     /* 48px */
--spacing-3xl: 4rem     /* 64px */
```

### Shadows

Three shadow levels:

```css
--shadow-soft    /* Subtle elevation */
--shadow-medium  /* Moderate elevation */
--shadow-hard    /* Strong elevation */
```

### Border Radius

```css
--radius-sm:   0.25rem   /* 4px */
--radius-md:   0.375rem  /* 6px */
--radius-lg:   0.5rem    /* 8px */
--radius-xl:   0.75rem   /* 12px */
--radius-2xl:  1rem      /* 16px */
--radius-full: 9999px    /* Fully rounded */
```

## Component Classes

### Metric Cards

```tsx
<div className="oee-metric-card oee-metric-card--world-class">
  <div className="metric-label">OEE</div>
  <div className="metric-value">87.5%</div>
</div>
```

Variants:

- `.oee-metric-card--world-class` - Green accent (OEE >= 85%)
- `.oee-metric-card--acceptable` - Amber accent (OEE 60-84%)
- `.oee-metric-card--needs-improvement` - Red accent (OEE < 60%)

### Source Pills

```tsx
<span className="source-pill source-pill--explicit">Explicit</span>
```

Variants:

- `.source-pill--explicit` - Blue (user provided)
- `.source-pill--inferred` - Purple (calculated)
- `.source-pill--default` - Gray (system default)

### Confidence Badges

```tsx
<span className="confidence-badge confidence-badge--high">High Confidence</span>
```

Variants:

- `.confidence-badge--high` - Green
- `.confidence-badge--medium` - Amber
- `.confidence-badge--low` - Red

### Loss Tree

```tsx
<div className="loss-tree">
  <div className="loss-tree-node loss-tree-node--availability">
    <span>Breakdowns</span>
  </div>
  <div className="loss-tree-children">{/* Child nodes */}</div>
</div>
```

Variants:

- `.loss-tree-node--availability` - Red accent
- `.loss-tree-node--performance` - Amber accent
- `.loss-tree-node--quality` - Yellow accent

### Assumption Entries

```tsx
<div className="assumption-entry assumption-entry--critical">
  <div className="source-label">Explicit</div>
  <div>Planned Time: 28800s</div>
</div>
```

Variants:

- `.assumption-entry--critical` - Red accent (high impact)
- `.assumption-entry--high` - Amber accent
- `.assumption-entry--medium` - Blue accent

### Validation Warnings

```tsx
<div className="validation-warning">
  <div className="warning-text">Scrap rate is elevated</div>
</div>
```

Types:

- `.validation-warning` - Amber (warnings)
- `.validation-error` - Red (errors)
- `.validation-info` - Blue (info)

### Economic Impact

```tsx
<div className="economic-impact">
  <div>Throughput Loss</div>
  <div className="economic-range">
    <span className="economic-range-low">$10,000</span>
    <span className="economic-range-central">$12,500</span>
    <span className="economic-range-high">$15,000</span>
  </div>
</div>
```

### Progress Bars

```tsx
<div className="oee-progress-bar">
  <div
    className="oee-progress-fill oee-progress-fill--world-class"
    style={{ width: "87%" }}
  />
</div>
```

## Animations

### Built-in Animations

```tsx
<div className="animate-fade-in">Fades in</div>
<div className="animate-slide-up">Slides up</div>
<div className="animate-float">Floats gently</div>
```

Available animations:

- `.animate-fade-in` - Fade in
- `.animate-slide-up` - Slide up from bottom
- `.animate-slide-down` - Slide down from top
- `.animate-slide-left` - Slide from right
- `.animate-slide-right` - Slide from left
- `.animate-float` - Gentle floating
- `.animate-pulse` - Pulsing opacity
- `.animate-spin` - Spinning rotation
- `.animate-bounce` - Bouncing

### OEE-Specific Animations

```tsx
<div className="animate-metric-update">Updates with bounce</div>
<div className="animate-calculating">Calculating pulse</div>
<div className="animate-expand-loss">Loss tree expand</div>
<div className="animate-highlight-assumption">Highlights briefly</div>
<div className="animate-warning-pulse">Warning pulse</div>
```

### Transition Classes

```tsx
<div className="transition-colors hover-lift">Lifts on hover</div>
```

Utilities:

- `.transition-fast` - 150ms
- `.transition-base` - 200ms
- `.transition-slow` - 300ms
- `.transition-slower` - 500ms
- `.transition-colors` - Color transitions
- `.transition-transform` - Transform transitions
- `.hover-lift` - Lifts on hover
- `.hover-glow` - Glows on hover

## Loading States

### Spinner

```tsx
<div className="oee-spinner" />
```

### Loading Text

```tsx
<div className="oee-loading-text">
  <div className="oee-spinner" />
  <span>Calculating...</span>
</div>
```

### Skeleton

```tsx
<div className="skeleton" style={{ height: "40px", width: "200px" }} />
```

## Empty States

```tsx
<div className="oee-empty-state">
  <div className="oee-empty-state-icon">📊</div>
  <p>No data available</p>
</div>
```

## Dark Mode

All styles automatically adapt to dark mode via the `.dark` class on a parent element:

```tsx
<div className="dark">{/* All styles automatically adjust */}</div>
```

Dark mode uses:

- Inverted backgrounds (charcoal palette)
- Adjusted shadows (darker)
- Maintained semantic colors
- Proper contrast ratios

## Responsive Design

Styles automatically adjust for mobile:

- Metric cards reduce padding
- Loss tree reduces indentation
- Grid layouts become single column
- Font sizes scale appropriately

## Print Styles

Special print optimizations:

- Removes shadows
- Adjusts borders
- Prevents page breaks inside cards
- Hides non-printable elements (use `.no-print` class)

## Customization

### Overriding Variables

Override CSS custom properties in your own CSS:

```css
:root {
  --color-oee-world-class: #your-color;
  --card-padding: 2rem;
}
```

### Extending Styles

Extend component styles:

```css
.oee-metric-card {
  /* Your additional styles */
}

.oee-metric-card--custom {
  /* Your variant */
}
```

## Best Practices

### Use Semantic Classes

✅ **Good:**

```tsx
<div className="oee-metric-card oee-metric-card--world-class">
```

❌ **Avoid:**

```tsx
<div style={{ border: '1px solid green', padding: '16px' }}>
```

### Combine with Tailwind

These styles work alongside Tailwind:

```tsx
<div className="oee-metric-card flex items-center gap-4">
  {/* OEE class + Tailwind utilities */}
</div>
```

### Prefer Variables

✅ **Good:**

```tsx
<div style={{ padding: 'var(--spacing-md)' }}>
```

❌ **Avoid:**

```tsx
<div style={{ padding: '16px' }}>
```

### Use Animation Classes

✅ **Good:**

```tsx
<div className="animate-slide-up">
```

❌ **Avoid:**

```tsx
<div style={{ animation: 'slideUp 0.6s ease-out' }}>
```

## Troubleshooting

### Styles Not Applying

1. Ensure `index.css` is imported
2. Check import order (variables must be first)
3. Verify Tailwind isn't purging these classes (add to `safelist` in config)

### Dark Mode Not Working

1. Ensure `.dark` class is on a parent element
2. Check Tailwind's `darkMode` config is set to `'class'`
3. Verify dark mode variables are defined

### Colors Not Matching

1. Ensure variables.css is imported first
2. Check that Tailwind config matches CSS variables
3. Verify browser supports CSS custom properties

## Integration with Tailwind

These styles complement Tailwind:

**Tailwind Config → CSS Variables:**

```javascript
// tailwind.config.ts
colors: {
  sand: { 50: '#faf9f7', ... }
}

// Becomes:
// :root { --color-sand-50: #faf9f7; }
```

**Usage:**

```tsx
{
  /* Tailwind utilities */
}
<div className="flex gap-4 bg-sand-50 dark:bg-charcoal-900">
  {/* OEE component classes */}
  <div className="oee-metric-card">{/* Mix both */}</div>
</div>;
```

---

**Remember:** These styles are designed to work _with_ Tailwind, not replace it. Use Tailwind for layout and spacing, these styles for complex component patterns.
