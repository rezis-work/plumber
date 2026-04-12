<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve -- links use pathWithLang for ?lang= */
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { ApiError, authRegisterPlumber } from '$lib/api/client';
	import { translateAuthApiError } from '$lib/auth/translateApiError';
	import {
		translateEmailValidation,
		translateFullNameError,
		translatePasswordValidation,
		translatePhoneError,
		translateYearsError
	} from '$lib/auth/validationMessages';
	import { pathWithLang } from '$lib/i18n/url';
	import { translate } from '$lib/i18n/translate';
	import SeoHead from '$lib/seo/SeoHead.svelte';
	import {
		validateEmailInput,
		validateFullNameInput,
		validatePasswordInput,
		validatePhoneInput,
		validateYearsOfExperienceInput
	} from '$lib/auth/validation';

	let fullName = $state('');
	let email = $state('');
	let phone = $state('');
	let yearsExperience = $state(0);
	let password = $state('');
	let confirmPassword = $state('');
	let termsAccepted = $state(false);
	let clientError = $state<string | null>(null);
	let submitting = $state(false);
	let succeeded = $state(false);

	const sp = $derived(page.url.searchParams);
	const loc = $derived(page.data.locale);
	const hrefHome = $derived(`${base}${pathWithLang('/', sp, loc)}`);
	const hrefLogin = $derived(`${base}${pathWithLang('/login', sp, loc)}`);
	const hrefRegister = $derived(`${base}${pathWithLang('/register', sp, loc)}`);

	const pageTitle = $derived(translate(loc, 'auth.register.plumber.title'));
	const pageDescription = $derived(translate(loc, 'auth.register.plumber.metaDescription'));

	async function onsubmit(e: Event) {
		e.preventDefault();
		clientError = null;

		const nameErr = translateFullNameError(loc, validateFullNameInput(fullName));
		if (nameErr) {
			clientError = nameErr;
			return;
		}

		const emailResult = translateEmailValidation(loc, validateEmailInput(email));
		if (!emailResult.ok) {
			clientError = emailResult.message;
			return;
		}

		const phoneErr = translatePhoneError(loc, validatePhoneInput(phone));
		if (phoneErr) {
			clientError = phoneErr;
			return;
		}

		const yearsErr = translateYearsError(loc, validateYearsOfExperienceInput(yearsExperience));
		if (yearsErr) {
			clientError = yearsErr;
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

		if (!termsAccepted) {
			clientError = translate(loc, 'auth.validation.termsRequired');
			return;
		}

		submitting = true;
		try {
			await authRegisterPlumber({
				email: emailResult.email,
				password,
				full_name: fullName.trim(),
				phone,
				years_of_experience: yearsExperience
			});
			succeeded = true;
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
				src="{base}/register/plumber/hero-plumber.png"
				alt=""
				width="800"
				height="1200"
			/>
			<div class="hero__overlay"></div>
		</div>
		<div class="hero__content">
			<a class="hero__logo" href={hrefHome}>{translate(loc, 'auth.shared.brand')}</a>
			<div class="hero__copy">
				<p class="hero__eyebrow">{translate(loc, 'auth.register.plumber.eyebrow')}</p>
				<h1 id="hero-heading" class="hero__title">{translate(loc, 'auth.register.plumber.heroTitle')}</h1>
				<ul class="hero__list">
					<li class="hero__item">
						<div class="hero__icon">
							<span class="material-symbols-outlined">trending_up</span>
						</div>
						<div>
							<p class="hero__item-title">{translate(loc, 'auth.register.plumber.item1Title')}</p>
							<p class="hero__item-meta">{translate(loc, 'auth.register.plumber.item1Meta')}</p>
						</div>
					</li>
					<li class="hero__item">
						<div class="hero__icon">
							<span class="material-symbols-outlined">dashboard_customize</span>
						</div>
						<div>
							<p class="hero__item-title">{translate(loc, 'auth.register.plumber.item2Title')}</p>
							<p class="hero__item-meta">{translate(loc, 'auth.register.plumber.item2Meta')}</p>
						</div>
					</li>
					<li class="hero__item">
						<div class="hero__icon">
							<span class="material-symbols-outlined hero__icon--fill">verified_user</span>
						</div>
						<div>
							<p class="hero__item-title">{translate(loc, 'auth.register.plumber.item3Title')}</p>
							<p class="hero__item-meta">{translate(loc, 'auth.register.plumber.item3Meta')}</p>
						</div>
					</li>
					<li class="hero__item">
						<div class="hero__icon">
							<span class="material-symbols-outlined">payments</span>
						</div>
						<div>
							<p class="hero__item-title">{translate(loc, 'auth.register.plumber.item4Title')}</p>
							<p class="hero__item-meta">{translate(loc, 'auth.register.plumber.item4Meta')}</p>
						</div>
					</li>
				</ul>
				<p class="hero__social">{translate(loc, 'auth.register.plumber.socialLine')}</p>
			</div>
		</div>
	</section>

	<section class="panel" aria-labelledby="register-heading">
		<div class="panel__inner">
			{#if succeeded}
				<div class="success-card">
					<div class="success-card__icon" aria-hidden="true">
						<span class="material-symbols-outlined">check_circle</span>
					</div>
					<h2 id="register-heading" class="success-card__title"
						>{translate(loc, 'auth.register.plumber.successTitle')}</h2
					>
					<p class="success-card__text">
						{translate(loc, 'auth.register.plumber.successText')}
					</p>
					<div class="success-card__actions">
						<a class="btn-primary" href={hrefLogin}>{translate(loc, 'auth.shared.logIn')}</a>
						<a class="btn-muted" href={hrefHome}>{translate(loc, 'auth.register.plumber.backHome')}</a>
					</div>
					<p class="success-card__hint">
						{translate(loc, 'auth.register.plumber.successHint')}
						<a href={hrefRegister}>{translate(loc, 'auth.register.plumber.clientSignUp')}</a>
					</p>
				</div>
			{:else}
				<div class="panel__header">
					<h2 id="register-heading" class="panel__title"
						>{translate(loc, 'auth.register.plumber.panelTitle')}</h2
					>
					<p class="panel__subtitle">
						{translate(loc, 'auth.register.plumber.panelSubtitle')}
					</p>
				</div>

				<div class="card">
					<form class="form" onsubmit={onsubmit}>
						{#if clientError}
							<p class="form__error" role="alert">{clientError}</p>
						{/if}

						<div class="field">
							<label class="field__label" for="plumb-name"
								>{translate(loc, 'auth.register.plumber.fullName')}</label
							>
							<input
								id="plumb-name"
								class="field__input field__input--plain"
								type="text"
								name="full_name"
								autocomplete="name"
								placeholder={translate(loc, 'auth.register.plumber.namePlaceholder')}
								bind:value={fullName}
								disabled={submitting}
								required
							/>
						</div>

						<div class="field-grid">
							<div class="field">
								<label class="field__label" for="plumb-email"
									>{translate(loc, 'auth.shared.email')}</label
								>
								<div class="field__input-wrap">
									<span class="material-symbols-outlined field__icon">mail</span>
									<input
										id="plumb-email"
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
								<label class="field__label" for="plumb-phone"
									>{translate(loc, 'auth.register.plumber.phone')}</label
								>
								<div class="field__input-wrap">
									<span class="material-symbols-outlined field__icon">call</span>
									<input
										id="plumb-phone"
										class="field__input"
										type="tel"
										name="phone"
										autocomplete="tel"
										placeholder={translate(loc, 'auth.register.plumber.phonePlaceholder')}
										bind:value={phone}
										disabled={submitting}
										required
									/>
								</div>
							</div>
						</div>

						<div class="field">
							<label class="field__label" for="plumb-years"
								>{translate(loc, 'auth.register.plumber.yearsExperience')}</label
							>
							<input
								id="plumb-years"
								class="field__input field__input--plain"
								type="number"
								name="years_of_experience"
								min="0"
								max="80"
								step="1"
								bind:value={yearsExperience}
								disabled={submitting}
								required
							/>
							<p class="field__hint">
								<span class="material-symbols-outlined field__hint-icon">info</span>
								{translate(loc, 'auth.register.plumber.yearsHint')}
							</p>
						</div>

						<div class="field-grid">
							<div class="field">
								<label class="field__label" for="plumb-password"
									>{translate(loc, 'auth.shared.password')}</label
								>
								<div class="field__input-wrap">
									<span class="material-symbols-outlined field__icon">lock</span>
									<input
										id="plumb-password"
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
							</div>
							<div class="field">
								<label class="field__label" for="plumb-confirm"
									>{translate(loc, 'auth.register.plumber.confirmPassword')}</label
								>
								<div class="field__input-wrap">
									<span class="material-symbols-outlined field__icon">shield</span>
									<input
										id="plumb-confirm"
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
						</div>

						<label class="terms">
							<input
								class="terms__check"
								type="checkbox"
								bind:checked={termsAccepted}
								disabled={submitting}
							/>
							<span class="terms__text">
								{translate(loc, 'auth.register.plumber.termsAgree')}
								<a href={hrefHome}>{translate(loc, 'auth.shared.termsOfService')}</a>
								{translate(loc, 'auth.register.plumber.termsAnd')}
								<a href={hrefHome}>{translate(loc, 'auth.shared.privacyPolicy')}</a
								>{translate(loc, 'auth.register.plumber.termsEnd')}
							</span>
						</label>

						<div class="form__actions">
							<button class="btn-submit" type="submit" disabled={submitting}>
								{submitting
									? translate(loc, 'auth.register.plumber.creating')
									: translate(loc, 'auth.register.plumber.submit')}
							</button>
						</div>
					</form>
				</div>

				<div class="panel__footer">
					<p class="panel__login">
						{translate(loc, 'auth.register.plumber.alreadyHave')}
						<a href={hrefLogin}>{translate(loc, 'auth.shared.logIn')}</a>
					</p>
					<hr class="panel__rule" />
					<a class="btn-client" href={hrefRegister}>
						<span class="material-symbols-outlined">person_add</span>
						{translate(loc, 'auth.register.plumber.signUpClient')}
					</a>
				</div>

				<div class="legal">
					<a href={hrefHome}>{translate(loc, 'auth.shared.termsOfService')}</a>
					<a href={hrefHome}>{translate(loc, 'auth.shared.privacyPolicy')}</a>
					<a href={hrefHome}>{translate(loc, 'auth.shared.contactSupport')}</a>
				</div>
			{/if}
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

	.hero__icon--fill {
		font-variation-settings: 'FILL' 1, 'wght' 400, 'GRAD' 0, 'opsz' 24;
	}

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
			width: 42%;
			max-width: 28rem;
			min-height: 100vh;
			height: 100vh;
			padding: var(--space-12) var(--space-8);
			position: sticky;
			top: 0;
			align-self: flex-start;
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
		opacity: 0.22;
	}

	.hero__overlay {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			145deg,
			color-mix(in srgb, var(--color-primary) 92%, black) 0%,
			color-mix(in srgb, var(--color-primary) 75%, transparent) 55%,
			color-mix(in srgb, var(--color-primary-container) 40%, #001849) 100%
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
		max-width: 22rem;
	}

	.hero__eyebrow {
		display: inline-block;
		margin: 0 0 var(--space-6);
		padding: var(--space-2) var(--space-3);
		border-radius: 9999px;
		background: var(--color-secondary-container);
		color: var(--color-on-secondary-container);
		font-size: 0.65rem;
		font-weight: var(--font-weight-bold);
		text-transform: uppercase;
		letter-spacing: 0.12em;
	}

	.hero__title {
		margin: 0 0 var(--space-8);
		font-size: clamp(1.85rem, 4vw, 2.75rem);
		font-weight: 900;
		line-height: 1.05;
		color: var(--color-on-primary);
		letter-spacing: -0.03em;
	}

	.hero__list {
		margin: 0;
		padding: 0;
		list-style: none;
		display: flex;
		flex-direction: column;
		gap: var(--space-8);
	}

	.hero__item {
		display: flex;
		align-items: flex-start;
		gap: var(--space-4);
	}

	.hero__icon {
		flex-shrink: 0;
		width: 2.5rem;
		height: 2.5rem;
		border-radius: var(--radius-lg);
		background: color-mix(in srgb, var(--color-on-primary) 12%, transparent);
		display: flex;
		align-items: center;
		justify-content: center;
		backdrop-filter: blur(12px);
		color: var(--color-on-primary);
	}

	.hero__item-title {
		margin: 0;
		font-weight: var(--font-weight-bold);
		font-size: var(--text-lg);
		color: var(--color-on-primary);
		line-height: 1.2;
	}

	.hero__item-meta {
		margin: var(--space-2) 0 0;
		font-size: var(--text-sm);
		line-height: 1.5;
		color: var(--color-primary-fixed);
		opacity: 0.8;
	}

	.hero__social {
		margin: var(--space-12) 0 0;
		padding-top: var(--space-8);
		border-top: 1px solid color-mix(in srgb, var(--color-on-primary) 15%, transparent);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-medium);
		color: var(--color-primary-fixed);
		opacity: 0.9;
	}

	.panel {
		flex: 1;
		background: var(--color-surface);
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-8) var(--space-6);
	}

	@media (min-width: 768px) {
		.panel {
			padding: var(--space-12) var(--space-10) var(--space-12) var(--space-8);
		}
	}

	@media (min-width: 1024px) {
		.panel {
			padding: var(--space-8) var(--space-12);
		}
	}

	.panel__inner {
		width: 100%;
		max-width: 42rem;
	}

	.panel__header {
		margin-bottom: var(--space-10);
		padding-bottom: var(--space-2);
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
		padding: var(--space-10) var(--space-8);
		box-shadow: var(--shadow-ambient);
		border: 1px solid color-mix(in srgb, var(--color-outline-variant) 35%, transparent);
	}

	@media (min-width: 768px) {
		.card {
			padding: var(--space-12) var(--space-10);
		}
	}

	@media (min-width: 1024px) {
		.card {
			padding: var(--space-12) var(--space-12);
		}
	}

	.form {
		display: flex;
		flex-direction: column;
		gap: var(--space-8);
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

	.field-grid {
		display: grid;
		gap: var(--space-8);
	}

	@media (min-width: 768px) {
		.field-grid {
			grid-template-columns: 1fr 1fr;
			gap: var(--space-8) var(--space-6);
		}
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
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
		min-height: 3.25rem;
		box-sizing: border-box;
		padding: 0.875rem 1.125rem 0.875rem 3rem;
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

	.field__input--plain {
		padding: 0.875rem 1.125rem;
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
		margin: var(--space-2) 0 0 0.35rem;
		font-size: 0.75rem;
		color: var(--color-text-muted);
		line-height: 1.4;
	}

	.field__hint-icon {
		font-size: var(--text-sm);
		color: var(--color-outline);
	}

	.terms {
		display: flex;
		align-items: flex-start;
		gap: var(--space-4);
		cursor: pointer;
		font-size: var(--text-sm);
		color: var(--color-text-muted);
		line-height: 1.55;
		padding: 0;
		margin-top: 0;
		border-radius: var(--radius-md);
	}

	.terms__check {
		width: 1.25rem;
		height: 1.25rem;
		margin-top: 0.1rem;
		accent-color: var(--color-primary);
		flex-shrink: 0;
	}

	.terms__text {
		min-width: 0;
		overflow-wrap: break-word;
	}

	.terms a {
		color: var(--color-primary);
		font-weight: var(--font-weight-semibold);
		text-decoration: none;
	}

	.terms a:hover {
		text-decoration: underline;
	}

	.form__actions {
		padding-top: 0;
		margin-top: 0;
	}

	.btn-submit {
		width: 100%;
		padding: 1.125rem var(--space-6);
		border: none;
		border-radius: var(--radius-xl);
		font-family: inherit;
		font-size: var(--text-lg);
		font-weight: 800;
		color: var(--color-on-primary);
		cursor: pointer;
		background: var(--color-primary);
		box-shadow: 0px 20px 40px color-mix(in srgb, var(--color-primary) 22%, transparent);
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

	.btn-client {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-6);
		border-radius: 9999px;
		background: var(--color-surface-container-low);
		color: var(--color-primary);
		font-weight: var(--font-weight-bold);
		font-size: var(--text-sm);
		text-decoration: none;
		border: 1px solid var(--color-outline-variant);
		transition: transform 0.15s ease;
	}

	.btn-client:hover {
		transform: scale(1.02);
	}

	.legal {
		margin-top: var(--space-8);
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

	.success-card {
		text-align: center;
		padding: var(--space-12) var(--space-8);
		background: var(--color-surface-elevated);
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-ambient);
		border: 1px solid color-mix(in srgb, var(--color-outline-variant) 35%, transparent);
	}

	.success-card__icon {
		width: 4rem;
		height: 4rem;
		margin: 0 auto var(--space-6);
		border-radius: 50%;
		background: color-mix(in srgb, var(--color-secondary) 15%, transparent);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-secondary);
	}

	.success-card__icon .material-symbols-outlined {
		font-size: 2.25rem !important;
		font-variation-settings: 'FILL' 1, 'wght' 400, 'GRAD' 0, 'opsz' 24;
	}

	.success-card__title {
		margin: 0 0 var(--space-4);
		font-size: 1.75rem;
		font-weight: 800;
		color: var(--color-text);
	}

	.success-card__text {
		margin: 0 0 var(--space-8);
		color: var(--color-text-muted);
		line-height: 1.6;
		max-width: 26rem;
		margin-left: auto;
		margin-right: auto;
	}

	.success-card__actions {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		align-items: center;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-4) var(--space-10);
		border-radius: var(--radius-xl);
		background: var(--color-primary);
		color: var(--color-on-primary);
		font-weight: var(--font-weight-bold);
		text-decoration: none;
		box-shadow: var(--shadow-ambient);
	}

	.btn-primary:hover {
		background: color-mix(in srgb, var(--color-primary) 88%, black);
	}

	.btn-muted {
		font-size: var(--text-sm);
		color: var(--color-text-muted);
		text-decoration: none;
		font-weight: var(--font-weight-medium);
	}

	.btn-muted:hover {
		color: var(--color-primary);
	}

	.success-card__hint {
		margin: var(--space-10) 0 0;
		font-size: var(--text-sm);
		color: var(--color-text-muted);
	}

	.success-card__hint a {
		color: var(--color-primary);
		font-weight: var(--font-weight-semibold);
		text-decoration: none;
	}

	.success-card__hint a:hover {
		text-decoration: underline;
	}
</style>
