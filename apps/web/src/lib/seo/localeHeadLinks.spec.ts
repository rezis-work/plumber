import { describe, expect, it } from 'vitest';
import { localeHeadLinks } from './localeHeadLinks';

describe('localeHeadLinks', () => {
	it('canonical uses siteOrigin and current lang from URL', () => {
		const pageUrl = new URL('https://internal.example/login?lang=en');
		const { canonicalHref } = localeHeadLinks(pageUrl, 'https://public.example');
		expect(canonicalHref).toBe('https://public.example/login?lang=en');
	});

	it('lists en, ka, ru and x-default pointing at default locale URL', () => {
		const pageUrl = new URL('https://x.test/login?lang=ru');
		const { alternates } = localeHeadLinks(pageUrl, 'https://x.test');
		expect(alternates).toHaveLength(4);
		expect(alternates.map((a) => a.hreflang)).toEqual(['en', 'ka', 'ru', 'x-default']);
		expect(alternates.find((a) => a.hreflang === 'en')!.href).toBe('https://x.test/login?lang=en');
		expect(alternates.find((a) => a.hreflang === 'ka')!.href).toBe('https://x.test/login?lang=ka');
		expect(alternates.find((a) => a.hreflang === 'ru')!.href).toBe('https://x.test/login?lang=ru');
		expect(alternates.find((a) => a.hreflang === 'x-default')!.href).toBe(
			'https://x.test/login?lang=ka'
		);
	});

	it('puts lang first and keeps other query keys', () => {
		const pageUrl = new URL('https://x.test/login?verified=1&lang=en');
		const { canonicalHref, alternates } = localeHeadLinks(pageUrl, 'https://x.test');
		expect(canonicalHref).toBe('https://x.test/login?lang=en&verified=1');
		expect(alternates.find((a) => a.hreflang === 'ru')!.href).toBe(
			'https://x.test/login?lang=ru&verified=1'
		);
	});

	it('uses default locale in canonical when lang query missing', () => {
		const pageUrl = new URL('https://x.test/login?verified=1');
		const { canonicalHref } = localeHeadLinks(pageUrl, 'https://x.test');
		expect(canonicalHref).toBe('https://x.test/login?lang=ka&verified=1');
	});
});
