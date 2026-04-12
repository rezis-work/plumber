<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve -- hrefs use pathWithLang for ?lang= */
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import LocaleSwitcher from '$lib/i18n/LocaleSwitcher.svelte';
	import { pathWithLang } from '$lib/i18n/url';
	import { profilePathForRole } from '$lib/auth/profilePaths';
	import { session } from '$lib/auth/session.svelte';

	const sp = $derived(page.url.searchParams);
	const loc = $derived(page.data.locale);

	const profileHref = $derived(
		session.user
			? `${base}${pathWithLang(profilePathForRole(session.user.role), sp, loc)}`
			: `${base}${pathWithLang('/', sp, loc)}`
	);
</script>

<header class="shell">
	<div class="shell__inner">
		<a class="shell__brand" href={profileHref}>Fixavon</a>
		<nav class="shell__nav" aria-label="Account">
			<LocaleSwitcher />
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
		gap: var(--space-4);
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
