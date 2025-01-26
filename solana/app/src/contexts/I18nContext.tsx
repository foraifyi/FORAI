import React, { createContext, useContext, useState, useEffect } from 'react';
import { i18nConfig } from '../config/i18n.config';

type SupportedLocale = typeof i18nConfig.supportedLocales[number];

interface I18nContextType {
  locale: SupportedLocale;
  setLocale: (locale: SupportedLocale) => void;
  t: (key: string, params?: Record<string, any>) => string;
}

const I18nContext = createContext<I18nContextType>({
  locale: i18nConfig.defaultLocale,
  setLocale: () => undefined,
  t: (key: string) => key
});

interface I18nProviderProps {
  children: React.ReactNode;
}

export const I18nProvider: React.FC<I18nProviderProps> = ({ children }) => {
  const [locale, setLocale] = useState<SupportedLocale>(i18nConfig.defaultLocale);
  const [messages, setMessages] = useState<Record<string, string>>({});

  useEffect(() => {
    const savedLocale = localStorage.getItem('locale') as SupportedLocale | null;
    if (savedLocale && i18nConfig.supportedLocales.includes(savedLocale)) {
      loadLocaleMessages(savedLocale);
    } else {
      loadLocaleMessages(i18nConfig.defaultLocale);
    }
  }, []);

  const loadLocaleMessages = async (locale: SupportedLocale) => {
    try {
      const namespaces = await Promise.all(
        i18nConfig.namespaces.map(async (ns) => {
          const response = await fetch(
            i18nConfig.loadPath.replace('{{lng}}', locale).replace('{{ns}}', ns)
          );
          return response.json();
        })
      );

      const messages = namespaces.reduce<Record<string, string>>((acc, curr) => ({ ...acc, ...curr }), {});
      setMessages(messages);
      setLocale(locale);
      localStorage.setItem('locale', locale);
      document.documentElement.setAttribute('lang', locale);
    } catch (error) {
      console.error('Failed to load locale messages:', error);
    }
  };

  const t = (key: string, params?: Record<string, any>): string => {
    const message = key.split('.').reduce<string>((obj: any, key) => obj?.[key], messages) || key;

    if (params) {
      return Object.entries(params).reduce((msg, [key, value]) => {
        return msg.replace(`{{${key}}}`, String(value));
      }, message);
    }

    return message;
  };

  const handleSetLocale = (newLocale: SupportedLocale) => {
    if (i18nConfig.supportedLocales.includes(newLocale)) {
      loadLocaleMessages(newLocale);
    }
  };

  return (
    <I18nContext.Provider value={{ locale, setLocale: handleSetLocale, t }}>
      {children}
    </I18nContext.Provider>
  );
};

export const useI18n = () => {
  const context = useContext(I18nContext);
  if (!context) {
    throw new Error('useI18n must be used within an I18nProvider');
  }
  return context;
}; 