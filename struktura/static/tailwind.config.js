var config = {
    content: ["./index.html", "./src/**/*.{js,jsx,ts,tsx}"],
    darkMode: "class",
    theme: {
        extend: {
            fontFamily: {
                sans: ['"Inter"', "system-ui", "-apple-system", "sans-serif"],
                display: ['"Space Grotesk"', "system-ui", "sans-serif"],
            },
            colors: {
                sand: {
                    50: "#faf9f7",
                    100: "#f4f1ed",
                    200: "#e8e2d9",
                    300: "#d6cbbf",
                    400: "#bfad9b",
                    500: "#a8927d",
                    600: "#8f7966",
                    700: "#776454",
                    800: "#625347",
                    900: "#53463d",
                },
                steel: {
                    50: "#f6f7f9",
                    100: "#eceef2",
                    200: "#d5dae2",
                    300: "#b0bac9",
                    400: "#8596ab",
                    500: "#667790",
                    600: "#516076",
                    700: "#424e60",
                    800: "#394251",
                    900: "#333a45",
                },
                charcoal: {
                    50: "#f6f6f6",
                    100: "#e7e7e7",
                    200: "#d1d1d1",
                    300: "#b0b0b0",
                    400: "#888888",
                    500: "#6d6d6d",
                    600: "#5d5d5d",
                    700: "#4f4f4f",
                    800: "#454545",
                    900: "#3d3d3d",
                    950: "#1a1a1a",
                },
            },
            animation: {
                "fade-in": "fadeIn 0.5s ease-in-out",
                "slide-up": "slideUp 0.6s ease-out",
                float: "float 6s ease-in-out infinite",
                "float-delayed": "float 6s ease-in-out 3s infinite",
                ticker: "ticker 40s linear infinite",
            },
            keyframes: {
                fadeIn: {
                    "0%": { opacity: "0" },
                    "100%": { opacity: "1" },
                },
                slideUp: {
                    "0%": { transform: "translateY(30px)", opacity: "0" },
                    "100%": { transform: "translateY(0)", opacity: "1" },
                },
                float: {
                    "0%, 100%": { transform: "translateY(0px)" },
                    "50%": { transform: "translateY(-15px)" },
                },
                ticker: {
                    "0%": { transform: "translateX(0)" },
                    "100%": { transform: "translateX(-50%)" },
                },
            },
            boxShadow: {
                soft: "0 2px 15px -3px rgba(0, 0, 0, 0.07), 0 10px 20px -2px rgba(0, 0, 0, 0.04)",
                medium: "0 10px 40px -10px rgba(0, 0, 0, 0.15)",
                hard: "0 20px 60px -15px rgba(0, 0, 0, 0.3)",
            },
        },
    },
    plugins: [],
};
export default config;
