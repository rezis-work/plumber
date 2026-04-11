<script lang="ts">
	import { base } from '$app/paths';
	import { profilePathForRole } from '$lib/auth/profilePaths';
	import { session } from '$lib/auth/session.svelte';

	const profileHref = $derived(
		session.user && session.accessToken
			? `${base}${profilePathForRole(session.user.role)}`
			: null
	);
</script>

<svelte:head>
	<title>Access denied | Fixavon</title>
</svelte:head>

<div class="page">
	<div class="card">
		<h1 class="title">Access denied</h1>
		<p class="body">You don’t have permission to view this page.</p>
		<div class="actions">
			{#if profileHref}
				<a class="btn btn--primary" href={profileHref}>Go to your profile</a>
			{:else}
				<a class="btn btn--primary" href={`${base}/login`}>Log in</a>
				<a class="btn btn--ghost" href={`${base}/`}>Home</a>
			{/if}
		</div>
	</div>
</div>

<style>
	.page {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-8) var(--space-6);
		background: var(--color-surface);
	}

	.card {
		max-width: 24rem;
		width: 100%;
		padding: var(--space-10) var(--space-8);
		background: var(--color-surface-elevated);
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-ambient);
		border: 1px solid color-mix(in srgb, var(--color-outline-variant) 35%, transparent);
	}

	.title {
		margin: 0 0 var(--space-4);
		font-size: 1.5rem;
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.body {
		margin: 0 0 var(--space-8);
		color: var(--color-text-muted);
		font-size: var(--text-sm);
		line-height: 1.5;
	}

	.actions {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-4);
	}

	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.625rem 1.25rem;
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-semibold);
		text-decoration: none;
		border: none;
		cursor: pointer;
		font-family: inherit;
	}

	.btn--primary {
		background: var(--color-primary);
		color: var(--color-on-primary);
	}

	.btn--primary:hover {
		background: var(--color-primary-container);
	}

	.btn--ghost {
		background: transparent;
		color: var(--color-text);
		border: 1px solid var(--color-outline-variant);
	}

	.btn--ghost:hover {
		background: var(--color-surface-container-low);
	}
</style>
