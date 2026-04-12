import { describe, expect, it } from 'vitest';
import type { AppLocale } from './config';
import { formatIcuMessage, lookup, translate } from './translate';
import en from './messages/en.json';

describe('lookup', () => {
	it('returns nested string', () => {
		expect(lookup(en, 'marketing.landing.title')).toContain('Fixavon');
	});

	it('returns undefined for missing path', () => {
		expect(lookup(en, 'nope')).toBeUndefined();
	});
});

describe('formatIcuMessage', () => {
	it('interpolates simple placeholders', () => {
		expect(formatIcuMessage('Hello {name}!', 'en', { name: 'Ada' })).toBe('Hello Ada!');
	});

	it('handles plural rules', () => {
		const tpl = '{count, plural, one {# item} other {# items}}';
		expect(formatIcuMessage(tpl, 'en', { count: 1 })).toBe('1 item');
		expect(formatIcuMessage(tpl, 'en', { count: 5 })).toBe('5 items');
	});
});

describe('translate', () => {
	it('returns locale-specific copy', () => {
		const kaTitle = translate('ka' as AppLocale, 'marketing.landing.title');
		expect(kaTitle).toContain('თბილის');
		const ruTitle = translate('ru' as AppLocale, 'marketing.landing.title');
		expect(ruTitle).toContain('Тбилиси');
	});

	it('returns key when missing everywhere', () => {
		expect(translate('en' as AppLocale, 'missing.key.path')).toBe('missing.key.path');
	});

	it('formats ICU from catalog when values are passed', () => {
		const out = translate('en' as AppLocale, 'marketing.hero.badge', { count: 2000 });
		expect(out).toMatch(/2[,\u202f\u00a0]?000/);
		expect(out.toLowerCase()).toContain('tbilisi');
	});
});
