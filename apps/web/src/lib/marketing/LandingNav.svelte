<script lang="ts">
	import { base } from '$app/paths';
	import { logoutEverywhere, logoutFromApp } from '$lib/auth/logout';
	import { session } from '$lib/auth/session.svelte';

	let menuOpen = $state(false);
	let logoutBusy = $state(false);

	const isAuthenticated = $derived(session.user !== null && session.accessToken !== null);
	const canLogoutEverywhere = $derived(session.accessToken !== null);

	function closeMenu() {
		menuOpen = false;
	}

	async function onLogout() {
		if (logoutBusy) return;
		logoutBusy = true;
		try {
			closeMenu();
			await logoutFromApp();
		} finally {
			logoutBusy = false;
		}
	}

	async function onLogoutEverywhere() {
		if (logoutBusy) return;
		logoutBusy = true;
		try {
			closeMenu();
			await logoutEverywhere();
		} finally {
			logoutBusy = false;
		}
	}
</script>

<header class="nav lp-glass-nav">
	<nav class="nav__inner lp-wrap">
		<a class="nav__logo" href={`${base}/`} onclick={closeMenu}>Fixavon</a>
		<div class="nav__links">
			<a class="nav__link nav__link--active" href="#services">Services</a>
			<a class="nav__link" href="#for-plumbers">For Plumbers</a>
			<a class="nav__link" href="#benefits">Benefits</a>
			<a class="nav__link" href="#faq">FAQ</a>
		</div>
		<div class="nav__actions">
			{#if isAuthenticated}
				<span class="nav__user" title={session.user?.email ?? ''}>{session.user?.email ?? ''}</span>
				<button
					type="button"
					class="nav__login nav__text-btn"
					disabled={logoutBusy}
					onclick={onLogout}
				>
					Log out
				</button>
				{#if canLogoutEverywhere}
					<button
						type="button"
						class="nav__logout-all nav__text-btn lg-only"
						disabled={logoutBusy}
						onclick={onLogoutEverywhere}
					>
						Log out everywhere
					</button>
				{/if}
			{:else}
				<a class="nav__login" href={`${base}/login`}>Log in</a>
			{/if}
			<a class="nav__plumber lg-only" href={`${base}/register/plumber`}>Join as Plumber</a>
			<a class="lp-btn lp-btn--primary lp-btn--primary-sm" href={`${base}/register`}>Book Now</a>
		</div>
		<button
			type="button"
			class="nav__menu-btn md-hide"
			aria-expanded={menuOpen}
			aria-controls="nav-mobile-panel"
			aria-label={menuOpen ? 'Close menu' : 'Open menu'}
			onclick={() => (menuOpen = !menuOpen)}
		>
			{menuOpen ? '✕' : '☰'}
		</button>
	</nav>
	{#if menuOpen}
		<div id="nav-mobile-panel" class="nav__mobile lp-glass-nav md-hide-panel">
			<a class="nav__mobile-link" href="#services" onclick={closeMenu}>Services</a>
			<a class="nav__mobile-link" href="#for-plumbers" onclick={closeMenu}>For Plumbers</a>
			<a class="nav__mobile-link" href="#benefits" onclick={closeMenu}>Benefits</a>
			<a class="nav__mobile-link" href="#faq" onclick={closeMenu}>FAQ</a>
			{#if isAuthenticated}
				<p class="nav__mobile-user">{session.user?.email ?? ''}</p>
				<button
					type="button"
					class="nav__mobile-link nav__mobile-link--btn"
					disabled={logoutBusy}
					onclick={onLogout}
				>
					Log out
				</button>
				{#if canLogoutEverywhere}
					<button
						type="button"
						class="nav__mobile-link nav__mobile-link--btn nav__mobile-link--muted"
						disabled={logoutBusy}
						onclick={onLogoutEverywhere}
					>
						Log out everywhere
					</button>
				{/if}
			{:else}
				<a class="nav__mobile-link" href={`${base}/login`} onclick={closeMenu}>Log in</a>
			{/if}
			<a class="nav__mobile-link" href={`${base}/register/plumber`} onclick={closeMenu}>Join as Plumber</a>
			<a class="nav__mobile-cta lp-btn lp-btn--primary lp-btn--primary-sm" href={`${base}/register`} onclick={closeMenu}
				>Book Now</a
			>
		</div>
	{/if}
</header>

<style>
	.nav {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 50;
		height: 5rem;
	}

	.nav__inner {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 5rem;
		gap: var(--space-4);
		font-size: var(--text-base);
		font-weight: var(--font-weight-medium);
	}

	.nav__logo {
		font-size: 1.5rem;
		font-weight: 900;
		color: var(--color-primary);
		letter-spacing: -0.02em;
		text-decoration: none;
	}

	.nav__links {
		display: none;
		align-items: center;
		gap: var(--space-8);
	}

	@media (min-width: 768px) {
		.nav__links {
			display: flex;
		}
	}

	.nav__link {
		color: var(--color-text);
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.nav__link:hover {
		color: var(--color-primary-container);
	}

	.nav__link--active {
		color: var(--color-primary);
		font-weight: var(--font-weight-bold);
		border-bottom: 2px solid var(--color-primary);
		padding-bottom: 0.25rem;
	}

	.nav__actions {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}

	.nav__user {
		display: none;
		max-width: 10rem;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		color: var(--color-text-muted);
		font-size: 0.875rem;
	}

	@media (min-width: 1024px) {
		.nav__user {
			display: inline-block;
		}
	}

	.nav__text-btn {
		border: none;
		background: none;
		cursor: pointer;
		font-family: inherit;
		font-size: inherit;
		padding: 0;
	}

	.nav__text-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.nav__login {
		display: none;
		color: var(--color-text);
		font-weight: var(--font-weight-semibold);
		text-decoration: none;
	}

	.nav__login:hover:not(:disabled) {
		color: var(--color-primary);
	}

	.nav__logout-all {
		color: var(--color-text-muted);
		font-weight: var(--font-weight-medium);
	}

	.nav__logout-all:hover:not(:disabled) {
		color: var(--color-primary);
	}

	@media (min-width: 1024px) {
		.nav__login {
			display: inline;
		}
	}

	.nav__plumber {
		display: none;
		color: var(--color-text);
		font-weight: var(--font-weight-semibold);
		text-decoration: none;
		border: none;
		background: none;
		cursor: pointer;
		font-family: inherit;
		font-size: inherit;
	}

	.nav__plumber:hover {
		color: var(--color-primary);
	}

	.lg-only {
		display: none;
	}

	@media (min-width: 1024px) {
		.lg-only {
			display: inline;
		}
	}

	.nav__menu-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2.5rem;
		height: 2.5rem;
		border: none;
		background: var(--color-surface-container-low);
		border-radius: var(--radius-md);
		font-size: 1.25rem;
		cursor: pointer;
		color: var(--color-text);
	}

	@media (min-width: 768px) {
		.nav__menu-btn {
			display: none;
		}
	}

	.md-hide {
		display: flex;
	}

	@media (min-width: 768px) {
		.md-hide {
			display: none;
		}
	}

	.nav__mobile {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		padding: var(--space-4) var(--space-6) var(--space-6);
		border-top: 1px solid color-mix(in srgb, var(--color-outline-variant) 30%, transparent);
	}

	.md-hide-panel {
		display: flex;
	}

	@media (min-width: 768px) {
		.md-hide-panel {
			display: none;
		}
	}

	.nav__mobile-link {
		color: var(--color-text);
		font-weight: var(--font-weight-medium);
		text-decoration: none;
		padding: var(--space-2) 0;
	}

	.nav__mobile-link:hover {
		color: var(--color-primary);
	}

	.nav__mobile-user {
		margin: 0;
		padding: var(--space-2) 0 0;
		font-size: 0.8125rem;
		color: var(--color-text-muted);
		word-break: break-all;
	}

	.nav__mobile-link--btn {
		display: block;
		width: 100%;
		text-align: left;
		border: none;
		background: none;
		cursor: pointer;
		font-family: inherit;
		font-size: inherit;
	}

	.nav__mobile-link--btn:disabled {
		opacity: 0.5;
	}

	.nav__mobile-link--muted {
		color: var(--color-text-muted);
		font-weight: var(--font-weight-medium);
	}

	.nav__mobile-cta {
		margin-top: var(--space-2);
		text-align: center;
	}
</style>
