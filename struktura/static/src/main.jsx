import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.css';
import { api } from './lib/api';

// Initialize API then mount the app
// This avoids top-level await which isn't supported in older browsers
api.init()
  .then(() => {
    ReactDOM.createRoot(document.getElementById('root')).render(
      <React.StrictMode>
        <App />
      </React.StrictMode>
    );
  })
  .catch((error) => {
    console.error('Failed to initialize API:', error);
    // Still render the app even if CSRF token fetch fails
    // The auth check will handle it on protected routes
    ReactDOM.createRoot(document.getElementById('root')).render(
      <React.StrictMode>
        <App />
      </React.StrictMode>
    );
  });