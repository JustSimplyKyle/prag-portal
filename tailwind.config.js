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

const svgPlugin = plugin(function({ matchComponents, theme, e }) {
  /// the first two arguments is, when it being selected, and its animation
  /// the last two argument is, when it selects the target, and its animation
    matchComponents(
    {
        "svg": (value) => {
          return {
              [`@apply w-[${value}] h-[${value}] [&_*]:max-w-[${value}] [&_*]:max-h-[${value}]`]: {},
          };
        },
        "svg-direct": (value) => {
          return {
              [`@apply w-[${value}] h-[${value}]`]: {},
          };
        },
    },
    
  );
});

module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  safelist: [
    'animate-slideLeft',
    'animate-slideRight',
    'animate-slideDown',
    'animate-slideOutDown',
    'animate-slideUp',
    'animate-slideOutUp',
  ],
  theme: {
    fontFamily: {
      'display': ['GenSenRounded TW'],
      'english': ['IBM Plex Mono'],
    },
    fontMetrics: {
      'display': {
        capHeight: 733,
        ascent: 880,
        lineGap: 0,
        descent: -120,
        unitsPerEm: 1000,
      },
      'english': {
        capHeight: 698,
        ascent: 1025,
        lineGap: 0,
        descent: -275,
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
        'green': '#CDE347',
        'red': '#C92B45',
        'black': "#232E3D",
        'light-blue': '#7CAED3',
        'tinted-yellow': '#D3E950',
        'purple': '9747FF',
      },
      animation: {
        slideRight: 'slideRight 500ms',
        slideLeft: 'slideLeft 500ms',
        'scroll-left': 'scrollLeft 10000ms infinite ease-in-out',
        slideDown: 'slideDown 1000ms ease-in-out',
        slideOutDown: 'slideOutDown 1000ms ease-in-out',
        slideUp: 'slideUp 1000ms ease-in-out',
        slideOutUp: 'slideOutUp 1000ms ease-in-out',
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
        scrollLeft: {
          '0%': {
            transform: 'translateX(var(--from-width))',
          },
          '50%': {
            transform: 'translateX(var(--to-width))',
          },
          '100%': {
            transform: 'translateX(var(--from-width))',
          },
        },
        slideDown: {
          '0%': {
            transform: 'translateY(-100dvh)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '50%': {
            transform: 'translateY(3dvh)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '75%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '100%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
        },
        slideOutDown: {
          '0%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '50%': {
            transform: 'translateY(103dvh)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '75%': {
            transform: 'translateY(100dvh)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '100%': {
            transform: 'translateY(100dvh)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
        },
        slideOutUp: {
          '0%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '50%': {
            transform: 'translateY(-103dvh)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '75%': {
            transform: 'translateY(-100dvh)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '100%': {
            transform: 'translateY(-100dvh)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
        },
        slideUp: {
          '0%': {
            transform: 'translateY(100dvh)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '50%': {
            transform: 'translateY(-3dvh)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '75%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
          '100%': {
            transform: 'translateY(0%)',
            'animation-timing-function': 'theme(animationTiming.linear)',
          },
        },
      },
    },
  },
  plugins: [
    require("daisyui"),
    require("tailwindcss-capsize"),
    animationPlugin,
    svgPlugin,
  ],
};
