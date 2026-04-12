/**
 * Locale allowlist and defaults — mirrors ADR 003 appendix (Implementation 002 T0).
 * URL routing (`?lang=`) is enforced in `hooks.server.ts` + `url.ts` (T1); this module is the typed source for allowlist / default / message fallback.
 */

export const SUPPORTED_LOCALES = ['en', 'ka', 'ru'] as const;

export type AppLocale = (typeof SUPPORTED_LOCALES)[number];

export const DEFAULT_LOCALE: AppLocale = 'ka';

const FALLBACK_AFTER_LOCALE: Record<AppLocale, readonly AppLocale[]> = {
	en: [],
	ka: ['en'],
	ru: ['en']
};

/** Locales to try after the active locale when a message key is missing (in order). */
export function messageFallbackLocales(locale: AppLocale): readonly AppLocale[] {
	return FALLBACK_AFTER_LOCALE[locale];
}

export function isAppLocale(value: string | null | undefined): value is AppLocale {
	return value !== null && value !== undefined && SUPPORTED_LOCALES.includes(value as AppLocale);
}

/** Resolve URL `lang` query: allowlisted value or default. */
export function resolveLocaleFromLangParam(lang: string | null | undefined): AppLocale {
	if (isAppLocale(lang)) return lang;
	return DEFAULT_LOCALE;
}
