import { Outlet } from 'react-router-dom';
import { useState, useEffect } from 'react';
import { getInitialLang, getInitialTheme } from '../utils';
import { TRANSLATIONS } from '../translations';
import Navbar from '../components/Navbar';
import Footer from '../components/Footer';

const RootLayout = () => {
  const [lang, setLang] = useState(getInitialLang);
  const [theme, setTheme] = useState(getInitialTheme);
  const [scrolled, setScrolled] = useState(false);

  // Theme management
  useEffect(() => {
    const root = document.documentElement;
    
    if (theme === 'dark') {
      root.classList.add('dark');
    } else {
      root.classList.remove('dark');
    }
    
    localStorage.setItem('struktura-theme', theme);
  }, [theme]);

  // Language management
  useEffect(() => {
    const t = TRANSLATIONS[lang] || TRANSLATIONS.en;
    
    document.title = t.pageTitle;
    document.documentElement.lang = lang;
    localStorage.setItem('struktura-lang', lang);
  }, [lang]);

  // Scroll detection
  useEffect(() => {
    const handleScroll = () => {
      setScrolled(window.scrollY > 50);
    };

    window.addEventListener('scroll', handleScroll, { passive: true });
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const t = TRANSLATIONS[lang] || TRANSLATIONS.en;

  return (
    <div className="min-h-screen flex flex-col relative bg-white dark:bg-charcoal-950 overflow-hidden">
      {/* Background Effects */}
      <div className="fixed inset-0 z-0 pointer-events-none overflow-hidden">
        <div className="absolute top-0 left-0 w-full h-[500px] bg-gradient-to-b from-sand-100/30 via-sand-50/20 to-transparent dark:from-charcoal-900/30 dark:via-charcoal-950/20" />
        <div className="absolute -top-[20%] -right-[10%] w-[600px] h-[600px] bg-gradient-radial from-sand-400/10 via-sand-300/5 to-transparent dark:from-sand-900/10 dark:via-sand-950/5 rounded-full blur-3xl animate-float" />
        <div className="absolute top-[60%] -left-[10%] w-[500px] h-[500px] bg-gradient-radial from-sand-300/10 via-sand-200/5 to-transparent dark:from-charcoal-800/10 dark:via-charcoal-900/5 rounded-full blur-3xl animate-float-delayed" />
      </div>

      {/* Navigation */}
      <Navbar
        scrolled={scrolled}
        lang={lang}
        setLang={setLang}
        theme={theme}
        setTheme={setTheme}
        t={t}
      />

      {/* Page Content */}
      <main className="flex-grow pt-20 md:pt-24 relative z-10">
        <Outlet context={{ lang, theme, t }} />
      </main>

      {/* Footer */}
      <Footer t={t}/>
    </div>
  );
};

export default RootLayout;