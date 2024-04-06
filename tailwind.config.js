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
    fontFamily: {
      'display': ['GenSenRounded TW'],
    },
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
        slideRight: 'slideRight 700ms',
        slideLeft: 'slideLeft 700ms',
        slideDown: 'slideDown 700ms',
        slideOutDown: 'slideOutDown 700ms',
        slideUp: 'slideUp 700ms',
        slideOutUp: 'slideOutUp 700ms',
      },
      keyframes: {
        slideRight: {
          '0%': {
            transform: 'translateX(-100dvw)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.1)',
          },
          '100%': {
            transform: 'translateX(0%)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.3)',
          },
        },
        slideLeft: {
          '0%': {
            transform: 'translateX(100dvw)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.1)',
          },
          '100%': {
            transform: 'translateX(0%)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.3)',
          },
        },
        slideDown: {
          '0%': {
            transform: 'translateY(-100dvh)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.1)',
          },
          '100%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.3)',
          },
        },
        slideOutDown: {
          '0%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.1)',
          },
          '100%': {
            transform: 'translateY(100dvh)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.3)',
          },
        },
        slideOutUp: {
          '0%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.1)',
          },
          '100%': {
            transform: 'translateY(-100dvh)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.3)',
          },
        },
        slideUp: {
          '0%': {
            transform: 'translateY(100dvh)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.1)',
          },
          '100%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.3)',
          },
        },
      },
    },
  },
  plugins: [require("daisyui")],
};
