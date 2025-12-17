export const getInitialLang = () => {
  const supportedLangs = ["en", "pt", "fr", "es", "de", "it", "ru"];

  const storedLang = localStorage.getItem("struktura-lang");
  if (storedLang && supportedLangs.includes(storedLang)) return storedLang;

  const params = new URLSearchParams(window.location.search);
  const urlLang = params.get("lang");
  if (urlLang && supportedLangs.includes(urlLang)) return urlLang;

  const browserLang = navigator.language.split("-")[0];
  return supportedLangs.includes(browserLang) ? browserLang : "en";
};

export const getInitialTheme = () => {
  const storedTheme = localStorage.getItem("struktura-theme");
  if (storedTheme && ["light", "dark"].includes(storedTheme))
    return storedTheme;
  return "dark";
};
