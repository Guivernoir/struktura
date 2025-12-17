import { Link } from 'react-router-dom';
import PropTypes from 'prop-types';
import Icon from './Icon';

const Hero = ({ t }) => {
  const [headlinePart1, headlinePart2] = t.headline.split('.');

  return (
    <section className="relative container mx-auto px-4 md:px-6 py-20 md:py-32 border-b border-sand-300/50 dark:border-charcoal-700 overflow-hidden">
      {/* Background Elements */}
      <div className="absolute inset-0 -z-10">
        {/* Gradient orbs */}
        <div className="absolute top-1/4 -left-20 w-96 h-96 bg-sand-300/30 dark:bg-sand-600/10 rounded-full blur-3xl animate-pulse" style={{ animationDuration: '4s' }} />
        <div className="absolute bottom-1/4 -right-20 w-96 h-96 bg-sky-300/20 dark:bg-sky-700/10 rounded-full blur-3xl animate-pulse" style={{ animationDuration: '6s', animationDelay: '1s' }} />
        
        {/* Grid pattern */}
        <div className="absolute inset-0 bg-[linear-gradient(to_right,#80808008_1px,transparent_1px),linear-gradient(to_bottom,#80808008_1px,transparent_1px)] bg-[size:64px_64px]" />
      </div>

      <div className="max-w-6xl mx-auto">
        {/* Content Container */}
        <div className="text-center space-y-8 animate-slide-up">
          {/* Badge */}
          <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full border border-sand-300/50 dark:border-charcoal-700 bg-gradient-to-r from-sand-50 to-sand-100/50 dark:from-charcoal-800/50 dark:to-charcoal-900/50 backdrop-blur-sm shadow-soft">
            <span className="relative flex h-2 w-2">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
              <span className="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span>
            </span>
            <span className="text-xs font-semibold uppercase tracking-wider text-charcoal-700 dark:text-sand-300">
              {t.tagline}
            </span>
          </div>

          {/* Headline */}
          <h1 className="font-display text-4xl sm:text-5xl md:text-6xl lg:text-7xl xl:text-8xl font-black leading-[1.1] text-charcoal-900 dark:text-white max-w-5xl mx-auto">
            {headlinePart1}.
            <br />
            <span className="gradient-text">
              {headlinePart2 && headlinePart2.trim()}.
            </span>
          </h1>

          {/* Subheading */}
          <p className="text-base sm:text-lg md:text-xl text-charcoal-600 dark:text-steel-300 leading-relaxed max-w-2xl mx-auto">
            {t.subhead}
          </p>

          {/* CTA Buttons */}
          <div className="flex flex-col sm:flex-row gap-4 justify-center pt-4">
            <Link
              to="/auth"
              className="group btn-primary flex items-center justify-center gap-2 text-base"
            >
              {t.signup}
              <Icon 
                name="ChevronRight" 
                size={18} 
                className="group-hover:translate-x-1 transition-transform" 
              />
            </Link>
            <Link
              to="/auth"
              className="btn-secondary flex items-center justify-center gap-2 text-base"
            >
              {t.login}
            </Link>
          </div>

          {/* Trust Indicators */}
          <div className="flex flex-wrap items-center justify-center gap-6 pt-6 text-sm text-charcoal-500 dark:text-steel-400">
            <div className="flex items-center gap-2">
              <div className="p-1 rounded-full bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400">
                <Icon name="CheckCircle" size={16} strokeWidth={2.5} />
              </div>
              <span className="font-medium">{t.hero_trust_1}</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="p-1 rounded-full bg-sand-200 dark:bg-sand-800/30 text-charcoal-600 dark:text-sand-300">
                <Icon name="Shield" size={16} strokeWidth={2.5} />
              </div>
              <span className="font-medium">{t.hero_trust_2}</span>
            </div>
          </div>
        </div>

        {/* Dashboard Preview */}
        <div className="relative mt-16 md:mt-24 animate-fade-in" style={{ animationDelay: '0.2s' }}>
          {/* Floating decoration cards */}
          <div className="absolute -top-8 -left-8 w-32 h-32 bg-gradient-to-br from-sand-200 to-sand-300 dark:from-sand-800 dark:to-sand-900 rounded-2xl shadow-2xl rotate-6 opacity-60 blur-sm -z-10" />
          <div className="absolute -bottom-8 -right-8 w-40 h-40 bg-gradient-to-br from-sky-200 to-sky-300 dark:from-sky-900 dark:to-sky-950 rounded-2xl shadow-2xl -rotate-6 opacity-60 blur-sm -z-10" />

          {/* Main Dashboard Card */}
          <div className="relative bg-white dark:bg-charcoal-900 rounded-3xl shadow-2xl border border-sand-200 dark:border-charcoal-800 overflow-hidden">
            {/* Window Controls */}
            <div className="h-12 bg-sand-50 dark:bg-charcoal-800 border-b border-sand-200 dark:border-charcoal-700 flex items-center px-6 gap-2">
              <div className="flex gap-2">
                <div className="w-3 h-3 rounded-full bg-red-400" />
                <div className="w-3 h-3 rounded-full bg-yellow-400" />
                <div className="w-3 h-3 rounded-full bg-green-400" />
              </div>
              <div className="ml-6 flex-1 flex items-center gap-2">
                <Icon name="Lock" size={14} className="text-charcoal-400 dark:text-steel-500" />
                <div className="h-6 flex-1 max-w-md bg-sand-100 dark:bg-charcoal-700 rounded-lg flex items-center px-3 text-xs text-charcoal-500 dark:text-steel-400">
                  struktura.app/dashboard
                </div>
              </div>
            </div>

            {/* Dashboard Content */}
            <div className="p-8 md:p-12">
              <div className="grid md:grid-cols-3 gap-6">
                {/* Stats Card 1 */}
                <div className="group relative bg-gradient-to-br from-sand-50 to-sand-100 dark:from-charcoal-800 dark:to-charcoal-850 rounded-2xl p-6 border border-sand-200 dark:border-charcoal-700 hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                  <div className="absolute top-4 right-4 w-12 h-12 bg-sky-500/10 dark:bg-sky-400/10 rounded-xl flex items-center justify-center">
                    <Icon name="Activity" size={24} className="text-sky-600 dark:text-sky-400" />
                  </div>
                  <div className="space-y-2">
                    <p className="text-sm font-medium text-charcoal-600 dark:text-steel-400">
                      {t.device_mockups.load_a}
                    </p>
                    <p className="text-3xl font-bold text-charcoal-900 dark:text-white">
                      {t.device_mockups.value_load_a}
                    </p>
                  </div>
                  <div className="mt-4 h-1 bg-sand-200 dark:bg-charcoal-700 rounded-full overflow-hidden">
                    <div className="h-full w-3/4 bg-gradient-to-r from-sky-500 to-sky-600 rounded-full" />
                  </div>
                </div>

                {/* Stats Card 2 */}
                <div className="group relative bg-gradient-to-br from-sand-50 to-sand-100 dark:from-charcoal-800 dark:to-charcoal-850 rounded-2xl p-6 border border-sand-200 dark:border-charcoal-700 hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
                  <div className="absolute top-4 right-4 w-12 h-12 bg-green-500/10 dark:bg-green-400/10 rounded-xl flex items-center justify-center">
                    <Icon name="Zap" size={24} className="text-green-600 dark:text-green-400" />
                  </div>
                  <div className="space-y-2">
                    <p className="text-sm font-medium text-charcoal-600 dark:text-steel-400">
                      {t.device_mockups.shear}
                    </p>
                    <p className="text-3xl font-bold text-charcoal-900 dark:text-white">
                      {t.device_mockups.value_shear}
                    </p>
                  </div>
                  <div className="mt-4 h-1 bg-sand-200 dark:bg-charcoal-700 rounded-full overflow-hidden">
                    <div className="h-full w-2/3 bg-gradient-to-r from-green-500 to-green-600 rounded-full" />
                  </div>
                </div>

                {/* Stats Card 3 */}
                <div className="group relative bg-gradient-to-br from-sky-500 to-sky-600 dark:from-sky-600 dark:to-sky-700 rounded-2xl p-6 border border-sky-400 dark:border-sky-500 hover:shadow-xl hover:shadow-sky-500/20 transition-all duration-300 hover:-translate-y-1">
                  <div className="absolute top-4 right-4 w-12 h-12 bg-white/20 rounded-xl flex items-center justify-center">
                    <Icon name="DollarSign" size={24} className="text-white" />
                  </div>
                  <div className="space-y-2">
                    <p className="text-sm font-medium text-sky-100">
                      {t.device_mockups.total_cost_prefix}
                    </p>
                    <p className="text-3xl font-bold text-white">
                      {t.device_mockups.value_total_cost}
                    </p>
                  </div>
                  <div className="mt-4 flex items-center gap-2 text-xs text-sky-100">
                    <Icon name="TrendingUp" size={14} />
                    <span>Updated live</span>
                  </div>
                </div>
              </div>

              {/* Chart Preview */}
              <div className="mt-6 bg-sand-50 dark:bg-charcoal-850 rounded-2xl p-6 border border-sand-200 dark:border-charcoal-700">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="font-semibold text-charcoal-900 dark:text-white">Structural Analysis</h3>
                  <div className="flex gap-2">
                    <div className="w-2 h-2 rounded-full bg-sky-500" />
                    <div className="w-2 h-2 rounded-full bg-green-500" />
                    <div className="w-2 h-2 rounded-full bg-sand-400" />
                  </div>
                </div>
                <div className="relative h-32">
                  <svg
                    className="w-full h-full"
                    viewBox="0 0 400 100"
                    preserveAspectRatio="none"
                  >
                    <defs>
                      <linearGradient id="chartGrad" x1="0%" y1="0%" x2="0%" y2="100%">
                        <stop offset="0%" stopColor="rgb(14 165 233)" stopOpacity="0.3" />
                        <stop offset="100%" stopColor="rgb(14 165 233)" stopOpacity="0" />
                      </linearGradient>
                    </defs>
                    <path
                      d="M 0,80 Q 50,20 100,40 T 200,60 T 300,30 T 400,50"
                      fill="url(#chartGrad)"
                      stroke="rgb(14 165 233)"
                      strokeWidth="2"
                      className="drop-shadow-lg"
                    />
                  </svg>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

Hero.propTypes = {
  t: PropTypes.object.isRequired,
};

export default Hero;