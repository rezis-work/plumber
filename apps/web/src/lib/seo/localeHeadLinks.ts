import {
	DEFAULT_LOCALE,
	SUPPORTED_LOCALES,
	type AppLocale,
	resolveLocaleFromLangParam
} from '$lib/i18n/config';
import { searchParamsWithLangFirst } from '$lib/i18n/url';

/** One `link rel="alternate"` row for hreflang (or x-default). */
export type HreflangAlternate = { hreflang: string; href: string };

/**
 * Canonical and hreflang URLs for the current path and query.
 * Query order is always `lang` first, then other keys (see `searchParamsWithLangFirst`).
 */
export function localeHeadLinks(
	pageUrl: URL,
	siteOrigin: string
): { canonicalHref: string; alternates: HreflangAlternate[] } {
	const origin = siteOrigin.replace(/\/$/, '');
	const { pathname } = pageUrl;
	const currentLocale = resolveLocaleFromLangParam(pageUrl.searchParams.get('lang'));

	const hrefForLocale = (loc: AppLocale): string => {
		const qs = searchParamsWithLangFirst(pageUrl, loc);
		return `${origin}${pathname}?${qs}`;
	};

	const canonicalHref = hrefForLocale(currentLocale);

	const alternates: HreflangAlternate[] = SUPPORTED_LOCALES.map((loc) => ({
		hreflang: loc,
		href: hrefForLocale(loc)
	}));

	alternates.push({
		hreflang: 'x-default',
		href: hrefForLocale(DEFAULT_LOCALE)
	});

	return { canonicalHref, alternates };
}
