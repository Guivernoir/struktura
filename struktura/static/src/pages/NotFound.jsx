import { Link } from 'react-router-dom';
import Icon from '../components/Icon';

const NotFound = () => {
  return (
    <div className="min-h-screen flex items-center justify-center px-4 bg-white dark:bg-charcoal-950">
      <div className="text-center space-y-6 max-w-md">
        <div className="inline-flex items-center justify-center w-20 h-20 bg-gradient-to-br from-sand-400 to-sand-600 rounded-2xl text-white shadow-medium">
          <Icon name="Box" size={40} strokeWidth={2.5} />
        </div>
        
        <div className="space-y-2">
          <h1 className="font-display text-6xl font-black text-charcoal-900 dark:text-white">
            404
          </h1>
          <h2 className="font-display text-2xl font-bold text-charcoal-900 dark:text-white">
            Page Not Found
          </h2>
          <p className="text-charcoal-600 dark:text-steel-400">
            The page you're looking for doesn't exist or has been moved.
          </p>
        </div>

        <Link to="/" className="inline-block btn-primary">
          Return Home
        </Link>
      </div>
    </div>
  );
};

export default NotFound;