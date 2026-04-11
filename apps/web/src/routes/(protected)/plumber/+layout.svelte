<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { session } from '$lib/auth/session.svelte';

	let { children } = $props();

	$effect(() => {
		if (!browser) return;
		if (session.hydrating) return;
		if (!session.user) return;
		if (session.user.role !== 'plumber') {
			void goto(`${base}/forbidden`, { replaceState: true });
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
