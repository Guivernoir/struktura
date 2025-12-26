import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";
// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    // Build configuration for production
    build: {
        outDir: "../static/dist",
        emptyOutDir: true,
        sourcemap: false,
        target: "es2015",
        rollupOptions: {
            output: {
                manualChunks: {
                    "react-vendor": ["react", "react-dom", "react-router-dom"],
                    "three-vendor": ["three"],
                },
            },
        },
        chunkSizeWarningLimit: 1000,
    },
    // Development server configuration
    server: {
        port: 5173,
        strictPort: false,
        proxy: {
            "/api": {
                target: "http://localhost:8000",
                changeOrigin: true,
            },
        },
    },
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
});
