<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve -- links use pathWithLang for ?lang= */
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { pathWithLang } from '$lib/i18n/url';
	import { translate } from '$lib/i18n/translate';
	import SeoHead from '$lib/seo/SeoHead.svelte';
	import { profilePathForRole } from '$lib/auth/profilePaths';
	import { session } from '$lib/auth/session.svelte';

	const sp = $derived(page.url.searchParams);
	const loc = $derived(page.data.locale);

	const profileHref = $derived(
		session.user && session.accessToken
			? `${base}${pathWithLang(profilePathForRole(session.user.role), sp, loc)}`
			: null
	);
	const hrefLogin = $derived(`${base}${pathWithLang('/login', sp, loc)}`);
	const hrefHome = $derived(`${base}${pathWithLang('/', sp, loc)}`);

	const pageTitle = $derived(translate(loc, 'error.forbidden.title'));
	const pageDescription = $derived(translate(loc, 'error.forbidden.metaDescription'));
	const heading = $derived(translate(loc, 'error.forbidden.heading'));
	const body = $derived(translate(loc, 'error.forbidden.body'));
	const goProfile = $derived(translate(loc, 'error.forbidden.goProfile'));
	const logIn = $derived(translate(loc, 'error.forbidden.logIn'));
	const home = $derived(translate(loc, 'error.forbidden.home'));
</script>

<SeoHead
	title={pageTitle}
	description={pageDescription}
	locale={loc}
	url={page.url}
	siteOrigin={page.data.siteOrigin}
/>

<div class="page">
	<div class="card">
		<h1 class="title">{heading}</h1>
		<p class="body">{body}</p>
		<div class="actions">
			{#if profileHref}
				<a class="btn btn--primary" href={profileHref}>{goProfile}</a>
			{:else}
				<a class="btn btn--primary" href={hrefLogin}>{logIn}</a>
				<a class="btn btn--ghost" href={hrefHome}>{home}</a>
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
