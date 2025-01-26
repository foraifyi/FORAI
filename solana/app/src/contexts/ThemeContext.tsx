import * as React from 'react';
import { themeConfig, ThemeConfig } from '../config/theme.config';

interface ThemeContextType {
  theme: keyof ThemeConfig['themes'];
  setTheme: (theme: keyof ThemeConfig['themes']) => void;
  toggleTheme: () => void;
}

const ThemeContext = React.createContext<ThemeContextType>({
  theme: 'light',
  setTheme: () => undefined,
  toggleTheme: () => undefined
});

interface ThemeProviderProps {
  children: React.ReactNode;
}

export const ThemeProvider = ({ children }: ThemeProviderProps) => {
  const [theme, setTheme] = React.useState<keyof ThemeConfig['themes']>('light');

  React.useEffect(() => {
    const savedTheme = localStorage.getItem('theme') as keyof ThemeConfig['themes'];
    if (savedTheme && themeConfig.themes[savedTheme]) {
      setTheme(savedTheme);
      document.documentElement.setAttribute('data-theme', savedTheme);
    }
  }, []);

  const toggleTheme = () => {
    const newTheme = theme === 'light' ? 'dark' : 'light';
    setTheme(newTheme);
    localStorage.setItem('theme', newTheme);
    document.documentElement.setAttribute('data-theme', newTheme);
  };

  return (
    <ThemeContext.Provider value={{ theme, setTheme, toggleTheme }}>
      {children}
    </ThemeContext.Provider>
  );
};

export const useTheme = () => {
  const context = React.useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
}; 