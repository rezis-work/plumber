<script lang="ts">
	import { base } from '$app/paths';
	import { goto } from '$app/navigation';
	import { ApiError, authRegisterClient } from '$lib/api/client';
	import { PENDING_EMAIL_VERIFICATION_KEY } from '$lib/auth/pendingVerification';
	import { validateEmailInput, validatePasswordInput } from '$lib/auth/validation';

	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let clientError = $state<string | null>(null);
	let submitting = $state(false);

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

		if (password !== confirmPassword) {
			clientError = 'Passwords do not match.';
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
			await goto(`${base}/verify-email`);
		} catch (err) {
			if (err instanceof ApiError) {
				if (err.status === 409 && err.code === 'conflict') {
					clientError = 'An account with this email already exists.';
				} else if (err.status === 400 && err.code === 'validation_error') {
					clientError = err.message ?? 'Please check your input and try again.';
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
	<title>Create account | Fixavon</title>
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
			<a class="hero__logo" href="{base}/">Fixavon</a>
			<div class="hero__copy">
				<h1 id="hero-heading" class="hero__title">Reliable Help is Just a Click Away.</h1>
				<p class="hero__lead">
					Join thousands of Tbilisi households who trust Fixavon for their plumbing needs.
				</p>
				<div class="hero__trust">
					<div class="hero__trust-row">
						<div class="hero__trust-icon">
							<span class="material-symbols-outlined">verified</span>
						</div>
						<div>
							<p class="hero__trust-title">Vetted Professionals</p>
							<p class="hero__trust-meta">Rigorous background checks for every plumber.</p>
						</div>
					</div>
					<div class="hero__trust-row">
						<div class="hero__trust-icon">
							<span class="material-symbols-outlined">schedule</span>
						</div>
						<div>
							<p class="hero__trust-title">24/7 Availability</p>
							<p class="hero__trust-meta">Emergency services across Tbilisi anytime.</p>
						</div>
					</div>
				</div>
			</div>
			<div class="hero__pill">
				<span class="hero__dot"></span>
				<span class="hero__pill-text">3 plumbers active in Tbilisi right now.</span>
			</div>
		</div>
	</section>

	<section class="panel" aria-labelledby="register-heading">
		<div class="panel__inner">
			<div class="panel__header">
				<h2 id="register-heading" class="panel__title">Create Client Account</h2>
				<p class="panel__subtitle">
					Create your account to request trusted plumbers in Tbilisi.
				</p>
			</div>

			<div class="card">
				<form class="form" onsubmit={onsubmit}>
					{#if clientError}
						<p class="form__error" role="alert">{clientError}</p>
					{/if}

					<div class="field">
						<label class="field__label" for="reg-email">Email</label>
						<div class="field__input-wrap">
							<span class="material-symbols-outlined field__icon">mail</span>
							<input
								id="reg-email"
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
						<label class="field__label" for="reg-password">Password</label>
						<div class="field__input-wrap">
							<span class="material-symbols-outlined field__icon">lock</span>
							<input
								id="reg-password"
								class="field__input"
								type="password"
								name="password"
								autocomplete="new-password"
								placeholder="••••••••"
								bind:value={password}
								disabled={submitting}
								required
							/>
						</div>
						<p class="field__hint">
							<span class="material-symbols-outlined field__hint-icon">info</span>
							Must be at least 8 characters long.
						</p>
					</div>

					<div class="field">
						<label class="field__label" for="reg-confirm">Confirm Password</label>
						<div class="field__input-wrap">
							<span class="material-symbols-outlined field__icon">shield</span>
							<input
								id="reg-confirm"
								class="field__input"
								type="password"
								name="confirm_password"
								autocomplete="new-password"
								placeholder="••••••••"
								bind:value={confirmPassword}
								disabled={submitting}
								required
							/>
						</div>
					</div>

					<div class="form__actions">
						<button class="btn-submit" type="submit" disabled={submitting}>
							{submitting ? 'Creating…' : 'Create Account'}
						</button>
					</div>
				</form>
			</div>

			<div class="panel__footer">
				<p class="panel__login">
					Already have an account?
					<a href="{base}/login">Log In</a>
				</p>
				<hr class="panel__rule" />
				<a class="btn-plumber" href="{base}/register/plumber">
					<span class="material-symbols-outlined">engineering</span>
					Are you a professional? Register as Plumber
				</a>
			</div>

			<div class="legal">
				<a href="{base}/">Terms of Service</a>
				<a href="{base}/">Privacy Policy</a>
				<a href="{base}/">Contact Support</a>
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
