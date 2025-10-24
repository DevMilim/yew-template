/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "jit",
    content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
    theme: {
        container: {
            center: true,
            padding: '2rem',
            screens: {
                'sm': "640px",
                'md': "768px",
                'lg': "1024px",
                'xl': "1280px",
                '2xl': "1536px"
            }
        },
        extend: {

        },
    },
    variants: {
        extend: {},
    },
    plugins: [],
};