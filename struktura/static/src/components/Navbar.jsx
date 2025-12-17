import { Link, useNavigate } from "react-router-dom";
import { useState, useEffect, useRef } from "react";
import PropTypes from "prop-types";
import { api } from "../lib";
import Icon from "./Icon";

const Navbar = ({ scrolled, lang, setLang, theme, setTheme, t }) => {
  const [showProfileMenu, setShowProfileMenu] = useState(false);
  const [showMobileMenu, setShowMobileMenu] = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const menuRef = useRef(null);
  const navigate = useNavigate();

  useEffect(() => {
    setIsAuthenticated(api.auth.isAuthenticated());
  }, []);

  useEffect(() => {
    const handleClickOutside = (event) => {
      if (menuRef.current && !menuRef.current.contains(event.target)) {
        setShowProfileMenu(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const handleLogout = async () => {
    try {
      await api.auth.logout();
      setIsAuthenticated(false);
      setShowProfileMenu(false);
      navigate("/auth");
    } catch (error) {
      console.error("Logout failed:", error);
      navigate("/auth");
    }
  };

  const toggleTheme = () => {
    setTheme(theme === "dark" ? "light" : "dark");
  };

  // Base classes for consistent background visibility (no longer transparent at top)
  const navClasses = `fixed top-0 w-full z-50 transition-all duration-300 border-b border-sand-200/50 dark:border-charcoal-800/50 backdrop-blur-md bg-sand-50/90 dark:bg-charcoal-950/90 ${
    scrolled ? "py-3 shadow-medium" : "py-4 md:py-6"
  }`;

  return (
    <nav className={navClasses}>
      <div className="container mx-auto px-4 md:px-6">
        <div className="flex justify-between items-center">
          {/* Logo */}
          <Link to="/" className="flex items-center gap-3 group">
            <div className="w-10 h-10 bg-gradient-to-br from-charcoal-900 to-charcoal-800 dark:from-sand-500 dark:to-sand-600 rounded-xl flex items-center justify-center text-white dark:text-charcoal-900 shadow-soft group-hover:shadow-medium transition-all group-hover:scale-105">
              <Icon name="Layers" size={20} strokeWidth={2.5} />
            </div>
            <span className="font-display font-bold text-xl tracking-tight text-charcoal-900 dark:text-white">
              Struktura
            </span>
          </Link>

          {/* Desktop Navigation */}
          <div className="hidden md:flex items-center gap-4">
            <Link
              to="/guide"
              className="flex items-center gap-2 px-3 py-2 text-sm font-medium text-charcoal-600 dark:text-sand-200 hover:text-charcoal-900 dark:hover:text-white rounded-lg hover:bg-charcoal-50 dark:hover:bg-charcoal-800 transition-all"
            >
              <Icon name="Book" size={16} />
              <span>{t.guide}</span>
            </Link>

            {/* Language Selector */}
            <select
              className="px-3 py-2 text-sm font-medium bg-transparent hover:bg-charcoal-50 dark:hover:bg-charcoal-800 rounded-lg cursor-pointer transition-all focus:outline-none focus:ring-2 focus:ring-sand-500/50 border-none"
              value={lang}
              onChange={(e) => setLang(e.target.value)}
              aria-label={t.language}
            >
              <option value="en">EN</option>
              <option value="pt">PT</option>
              <option value="fr">FR</option>
              <option value="es">ES</option>
              <option value="de">DE</option>
              <option value="ru">РУ</option>
              <option value="it">IT</option>
            </select>

            {/* Theme Toggle */}
            <button
              onClick={toggleTheme}
              className="p-2 hover:bg-charcoal-50 dark:hover:bg-charcoal-800 rounded-lg transition-all focus:outline-none focus:ring-2 focus:ring-sand-500/50"
              aria-label={t.theme}
            >
              <Icon name={theme === "dark" ? "Sun" : "Moon"} size={20} />
            </button>

            {isAuthenticated ? (
              <>
                <Link to="/dashboard" className="btn-secondary text-sm py-2">
                  {t.dashboard}
                </Link>
                <div className="relative" ref={menuRef}>
                  <button
                    onClick={() => setShowProfileMenu(!showProfileMenu)}
                    className="p-2 hover:bg-charcoal-50 dark:hover:bg-charcoal-800 rounded-lg transition-all text-charcoal-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-sand-500/50"
                    aria-label="User menu"
                    aria-expanded={showProfileMenu}
                  >
                    <Icon name="User" size={20} />
                  </button>

                  {showProfileMenu && (
                    <div className="absolute right-0 mt-2 w-48 glass-strong rounded-xl shadow-hard py-2 animate-fade-in bg-white dark:bg-charcoal-900 border border-sand-200 dark:border-charcoal-800">
                      <button
                        onClick={handleLogout}
                        className="w-full text-left px-4 py-2.5 text-sm text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 flex items-center gap-2 transition-colors"
                      >
                        <Icon name="LogOut" size={16} />
                        {t.logout}
                      </button>
                    </div>
                  )}
                </div>
              </>
            ) : (
              <div className="flex items-center gap-2 ml-2">
                <Link to="/auth" className="btn-secondary text-sm py-2">
                  {t.login}
                </Link>
                <Link to="/auth" className="btn-primary text-sm py-2">
                  {t.signup}
                </Link>
              </div>
            )}
          </div>

          {/* Mobile Menu Button */}
          <button
            onClick={() => setShowMobileMenu(!showMobileMenu)}
            className="md:hidden p-2 hover:bg-charcoal-50 dark:hover:bg-charcoal-800 rounded-lg transition-all text-charcoal-900 dark:text-white"
            aria-label="Toggle menu"
            aria-expanded={showMobileMenu}
          >
            <Icon name={showMobileMenu ? "X" : "Menu"} size={24} />
          </button>
        </div>

        {/* Mobile Menu */}
        {showMobileMenu && (
          <div className="md:hidden mt-4 pb-4 space-y-2 animate-fade-in border-t border-sand-200 dark:border-charcoal-800 pt-4">
            <Link
              to="/guide"
              className="flex items-center gap-2 px-4 py-3 text-sm font-medium text-charcoal-600 dark:text-sand-200 hover:bg-charcoal-50 dark:hover:bg-charcoal-800 rounded-lg transition-all"
              onClick={() => setShowMobileMenu(false)}
            >
              <Icon name="Book" size={16} />
              {t.guide}
            </Link>

            <div className="flex items-center justify-between px-4 py-3">
              <span className="text-sm font-medium text-charcoal-600 dark:text-sand-200">
                {t.language}
              </span>
              <select
                className="px-3 py-1 text-sm font-medium bg-transparent hover:bg-charcoal-50 dark:hover:bg-charcoal-800 rounded-lg cursor-pointer transition-all border border-sand-200 dark:border-charcoal-700"
                value={lang}
                onChange={(e) => setLang(e.target.value)}
              >
                <option value="en">EN</option>
                <option value="pt">PT</option>
                <option value="fr">FR</option>
                <option value="es">ES</option>
                <option value="de">DE</option>
                <option value="ru">РУ</option>
                <option value="it">IT</option>
              </select>
            </div>

            <button
              onClick={toggleTheme}
              className="w-full flex items-center justify-between px-4 py-3 text-sm font-medium text-charcoal-600 dark:text-sand-200 hover:bg-charcoal-50 dark:hover:bg-charcoal-800 rounded-lg transition-all"
            >
              <span>{t.theme}</span>
              <Icon name={theme === "dark" ? "Sun" : "Moon"} size={20} />
            </button>

            <div className="pt-2 space-y-2 px-4">
              {isAuthenticated ? (
                <>
                  <Link
                    to="/dashboard"
                    className="block btn-secondary text-center text-sm py-2"
                    onClick={() => setShowMobileMenu(false)}
                  >
                    {t.dashboard}
                  </Link>
                  <button
                    onClick={() => {
                      setShowMobileMenu(false);
                      handleLogout();
                    }}
                    className="w-full text-red-600 dark:text-red-400 border-2 border-red-600 dark:border-red-400 rounded-xl px-6 py-2 text-sm font-semibold hover:bg-red-50 dark:hover:bg-red-900/20 transition-all flex items-center justify-center gap-2"
                  >
                    <Icon name="LogOut" size={16} />
                    {t.logout}
                  </button>
                </>
              ) : (
                <>
                  <Link
                    to="/auth"
                    className="block btn-secondary text-center text-sm py-2"
                    onClick={() => setShowMobileMenu(false)}
                  >
                    {t.login}
                  </Link>
                  <Link
                    to="/auth"
                    className="block btn-primary text-center text-sm py-2"
                    onClick={() => setShowMobileMenu(false)}
                  >
                    {t.signup}
                  </Link>
                </>
              )}
            </div>
          </div>
        )}
      </div>
    </nav>
  );
};

Navbar.propTypes = {
  scrolled: PropTypes.bool.isRequired,
  lang: PropTypes.string.isRequired,
  setLang: PropTypes.func.isRequired,
  theme: PropTypes.string.isRequired,
  setTheme: PropTypes.func.isRequired,
  t: PropTypes.object.isRequired,
};

export default Navbar;
