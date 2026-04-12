import type { AppLocale } from '$lib/i18n/config';
import { SUPPORTED_LOCALES } from '$lib/i18n/config';

/** Open Graph locale tags (Facebook / OGP), aligned with ADR 003 `AppLocale`. */
const OG_LOCALE: Record<AppLocale, string> = {
	en: 'en_US',
	ka: 'ka_GE',
	ru: 'ru_RU'
};

export function ogLocaleFor(locale: AppLocale): string {
	return OG_LOCALE[locale];
}

/** Other site locales as `og:locale:alternate` values. */
export function ogLocaleAlternates(locale: AppLocale): string[] {
	return SUPPORTED_LOCALES.filter((l) => l !== locale).map((l) => OG_LOCALE[l]);
}
