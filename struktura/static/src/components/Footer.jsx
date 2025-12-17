import Icon from './Icon';
import PropTypes from 'prop-types';

const Footer = ({t}) => {
  const currentYear = new Date().getFullYear();

  const footerLinks = {
    product: [
      { label: t.footer.product.features, href: '/features' },
      { label: t.footer.product.pricing, href: '/pricing' },
      { label: t.footer.product.changelog, href: '/changelog' },
      { label: t.footer.product.documentation, href: '/docs' },
    ],
    company: [
      { label: t.footer.company.about, href: '/about' },
      { label: t.footer.company.blog, href: '/blog' },
      { label: t.footer.company.careers, href: '/careers' },
      { label: t.footer.company.contact, href: '/contact' },
    ],
    legal: [
      { label: t.footer.legal.privacy, href: '/privacy' },
      { label: t.footer.legal.terms, href: '/terms' },
      { label: t.footer.legal.security, href: '/security' },
      { label: t.footer.legal.cookies, href: '/cookies' },
    ],
  };

  return (
    <footer className="relative border-t border-sand-200 dark:border-charcoal-800 bg-white dark:bg-charcoal-950 mt-auto">
      {/* Background Gradient */}
      <div className="absolute inset-0 bg-gradient-to-b from-transparent to-sand-50/30 dark:to-charcoal-900/30 pointer-events-none" />

      <div className="relative container mx-auto px-4 md:px-6 py-12 md:py-16">
        <div className="grid grid-cols-2 md:grid-cols-5 gap-8 md:gap-12 mb-12">
          {/* Brand Column */}
          <div className="col-span-2 md:col-span-2">
            <div className="flex items-center gap-3 mb-4">
              <div className="w-10 h-10 bg-gradient-to-br from-charcoal-900 to-charcoal-800 dark:from-sand-500 dark:to-sand-600 rounded-xl flex items-center justify-center text-white dark:text-charcoal-900 shadow-soft">
                <Icon name="Layers" size={20} strokeWidth={2.5} />
              </div>
              <span className="font-display font-bold text-xl tracking-tight text-charcoal-900 dark:text-white">
                Struktura
              </span>
            </div>
            <p className="text-sm text-charcoal-600 dark:text-steel-400 leading-relaxed mb-6 max-w-xs">
              {t.footer.tagline}
            </p>
            <div className="flex items-center gap-2 text-sm">
              <div className="flex items-center gap-2 px-3 py-1 rounded-full bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800">
                <span className="relative flex h-2 w-2">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
                  <span className="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span>
                </span>
                <span className="text-green-700 dark:text-green-400 font-medium">
                  {t.footer.status}
                </span>
              </div>
            </div>
          </div>

          {/* Product Links */}
          <div>
            <h3 className="font-display font-bold text-sm uppercase tracking-wider text-charcoal-900 dark:text-white mb-4">
              {t.footer.product.title}
            </h3>
            <ul className="space-y-3">
              {footerLinks.product.map((link) => (
                <li key={link.label}>
                  <a
                    href={link.href}
                    className="text-sm text-charcoal-600 dark:text-steel-400 hover:text-sand-600 dark:hover:text-sand-400 transition-colors"
                  >
                    {link.label}
                  </a>
                </li>
              ))}
            </ul>
          </div>

          {/* Company Links */}
          <div>
            <h3 className="font-display font-bold text-sm uppercase tracking-wider text-charcoal-900 dark:text-white mb-4">
              {t.footer.company.title}
            </h3>
            <ul className="space-y-3">
              {footerLinks.company.map((link) => (
                <li key={link.label}>
                  <a
                    href={link.href}
                    className="text-sm text-charcoal-600 dark:text-steel-400 hover:text-sand-600 dark:hover:text-sand-400 transition-colors"
                  >
                    {link.label}
                  </a>
                </li>
              ))}
            </ul>
          </div>

          {/* Legal Links */}
          <div>
            <h3 className="font-display font-bold text-sm uppercase tracking-wider text-charcoal-900 dark:text-white mb-4">
              {t.footer.legal.title}
            </h3>
            <ul className="space-y-3">
              {footerLinks.legal.map((link) => (
                <li key={link.label}>
                  <a
                    href={link.href}
                    className="text-sm text-charcoal-600 dark:text-steel-400 hover:text-sand-600 dark:hover:text-sand-400 transition-colors"
                  >
                    {link.label}
                  </a>
                </li>
              ))}
            </ul>
          </div>
        </div>

        {/* Bottom Bar */}
        <div className="pt-8 border-t border-sand-200 dark:border-charcoal-800 flex flex-col md:flex-row justify-between items-center gap-4">
          <div className="flex items-center gap-2 text-sm text-charcoal-500 dark:text-steel-500">
            <span className="font-medium">© {currentYear} Struktura.</span>
            <span className="hidden sm:inline">{t.footer.copyright}</span>
          </div>

          {/* Social Links */}
          <div className="flex items-center gap-4">
            <a
              href="https://twitter.com"
              target="_blank"
              rel="noopener noreferrer"
              className="text-charcoal-400 dark:text-steel-500 hover:text-sand-600 dark:hover:text-sand-400 transition-colors"
              aria-label={t.footer.social.twitter}
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M23 3a10.9 10.9 0 01-3.14 1.53 4.48 4.48 0 00-7.86 3v1A10.66 10.66 0 013 4s-4 9 5 13a11.64 11.64 0 01-7 2c9 5 20 0 20-11.5a4.5 4.5 0 00-.08-.83A7.72 7.72 0 0023 3z" />
              </svg>
            </a>
            <a
              href="https://github.com"
              target="_blank"
              rel="noopener noreferrer"
              className="text-charcoal-400 dark:text-steel-500 hover:text-sand-600 dark:hover:text-sand-400 transition-colors"
              aria-label={t.footer.social.github}
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.17 6.839 9.49.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.463-1.11-1.463-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.578 9.578 0 0112 6.836c.85.004 1.705.114 2.504.336 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.138 20.167 22 16.418 22 12c0-5.523-4.477-10-10-10z" />
              </svg>
            </a>
            <a
              href="https://linkedin.com"
              target="_blank"
              rel="noopener noreferrer"
              className="text-charcoal-400 dark:text-steel-500 hover:text-sand-600 dark:hover:text-sand-400 transition-colors"
              aria-label={t.footer.social.linkedin}
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" />
              </svg>
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
};

Footer.propTypes = {
  t: PropTypes.object.isRequired,
};

export default Footer;