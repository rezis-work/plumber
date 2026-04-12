import { redirect, type Handle } from '@sveltejs/kit';
import { resolveLocaleFromLangParam } from '$lib/i18n/config';
import { searchParamsWithLangFirst, shouldNormalizeLangQuery } from '$lib/i18n/url';

/** 307: preserve method; consider 301 later for production HTML navigations only. */
const LANG_REDIRECT_STATUS = 307;

const STATIC_LIKE_PATH = /\.(?:svg|png|ico|jpg|jpeg|gif|webp|woff2?|ttf|css|js|map|txt|xml)$/i;

function shouldSkipLangRedirect(pathname: string): boolean {
	if (pathname.startsWith('/_app/')) return true;
	if (STATIC_LIKE_PATH.test(pathname)) return true;
	return false;
}

export const handle: Handle = async ({ event, resolve }) => {
	const htmlLang = resolveLocaleFromLangParam(event.url.searchParams.get('lang'));
	const resolvePage = () =>
		resolve(event, {
			transformPageChunk: ({ html }) =>
				html.includes('%lang%') ? html.replaceAll('%lang%', htmlLang) : html
		});

	if (event.request.method !== 'GET' && event.request.method !== 'HEAD') {
		return resolvePage();
	}
	if (shouldSkipLangRedirect(event.url.pathname)) {
		return resolvePage();
	}
	if (shouldNormalizeLangQuery(event.url)) {
		const locale = resolveLocaleFromLangParam(event.url.searchParams.get('lang'));
		const qs = searchParamsWithLangFirst(event.url, locale);
		redirect(LANG_REDIRECT_STATUS, `${event.url.pathname}?${qs}${event.url.hash}`);
	}
	return resolvePage();
};
