<script lang="ts">
	import { browser } from '$app/environment';
	import { base } from '$app/paths';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { ApiError, authLogin } from '$lib/api/client';
	import { profilePathForRole } from '$lib/auth/profilePaths';
	import { session, setSessionFromLogin } from '$lib/auth/session.svelte';
	import { validateEmailInput, validatePasswordInput } from '$lib/auth/validation';

	let email = $state('');
	let password = $state('');
	let showPassword = $state(false);
	let clientError = $state<string | null>(null);
	let submitting = $state(false);
	let verifiedBanner = $state(false);

	onMount(() => {
		if (!browser) return;
		const p = new URLSearchParams(window.location.search);
		if (p.get('verified') === '1') {
			verifiedBanner = true;
		}
	});

	async function onsubmit(e: Event) {
		e.preventDefault();
		clientError = null;

		const emailResult = validateEmailInput(email);
		if (!emailResult.ok) {
			clientError = emailResult.message;
			return;
		}

		const pwErr = validatePasswordInput(password);
		if (pwErr) {
			clientError = pwErr;
			return;
		}

		submitting = true;
		try {
			const loginResponse = await authLogin({
				email: emailResult.email,
				password
			});
			await setSessionFromLogin(loginResponse);
			if (session.user) {
				await goto(`${base}${profilePathForRole(session.user.role)}`);
			} else {
				await goto(`${base}/`);
			}
		} catch (err) {
			if (err instanceof ApiError) {
				if (err.status === 401) {
					clientError = 'Invalid email or password.';
				} else if (err.status === 400 && err.code === 'validation_error') {
					clientError = err.message ?? 'Please check your input and try again.';
				} else if (err.status === 403 && err.code === 'account_inactive') {
					clientError = 'This account is disabled. Contact support if you need help.';
				} else {
					clientError = 'Something went wrong. Please try again.';
				}
			} else {
				clientError = 'Network error. Check your connection and try again.';
			}
		} finally {
			submitting = false;
		}
	}
</script>

<svelte:head>
	<title>Log in | Fixavon</title>
	<link
		rel="stylesheet"
		href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0&display=swap"
	/>
</svelte:head>

<div class="page">
	<section class="hero" aria-labelledby="hero-heading">
		<div class="hero__pattern" aria-hidden="true"></div>
		<div class="hero__blob" aria-hidden="true"></div>
		<div class="hero__content">
			<a class="hero__logo" href="{base}/">Fixavon</a>
			<div class="hero__copy">
				<h1 id="hero-heading" class="hero__title">Connecting Tbilisi with trusted professionals.</h1>
				<p class="hero__lead">
					Access your Fixavon account to manage services or bookings. Professional help at the turn of a tap.
				</p>
			</div>
			<div class="hero__trust glass">
				<div class="hero__avatars">
					<img
						class="hero__avatar"
						src="{base}/login/trust-1.png"
						alt=""
						width="40"
						height="40"
					/>
					<img
						class="hero__avatar"
						src="{base}/login/trust-2.png"
						alt=""
						width="40"
						height="40"
					/>
					<img
						class="hero__avatar"
						src="{base}/login/trust-3.png"
						alt=""
						width="40"
						height="40"
					/>
				</div>
				<div class="hero__trust-text">
					<div class="hero__live">
						<span class="hero__dot" aria-hidden="true"></span>
						<span class="hero__live-label">3 plumbers active</span>
					</div>
					<p class="hero__live-sub">Available now in Tbilisi</p>
				</div>
			</div>
		</div>
	</section>

	<section class="panel" aria-labelledby="login-heading">
		<div class="panel__inner">
			<a class="panel__logo-mobile" href="{base}/">Fixavon</a>

			{#if verifiedBanner}
				<div class="banner" role="status">
					<span class="material-symbols-outlined banner__icon">mark_email_read</span>
					<p class="banner__text">Email verified — you can log in.</p>
				</div>
			{/if}

			<div class="panel__header">
				<h2 id="login-heading" class="panel__title">Welcome back</h2>
				<p class="panel__subtitle">Log in to continue to Fixavon</p>
			</div>

			<div class="card">
				<form class="form" onsubmit={onsubmit}>
					{#if clientError}
						<p class="form__error" role="alert">{clientError}</p>
					{/if}

					<div class="field">
						<label class="field__label" for="login-email">Email</label>
						<div class="field__input-wrap">
							<span class="material-symbols-outlined field__icon">mail</span>
							<input
								id="login-email"
								class="field__input"
								type="email"
								name="email"
								autocomplete="email"
								placeholder="you@example.com"
								bind:value={email}
								disabled={submitting}
								required
							/>
						</div>
					</div>

					<div class="field">
						<div class="field__row">
							<label class="field__label" for="login-password">Password</label>
							<a class="field__link" href="{base}/">Forgot password?</a>
						</div>
						<div class="field__input-wrap">
							<span class="material-symbols-outlined field__icon">lock</span>
							<input
								id="login-password"
								class="field__input field__input--with-toggle"
								type={showPassword ? 'text' : 'password'}
								name="password"
								autocomplete="current-password"
								placeholder="••••••••"
								bind:value={password}
								disabled={submitting}
								required
							/>
							<button
								class="field__toggle"
								type="button"
								aria-label={showPassword ? 'Hide password' : 'Show password'}
								disabled={submitting}
								onclick={() => (showPassword = !showPassword)}
							>
								<span class="material-symbols-outlined">
									{showPassword ? 'visibility_off' : 'visibility'}
								</span>
							</button>
						</div>
					</div>

					<div class="form__actions">
						<button class="btn-submit" type="submit" disabled={submitting}>
							{submitting ? 'Signing in…' : 'Log in'}
						</button>
					</div>
				</form>
			</div>

			<div class="divider">
				<span class="divider__line"></span>
				<span class="divider__text">New to Fixavon?</span>
				<span class="divider__line"></span>
			</div>

			<div class="signup-grid">
				<a class="signup-card signup-card--client" href="{base}/register">
					<span class="material-symbols-outlined signup-card__icon">person</span>
					<span class="signup-card__label">Register as client</span>
				</a>
				<a class="signup-card signup-card--plumber" href="{base}/register/plumber">
					<span class="material-symbols-outlined signup-card__icon">engineering</span>
					<span class="signup-card__label">Become a plumber</span>
				</a>
			</div>

			<p class="legal">
				By logging in, you agree to Fixavon&apos;s
				<a href="{base}/">Terms of Service</a>
				and
				<a href="{base}/">Privacy Policy</a>.
			</p>
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

	.hero {
		display: none;
		position: relative;
		overflow: hidden;
		background: linear-gradient(
			135deg,
			#001849 0%,
			var(--color-primary) 55%,
			color-mix(in srgb, var(--color-primary-container) 70%, #001849) 100%
		);
		color: var(--color-on-primary);
		flex-direction: column;
		justify-content: space-between;
		padding: var(--space-12) var(--space-8);
		min-height: 22rem;
	}

	@media (min-width: 768px) {
		.hero {
			display: flex;
			width: 42%;
			max-width: 32rem;
			height: 100vh;
			padding: var(--space-8);
			position: sticky;
			top: 0;
			align-self: flex-start;
		}
	}

	.hero__pattern {
		position: absolute;
		inset: 0;
		opacity: 0.1;
		pointer-events: none;
		background-image: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='white' fill-opacity='0.4'%3E%3Cpath d='M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
	}

	.hero__blob {
		position: absolute;
		bottom: -6rem;
		left: -6rem;
		width: 24rem;
		height: 24rem;
		border-radius: 50%;
		background: var(--color-primary-container);
		filter: blur(120px);
		opacity: 0.2;
		pointer-events: none;
	}

	.hero__content {
		position: relative;
		z-index: 1;
		display: flex;
		flex-direction: column;
		height: 100%;
		gap: var(--space-12);
	}

	.hero__logo {
		font-size: 1.875rem;
		font-weight: 900;
		color: var(--color-on-primary);
		text-decoration: none;
		letter-spacing: -0.02em;
		margin-bottom: var(--space-8);
	}

	.hero__copy {
		max-width: 28rem;
	}

	.hero__title {
		margin: 0 0 var(--space-6);
		font-size: clamp(2rem, 4vw, 3rem);
		font-weight: 800;
		line-height: 1.08;
		letter-spacing: -0.03em;
		color: var(--color-on-primary);
	}

	.hero__lead {
		margin: 0;
		font-size: var(--text-lg);
		line-height: 1.65;
		color: var(--color-primary-fixed);
		opacity: 0.92;
	}

	.glass {
		background: color-mix(in srgb, var(--color-on-primary) 8%, transparent);
		backdrop-filter: blur(12px);
		border: 1px solid color-mix(in srgb, var(--color-on-primary) 14%, transparent);
		border-radius: var(--radius-xl);
	}

	.hero__trust {
		display: inline-flex;
		align-items: center;
		gap: var(--space-4);
		padding: var(--space-4);
		margin-top: auto;
		max-width: 100%;
	}

	.hero__avatars {
		display: none;
		margin-left: 0.35rem;
	}

	@media (min-width: 1024px) {
		.hero__avatars {
			display: flex;
		}
	}

	.hero__avatar {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 50%;
		object-fit: cover;
		border: 2px solid var(--color-primary-container);
		margin-left: -0.65rem;
	}

	.hero__avatar:first-child {
		margin-left: 0;
	}

	.hero__trust-text {
		min-width: 0;
	}

	.hero__live {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.hero__dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 50%;
		background: #22c55e;
		flex-shrink: 0;
		animation: pulse-dot 2s ease-in-out infinite;
		margin-left: var(--space-2);
	}

	@keyframes pulse-dot {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.45;
		}
	}

	.hero__live-label {
		font-size: var(--text-sm);
		font-weight: var(--font-weight-semibold);
		color: var(--color-on-primary);
	}

	.hero__live-sub {
		margin: var(--space-1) 0 0;
		padding-left: 1.5rem;
		font-size: 0.7rem;
		color: var(--color-primary-fixed);
		opacity: 0.85;
	}

	.panel {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-8) var(--space-6);
		background: var(--color-surface);
	}

	@media (min-width: 769px) and (max-width: 1023px) {
		.panel {
			padding: var(--space-12) var(--space-8);
		}
	}

	@media (min-width: 1024px) {
		.panel {
			padding: var(--space-12);
		}
	}

	.panel__inner {
		width: 100%;
		max-width: 28rem;
	}

	@media (min-width: 1024px) {
		.panel__inner {
			max-width: 38rem;
		}
	}

	.panel__logo-mobile {
		display: block;
		font-size: 1.875rem;
		font-weight: 900;
		color: var(--color-primary);
		text-decoration: none;
		letter-spacing: -0.02em;
		text-align: center;
		margin-bottom: var(--space-8);
	}

	@media (min-width: 768px) {
		.panel__logo-mobile {
			display: none;
		}
	}

	.banner {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
		padding: var(--space-4);
		margin-bottom: var(--space-8);
		border-radius: var(--radius-lg);
		background: color-mix(in srgb, var(--color-secondary) 12%, transparent);
		border: 1px solid color-mix(in srgb, var(--color-secondary) 25%, transparent);
	}

	.banner__icon {
		color: var(--color-secondary);
		flex-shrink: 0;
		margin-top: 0.1rem;
	}

	.banner__text {
		margin: 0;
		font-size: var(--text-sm);
		font-weight: var(--font-weight-medium);
		color: var(--color-text);
		line-height: 1.5;
	}

	.panel__header {
		margin-bottom: var(--space-2);
		padding-bottom: 0;
	}

	.panel__title {
		margin: 0 0;
		font-size: clamp(1.375rem, 3vw, 1.875rem);
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
		padding: var(--space-6) var(--space-8);
		box-shadow: 0 4px 24px rgba(0, 0, 0, 0.08), 0 1px 4px rgba(0, 0, 0, 0.04);
		border: 1px solid var(--color-outline-variant);
	}

	@media (min-width: 769px) and (max-width: 1023px) {
		.card {
			padding: var(--space-6) var(--space-8);
		}
	}

	@media (min-width: 1024px) {
		.card {
			padding: var(--space-8) var(--space-8);
		}
	}

	.form {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		padding: 0;
	}

	@media (min-width: 768px) {
		.form {
			gap: var(--space-6);
		}
	}

	.form__error {
		margin: 0;
		padding: var(--space-4) var(--space-5);
		border-radius: var(--radius-md);
		background: color-mix(in srgb, var(--color-error) 12%, transparent);
		color: var(--color-error);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-medium);
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
	}

	.field__row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-4);
		flex-wrap: wrap;
	}

	.field__label {
		font-size: 0.75rem;
		font-weight: var(--font-weight-bold);
		text-transform: uppercase;
		letter-spacing: 0.06em;
		color: var(--color-text-muted);
		margin-left: 0.35rem;
		padding-bottom: 0.125rem;
	}

	.field__link {
		font-size: 0.75rem;
		font-weight: var(--font-weight-bold);
		color: var(--color-primary);
		text-decoration: none;
	}

	.field__link:hover {
		text-decoration: underline;
	}

	.field__input-wrap {
		position: relative;
		display: flex;
		align-items: center;
	}

	.field__icon {
		position: absolute;
		left: 1.125rem;
		color: var(--color-outline);
		pointer-events: none;
		z-index: 1;
	}

	.field__input {
		width: 100%;
		min-height: 3.35rem;
		box-sizing: border-box;
		padding: 1rem 1.25rem 1rem 3.25rem;
		border: none;
		border-radius: var(--radius-xl);
		background: var(--color-surface-container-low);
		color: var(--color-text);
		font-family: inherit;
		font-size: var(--text-base);
		line-height: 1.45;
		outline: none;
		transition:
			background 0.15s ease,
			box-shadow 0.15s ease;
	}

	.field__input--with-toggle {
		padding-right: 3.5rem;
	}

	.field__input::placeholder {
		color: var(--color-outline);
		opacity: 0.65;
	}

	.field__input:focus {
		background: var(--color-surface-elevated);
		box-shadow: 0 0 0 2px color-mix(in srgb, var(--color-primary) 30%, transparent);
	}

	.field__input:disabled {
		opacity: 0.65;
		cursor: not-allowed;
	}

	.field__toggle {
		position: absolute;
		right: var(--space-2);
		top: 50%;
		transform: translateY(-50%);
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2.5rem;
		height: 2.5rem;
		padding: 0;
		border: none;
		background: transparent;
		color: var(--color-outline);
		cursor: pointer;
		border-radius: var(--radius-md);
	}

	.field__toggle:hover:not(:disabled) {
		color: var(--color-text);
		background: color-mix(in srgb, var(--color-outline-variant) 40%, transparent);
	}

	.field__toggle:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.form__actions {
		padding-top: 0;
		margin-top: 0;
	}

	.btn-submit {
		width: 100%;
		padding: 1.125rem var(--space-8);
		border: none;
		border-radius: var(--radius-xl);
		font-family: inherit;
		font-size: var(--text-lg);
		font-weight: var(--font-weight-bold);
		color: var(--color-on-primary);
		cursor: pointer;
		background: var(--color-primary);
		box-shadow: 0px 20px 40px color-mix(in srgb, var(--color-primary) 18%, transparent);
		transition: transform 0.15s ease;
	}

	.btn-submit:hover:not(:disabled) {
		background: color-mix(in srgb, var(--color-primary) 88%, black);
	}

	.btn-submit:active:not(:disabled) {
		transform: scale(0.98);
	}

	.btn-submit:disabled {
		opacity: 0.75;
		cursor: not-allowed;
	}

	.divider {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		margin: var(--space-6) 0;
	}

	.divider__line {
		flex: 1;
		height: 1px;
		background: var(--color-surface-container-high);
	}

	.divider__text {
		font-size: 0.65rem;
		font-weight: var(--font-weight-semibold);
		text-transform: uppercase;
		letter-spacing: 0.14em;
		color: var(--color-outline);
		white-space: nowrap;
	}

	.signup-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-4);
	}

	@media (min-width: 480px) {
		.signup-grid {
			grid-template-columns: 1fr 1fr;
		}
	}

	.signup-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		padding: var(--space-6) var(--space-4);
		border-radius: var(--radius-xl);
		background: var(--color-surface-container-low);
		text-decoration: none;
		transition:
			background 0.15s ease,
			transform 0.15s ease;
		border: 1px solid transparent;
	}

	.signup-card:hover {
		transform: translateY(-2px);
	}

	.signup-card--client:hover {
		background: var(--color-secondary-container);
	}

	.signup-card--plumber:hover {
		background: var(--color-surface-container-high);
		border-color: color-mix(in srgb, var(--color-outline-variant) 50%, transparent);
	}

	.signup-card__icon {
		font-size: 1.75rem !important;
		color: var(--color-secondary);
	}

	.signup-card--plumber .signup-card__icon {
		color: var(--color-primary);
	}

	.signup-card__label {
		font-size: var(--text-sm);
		font-weight: var(--font-weight-bold);
		color: var(--color-on-secondary-container);
		text-align: center;
	}

	.signup-card--plumber .signup-card__label {
		color: var(--color-primary);
	}

	.legal {
		margin: var(--space-12) 0 0;
		text-align: center;
		font-size: 0.75rem;
		line-height: 1.6;
		color: var(--color-outline);
	}

	.legal a {
		color: var(--color-primary);
		text-decoration: underline;
		font-weight: var(--font-weight-medium);
	}

	.legal a:hover {
		text-decoration: none;
	}
</style>
