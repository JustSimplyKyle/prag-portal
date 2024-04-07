/** @type {import('tailwindcss').Config} */
const plugin = require('tailwindcss/plugin')

const animationPlugin = plugin(function({ matchComponents, theme, e }) {

  /// the first two arguments is, when it being selected, and its animation
  /// the last two argument is, when it selects the target, and its animation
    matchComponents({
        flyinout: (value) => {
          return {
            [`@apply animation-[${value}^slideLeft] transform-all group-data-[prev=${value}]:start-[100dvw] group-data-[prev=${value}]:z-[100] group-data-[prev=${value}]:animate-slideRight`]: {},
          }
        },
        animation: (value) => {
            const [primaryValue, primaryAnimation, secondaryValue, secondaryAnimation] = value.split('^');
            if (secondaryValue == null) {
              return {
                  [`@apply hidden group-data-[prev=${primaryValue}]:block group-data-[selected=${primaryValue}]:block group-data-[selected=${primaryValue}]:z-50 group-data-[selected=${primaryValue}]:animate-${primaryAnimation}`]: {},
              };
            } else {
              return {
                  [`@apply hidden group-data-[prev=${primaryValue}]:block group-data-[selected=${primaryValue}]:block group-data-[selected=${primaryValue}]:z-50 group-data-[prev=${secondaryValue}]:group-data-[selected=${primaryValue}]:animate-${primaryAnimation} group-data-[selected=${secondaryValue}]:animate-${secondaryAnimation}`]: {},
              };
            }
        },
    });
});

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
        slideRight: 'slideRight 500ms',
        slideOutRight: 'slideOutRight 500ms',
        slideLeft: 'slideLeft 500ms',
        slideOutLeft: 'slideOutLeft 500ms',
        slideDown: 'slideDown 500ms',
        slideOutDown: 'slideOutDown 500ms',
        slideUp: 'slideUp 500ms',
        slideOutUp: 'slideOutUp 500ms',
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
        slideOutRight: {
          '0%': {
            transform: 'translateX(0%)',
            'animation-timing-function': 'cubic-bezier(0.47,0.0,0.23,1.1)',
          },
          '100%': {
            transform: 'translateX(100%)',
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
  plugins: [
    require("daisyui"),
    animationPlugin 
  ],
};
