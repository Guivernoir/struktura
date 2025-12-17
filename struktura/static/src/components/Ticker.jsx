import PropTypes from 'prop-types';

const Ticker = ({ t }) => {
  return (
    <div className="mt-16 md:mt-24 border-y border-sand-200/50 dark:border-charcoal-800 bg-gradient-to-r from-sand-50/30 via-sand-100/50 to-sand-50/30 dark:from-charcoal-900/30 dark:via-charcoal-900/50 dark:to-charcoal-900/30 backdrop-blur-sm overflow-hidden py-4 md:py-5">
      <div className="relative">
        <div className="whitespace-nowrap flex gap-8 md:gap-12 animate-ticker">
          {/* Duplicate content for seamless loop */}
          {[1, 2].map((set) => (
            <div key={set} className="flex gap-8 md:gap-12 items-center">
              {t.ticker.split(' • ').map((item, i) => (
                <div key={i} className="flex items-center gap-4">
                  <span className="text-xs md:text-sm font-display font-bold uppercase tracking-[0.15em] text-charcoal-400 dark:text-charcoal-600">
                    {item.trim()}
                  </span>
                  <span className="text-sand-400 dark:text-sand-700 text-lg">•</span>
                </div>
              ))}
            </div>
          ))}
        </div>

        {/* Gradient overlays for fade effect */}
        <div className="absolute inset-y-0 left-0 w-20 bg-gradient-to-r from-sand-50/50 dark:from-charcoal-900/50 to-transparent pointer-events-none" />
        <div className="absolute inset-y-0 right-0 w-20 bg-gradient-to-l from-sand-50/50 dark:from-charcoal-900/50 to-transparent pointer-events-none" />
      </div>
    </div>
  );
};

Ticker.propTypes = {
  t: PropTypes.object.isRequired,
};

export default Ticker;