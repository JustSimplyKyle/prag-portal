/** @type {import('tailwindcss').Config} */
const plugin = require('tailwindcss/plugin')

const animationPlugin = plugin(function({ matchComponents, theme, e }) {
  /// the first two arguments is, when it being selected, and its animation
  /// the last two argument is, when it selects the target, and its animation
    matchComponents({
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
  safelist: [
    'animate-slideLeft',
    'animate-slideRight',
  ],
  theme: {
    fontFamily: {
      'display': ['GenSenRounded TW'],
    },
    fontMetrics: {
      'display': {
        capHeight: 733,
        ascent: 880,
        lineGap: 0,
        descent: -120,
        unitsPerEm: 1000,
      }
    },
    animationTiming: {
      'bounce': 'cubic-bezier(.08,.06,0,1.28)',
      'linear': 'cubic-bezier(.17,.67,.14,1.02)'
    },
    extend: {
      transitionTimingFunction: {
        'bounce': 'theme(animationTiming.bounce)',
        'linear': 'theme(animationTiming.linear)',
      },
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
        slideRight: 'slideRight 500ms ease-in-out',
        slideLeft: 'slideLeft 500ms',
        slideDown: 'slideDown 500ms',
        slideOutDown: 'slideOutDown 500ms',
        slideUp: 'slideUp 500ms',
        slideOutUp: 'slideOutUp 500ms',
      },
      keyframes: {
        slideRight: {
          '0%': {
            transform: 'translateX(-100dvw)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '100%': {
            transform: 'translateX(0%)',
            'animation-timing-function': 'theme(animationTiming.linear)'
          },
        },
        slideLeft: {
          '0%': {
            transform: 'translateX(100dvw)',
            'animation-timing-function': 'theme(animationTiming.linear)'
          },
          '100%': {
            transform: 'translateX(0%)',
            'animation-timing-function': 'theme(animationTiming.linear)'
          },
        },
        slideDown: {
          '0%': {
            transform: 'translateY(-100dvh)',
            'animation-timing-function': 'theme(animationTiming.bounce)',
          },
          '100%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'theme(animationTiming.bounce)',
          },
        },
        slideOutDown: {
          '0%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'theme(animationTiming.bounce)',
          },
          '100%': {
            transform: 'translateY(100dvh)',
            'animation-timing-function': 'theme(animationTiming.bounce)',
          },
        },
        slideOutUp: {
          '0%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'theme(animationTiming.bounce)',
          },
          '100%': {
            transform: 'translateY(-100dvh)',
            'animation-timing-function': 'theme(animationTiming.bounce)',
          },
        },
        slideUp: {
          '0%': {
            transform: 'translateY(100dvh)',
            'animation-timing-function': 'theme(animationTiming.bounce)',
          },
          '100%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'theme(animationTiming.bounce)',
          },
        },
      },
    },
  },
  plugins: [
    require("daisyui"),
    require("tailwindcss-capsize"),
    animationPlugin 
  ],
};
