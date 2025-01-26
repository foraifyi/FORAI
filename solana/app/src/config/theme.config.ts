export const themeConfig = {
  themes: {
    light: {
      colors: {
        primary: '#1890ff',
        secondary: '#52c41a',
        success: '#52c41a',
        warning: '#faad14',
        error: '#f5222d',
        background: '#ffffff',
        text: '#000000',
        border: '#d9d9d9'
      },
      shadows: {
        small: '0 2px 8px rgba(0, 0, 0, 0.15)',
        medium: '0 4px 12px rgba(0, 0, 0, 0.15)',
        large: '0 8px 24px rgba(0, 0, 0, 0.15)'
      }
    },
    dark: {
      colors: {
        primary: '#177ddc',
        secondary: '#49aa19',
        success: '#49aa19',
        warning: '#d89614',
        error: '#d32029',
        background: '#141414',
        text: '#ffffff',
        border: '#434343'
      },
      shadows: {
        small: '0 2px 8px rgba(0, 0, 0, 0.45)',
        medium: '0 4px 12px rgba(0, 0, 0, 0.45)',
        large: '0 8px 24px rgba(0, 0, 0, 0.45)'
      }
    }
  },
  breakpoints: {
    xs: '320px',
    sm: '576px',
    md: '768px',
    lg: '992px',
    xl: '1200px',
    xxl: '1600px'
  },
  typography: {
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial',
    fontSize: {
      small: '12px',
      base: '14px',
      large: '16px',
      xlarge: '20px',
      xxlarge: '24px'
    },
    lineHeight: {
      small: 1.33,
      base: 1.5,
      large: 1.75
    }
  }
} as const;

export type ThemeConfig = typeof themeConfig; 