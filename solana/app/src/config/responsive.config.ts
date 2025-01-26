export const responsiveConfig = {
  grid: {
    columns: 24,
    gutterWidth: {
      xs: 8,
      sm: 16,
      md: 24,
      lg: 24,
      xl: 32,
      xxl: 32
    },
    breakpoints: {
      xs: {
        min: 0,
        max: 575
      },
      sm: {
        min: 576,
        max: 767
      },
      md: {
        min: 768,
        max: 991
      },
      lg: {
        min: 992,
        max: 1199
      },
      xl: {
        min: 1200,
        max: 1599
      },
      xxl: {
        min: 1600,
        max: Infinity
      }
    }
  },
  container: {
    padding: {
      xs: 16,
      sm: 16,
      md: 24,
      lg: 24,
      xl: 32,
      xxl: 32
    },
    maxWidth: {
      sm: 540,
      md: 720,
      lg: 960,
      xl: 1140,
      xxl: 1320
    }
  }
} as const;

export type ResponsiveConfig = typeof responsiveConfig; 