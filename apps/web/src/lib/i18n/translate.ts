import IntlMessageFormat from 'intl-messageformat';
import type { AppLocale } from './config';
import { messageFallbackLocales } from './config';
import en from './messages/en.json';
import ka from './messages/ka.json';
import ru from './messages/ru.json';

export type MessageCatalog = typeof en;

export type TranslateValues = Record<string, string | number | Date | boolean>;

const CATALOGS: Record<AppLocale, MessageCatalog> = {
	en,
	ka,
	ru
};

const isDev = import.meta.env.DEV;

export function messageCatalogForLocale(locale: AppLocale): MessageCatalog {
	return CATALOGS[locale];
}

/** Walk nested objects by dot-separated key; returns a string leaf or undefined. */
export function lookup(catalog: MessageCatalog, dotKey: string): string | undefined {
	const parts = dotKey.split('.');
	let cur: unknown = catalog;
	for (const p of parts) {
		if (cur === null || typeof cur !== 'object' || !Object.prototype.hasOwnProperty.call(cur, p)) {
			return undefined;
		}
		cur = (cur as Record<string, unknown>)[p];
	}
	return typeof cur === 'string' ? cur : undefined;
}

/** Format an ICU MessageFormat template; `locale` drives plural/number/date rules. */
export function formatIcuMessage(
	template: string,
	locale: AppLocale,
	values?: TranslateValues
): string {
	const hasIcu = /\{/.test(template);
	const hasValues = values !== undefined && Object.keys(values).length > 0;
	if (!hasIcu && !hasValues) return template;

	const tryFormat = (loc: string): string => {
		const mf = new IntlMessageFormat(template, loc);
		return String(mf.format((values ?? {}) as Record<string, string | number | boolean | Date | null | undefined>));
	};

	try {
		return tryFormat(locale);
	} catch {
		try {
			return tryFormat('en');
		} catch {
			return template;
		}
	}
}

type ResolveResult =
	| { template: string; catalogLocale: AppLocale }
	| { template: undefined; catalogLocale: null };

function resolveTemplate(locale: AppLocale, dotKey: string): ResolveResult {
	const primary = lookup(CATALOGS[locale], dotKey);
	if (primary !== undefined) {
		return { template: primary, catalogLocale: locale };
	}

	for (const fb of messageFallbackLocales(locale)) {
		const v = lookup(CATALOGS[fb], dotKey);
		if (v !== undefined) {
			if (isDev && fb !== locale) {
				console.debug(
					`[i18n] "${dotKey}" resolved from fallback locale "${fb}" (requested "${locale}")`
				);
			}
			return { template: v, catalogLocale: fb };
		}
	}

	return { template: undefined, catalogLocale: null };
}

/**
 * Resolve a message for `locale` using catalogs and ADR fallback order, then apply ICU when needed.
 * Returns `dotKey` if nothing is found (and warns in dev).
 */
export function translate(locale: AppLocale, dotKey: string, values?: TranslateValues): string {
	const resolved = resolveTemplate(locale, dotKey);
	if (resolved.template === undefined) {
		if (isDev) {
			console.warn(`[i18n] Missing key: "${dotKey}" (locale "${locale}")`, values ?? '');
		}
		return dotKey;
	}

	return formatIcuMessage(resolved.template, locale, values);
}
