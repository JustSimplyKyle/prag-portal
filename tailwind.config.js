/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  safelift: [
    {
      pattern: /./,
      variants: ['hover'],
    },
  ],
  theme: {
    extend: {
      colors: {
        'white': '#FFFFFF',
        'secondary-surface': '#2F2F2F',
        'hint': '#4F4F4F',
        'deep-background': '#0E0E0E',
        'background': '#191919',
        'green': '#14AE5C',
        'red': '#C92B45',
        'black': "#232E3D",
        'light-blue': '#7CAED3',
        'tinted-yellow': '#D3E950',
        'purple': '9747FF',
      },
      animation: {
        moveRight: 'moveRight 2s ease-out',
        moveLeft: 'moveLeft 2s ease-out'
      },
    },
  },
  plugins: [require("daisyui")],
};
