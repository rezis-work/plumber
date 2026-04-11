<script lang="ts">
	import { base } from '$app/paths';
	import { profilePathForRole } from '$lib/auth/profilePaths';
	import { session } from '$lib/auth/session.svelte';

	const profileHref = $derived(
		session.user ? `${base}${profilePathForRole(session.user.role)}` : `${base}/`
	);
</script>

<header class="shell">
	<div class="shell__inner">
		<a class="shell__brand" href={profileHref}>Fixavon</a>
		<nav class="shell__nav" aria-label="Account">
			{#if session.user}
				<a class="shell__link" href={profileHref}>Profile</a>
			{/if}
		</nav>
	</div>
</header>

<style>
	.shell {
		border-bottom: 1px solid color-mix(in srgb, var(--color-outline-variant) 35%, transparent);
		background: var(--color-surface-elevated);
	}

	.shell__inner {
		max-width: 56rem;
		margin: 0 auto;
		padding: var(--space-4) var(--space-6);
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-4);
	}

	.shell__brand {
		font-size: 1.125rem;
		font-weight: var(--font-weight-bold);
		color: var(--color-primary);
		text-decoration: none;
	}

	.shell__brand:hover {
		color: var(--color-primary-container);
	}

	.shell__nav {
		display: flex;
		align-items: center;
		gap: var(--space-6);
	}

	.shell__link {
		font-size: var(--text-sm);
		font-weight: var(--font-weight-semibold);
		color: var(--color-text);
		text-decoration: none;
	}

	.shell__link:hover {
		color: var(--color-primary);
	}
</style>
