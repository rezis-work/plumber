<script lang="ts">
	import { base } from '$app/paths';
</script>

<svelte:head>
	<title>Become a plumber | Fixavon</title>
</svelte:head>

<main class="placeholder">
	<p class="placeholder__eyebrow">Phase C</p>
	<h1 class="placeholder__title">Plumber registration</h1>
	<p class="placeholder__text">
		This route is reserved for <strong>Plumber Registration (C1b)</strong>. Application flow will ship in Phase C.
	</p>
	<p class="placeholder__links">
		<a href="{base}/">← Home</a>
		·
		<a href="{base}/register">Client sign up</a>
	</p>
</main>

<style>
	.placeholder {
		max-width: 32rem;
		margin: 0 auto;
		padding: var(--space-24) var(--space-4);
		min-height: 60vh;
	}

	.placeholder__eyebrow {
		margin: 0 0 var(--space-2);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-semibold);
		color: var(--color-primary);
		text-transform: uppercase;
		letter-spacing: 0.06em;
	}

	.placeholder__title {
		margin: 0 0 var(--space-4);
		font-size: var(--text-lg);
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.placeholder__text {
		margin: 0 0 var(--space-8);
		color: var(--color-text-muted);
		line-height: 1.6;
	}

	.placeholder__links {
		margin: 0;
		font-size: var(--text-sm);
	}

	.placeholder__links a {
		color: var(--color-primary);
		font-weight: var(--font-weight-medium);
	}
</style>
