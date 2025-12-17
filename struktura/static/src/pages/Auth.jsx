import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api, getErrorMessage } from "../lib";
import Icon from "../components/Icon";

const Auth = () => {
  const [isLogin, setIsLogin] = useState(true);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [formData, setFormData] = useState({
    username: "",
    email: "",
    password: "",
  });

  const navigate = useNavigate();

  // Redirect if already logged in
  useEffect(() => {
    if (api.auth.isAuthenticated()) {
      navigate("/dashboard");
    }
  }, [navigate]);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError("");
    setLoading(true);

    try {
      if (isLogin) {
        await api.auth.login(formData.username, formData.password);
      } else {
        // Validate email format
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        if (!emailRegex.test(formData.email)) {
          setError("Please enter a valid email address");
          setLoading(false);
          return;
        }

        // Validate password length (matches backend requirement)
        if (formData.password.length < 8) {
          setError("Password must be at least 8 characters long");
          setLoading(false);
          return;
        }

        await api.auth.signup(
          formData.username,
          formData.email,
          formData.password
        );
      }
      navigate("/dashboard");
    } catch (err) {
      const errorMsg = getErrorMessage(err);
      setError(errorMsg);
    } finally {
      setLoading(false);
    }
  };

  const handleChange = (e) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  const toggleMode = () => {
    setIsLogin(!isLogin);
    setError("");
    setFormData({
      username: "",
      email: "",
      password: "",
    });
  };

  return (
    <div className="min-h-[calc(100vh-80px)] flex items-center justify-center px-4 py-12">
      <div className="w-full max-w-md">
        {/* Card */}
        <div className="glass-strong rounded-2xl shadow-hard p-8 space-y-6">
          {/* Header */}
          <div className="text-center space-y-2">
            <div className="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-br from-charcoal-900 to-charcoal-800 dark:from-sand-500 dark:to-sand-600 rounded-2xl text-white dark:text-charcoal-900 shadow-medium mb-4">
              <Icon name="Layers" size={32} strokeWidth={2.5} />
            </div>
            <h1 className="font-display text-3xl font-black text-charcoal-900 dark:text-white">
              {isLogin ? "Welcome Back" : "Get Started"}
            </h1>
            <p className="text-sm text-charcoal-600 dark:text-steel-400">
              {isLogin
                ? "Sign in to your account"
                : "Create your Struktura account"}
            </p>
          </div>

          {/* Error Message */}
          {error && (
            <div className="p-4 rounded-xl bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-400 text-sm">
              {error}
            </div>
          )}

          {/* Form */}
          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <label className="block text-sm font-semibold text-charcoal-700 dark:text-steel-300 mb-2">
                Username
              </label>
              <input
                type="text"
                name="username"
                value={formData.username}
                onChange={handleChange}
                required
                minLength={3}
                maxLength={50}
                className="w-full px-4 py-3 rounded-xl bg-white dark:bg-charcoal-800 border border-sand-200 dark:border-charcoal-700 text-charcoal-900 dark:text-white placeholder-charcoal-400 dark:placeholder-steel-500 focus:outline-none focus:ring-2 focus:ring-sand-500/50 transition-all"
                placeholder="Enter your username"
              />
            </div>

            {!isLogin && (
              <div>
                <label className="block text-sm font-semibold text-charcoal-700 dark:text-steel-300 mb-2">
                  Email
                </label>
                <input
                  type="email"
                  name="email"
                  value={formData.email}
                  onChange={handleChange}
                  required
                  className="w-full px-4 py-3 rounded-xl bg-white dark:bg-charcoal-800 border border-sand-200 dark:border-charcoal-700 text-charcoal-900 dark:text-white placeholder-charcoal-400 dark:placeholder-steel-500 focus:outline-none focus:ring-2 focus:ring-sand-500/50 transition-all"
                  placeholder="Enter your email"
                />
              </div>
            )}

            <div>
              <label className="block text-sm font-semibold text-charcoal-700 dark:text-steel-300 mb-2">
                Password
              </label>
              <input
                type="password"
                name="password"
                value={formData.password}
                onChange={handleChange}
                required
                minLength={8}
                className="w-full px-4 py-3 rounded-xl bg-white dark:bg-charcoal-800 border border-sand-200 dark:border-charcoal-700 text-charcoal-900 dark:text-white placeholder-charcoal-400 dark:placeholder-steel-500 focus:outline-none focus:ring-2 focus:ring-sand-500/50 transition-all"
                placeholder="Enter your password"
              />
              {!isLogin && (
                <p className="mt-1 text-xs text-charcoal-500 dark:text-steel-500">
                  Must be at least 8 characters
                </p>
              )}
            </div>

            <button
              type="submit"
              disabled={loading}
              className="w-full btn-primary py-3 text-base disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {loading
                ? "Processing..."
                : isLogin
                ? "Sign In"
                : "Create Account"}
            </button>
          </form>

          {/* Toggle */}
          <div className="text-center">
            <button
              onClick={toggleMode}
              className="text-sm text-sand-600 dark:text-sand-400 hover:text-sand-700 dark:hover:text-sand-300 font-medium transition-colors"
            >
              {isLogin
                ? "Don't have an account? Sign up"
                : "Already have an account? Sign in"}
            </button>
          </div>

          {/* Security Notice */}
          <div className="pt-4 border-t border-sand-200 dark:border-charcoal-800">
            <p className="text-xs text-center text-charcoal-500 dark:text-steel-500">
              Protected by enterprise-grade security (Argon2id, JWT, CSRF)
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Auth;
