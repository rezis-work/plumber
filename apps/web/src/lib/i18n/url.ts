import {
	DEFAULT_LOCALE,
	type AppLocale,
	isAppLocale,
	resolveLocaleFromLangParam
} from './config';

export const LANG_QUERY = 'lang';

function searchStringFromLocaleAndRest(locale: AppLocale, rest: URLSearchParams): string {
	const parts: string[] = [`${LANG_QUERY}=${encodeURIComponent(locale)}`];
	for (const [k, v] of rest.entries()) {
		if (k === LANG_QUERY) continue;
		parts.push(`${encodeURIComponent(k)}=${encodeURIComponent(v)}`);
	}
	return parts.join('&');
}

/** Query string with `lang` first, then other keys (excluding any prior `lang`). */
export function searchParamsWithLangFirst(url: URL, locale: AppLocale): string {
	const rest = new URLSearchParams();
	for (const [k, v] of url.searchParams.entries()) {
		if (k === LANG_QUERY) continue;
		rest.append(k, v);
	}
	return searchStringFromLocaleAndRest(locale, rest);
}

/** Merge `source` params with `locale`; `lang` is always first in the result. */
export function pathWithLang(
	pathname: string,
	source: URLSearchParams | string,
	locale: AppLocale
): string {
	const raw = typeof source === 'string' ? source.replace(/^\?/, '') : source.toString();
	const params = new URLSearchParams(raw);
	const rest = new URLSearchParams();
	for (const [k, v] of params.entries()) {
		if (k === LANG_QUERY) continue;
		rest.append(k, v);
	}
	const qs = searchStringFromLocaleAndRest(locale, rest);
	return `${pathname}?${qs}`;
}

export function shouldNormalizeLangQuery(url: URL): boolean {
	return !isAppLocale(url.searchParams.get(LANG_QUERY));
}

/** Full URL with normalized `lang` (for redirects). */
export function normalizedUrlForRequest(url: URL): URL {
	const locale = resolveLocaleFromLangParam(url.searchParams.get(LANG_QUERY));
	const next = new URL(url);
	next.search = searchParamsWithLangFirst(url, locale);
	return next;
}

/** Browser-only: locale from current `window` URL (for client `goto` without `page`). */
export function resolveLocaleFromCurrentWindow(): AppLocale {
	if (typeof window === 'undefined') return DEFAULT_LOCALE;
	return resolveLocaleFromLangParam(new URLSearchParams(window.location.search).get(LANG_QUERY));
}

/** `pathname?lang=…` using current browser search params (client-only). */
export function pathWithLangFromWindow(pathname: string): string {
	if (typeof window === 'undefined') {
		return pathWithLang(pathname, '', DEFAULT_LOCALE);
	}
	return pathWithLang(pathname, window.location.search, resolveLocaleFromCurrentWindow());
}

/** Path relative to `config.kit.paths.base` (leading segment only) for composing `${base}${…}`. */
export function stripBasePath(pathname: string, basePath: string): string {
	if (!basePath) return pathname;
	if (pathname === basePath || pathname === `${basePath}/`) return '/';
	if (pathname.startsWith(`${basePath}/`)) return pathname.slice(basePath.length);
	return pathname;
}
