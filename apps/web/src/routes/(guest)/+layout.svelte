<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { pathWithLang } from '$lib/i18n/url';
	import { translate } from '$lib/i18n/translate';
	import { profilePathForRole } from '$lib/auth/profilePaths';
	import { session } from '$lib/auth/session.svelte';

	let { children } = $props();

	const loading = $derived(translate(page.data.locale, 'common.loading'));
	const redirecting = $derived(translate(page.data.locale, 'common.redirecting'));

	$effect(() => {
		if (!browser) return;
		if (session.hydrating) return;
		if (session.user !== null && session.accessToken !== null) {
			const path = pathWithLang(
				profilePathForRole(session.user.role),
				page.url.searchParams,
				page.data.locale
			);
			// eslint-disable-next-line svelte/no-navigation-without-resolve
			void goto(`${base}${path}`, { replaceState: true });
		}
	});
</script>

{#if session.hydrating}
	<div class="gate">
		<p class="gate__text">{loading}</p>
	</div>
{:else if session.user !== null && session.accessToken !== null}
	<div class="gate">
		<p class="gate__text">{redirecting}</p>
	</div>
{:else}
	{@render children()}
{/if}

<style>
	.gate {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-surface);
	}

	.gate__text {
		margin: 0;
		color: var(--color-text-muted);
		font-size: var(--text-sm);
	}
</style>
