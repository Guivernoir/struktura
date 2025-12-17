import { Link } from 'react-router-dom';
import PropTypes from 'prop-types';
import Icon from './Icon';

const modeConfig = {
  beginner: {
    icon: 'Box',
    gradient: 'from-blue-500/10 to-blue-600/5',
    iconBg: 'bg-blue-50 dark:bg-blue-900/20',
    iconColor: 'text-blue-600 dark:text-blue-400',
    hoverBorder: 'hover:border-blue-400 dark:hover:border-blue-600',
    link: '/beginner',
  },
  builder: {
    icon: 'HardHat',
    gradient: 'from-orange-500/10 to-orange-600/5',
    iconBg: 'bg-orange-50 dark:bg-orange-900/20',
    iconColor: 'text-orange-600 dark:text-orange-400',
    hoverBorder: 'hover:border-orange-400 dark:hover:border-orange-600',
    link: '/contractor',
  },
  professional: {
    icon: 'Ruler',
    gradient: 'from-purple-500/10 to-purple-600/5',
    iconBg: 'bg-purple-50 dark:bg-purple-900/20',
    iconColor: 'text-purple-600 dark:text-purple-400',
    hoverBorder: 'hover:border-purple-400 dark:hover:border-purple-600',
    link: '/engineer',
  },
};

const ModeCard = ({ modeKey, mode, config, t }) => {
  return (
    <Link
      to={config.link}
      className={`group p-6 md:p-8 rounded-2xl md:rounded-3xl bg-white dark:bg-charcoal-900 border border-sand-200 dark:border-charcoal-800 ${config.hoverBorder} transition-all duration-300 shadow-soft hover:shadow-medium cursor-pointer relative overflow-hidden block card-hover`}
    >
      {/* Background Gradient */}
      <div className={`absolute top-0 right-0 w-32 h-32 bg-gradient-to-br ${config.gradient} opacity-50 rounded-bl-[100px] group-hover:scale-110 transition-transform duration-500`} />

      {/* Icon */}
      <div className={`relative w-12 h-12 md:w-14 md:h-14 mb-5 md:mb-6 rounded-xl md:rounded-2xl ${config.iconBg} ${config.iconColor} flex items-center justify-center shadow-soft group-hover:shadow-medium transition-all group-hover:scale-105`}>
        <Icon name={config.icon} size={24} strokeWidth={2.5} />
      </div>

      {/* Title */}
      <h3 className="relative text-xl md:text-2xl font-bold text-charcoal-900 dark:text-white mb-3 font-display group-hover:text-sand-700 dark:group-hover:text-sand-300 transition-colors">
        {mode.title}
      </h3>

      {/* Description */}
      <p className="relative text-charcoal-600 dark:text-steel-400 text-sm md:text-base leading-relaxed mb-6 min-h-[4rem]">
        {mode.desc}
      </p>

      {/* CTA Link */}
      <div className="relative flex items-center text-sand-600 dark:text-sand-400 font-semibold text-sm group-hover:text-sand-700 dark:group-hover:text-sand-300 transition-all">
        <span className="mr-1">{t.mode_cards.select_mode}</span>
        <Icon 
          name="ChevronRight" 
          size={16} 
          className="group-hover:translate-x-1 transition-transform" 
        />
      </div>

      {/* Hover Effect Line */}
      <div className="absolute bottom-0 left-0 right-0 h-1 bg-gradient-to-r from-sand-400 to-sand-600 transform scale-x-0 group-hover:scale-x-100 transition-transform origin-left duration-300 rounded-b-2xl md:rounded-b-3xl" />
    </Link>
  );
};

ModeCard.propTypes = {
  modeKey: PropTypes.string.isRequired,
  mode: PropTypes.object.isRequired,
  config: PropTypes.object.isRequired,
  t: PropTypes.object.isRequired, // Add t to propTypes
};

const ModeCards = ({ t }) => {
  return (
    <section className="container mx-auto px-4 md:px-6 py-16 md:py-24">
      {/* Section Header */}
      <div className="text-center mb-12 md:mb-16 space-y-4">
        <h2 className="font-display text-3xl md:text-4xl lg:text-5xl font-black text-charcoal-900 dark:text-white">
          {t.mode_cards.title_prefix}{" "}
          <span className="gradient-text">{t.mode_cards.title_highlight}</span>
        </h2>
        <p className="text-base md:text-lg text-charcoal-600 dark:text-steel-300 max-w-2xl mx-auto">
          {t.mode_cards.subtitle}
        </p>
      </div>

      {/* Cards Grid */}
      <div className="grid md:grid-cols-3 gap-6 md:gap-8">
        {Object.entries(t.modes).map(([key, mode]) => (
          <ModeCard
            key={key}
            modeKey={key}
            mode={mode}
            config={modeConfig[key]}
            t={t} // Pass t to ModeCard
          />
        ))}
      </div>

      {/* Bottom CTA */}
      <div className="text-center mt-12 md:mt-16">
        <p className="text-sm text-charcoal-500 dark:text-steel-500 mb-4">
          {t.mode_cards.cta_text}
        </p>
        <Link
          to="/guide"
          className="inline-flex items-center gap-2 text-sand-600 dark:text-sand-400 hover:text-sand-700 dark:hover:text-sand-300 font-semibold text-sm group transition-colors"
        >
          <Icon name="Book" size={16} />
          {t.mode_cards.cta_link}
          <Icon 
            name="ChevronRight" 
            size={14} 
            className="group-hover:translate-x-1 transition-transform" 
          />
        </Link>
      </div>
    </section>
  );
};

ModeCards.propTypes = {
  t: PropTypes.object.isRequired,
};

export default ModeCards;