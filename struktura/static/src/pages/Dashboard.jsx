import { useEffect, useState } from 'react';
import { api } from '../lib/api';

const Dashboard = () => {
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchProfile = async () => {
      try {
        const profile = await api.user.me();
        setUser(profile);
      } catch (error) {
        console.error('Failed to fetch profile:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchProfile();
  }, []);

  if (loading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-sand-500"></div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="font-display text-3xl font-black text-charcoal-900 dark:text-white">
          Dashboard
        </h1>
        <p className="text-charcoal-600 dark:text-steel-400 mt-2">
          Welcome back{user?.username ? `, ${user.username}` : ''}!
        </p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="p-6 rounded-2xl bg-white dark:bg-charcoal-900 border border-sand-200 dark:border-charcoal-800 shadow-soft">
          <div className="text-sm font-medium text-charcoal-600 dark:text-steel-400 mb-2">
            Total Projects
          </div>
          <div className="text-3xl font-display font-black text-charcoal-900 dark:text-white">
            0
          </div>
        </div>

        <div className="p-6 rounded-2xl bg-white dark:bg-charcoal-900 border border-sand-200 dark:border-charcoal-800 shadow-soft">
          <div className="text-sm font-medium text-charcoal-600 dark:text-steel-400 mb-2">
            Calculations
          </div>
          <div className="text-3xl font-display font-black text-charcoal-900 dark:text-white">
            0
          </div>
        </div>

        <div className="p-6 rounded-2xl bg-white dark:bg-charcoal-900 border border-sand-200 dark:border-charcoal-800 shadow-soft">
          <div className="text-sm font-medium text-charcoal-600 dark:text-steel-400 mb-2">
            Saved Materials
          </div>
          <div className="text-3xl font-display font-black text-charcoal-900 dark:text-white">
            0
          </div>
        </div>
      </div>

      {/* Quick Actions */}
      <div className="p-6 rounded-2xl bg-white dark:bg-charcoal-900 border border-sand-200 dark:border-charcoal-800 shadow-soft">
        <h2 className="font-display text-xl font-bold text-charcoal-900 dark:text-white mb-4">
          Quick Actions
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <button className="btn-primary text-left">New Calculation</button>
          <button className="btn-secondary text-left">Create Project</button>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;