export const i18nConfig = {
  defaultLocale: 'en',
  supportedLocales: ['en', 'zh', 'es', 'ja'],
  fallbackLocale: 'en',
  namespaces: ['common', 'project', 'investment', 'admin'],
  loadPath: '/locales/{{lng}}/{{ns}}.json',
  detection: {
    order: ['querystring', 'cookie', 'localStorage', 'navigator'],
    lookupQuerystring: 'lang',
    lookupCookie: 'i18next',
    lookupLocalStorage: 'i18nextLng',
    caches: ['localStorage', 'cookie']
  },
  interpolation: {
    escapeValue: false
  }
} as const;

export type I18nConfig = typeof i18nConfig; 