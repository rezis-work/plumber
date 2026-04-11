<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { profilePathForRole } from '$lib/auth/profilePaths';
	import { session } from '$lib/auth/session.svelte';

	let { children } = $props();

	$effect(() => {
		if (!browser) return;
		if (session.hydrating) return;
		if (session.user !== null && session.accessToken !== null) {
			// eslint-disable-next-line svelte/no-navigation-without-resolve
			void goto(`${base}${profilePathForRole(session.user.role)}`, { replaceState: true });
		}
	});
</script>

{#if session.hydrating}
	<div class="gate">
		<p class="gate__text">Loading…</p>
	</div>
{:else if session.user !== null && session.accessToken !== null}
	<div class="gate">
		<p class="gate__text">Redirecting…</p>
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
