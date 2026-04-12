<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve -- links use pathWithLang for ?lang= */
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { ApiError, authVerifyEmail } from '$lib/api/client';
	import { translateAuthApiError } from '$lib/auth/translateApiError';
	import { pathWithLang } from '$lib/i18n/url';
	import { translate } from '$lib/i18n/translate';
	import SeoHead from '$lib/seo/SeoHead.svelte';
	import {
		PENDING_EMAIL_VERIFICATION_KEY,
		type PendingEmailVerification
	} from '$lib/auth/pendingVerification';

	/** Guard deep-link query values (avoid huge pasted strings). */
	const URL_TOKEN_MAX_LEN = 128;

	let token = $state('');
	let clientError = $state<string | null>(null);
	let submitting = $state(false);
	let succeeded = $state(false);
	/** Dev-only: expiry from registration handoff */
	let devExpiresAt = $state<string | null>(null);

	const sp = $derived(page.url.searchParams);
	const loc = $derived(page.data.locale);
	const hrefHome = $derived(`${base}${pathWithLang('/', sp, loc)}`);
	const hrefLogin = $derived(`${base}${pathWithLang('/login', sp, loc)}`);

	const pageTitle = $derived(translate(loc, 'auth.verify.title'));
	const pageDescription = $derived(translate(loc, 'auth.verify.metaDescription'));

	function readUrlToken(): string {
		if (!browser) return '';
		const params = new URLSearchParams(window.location.search);
		const raw = params.get('token')?.trim() ?? '';
		if (raw.length > URL_TOKEN_MAX_LEN) return '';
		return raw;
	}

	onMount(() => {
		if (!browser) return;

		let fromStorage = '';
		const raw = sessionStorage.getItem(PENDING_EMAIL_VERIFICATION_KEY);
		if (raw) {
			try {
				const p = JSON.parse(raw) as PendingEmailVerification;
				fromStorage = p.token ?? '';
				if (import.meta.env.DEV && p.expires_at) {
					devExpiresAt = p.expires_at;
				}
			} catch {
				fromStorage = '';
			}
			sessionStorage.removeItem(PENDING_EMAIL_VERIFICATION_KEY);
		}

		if (fromStorage) {
			token = fromStorage;
		} else {
			const fromUrl = readUrlToken();
			if (fromUrl) token = fromUrl;
		}
	});

	async function onsubmit(e: Event) {
		e.preventDefault();
		clientError = null;
		const t = token.trim();
		if (!t) {
			clientError = translate(loc, 'auth.verify.enterToken');
			return;
		}

		submitting = true;
		try {
			await authVerifyEmail({ token: t });
			succeeded = true;
			await new Promise((r) => setTimeout(r, 900));
			// eslint-disable-next-line svelte/prefer-svelte-reactivity -- one-shot query merge for post-verify redirect
			const loginParams = new URLSearchParams(page.url.searchParams);
			loginParams.set('verified', '1');
			await goto(`${base}${pathWithLang('/login', loginParams, page.data.locale)}`);
		} catch (err) {
			if (err instanceof ApiError) {
				clientError = translateAuthApiError(loc, err);
			} else {
				clientError = translate(loc, 'auth.api.network');
			}
		} finally {
			submitting = false;
		}
	}
</script>

<SeoHead
	title={pageTitle}
	description={pageDescription}
	locale={loc}
	url={page.url}
	siteOrigin={page.data.siteOrigin}
/>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0&display=swap"
	/>
</svelte:head>

<div class="page">
	<div class="blob blob--tl" aria-hidden="true"></div>
	<div class="blob blob--br" aria-hidden="true"></div>

	<main class="main">
		<div class="card">
			<div class="brand">
				<a class="brand__link" href={hrefHome}>{translate(loc, 'auth.shared.brand')}</a>
			</div>

			<div class="body">
				<div class="hero-icon" aria-hidden="true">
					<div class="hero-icon__glow"></div>
					<div class="hero-icon__circle">
						<span class="material-symbols-outlined hero-icon__mail">mark_email_unread</span>
					</div>
					<div class="hero-icon__badge">
						<span class="material-symbols-outlined hero-icon__check">check</span>
					</div>
				</div>

				<h1 class="title">{translate(loc, 'auth.verify.heading')}</h1>
				<p class="lead">
					{translate(loc, 'auth.verify.lead')}
				</p>

				{#if import.meta.env.DEV && devExpiresAt}
					<p class="dev-hint">{translate(loc, 'auth.verify.devExpires', { expiresAt: devExpiresAt })}</p>
				{/if}

				{#if succeeded}
					<p class="success" role="status">{translate(loc, 'auth.verify.success')}</p>
				{:else}
					<form class="form" onsubmit={onsubmit}>
						<label class="label" for="verify-token">{translate(loc, 'auth.verify.codeLabel')}</label>
						<input
							id="verify-token"
							class="input"
							type="text"
							name="token"
							autocomplete="one-time-code"
							spellcheck="false"
							placeholder={translate(loc, 'auth.verify.codePlaceholder')}
							bind:value={token}
							disabled={submitting}
						/>

						{#if clientError}
							<p class="error" role="alert">{clientError}</p>
						{/if}

						<button class="btn btn--primary" type="submit" disabled={submitting}>
							{submitting ? translate(loc, 'auth.verify.verifying') : translate(loc, 'auth.verify.submit')}
						</button>

						<button class="btn btn--ghost" type="button" disabled aria-disabled="true">
							{translate(loc, 'auth.verify.resend')}
							<span class="btn__soon">{translate(loc, 'auth.verify.comingSoon')}</span>
						</button>
					</form>
				{/if}

				<a class="back" href={hrefLogin}>
					<span class="material-symbols-outlined back__icon">arrow_back</span>
					{translate(loc, 'auth.verify.backToLogin')}
				</a>

				<div class="hints">
					<div class="hints__item">
						<span class="material-symbols-outlined hints__icon">info</span>
						<span>{translate(loc, 'auth.verify.hintSpam')}</span>
					</div>
					<div class="hints__item">
						<span class="material-symbols-outlined hints__icon">schedule</span>
						<span>{translate(loc, 'auth.verify.hintExpiry')}</span>
					</div>
				</div>
			</div>

			<div class="trust">
				<p class="trust__text">{translate(loc, 'auth.verify.trustLine')}</p>
			</div>
		</div>
	</main>
</div>

<style>
	.page {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
		background: var(--color-surface);
		position: relative;
		overflow-x: hidden;
	}

	.blob {
		position: absolute;
		width: 40%;
		height: 40%;
		border-radius: 50%;
		filter: blur(120px);
		pointer-events: none;
		opacity: 0.35;
	}

	.blob--tl {
		top: -10%;
		left: -10%;
		background: var(--color-primary);
	}

	.blob--br {
		bottom: -10%;
		right: -10%;
		background: var(--color-secondary);
	}

	.main {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-6) var(--space-4);
		position: relative;
		z-index: 1;
	}

	.card {
		width: 100%;
		max-width: 42rem;
		background: var(--color-surface-elevated);
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-ambient);
		overflow: hidden;
	}

	.brand {
		padding-top: var(--space-12);
		display: flex;
		justify-content: center;
	}

	.brand__link {
		font-size: 1.875rem;
		font-weight: 900;
		color: var(--color-primary);
		text-decoration: none;
		letter-spacing: -0.02em;
	}

	.brand__link:hover {
		text-decoration: underline;
	}

	.body {
		padding: var(--space-8) var(--space-8) var(--space-16);
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
	}

	.hero-icon {
		position: relative;
		margin-bottom: var(--space-8);
	}

	.hero-icon__glow {
		position: absolute;
		inset: 0;
		background: color-mix(in srgb, var(--color-primary) 12%, transparent);
		border-radius: 50%;
		transform: scale(1.5);
		filter: blur(24px);
	}

	.hero-icon__circle {
		position: relative;
		width: 6rem;
		height: 6rem;
		border-radius: 50%;
		background: linear-gradient(
			135deg,
			var(--color-primary) 0%,
			var(--color-primary-container) 100%
		);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-on-primary);
		box-shadow: var(--shadow-ambient);
	}

	.hero-icon__mail {
		font-size: 2.5rem !important;
		font-variation-settings: 'FILL' 1, 'wght' 400, 'GRAD' 0, 'opsz' 24;
	}

	.hero-icon__badge {
		position: absolute;
		bottom: -0.35rem;
		right: -0.35rem;
		background: var(--color-surface-elevated);
		padding: 0.2rem;
		border-radius: 999px;
		box-shadow: var(--shadow-ambient);
	}

	.hero-icon__check {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 1.75rem;
		height: 1.75rem;
		font-size: 0.95rem !important;
		background: var(--color-secondary);
		color: var(--color-on-primary);
		border-radius: 50%;
		font-variation-settings: 'FILL' 0, 'wght' 700, 'GRAD' 0, 'opsz' 24;
	}

	.title {
		margin: 0 0 var(--space-4);
		font-size: clamp(1.75rem, 4vw, 2.25rem);
		font-weight: 800;
		color: var(--color-text);
		letter-spacing: -0.02em;
	}

	.lead {
		margin: 0 0 var(--space-8);
		max-width: 28rem;
		font-size: var(--text-lg);
		color: var(--color-text-muted);
		line-height: 1.6;
	}

	.dev-hint {
		margin: 0 0 var(--space-4);
		font-size: var(--text-sm);
		color: var(--color-primary);
		font-weight: var(--font-weight-semibold);
	}

	.form {
		width: 100%;
		max-width: 22rem;
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		text-align: left;
	}

	.label {
		font-size: var(--text-sm);
		font-weight: var(--font-weight-semibold);
		color: var(--color-text);
	}

	.input {
		width: 100%;
		padding: var(--space-4);
		border-radius: var(--radius-lg);
		border: 1px solid var(--color-outline-variant);
		background: var(--color-surface-container-low);
		color: var(--color-text);
		font-family: ui-monospace, monospace;
		font-size: var(--text-sm);
	}

	.input:focus {
		outline: 2px solid var(--color-primary);
		outline-offset: 2px;
	}

	.input:disabled {
		opacity: 0.7;
	}

	.error {
		margin: 0;
		font-size: var(--text-sm);
		color: var(--color-error);
	}

	.success {
		margin: 0 0 var(--space-4);
		font-size: var(--text-lg);
		font-weight: var(--font-weight-semibold);
		color: var(--color-success);
	}

	.btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		width: 100%;
		padding: var(--space-4) var(--space-8);
		border-radius: var(--radius-lg);
		font-size: var(--text-lg);
		font-weight: var(--font-weight-bold);
		font-family: inherit;
		cursor: pointer;
		border: none;
		transition:
			transform 0.15s ease,
			opacity 0.15s ease;
	}

	.btn:disabled {
		cursor: not-allowed;
		opacity: 0.65;
	}

	.btn--primary {
		background: var(--color-primary);
		color: var(--color-on-primary);
		box-shadow: var(--shadow-ambient);
	}

	.btn--primary:not(:disabled):hover {
		background: var(--color-primary-container);
	}

	.btn--primary:not(:disabled):active {
		transform: scale(0.98);
	}

	.btn--ghost {
		background: transparent;
		color: var(--color-primary);
		font-size: var(--text-base);
		flex-wrap: wrap;
	}

	.btn__soon {
		font-weight: var(--font-weight-normal);
		font-size: var(--text-sm);
		color: var(--color-text-muted);
	}

	.back {
		margin-top: var(--space-8);
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		color: var(--color-primary);
		font-weight: var(--font-weight-bold);
		text-decoration: none;
	}

	.back:hover {
		text-decoration: underline;
	}

	.back__icon {
		font-size: 1.25rem !important;
	}

	.hints {
		margin-top: var(--space-12);
		padding-top: var(--space-8);
		border-top: 1px solid var(--color-surface-container-high);
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		align-items: center;
	}

	@media (min-width: 768px) {
		.hints {
			flex-direction: row;
			justify-content: center;
			gap: var(--space-12);
		}
	}

	.hints__item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-medium);
		color: var(--color-text-muted);
	}

	.hints__icon {
		font-size: 1.25rem !important;
		color: var(--color-secondary);
	}

	.trust {
		background: color-mix(in srgb, var(--color-surface-container-high) 50%, transparent);
		padding: var(--space-4) var(--space-8);
		text-align: center;
	}

	.trust__text {
		margin: 0;
		font-size: 0.7rem;
		font-weight: var(--font-weight-medium);
		color: var(--color-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.06em;
	}
</style>
