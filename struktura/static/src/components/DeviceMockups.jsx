import PropTypes from 'prop-types';

const DeviceMockups = ({ t }) => {
  // Utility classes for themes
  const laptopContentDark = 'bg-zinc-900 border-zinc-700';
  const laptopContentLight = 'bg-white border-zinc-200';
  
  return (
    // **Adaptable Height & Centering:** Use a generous min-height and padding for responsiveness, allowing the content to determine the final height.
    <div className="relative w-full min-h-[500px] py-16 md:py-24 flex items-center justify-center overflow-hidden">
      
      {/* Decorative Background Elements */}
      <div className="absolute -z-10 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[150%] h-[150%] bg-gradient-radial from-sand-400/10 dark:from-sky-700/10 to-transparent blur-3xl opacity-50 transition-opacity duration-500" />
      
      {/* Laptop Mockup - Adjusted position, size, and rotation */}
      {/* Old: left-[40%] w-[90%] max-w-[700px] -rotate-2 */}
      <div className="absolute w-[80%] max-w-[600px] aspect-[16/10] z-10 animate-float-slow left-[40%] top-[50%] -translate-x-1/2 -translate-y-1/2 -rotate-5 transform">
        {/* Mockup Frame (Simplification of the original mockup-laptop class) */}
        <div className="relative w-full h-full rounded-3xl overflow-hidden shadow-2xl shadow-zinc-900/50 border border-zinc-300 dark:border-zinc-700/70">
          
          {/* Screen Content - Theme-Aware */}
          <div className={`w-full h-full p-[2%] flex flex-col transition-colors duration-500 ${laptopContentLight} dark:${laptopContentDark}`}>
            
            {/* Window Controls & Title Bar - Theme-Aware */}
            <div className="h-10 bg-zinc-50 dark:bg-zinc-800 border-b border-zinc-200 dark:border-zinc-700 flex items-center px-4 gap-2 rounded-t-xl">
              <div className="flex gap-2">
                <div className="w-3 h-3 rounded-full bg-red-400 hover:bg-red-500 transition-colors cursor-pointer" />
                <div className="w-3 h-3 rounded-full bg-yellow-400 hover:bg-yellow-500 transition-colors cursor-pointer" />
                <div className="w-3 h-3 rounded-full bg-green-400 hover:bg-green-500 transition-colors cursor-pointer" />
              </div>
              <div className="ml-4 h-4 w-40 bg-zinc-200 dark:bg-zinc-700 rounded-lg transition-colors duration-500" />
            </div>

            {/* Dashboard Content - Theme-Aware */}
            <div className="flex-1 flex text-[12px] dark:text-white/60 text-zinc-600 font-sans p-4 gap-4 bg-white dark:bg-zinc-900 transition-colors duration-500">
              
              {/* Sidebar */}
              <div className="w-20 flex flex-col gap-3 border-r border-zinc-200 dark:border-zinc-700 pr-4 transition-colors duration-500">
                {[1, 2, 3, 4, 5].map((i) => (
                  <div
                    key={i}
                    className={`h-8 w-full rounded-lg transition-all duration-300 ${
                      i === 2 ? 'bg-sky-500/80 shadow-md shadow-sky-500/20' : 'bg-zinc-100 dark:bg-zinc-800 hover:bg-zinc-200 dark:hover:bg-zinc-700'
                    }`}
                  />
                ))}
              </div>

              {/* Main Content */}
              <div className="flex-1 flex flex-col gap-4">
                
                {/* Chart - Modernized style */}
                <div className="relative h-40 p-4 rounded-xl bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 shadow-inner">
                  <svg
                    className="absolute inset-0 w-full h-full text-sky-500 transition-colors duration-500"
                    preserveAspectRatio="none"
                    viewBox="0 0 400 100"
                    style={{ overflow: 'visible' }}
                  >
                    <defs>
                      <linearGradient id="chartGradientNew" x1="0%" y1="0%" x2="0%" y2="100%">
                        <stop offset="0%" stopColor="currentColor" stopOpacity="0.4" />
                        <stop offset="100%" stopColor="currentColor" stopOpacity="0" />
                      </linearGradient>
                    </defs>
                    <path
                      d="M 20,80 C 100,20 200,80 300,40 S 380,60 380,60"
                      fill="url(#chartGradientNew)"
                      stroke="currentColor"
                      strokeWidth="3"
                      strokeLinecap="round"
                    />
                  </svg>
                  <div className="absolute top-2 left-2 text-sm font-semibold text-zinc-800 dark:text-white transition-colors duration-500">Analytics Overview</div>
                </div>

                {/* Stats Cards - Translated Content - Theme-Aware */}
                <div className="grid grid-cols-2 gap-4">
                  <div className="h-16 bg-white dark:bg-zinc-700 rounded-xl border border-zinc-200 dark:border-zinc-600 p-3 shadow-soft-sm hover:shadow-soft transition-all duration-300 cursor-pointer">
                    <div className="text-sky-600 dark:text-sky-400 text-xs font-medium transition-colors duration-500">{t.device_mockups.load_a}</div>
                    <div className="text-lg font-bold text-zinc-900 dark:text-white transition-colors duration-500">{t.device_mockups.value_load_a}</div>
                  </div>
                  <div className="h-16 bg-white dark:bg-zinc-700 rounded-xl border border-zinc-200 dark:border-zinc-600 p-3 shadow-soft-sm hover:shadow-soft transition-all duration-300 cursor-pointer">
                    <div className="text-sky-600 dark:text-sky-400 text-xs font-medium transition-colors duration-500">{t.device_mockups.shear}</div>
                    <div className="text-lg font-bold text-zinc-900 dark:text-white transition-colors duration-500">{t.device_mockups.value_shear}</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          {/* Laptop Base - Modernized */}
          <div className="absolute -bottom-1 left-1/2 -translate-x-1/2 w-[98%] h-2 bg-zinc-300 dark:bg-zinc-800 rounded-b-lg shadow-inner-lg" />
        </div>
      </div>

      {/* Phone Mockup - Adjusted position, size, and rotation */}
      {/* Changes: w-[180px] md:w-[280px] -> w-[160px] md:w-[250px] (Smaller) and left-[65%] top-1/2 -> left-[70%] top-[45%] (Over right side) */}
      <div className="absolute w-[140px] md:w-[250px] aspect-[9/19] z-20 animate-float-delayed transform rotate-5 left-[55%] bottom-[10%] -translate-x-1/2 -translate-y-1/2 pt-16 md:pt-24">
        {/* Mockup Frame (Simplification of the original mockup-phone class) */}
        <div className="relative w-full-10 h-full rounded-[40px] overflow-hidden shadow-2xl shadow-zinc-900/50 border-[6px] border-zinc-900 dark:border-white transition-colors duration-500">
          
          {/* Notch - Modernized */}
          <div className="absolute top-0 left-1/2 -translate-x-1/2 w-20 h-6 bg-zinc-900 rounded-b-3xl z-30 shadow-lg" />

          {/* Screen Content - Uses dark: prefix for theme switching */}
          <div className="w-full h-full p-0 m-0 bg-sand-50 dark:bg-zinc-950 flex flex-col pt-10 px-4 transition-colors duration-500">
            
            {/* App Icon */}
            <div className="w-14 h-14 rounded-2xl bg-gradient-to-br from-sky-500 to-indigo-600 mb-6 shadow-xl shadow-sky-500/30 flex items-center justify-center">
              <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
              </svg>
            </div>

            {/* Title Text Content - Modernized skeleton/placeholder */}
            <div className="space-y-3 mb-8">
              <div className="w-11/12 h-4 bg-zinc-300 dark:bg-white/20 rounded-full animate-pulse" />
              <div className="w-3/4 h-3 bg-zinc-200 dark:bg-white/10 rounded-full animate-pulse" style={{ animationDelay: '0.1s' }} />
            </div>

            {/* List Items - Theme-Aware and modernized appearance */}
            <div className="space-y-3">
              {[1, 2, 3].map((i) => (
                <div
                  key={i}
                  className="flex items-center gap-3 p-3 bg-white dark:bg-zinc-800/80 rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 border border-zinc-100 dark:border-zinc-800"
                >
                  <div className="w-5 h-5 rounded-md border-3 border-sky-500 flex-shrink-0 bg-sky-500/10" />
                  <div className="flex-1 h-3 bg-zinc-200 dark:bg-white/15 rounded-full" />
                </div>
              ))}
            </div>

            {/* Total Cost - Translated Content - Modernized CTA look */}
            <div className="mt-auto mb-8 bg-gradient-to-r from-sky-500 to-indigo-600 text-white text-base font-extrabold p-4 rounded-2xl text-center shadow-lg shadow-sky-500/30 transition-shadow duration-300 cursor-pointer">
              {t.device_mockups.total_cost_prefix}: {t.device_mockups.value_total_cost}
            </div>
          </div>
        </div>
      </div>
      
    </div>
  );
};

DeviceMockups.propTypes = {
  t: PropTypes.object.isRequired,
};

export default DeviceMockups;