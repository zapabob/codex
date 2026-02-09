// SPDX-License-Identifier: Apache-2.0

"use client";

import {
  createContext,
  useContext,
  useState,
  useEffect,
  useCallback,
  ReactNode,
} from "react";
import { loadConfig } from "@/lib/config";
import { translations, Locale, TranslationDict } from "./translations";

interface I18nContextType {
  locale: Locale;
  setLocale: (locale: Locale) => void;
  t: (key: string) => string;
  supportedLocales: Locale[];
  isLoading: boolean;
}

const I18nContext = createContext<I18nContextType | undefined>(undefined);

export function I18nProvider({ children }: { children: ReactNode }) {
  const [locale, setLocaleState] = useState<Locale>("ja");
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    try {
      const config = loadConfig();
      const savedLocale = localStorage.getItem("codex-locale") as Locale;
      const localeToUse: Locale =
        savedLocale || (config.i18n.default_locale as Locale);

      if (translations[localeToUse]) {
        setLocaleState(localeToUse);
      }
    } catch (error) {
      console.warn("[i18n] Failed to load config, using default locale");
    } finally {
      setIsLoading(false);
    }
  }, []);

  const setLocale = useCallback((newLocale: Locale) => {
    if (translations[newLocale]) {
      setLocaleState(newLocale);
      localStorage.setItem("codex-locale", newLocale);
      document.documentElement.lang = newLocale;
    }
  }, []);

  const t = useCallback(
    (key: string): string => {
      const keys = key.split(".");
      let result: string | TranslationDict = translations[locale];

      for (const k of keys) {
        if (result && typeof result === "object" && k in result) {
          result = result[k] as string | TranslationDict;
        } else {
          // Fallback to English
          const fallback = translations.en;
          let fallbackResult: string | TranslationDict = fallback;
          for (const k of keys) {
            if (
              fallbackResult &&
              typeof fallbackResult === "object" &&
              k in fallbackResult
            ) {
              fallbackResult = fallbackResult[k] as string | TranslationDict;
            } else {
              return key;
            }
          }
          return typeof fallbackResult === "string" ? fallbackResult : key;
        }
      }

      return typeof result === "string" ? result : key;
    },
    [locale],
  );

  const supportedLocales = Object.keys(translations) as Locale[];

  return (
    <I18nContext.Provider
      value={{ locale, setLocale, t, supportedLocales, isLoading }}
    >
      {children}
    </I18nContext.Provider>
  );
}

export function useTranslation(): I18nContextType {
  const context = useContext(I18nContext);
  if (context === undefined) {
    throw new Error("useTranslation must be used within an I18nProvider");
  }
  return context;
}

export function useLocale(): Locale {
  const { locale } = useTranslation();
  return locale;
}

export function useSetLocale(): (locale: Locale) => void {
  const { setLocale } = useTranslation();
  return setLocale;
}

export function useT(): (key: string) => string {
  const { t } = useTranslation();
  return t;
}
