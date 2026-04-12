<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { pathWithLang } from '$lib/i18n/url';
	import { session } from '$lib/auth/session.svelte';

	let { children } = $props();

	$effect(() => {
		if (!browser) return;
		if (session.hydrating) return;
		if (!session.user) return;
		if (session.user.role !== 'plumber') {
			const path = pathWithLang('/forbidden', page.url.searchParams, page.data.locale);
			// eslint-disable-next-line svelte/no-navigation-without-resolve
			void goto(`${base}${path}`, { replaceState: true });
		}
	});
</script>

{#if !session.hydrating && session.user?.role === 'plumber'}
	{@render children()}
{:else if session.hydrating}
	<div class="role-gate" aria-hidden="true"></div>
{/if}

<style>
	.role-gate {
		min-height: 4rem;
	}
</style>
