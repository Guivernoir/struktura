import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],

  // Build configuration for production
  build: {
    // Output directory (Axum will serve from here)
    outDir: "../static/dist",

    // Empty the output directory before building
    emptyOutDir: true,

    // Generate source maps for debugging (optional, remove in production)
    sourcemap: false,

    // Target modern browsers to enable better optimizations
    target: "es2015",

    // Optimize chunk splitting
    rollupOptions: {
      output: {
        // Manual chunk splitting for better caching
        manualChunks: {
          "react-vendor": ["react", "react-dom", "react-router-dom"],
          "three-vendor": ["three"],
        },
      },
    },

    // Increase chunk size warning limit (optional)
    chunkSizeWarningLimit: 1000,
  },

  // Development server configuration
  server: {
    port: 5173,
    strictPort: false,
    // Proxy API requests to Axum backend during development
    proxy: {
      "/api": {
        target: "http://localhost:8000",
        changeOrigin: true,
      },
    },
  },

  // Path aliases (optional, for cleaner imports)
  //resolve: {
  //  alias: {
  //    "@": path.resolve(__dirname, "./src"),
  //    "@components": path.resolve(__dirname, "./src/components"),
  //    "@lib": path.resolve(__dirname, "./src/lib"),
  //    "@hooks": path.resolve(__dirname, "./src/hooks"),
  //    "@pages": path.resolve(__dirname, "./src/pages"),
  //  },
  //},
});
