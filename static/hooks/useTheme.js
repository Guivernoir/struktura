import {
  useState,
  useEffect,
  useCallback,
  createContext,
  useContext,
} from "react";

// ============================================
// THEME CONTEXT
// ============================================
const ThemeContext = createContext(null);

// ============================================
// THEME CONSTANTS
// ============================================
const THEME_STORAGE_KEY = "calcobra-theme";
const THEMES = {
  LIGHT: "light",
  DARK: "dark",
  SYSTEM: "system",
};

// ============================================
// THEME UTILITIES
// ============================================

/**
 * Get system theme preference
 * @returns {'light' | 'dark'}
 */
const getSystemTheme = () => {
  if (typeof window === "undefined") return THEMES.LIGHT;
  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? THEMES.DARK
    : THEMES.LIGHT;
};

/**
 * Get stored theme preference with fallback
 * @returns {'light' | 'dark' | 'system'}
 */
const getStoredTheme = () => {
  if (typeof window === "undefined") return THEMES.SYSTEM;

  try {
    const stored = localStorage.getItem(THEME_STORAGE_KEY);
    if (stored && Object.values(THEMES).includes(stored)) {
      return stored;
    }
  } catch (error) {
    console.warn("Failed to read theme from localStorage:", error);
  }

  return THEMES.SYSTEM;
};

/**
 * Store theme preference
 * @param {string} theme
 */
const storeTheme = (theme) => {
  if (typeof window === "undefined") return;

  try {
    localStorage.setItem(THEME_STORAGE_KEY, theme);
  } catch (error) {
    console.warn("Failed to save theme to localStorage:", error);
  }
};

/**
 * Resolve actual theme to apply (handles 'system' option)
 * @param {string} themePreference - User's theme preference
 * @returns {'light' | 'dark'}
 */
const resolveTheme = (themePreference) => {
  if (themePreference === THEMES.SYSTEM) {
    return getSystemTheme();
  }
  return themePreference;
};

/**
 * Apply theme to document
 * @param {'light' | 'dark'} theme
 */
const applyTheme = (theme) => {
  if (typeof document === "undefined") return;

  const root = document.documentElement;

  if (theme === THEMES.DARK) {
    root.classList.add("dark");
  } else {
    root.classList.remove("dark");
  }

  // Set data attribute for CSS targeting
  root.setAttribute("data-theme", theme);

  // Set meta theme-color for mobile browsers
  const metaThemeColor = document.querySelector('meta[name="theme-color"]');
  if (metaThemeColor) {
    metaThemeColor.setAttribute(
      "content",
      theme === THEMES.DARK ? "#111827" : "#ffffff"
    );
  } else {
    const meta = document.createElement("meta");
    meta.name = "theme-color";
    meta.content = theme === THEMES.DARK ? "#111827" : "#ffffff";
    document.head.appendChild(meta);
  }
};

// ============================================
// THEME PROVIDER
// ============================================

/**
 * Theme Provider Component
 * Manages theme state and system preference synchronization
 */
export const ThemeProvider = ({ children, defaultTheme = THEMES.SYSTEM }) => {
  const [themePreference, setThemePreference] = useState(() => {
    const stored = getStoredTheme();
    return stored || defaultTheme;
  });

  const [resolvedTheme, setResolvedTheme] = useState(() =>
    resolveTheme(themePreference)
  );

  // Apply theme to DOM whenever it changes
  useEffect(() => {
    applyTheme(resolvedTheme);
  }, [resolvedTheme]);

  // Listen for system theme changes when preference is 'system'
  useEffect(() => {
    if (themePreference !== THEMES.SYSTEM) return;

    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

    const handleChange = (e) => {
      const newSystemTheme = e.matches ? THEMES.DARK : THEMES.LIGHT;
      setResolvedTheme(newSystemTheme);
    };

    // Modern browsers
    if (mediaQuery.addEventListener) {
      mediaQuery.addEventListener("change", handleChange);
      return () => mediaQuery.removeEventListener("change", handleChange);
    }
    // Legacy browsers
    else if (mediaQuery.addListener) {
      mediaQuery.addListener(handleChange);
      return () => mediaQuery.removeListener(handleChange);
    }
  }, [themePreference]);

  // Update resolved theme when preference changes
  useEffect(() => {
    const newResolvedTheme = resolveTheme(themePreference);
    setResolvedTheme(newResolvedTheme);
    storeTheme(themePreference);
  }, [themePreference]);

  // Set theme (light, dark, or system)
  const setTheme = useCallback((newTheme) => {
    if (!Object.values(THEMES).includes(newTheme)) {
      console.warn(
        `Invalid theme: ${newTheme}. Must be one of: ${Object.values(
          THEMES
        ).join(", ")}`
      );
      return;
    }
    setThemePreference(newTheme);
  }, []);

  // Toggle between light and dark (preserves system if active)
  const toggleTheme = useCallback(() => {
    setThemePreference((prev) => {
      // If on system, switch to opposite of current resolved theme
      if (prev === THEMES.SYSTEM) {
        return resolvedTheme === THEMES.DARK ? THEMES.LIGHT : THEMES.DARK;
      }
      // Otherwise toggle between light and dark
      return prev === THEMES.DARK ? THEMES.LIGHT : THEMES.DARK;
    });
  }, [resolvedTheme]);

  // Cycle through all theme options (light → dark → system)
  const cycleTheme = useCallback(() => {
    setThemePreference((prev) => {
      if (prev === THEMES.LIGHT) return THEMES.DARK;
      if (prev === THEMES.DARK) return THEMES.SYSTEM;
      return THEMES.LIGHT;
    });
  }, []);

  const value = {
    // Current preference (light, dark, or system)
    theme: themePreference,
    // Actual applied theme (light or dark)
    resolvedTheme,
    // Theme setters
    setTheme,
    toggleTheme,
    cycleTheme,
    // Theme checks
    isDark: resolvedTheme === THEMES.DARK,
    isLight: resolvedTheme === THEMES.LIGHT,
    isSystem: themePreference === THEMES.SYSTEM,
    // Constants
    themes: THEMES,
  };

  return (
    <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>
  );
};

// ============================================
// THEME HOOK
// ============================================

/**
 * Hook to access theme context
 * @returns {Object} Theme context value
 * @throws {Error} If used outside ThemeProvider
 */
export const useTheme = () => {
  const context = useContext(ThemeContext);

  if (!context) {
    throw new Error(
      "useTheme must be used within a ThemeProvider. " +
        "Wrap your component tree with <ThemeProvider>.</ThemeProvider>"
    );
  }

  return context;
};

// ============================================
// THEME HOOK WITH SSR SUPPORT
// ============================================

/**
 * Hook with SSR-safe defaults
 * Useful for Next.js or other SSR frameworks
 */
export const useThemeWithSSR = () => {
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  const theme = useTheme();

  // Return safe defaults during SSR
  if (!mounted) {
    return {
      ...theme,
      resolvedTheme: THEMES.LIGHT,
      isDark: false,
      isLight: true,
    };
  }

  return theme;
};

// ============================================
// EXPORTS
// ============================================
export { THEMES };
export default useTheme;
