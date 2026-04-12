// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
import type { AppLocale } from '$lib/i18n/config';
import type { MessageCatalog } from '$lib/i18n/translate';

declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
		interface LayoutData {
			locale: AppLocale;
			messages: MessageCatalog;
			/** Public site origin (no trailing slash); canonical/hreflang/og:url. */
			siteOrigin: string;
		}
	}
}

export {};
