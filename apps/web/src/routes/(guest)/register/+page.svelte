<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve -- links use pathWithLang for ?lang= */
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { ApiError, authRegisterClient } from '$lib/api/client';
	import { PENDING_EMAIL_VERIFICATION_KEY } from '$lib/auth/pendingVerification';
	import { translateAuthApiError } from '$lib/auth/translateApiError';
	import {
		translateEmailValidation,
		translatePasswordValidation
	} from '$lib/auth/validationMessages';
	import { pathWithLang } from '$lib/i18n/url';
	import { translate } from '$lib/i18n/translate';
	import SeoHead from '$lib/seo/SeoHead.svelte';
	import { validateEmailInput, validatePasswordInput } from '$lib/auth/validation';

	const ACTIVE_PLUMBERS = 3;

	const sp = $derived(page.url.searchParams);
	const loc = $derived(page.data.locale);
	const hrefHome = $derived(`${base}${pathWithLang('/', sp, loc)}`);
	const hrefLogin = $derived(`${base}${pathWithLang('/login', sp, loc)}`);
	const hrefRegisterPlumber = $derived(`${base}${pathWithLang('/register/plumber', sp, loc)}`);

	const pageTitle = $derived(translate(loc, 'auth.register.client.title'));
	const pageDescription = $derived(translate(loc, 'auth.register.client.metaDescription'));

	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let clientError = $state<string | null>(null);
	let submitting = $state(false);

	async function onsubmit(e: Event) {
		e.preventDefault();
		clientError = null;

		const emailResult = translateEmailValidation(loc, validateEmailInput(email));
		if (!emailResult.ok) {
			clientError = emailResult.message;
			return;
		}

		const pwErr = translatePasswordValidation(loc, validatePasswordInput(password));
		if (pwErr) {
			clientError = pwErr;
			return;
		}

		if (password !== confirmPassword) {
			clientError = translate(loc, 'auth.validation.passwordMismatch');
			return;
		}

		submitting = true;
		try {
			const res = await authRegisterClient({
				email: emailResult.email,
				password
			});
			sessionStorage.setItem(
				PENDING_EMAIL_VERIFICATION_KEY,
				JSON.stringify({
					token: res.email_verification_token,
					expires_at: res.email_verification_expires_at
				})
			);
			await goto(`${base}${pathWithLang('/verify-email', page.url.searchParams, page.data.locale)}`);
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
	<section class="hero" aria-labelledby="hero-heading">
		<div class="hero__bg">
			<img
				class="hero__img"
				src="{base}/register/client/hero-bathroom.png"
				alt=""
				width="800"
				height="1200"
			/>
			<div class="hero__overlay"></div>
		</div>
		<div class="hero__content">
			<a class="hero__logo" href={hrefHome}>{translate(loc, 'auth.shared.brand')}</a>
			<div class="hero__copy">
				<h1 id="hero-heading" class="hero__title">{translate(loc, 'auth.register.client.heroTitle')}</h1>
				<p class="hero__lead">
					{translate(loc, 'auth.register.client.heroLead')}
				</p>
				<div class="hero__trust">
					<div class="hero__trust-row">
						<div class="hero__trust-icon">
							<span class="material-symbols-outlined">verified</span>
						</div>
						<div>
							<p class="hero__trust-title">{translate(loc, 'auth.register.client.trustVettedTitle')}</p>
							<p class="hero__trust-meta">{translate(loc, 'auth.register.client.trustVettedMeta')}</p>
						</div>
					</div>
					<div class="hero__trust-row">
						<div class="hero__trust-icon">
							<span class="material-symbols-outlined">schedule</span>
						</div>
						<div>
							<p class="hero__trust-title">{translate(loc, 'auth.register.client.trust247Title')}</p>
							<p class="hero__trust-meta">{translate(loc, 'auth.register.client.trust247Meta')}</p>
						</div>
					</div>
				</div>
			</div>
			<div class="hero__pill">
				<span class="hero__dot"></span>
				<span class="hero__pill-text"
					>{translate(loc, 'auth.register.client.pillText', { count: ACTIVE_PLUMBERS })}</span
				>
			</div>
		</div>
	</section>

	<section class="panel" aria-labelledby="register-heading">
		<div class="panel__inner">
			<div class="panel__header">
				<h2 id="register-heading" class="panel__title">{translate(loc, 'auth.register.client.panelTitle')}</h2>
				<p class="panel__subtitle">
					{translate(loc, 'auth.register.client.panelSubtitle')}
				</p>
			</div>

			<div class="card">
				<form class="form" onsubmit={onsubmit}>
					{#if clientError}
						<p class="form__error" role="alert">{clientError}</p>
					{/if}

					<div class="field">
						<label class="field__label" for="reg-email">{translate(loc, 'auth.shared.email')}</label>
						<div class="field__input-wrap">
							<span class="material-symbols-outlined field__icon">mail</span>
							<input
								id="reg-email"
								class="field__input"
								type="email"
								name="email"
								autocomplete="email"
								placeholder={translate(loc, 'auth.login.emailPlaceholder')}
								bind:value={email}
								disabled={submitting}
								required
							/>
						</div>
					</div>

					<div class="field">
						<label class="field__label" for="reg-password">{translate(loc, 'auth.shared.password')}</label>
						<div class="field__input-wrap">
							<span class="material-symbols-outlined field__icon">lock</span>
							<input
								id="reg-password"
								class="field__input"
								type="password"
								name="password"
								autocomplete="new-password"
								placeholder={translate(loc, 'auth.login.passwordPlaceholder')}
								bind:value={password}
								disabled={submitting}
								required
							/>
						</div>
						<p class="field__hint">
							<span class="material-symbols-outlined field__hint-icon">info</span>
							{translate(loc, 'auth.register.client.passwordHint')}
						</p>
					</div>

					<div class="field">
						<label class="field__label" for="reg-confirm"
							>{translate(loc, 'auth.register.client.confirmPassword')}</label
						>
						<div class="field__input-wrap">
							<span class="material-symbols-outlined field__icon">shield</span>
							<input
								id="reg-confirm"
								class="field__input"
								type="password"
								name="confirm_password"
								autocomplete="new-password"
								placeholder={translate(loc, 'auth.login.passwordPlaceholder')}
								bind:value={confirmPassword}
								disabled={submitting}
								required
							/>
						</div>
					</div>

					<div class="form__actions">
						<button class="btn-submit" type="submit" disabled={submitting}>
							{submitting
								? translate(loc, 'auth.register.client.creating')
								: translate(loc, 'auth.register.client.createAccount')}
						</button>
					</div>
				</form>
			</div>

			<div class="panel__footer">
				<p class="panel__login">
					{translate(loc, 'auth.register.client.alreadyHave')}
					<a href={hrefLogin}>{translate(loc, 'auth.shared.logIn')}</a>
				</p>
				<hr class="panel__rule" />
				<a class="btn-plumber" href={hrefRegisterPlumber}>
					<span class="material-symbols-outlined">engineering</span>
					{translate(loc, 'auth.register.client.plumberCta')}
				</a>
			</div>

			<div class="legal">
				<a href={hrefHome}>{translate(loc, 'auth.shared.termsOfService')}</a>
				<a href={hrefHome}>{translate(loc, 'auth.shared.privacyPolicy')}</a>
				<a href={hrefHome}>{translate(loc, 'auth.shared.contactSupport')}</a>
			</div>
		</div>
	</section>
</div>

<style>
	.page {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	@media (min-width: 768px) {
		.page {
			flex-direction: row;
		}
	}

	.material-symbols-outlined {
		font-family: 'Material Symbols Outlined', sans-serif;
		font-weight: normal;
		font-style: normal;
		font-size: 1.25rem;
		line-height: 1;
		font-variation-settings: 'FILL' 0, 'wght' 400, 'GRAD' 0, 'opsz' 24;
		vertical-align: middle;
	}

	/* Left hero */
	.hero {
		position: relative;
		width: 100%;
		min-height: 22rem;
		background: var(--color-primary);
		color: var(--color-on-primary);
		display: flex;
		flex-direction: column;
		padding: var(--space-8);
		overflow: hidden;
	}

	@media (min-width: 768px) {
		.hero {
			width: 45%;
			max-width: 42rem;
			min-height: 100vh;
			padding: var(--space-12) var(--space-8);
			justify-content: space-between;
		}
	}

	.hero__bg {
		position: absolute;
		inset: 0;
		z-index: 0;
	}

	.hero__img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0.35;
		mix-blend-mode: overlay;
	}

	.hero__overlay {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			135deg,
			color-mix(in srgb, var(--color-primary) 90%, black),
			color-mix(in srgb, var(--color-primary-container) 85%, transparent)
		);
	}

	.hero__content {
		position: relative;
		z-index: 1;
		display: flex;
		flex-direction: column;
		height: 100%;
		gap: var(--space-8);
	}

	.hero__logo {
		font-size: 1.5rem;
		font-weight: 900;
		color: var(--color-on-primary);
		text-decoration: none;
		letter-spacing: -0.02em;
	}

	.hero__copy {
		margin-top: auto;
		max-width: 28rem;
	}

	.hero__title {
		margin: 0 0 var(--space-6);
		font-size: clamp(1.75rem, 4vw, 2.5rem);
		font-weight: 800;
		line-height: 1.1;
		color: var(--color-on-primary);
	}

	.hero__lead {
		margin: 0;
		font-size: var(--text-lg);
		line-height: 1.6;
		color: var(--color-primary-fixed);
		opacity: 0.95;
		font-weight: var(--font-weight-medium);
	}

	.hero__trust {
		margin-top: var(--space-12);
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
	}

	.hero__trust-row {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}

	.hero__trust-icon {
		width: 3rem;
		height: 3rem;
		border-radius: var(--radius-lg);
		background: color-mix(in srgb, var(--color-on-primary) 12%, transparent);
		display: flex;
		align-items: center;
		justify-content: center;
		backdrop-filter: blur(12px);
		color: var(--color-secondary-container);
	}

	.hero__trust-title {
		margin: 0;
		font-weight: var(--font-weight-bold);
		color: var(--color-on-primary);
	}

	.hero__trust-meta {
		margin: 0;
		font-size: var(--text-sm);
		color: var(--color-primary-fixed);
		opacity: 0.75;
	}

	.hero__pill {
		display: inline-flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-3) var(--space-5);
		border-radius: 9999px;
		background: color-mix(in srgb, var(--color-surface-elevated) 70%, transparent);
		backdrop-filter: blur(24px);
		border: 1px solid color-mix(in srgb, var(--color-on-primary) 10%, transparent);
		box-shadow: var(--shadow-ambient);
		margin-top: var(--space-8);
	}

	.hero__dot {
		width: 0.75rem;
		height: 0.75rem;
		border-radius: 50%;
		background: var(--color-tertiary);
		margin-left: var(--space-2);
	}

	.hero__pill-text {
		font-size: var(--text-sm);
		font-weight: var(--font-weight-semibold);
		color: var(--color-primary);
	}

	/* Right panel */
	.panel {
		flex: 1;
		background: var(--color-surface);
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-6);
	}

	@media (min-width: 768px) {
		.panel {
			padding: var(--space-12) var(--space-12) var(--space-12) var(--space-8);
		}
	}

	@media (min-width: 1024px) {
		.panel {
			padding: var(--space-24);
		}
	}

	.panel__inner {
		width: 100%;
		max-width: 32rem;
	}

	.panel__header {
		margin-bottom: var(--space-10);
	}

	.panel__title {
		margin: 0 0 var(--space-3);
		font-size: 1.875rem;
		font-weight: 800;
		color: var(--color-text);
		letter-spacing: -0.02em;
	}

	.panel__subtitle {
		margin: 0;
		color: var(--color-text-muted);
		font-weight: var(--font-weight-medium);
	}

	.card {
		background: var(--color-surface-elevated);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		box-shadow: var(--shadow-ambient);
	}

	.form {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
	}

	.form__error {
		margin: 0;
		padding: var(--space-3) var(--space-4);
		border-radius: var(--radius-md);
		background: color-mix(in srgb, var(--color-error) 12%, transparent);
		color: var(--color-error);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-medium);
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.field__label {
		font-size: 0.75rem;
		font-weight: var(--font-weight-bold);
		text-transform: uppercase;
		letter-spacing: 0.06em;
		color: var(--color-text-muted);
		margin-left: 0.25rem;
	}

	.field__input-wrap {
		position: relative;
		display: flex;
		align-items: center;
	}

	.field__icon {
		position: absolute;
		left: var(--space-4);
		color: var(--color-outline);
		pointer-events: none;
	}

	.field__input {
		width: 100%;
		padding: var(--space-4) var(--space-4) var(--space-4) 3rem;
		border: none;
		border-radius: var(--radius-xl);
		background: var(--color-surface-container-low);
		color: var(--color-text);
		font-family: inherit;
		font-size: var(--text-base);
		outline: none;
		transition:
			background 0.15s ease,
			box-shadow 0.15s ease;
	}

	.field__input::placeholder {
		color: var(--color-outline);
	}

	.field__input:focus {
		background: var(--color-surface-elevated);
		box-shadow: 0 0 0 2px color-mix(in srgb, var(--color-primary) 30%, transparent);
	}

	.field__input:disabled {
		opacity: 0.65;
		cursor: not-allowed;
	}

	.field__hint {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		margin: var(--space-1) 0 0 0.25rem;
		font-size: 0.75rem;
		color: var(--color-text-muted);
	}

	.field__hint-icon {
		font-size: var(--text-sm);
		color: var(--color-outline);
	}

	.form__actions {
		padding-top: var(--space-4);
	}

	.btn-submit {
		width: 100%;
		padding: var(--space-4);
		border: none;
		border-radius: var(--radius-xl);
		font-family: inherit;
		font-size: var(--text-lg);
		font-weight: var(--font-weight-bold);
		color: var(--color-on-primary);
		cursor: pointer;
		background: linear-gradient(
			90deg,
			var(--color-primary),
			var(--color-primary-container)
		);
		box-shadow: 0px 20px 40px color-mix(in srgb, var(--color-primary) 15%, transparent);
		transition: transform 0.15s ease;
	}

	.btn-submit:hover:not(:disabled) {
		transform: scale(1.01);
	}

	.btn-submit:active:not(:disabled) {
		transform: scale(0.98);
	}

	.btn-submit:disabled {
		opacity: 0.75;
		cursor: not-allowed;
	}

	.panel__footer {
		margin-top: var(--space-10);
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-4);
		text-align: center;
	}

	.panel__login {
		margin: 0;
		margin-top: var(--space-6);
		color: var(--color-text-muted);
		font-weight: var(--font-weight-medium);
	}

	.panel__login a {
		color: var(--color-primary);
		font-weight: var(--font-weight-bold);
		text-decoration: none;
		margin-left: 0.25rem;
	}

	.panel__login a:hover {
		text-decoration: underline;
	}

	.panel__rule {
		width: 100%;
		border: none;
		border-top: 1px solid var(--color-surface-container-high);
		margin: var(--space-2) 0;
	}

	.btn-plumber {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-6);
		border-radius: 9999px;
		background: var(--color-secondary-container);
		color: var(--color-on-secondary-container);
		font-weight: var(--font-weight-bold);
		font-size: var(--text-sm);
		text-decoration: none;
		transition: transform 0.15s ease;
	}

	.btn-plumber:hover {
		transform: scale(1.03);
	}

	.legal {
		margin-top: var(--space-16);
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: var(--space-6);
		font-size: 0.75rem;
		color: var(--color-outline);
		font-weight: var(--font-weight-medium);
	}

	.legal a {
		color: inherit;
		text-decoration: none;
	}

	.legal a:hover {
		color: var(--color-primary);
	}
</style>
