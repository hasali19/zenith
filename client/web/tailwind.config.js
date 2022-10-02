/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{jsx,tsx}"],
  theme: {
    extend: {
      aspectRatio: {
        poster: "2 / 3",
      },
    },
  },
  plugins: [],
};
