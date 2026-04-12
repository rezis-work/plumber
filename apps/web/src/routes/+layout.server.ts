import { env } from '$env/dynamic/public';
import { resolveLocaleFromLangParam } from '$lib/i18n/config';
import { messageCatalogForLocale } from '$lib/i18n/translate';
import type { LayoutServerLoad } from './$types';

function siteOriginForRequest(url: URL): string {
	const raw = env.PUBLIC_SITE_URL?.trim() ?? '';
	if (raw) return raw.replace(/\/$/, '');
	return url.origin;
}

export const load: LayoutServerLoad = ({ url }) => {
	const locale = resolveLocaleFromLangParam(url.searchParams.get('lang'));
	return {
		locale,
		messages: messageCatalogForLocale(locale),
		siteOrigin: siteOriginForRequest(url)
	};
};
