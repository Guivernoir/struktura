import { Outlet, NavLink, useNavigate, Link } from 'react-router-dom';
import { useState, useEffect } from 'react';
import { api } from '../lib/api';
import Icon from '../components/Icon';
import { getInitialTheme } from '../utils';

const DashboardLayout = () => {
  const [theme, setTheme] = useState(getInitialTheme);
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const navigate = useNavigate();

  useEffect(() => {
    const root = document.documentElement;
    if (theme === 'dark') {
      root.classList.add('dark');
    } else {
      root.classList.remove('dark');
    }
    localStorage.setItem('struktura-theme', theme);
  }, [theme]);

  const handleLogout = async () => {
    try {
      await api.auth.logout();
      navigate('/auth');
    } catch (error) {
      console.error('Logout failed:', error);
      navigate('/auth');
    }
  };

  const navItems = [
    { to: '/dashboard', icon: 'Layers', label: 'Dashboard', end: true },
    { to: '/dashboard/projects', icon: 'Box', label: 'Projects' },
    { to: '/dashboard/history', icon: 'Book', label: 'History' },
    { to: '/dashboard/profile', icon: 'User', label: 'Profile' },
  ];

  return (
    <div className="min-h-screen bg-sand-50 dark:bg-charcoal-950 flex">
      {/* Sidebar - Desktop */}
      <aside className="hidden md:flex md:flex-shrink-0">
        <div className="flex flex-col w-64 bg-white dark:bg-charcoal-900 border-r border-sand-200 dark:border-charcoal-800">
          {/* Logo */}
          <Link to="/" className="flex items-center gap-3 px-6 py-6 border-b border-sand-200 dark:border-charcoal-800">
            <div className="w-10 h-10 bg-gradient-to-br from-charcoal-900 to-charcoal-800 dark:from-sand-500 dark:to-sand-600 rounded-xl flex items-center justify-center text-white dark:text-charcoal-900 shadow-soft">
              <Icon name="Layers" size={20} strokeWidth={2.5} />
            </div>
            <span className="font-display font-bold text-xl tracking-tight text-charcoal-900 dark:text-white">
              Struktura
            </span>
          </Link>

          {/* Navigation */}
          <nav className="flex-1 px-4 py-6 space-y-1">
            {navItems.map((item) => (
              <NavLink
                key={item.to}
                to={item.to}
                end={item.end}
                className={({ isActive }) =>
                  `flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium transition-all ${
                    isActive
                      ? 'bg-sand-100 dark:bg-charcoal-800 text-charcoal-900 dark:text-white'
                      : 'text-charcoal-600 dark:text-steel-400 hover:bg-sand-50 dark:hover:bg-charcoal-800/50'
                  }`
                }
              >
                <Icon name={item.icon} size={18} />
                {item.label}
              </NavLink>
            ))}
          </nav>

          {/* Bottom Actions */}
          <div className="px-4 py-4 border-t border-sand-200 dark:border-charcoal-800 space-y-2">
            <button
              onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
              className="w-full flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium text-charcoal-600 dark:text-steel-400 hover:bg-sand-50 dark:hover:bg-charcoal-800/50 transition-all"
            >
              <Icon name={theme === 'dark' ? 'Sun' : 'Moon'} size={18} />
              {theme === 'dark' ? 'Light Mode' : 'Dark Mode'}
            </button>
            <button
              onClick={handleLogout}
              className="w-full flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-all"
            >
              <Icon name="LogOut" size={18} />
              Logout
            </button>
          </div>
        </div>
      </aside>

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col min-w-0">
        {/* Mobile Header */}
        <header className="md:hidden flex items-center justify-between px-4 py-4 bg-white dark:bg-charcoal-900 border-b border-sand-200 dark:border-charcoal-800">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 bg-gradient-to-br from-charcoal-900 to-charcoal-800 dark:from-sand-500 dark:to-sand-600 rounded-lg flex items-center justify-center text-white dark:text-charcoal-900">
              <Icon name="Layers" size={16} strokeWidth={2.5} />
            </div>
            <span className="font-display font-bold text-lg tracking-tight text-charcoal-900 dark:text-white">
              Struktura
            </span>
          </div>
          <button
            onClick={() => setSidebarOpen(!sidebarOpen)}
            className="p-2 hover:bg-sand-50 dark:hover:bg-charcoal-800 rounded-lg transition-all"
          >
            <Icon name={sidebarOpen ? 'X' : 'Menu'} size={24} />
          </button>
        </header>

        {/* Mobile Sidebar */}
        {sidebarOpen && (
          <div className="md:hidden fixed inset-0 z-50 bg-charcoal-900/50 backdrop-blur-sm">
            <div className="absolute inset-y-0 left-0 w-64 bg-white dark:bg-charcoal-900 shadow-hard">
              <div className="flex flex-col h-full">
                <div className="flex items-center justify-between px-4 py-4 border-b border-sand-200 dark:border-charcoal-800">
                  <span className="font-display font-bold text-lg text-charcoal-900 dark:text-white">
                    Menu
                  </span>
                  <button
                    onClick={() => setSidebarOpen(false)}
                    className="p-2 hover:bg-sand-50 dark:hover:bg-charcoal-800 rounded-lg transition-all"
                  >
                    <Icon name="X" size={20} />
                  </button>
                </div>

                <nav className="flex-1 px-4 py-6 space-y-1">
                  {navItems.map((item) => (
                    <NavLink
                      key={item.to}
                      to={item.to}
                      end={item.end}
                      onClick={() => setSidebarOpen(false)}
                      className={({ isActive }) =>
                        `flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium transition-all ${
                          isActive
                            ? 'bg-sand-100 dark:bg-charcoal-800 text-charcoal-900 dark:text-white'
                            : 'text-charcoal-600 dark:text-steel-400 hover:bg-sand-50 dark:hover:bg-charcoal-800/50'
                        }`
                      }
                    >
                      <Icon name={item.icon} size={18} />
                      {item.label}
                    </NavLink>
                  ))}
                </nav>

                <div className="px-4 py-4 border-t border-sand-200 dark:border-charcoal-800 space-y-2">
                  <button
                    onClick={() => {
                      setTheme(theme === 'dark' ? 'light' : 'dark');
                      setSidebarOpen(false);
                    }}
                    className="w-full flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium text-charcoal-600 dark:text-steel-400 hover:bg-sand-50 dark:hover:bg-charcoal-800/50 transition-all"
                  >
                    <Icon name={theme === 'dark' ? 'Sun' : 'Moon'} size={18} />
                    {theme === 'dark' ? 'Light Mode' : 'Dark Mode'}
                  </button>
                  <button
                    onClick={() => {
                      setSidebarOpen(false);
                      handleLogout();
                    }}
                    className="w-full flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-all"
                  >
                    <Icon name="LogOut" size={18} />
                    Logout
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Page Content */}
        <main className="flex-1 overflow-y-auto">
          <div className="container mx-auto px-4 md:px-6 py-6 md:py-8">
            <Outlet />
          </div>
        </main>
      </div>
    </div>
  );
};

export default DashboardLayout;