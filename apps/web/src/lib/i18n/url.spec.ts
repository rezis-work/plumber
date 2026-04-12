import { describe, expect, it } from 'vitest';
import type { AppLocale } from './config';
import { pathWithLang, searchParamsWithLangFirst, shouldNormalizeLangQuery, stripBasePath } from './url';

describe('searchParamsWithLangFirst', () => {
	it('puts lang first and drops duplicate lang', () => {
		const u = new URL('https://x.test/login?verified=1&lang=xx&foo=bar');
		expect(searchParamsWithLangFirst(u, 'en')).toBe('lang=en&verified=1&foo=bar');
	});

	it('handles empty rest', () => {
		const u = new URL('https://x.test/');
		expect(searchParamsWithLangFirst(u, 'ka' as AppLocale)).toBe('lang=ka');
	});
});

describe('pathWithLang', () => {
	it('merges from URLSearchParams', () => {
		const sp = new URLSearchParams('verified=1');
		expect(pathWithLang('/login', sp, 'ru')).toBe('/login?lang=ru&verified=1');
	});
});

describe('shouldNormalizeLangQuery', () => {
	it('is true when lang missing', () => {
		expect(shouldNormalizeLangQuery(new URL('https://x.test/login'))).toBe(true);
	});

	it('is false when lang valid', () => {
		expect(shouldNormalizeLangQuery(new URL('https://x.test/login?lang=en'))).toBe(false);
	});

	it('is true when lang invalid', () => {
		expect(shouldNormalizeLangQuery(new URL('https://x.test/?lang=de'))).toBe(true);
	});
});

describe('stripBasePath', () => {
	it('returns pathname when base empty', () => {
		expect(stripBasePath('/login', '')).toBe('/login');
	});

	it('strips base prefix', () => {
		expect(stripBasePath('/app/login', '/app')).toBe('/login');
	});

	it('maps base root to slash', () => {
		expect(stripBasePath('/app', '/app')).toBe('/');
	});
});
