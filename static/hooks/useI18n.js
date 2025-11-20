import {
  useState,
  useEffect,
  useCallback,
  createContext,
  useContext,
  useMemo,
} from "react";

// ============================================
// I18N CONTEXT
// ============================================
const I18nContext = createContext(null);

// ============================================
// I18N CONSTANTS
// ============================================
const LANGUAGE_STORAGE_KEY = "calcobra-language";

const SUPPORTED_LANGUAGES = {
  "pt-BR": {
    code: "pt-BR",
    name: "Português",
    nativeName: "Português (Brasil)",
    flag: "🇧🇷",
  },
  en: { code: "en", name: "English", nativeName: "English", flag: "🇺🇸" },
  es: { code: "es", name: "Spanish", nativeName: "Español", flag: "🇪🇸" },
  fr: { code: "fr", name: "French", nativeName: "Français", flag: "🇫🇷" },
  it: { code: "it", name: "Italian", nativeName: "Italiano", flag: "🇮🇹" },
  de: { code: "de", name: "German", nativeName: "Deutsch", flag: "🇩🇪" },
};

const DEFAULT_LANGUAGE = "en";
const FALLBACK_LANGUAGE = "en";

// ============================================
// I18N UTILITIES
// ============================================

/**
 * Detect browser language with intelligent fallbacks
 * @returns {string} Language code
 */
const detectBrowserLanguage = () => {
  if (typeof navigator === "undefined") return DEFAULT_LANGUAGE;

  // Get all potential language indicators
  const browserLang = navigator.language || navigator.userLanguage;
  const browserLangs = navigator.languages || [browserLang];

  // Try exact match first
  for (const lang of browserLangs) {
    if (SUPPORTED_LANGUAGES[lang]) {
      return lang;
    }
  }

  // Try language prefix match (e.g., 'pt-PT' → 'pt-BR')
  for (const lang of browserLangs) {
    const prefix = lang.split("-")[0];

    // Special handling for Portuguese variants
    if (prefix === "pt") return "pt-BR";

    // Check if we have any language with this prefix
    const matchingLang = Object.keys(SUPPORTED_LANGUAGES).find((supported) =>
      supported.startsWith(prefix)
    );
    if (matchingLang) return matchingLang;
  }

  return DEFAULT_LANGUAGE;
};

/**
 * Get stored language preference
 * @returns {string | null}
 */
const getStoredLanguage = () => {
  if (typeof window === "undefined") return null;

  try {
    const stored = localStorage.getItem(LANGUAGE_STORAGE_KEY);
    if (stored && SUPPORTED_LANGUAGES[stored]) {
      return stored;
    }
  } catch (error) {
    console.warn("Failed to read language from localStorage:", error);
  }

  return null;
};

/**
 * Store language preference
 * @param {string} language
 */
const storeLanguage = (language) => {
  if (typeof window === "undefined") return;

  try {
    localStorage.setItem(LANGUAGE_STORAGE_KEY, language);
  } catch (error) {
    console.warn("Failed to save language to localStorage:", error);
  }
};

/**
 * Apply language to document
 * @param {string} language
 */
const applyLanguage = (language) => {
  if (typeof document === "undefined") return;

  // Set html lang attribute for accessibility and SEO
  document.documentElement.lang = language;

  // Set dir attribute for RTL languages (future-proofing)
  const rtlLanguages = ["ar", "he", "fa", "ur"];
  const isRTL = rtlLanguages.some((rtl) => language.startsWith(rtl));
  document.documentElement.dir = isRTL ? "rtl" : "ltr";
};

/**
 * Load translation file dynamically
 * @param {string} language
 * @param {Object} translations - Pre-loaded translations object
 * @returns {Promise<Object>}
 */
const loadTranslations = async (language, translations) => {
  // If translations are pre-loaded (bundled), use them
  if (translations && translations[language]) {
    return translations[language];
  }

  // Otherwise, try to fetch dynamically (for code-splitting scenarios)
  try {
    const module = await import(`./i18n/${language}.json`);
    return module.default || module;
  } catch (error) {
    console.warn(`Failed to load translations for ${language}:`, error);

    // Fallback to default language
    if (language !== FALLBACK_LANGUAGE && translations?.[FALLBACK_LANGUAGE]) {
      return translations[FALLBACK_LANGUAGE];
    }

    return {};
  }
};

// ============================================
// I18N PROVIDER
// ============================================

/**
 * I18n Provider Component
 * Manages language state and translation loading
 *
 * @param {Object} props
 * @param {React.ReactNode} props.children
 * @param {Object} props.translations - Pre-loaded translations object
 * @param {string} props.defaultLanguage - Default language if no preference
 */
export const I18nProvider = ({
  children,
  translations: preloadedTranslations = {},
  defaultLanguage,
}) => {
  const [language, setLanguageState] = useState(() => {
    const stored = getStoredLanguage();
    if (stored) return stored;

    if (defaultLanguage && SUPPORTED_LANGUAGES[defaultLanguage]) {
      return defaultLanguage;
    }

    return detectBrowserLanguage();
  });

  const [translations, setTranslations] = useState(
    () => preloadedTranslations[language] || {}
  );

  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  // Load translations when language changes
  useEffect(() => {
    let isMounted = true;

    const loadLanguage = async () => {
      setIsLoading(true);
      setError(null);

      try {
        const newTranslations = await loadTranslations(
          language,
          preloadedTranslations
        );

        if (isMounted) {
          setTranslations(newTranslations);
          applyLanguage(language);
          storeLanguage(language);
        }
      } catch (err) {
        if (isMounted) {
          setError(err);
          console.error("Failed to load language:", err);
        }
      } finally {
        if (isMounted) {
          setIsLoading(false);
        }
      }
    };

    loadLanguage();

    return () => {
      isMounted = false;
    };
  }, [language, preloadedTranslations]);

  // Change language
  const setLanguage = useCallback((newLanguage) => {
    if (!SUPPORTED_LANGUAGES[newLanguage]) {
      console.warn(
        `Invalid language: ${newLanguage}. Supported languages: ${Object.keys(
          SUPPORTED_LANGUAGES
        ).join(", ")}`
      );
      return;
    }
    setLanguageState(newLanguage);
  }, []);

  // Translation function with fallback and interpolation
  const t = useCallback(
    (key, params = {}) => {
      if (!key) return "";

      // Get translation
      let translation = translations[key];

      // Fallback to key if translation not found
      if (translation === undefined) {
        console.warn(
          `Translation missing for key: "${key}" in language: ${language}`
        );
        return key;
      }

      // Handle interpolation (e.g., "Hello {name}" with params = { name: "World" })
      if (params && Object.keys(params).length > 0) {
        translation = translation.replace(/\{(\w+)\}/g, (match, param) => {
          return params[param] !== undefined ? params[param] : match;
        });
      }

      return translation;
    },
    [translations, language]
  );

  // Translation function with pluralization support
  const tn = useCallback(
    (key, count, params = {}) => {
      const pluralKey = count === 1 ? key : `${key}_plural`;
      return t(pluralKey, { ...params, count });
    },
    [t]
  );

  // Check if translation key exists
  const hasTranslation = useCallback(
    (key) => {
      return translations[key] !== undefined;
    },
    [translations]
  );

  // Get current language metadata
  const languageInfo = useMemo(
    () =>
      SUPPORTED_LANGUAGES[language] || SUPPORTED_LANGUAGES[DEFAULT_LANGUAGE],
    [language]
  );

  // Get list of available languages
  const availableLanguages = useMemo(
    () => Object.values(SUPPORTED_LANGUAGES),
    []
  );

  const value = {
    // Current language
    language,
    languageInfo,
    availableLanguages,

    // Language setters
    setLanguage,

    // Translation functions
    t,
    tn,
    hasTranslation,

    // State
    isLoading,
    error,

    // Raw translations (for advanced use cases)
    translations,
  };

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
};

// ============================================
// I18N HOOK
// ============================================

/**
 * Hook to access i18n context
 * @returns {Object} I18n context value
 * @throws {Error} If used outside I18nProvider
 */
export const useI18n = () => {
  const context = useContext(I18nContext);

  if (!context) {
    throw new Error(
      "useI18n must be used within an I18nProvider. " +
        "Wrap your component tree with <I18nProvider>.</I18nProvider>"
    );
  }

  return context;
};

// ============================================
// CONVENIENCE HOOKS
// ============================================

/**
 * Hook for just the translation function
 * Useful when you don't need full i18n context
 */
export const useTranslation = () => {
  const { t, tn } = useI18n();
  return { t, tn };
};

/**
 * Hook for language switching functionality
 */
export const useLanguage = () => {
  const { language, languageInfo, availableLanguages, setLanguage } = useI18n();
  return { language, languageInfo, availableLanguages, setLanguage };
};

/**
 * Hook with SSR-safe defaults
 */
export const useI18nWithSSR = () => {
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  const i18n = useI18n();

  if (!mounted) {
    return {
      ...i18n,
      language: DEFAULT_LANGUAGE,
      languageInfo: SUPPORTED_LANGUAGES[DEFAULT_LANGUAGE],
      isLoading: false,
    };
  }

  return i18n;
};

// ============================================
// EXPORTS
// ============================================
export { SUPPORTED_LANGUAGES, DEFAULT_LANGUAGE };
export default useI18n;
