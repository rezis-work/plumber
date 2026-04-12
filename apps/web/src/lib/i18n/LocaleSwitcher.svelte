<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve -- goto uses pathWithLang for ?lang= */
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { SUPPORTED_LOCALES, type AppLocale } from '$lib/i18n/config';
	import { pathWithLang, stripBasePath } from '$lib/i18n/url';

	async function pick(loc: AppLocale) {
		if (loc === page.data.locale) return;
		const pathOnly = stripBasePath(page.url.pathname, base);
		const target = pathWithLang(pathOnly, page.url.searchParams, loc);
		await goto(`${base}${target}`);
	}
</script>

<div class="locale-switcher" role="group" aria-label="Language">
	{#each SUPPORTED_LOCALES as loc (loc)}
		<button
			type="button"
			class="locale-switcher__btn"
			class:locale-switcher__btn--active={page.data.locale === loc}
			aria-current={page.data.locale === loc ? 'true' : undefined}
			onclick={() => pick(loc)}
		>
			{loc.toUpperCase()}
		</button>
	{/each}
</div>

<style>
	.locale-switcher {
		display: inline-flex;
		gap: 0.125rem;
		align-items: center;
		border-radius: var(--radius-md, 0.375rem);
		padding: 0.125rem;
		background: color-mix(in srgb, var(--color-outline-variant, #ccc) 25%, transparent);
	}

	.locale-switcher__btn {
		border: none;
		background: transparent;
		cursor: pointer;
		font-family: inherit;
		font-size: 0.6875rem;
		font-weight: 700;
		letter-spacing: 0.04em;
		padding: 0.25rem 0.4rem;
		border-radius: var(--radius-sm, 0.25rem);
		color: var(--color-text-muted, #666);
	}

	.locale-switcher__btn:hover {
		color: var(--color-text, #111);
		background: color-mix(in srgb, var(--color-surface-elevated, #fff) 80%, transparent);
	}

	.locale-switcher__btn--active {
		color: var(--color-on-primary, #fff);
		background: var(--color-primary, #2563eb);
	}

	.locale-switcher__btn--active:hover {
		color: var(--color-on-primary, #fff);
		background: var(--color-primary-container, #1d4ed8);
	}
</style>
