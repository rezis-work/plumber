<script lang="ts">
	import { base } from '$app/paths';
	import { logoutEverywhere, logoutFromApp } from '$lib/auth/logout';
	import { session } from '$lib/auth/session.svelte';

	let menuOpen = $state(false);
	let logoutBusy = $state(false);
	let activeSection = $state('benefits');
	let isNavigating = $state(false);

	const isAuthenticated = $derived(session.user !== null && session.accessToken !== null);
	const canLogoutEverywhere = $derived(session.accessToken !== null);

	const sections = ['benefits', 'services', 'for-plumbers',  'faq'];

	function setActive(id: string) {
		isNavigating = true;
		activeSection = id;
	}

	$effect(() => {
		const visible = new Set<string>();

		const observer = new IntersectionObserver(
			(entries) => {
				for (const entry of entries) {
					const id = entry.target.id;
					if (entry.isIntersecting) {
						visible.add(id);
					} else {
						visible.delete(id);
					}
				}
				if (isNavigating) return;
				for (const id of sections) {
					if (visible.has(id)) {
						activeSection = id;
						break;
					}
				}
			},
			{ rootMargin: '-80px 0px -70% 0px', threshold: 0 }
		);

		for (const id of sections) {
			const el = document.getElementById(id);
			if (el) observer.observe(el);
		}

		function onScrollEnd() {
			isNavigating = false;
			for (const id of sections) {
				if (visible.has(id)) {
					activeSection = id;
					break;
				}
			}
		}

		window.addEventListener('scrollend', onScrollEnd, { passive: true });
		return () => {
			observer.disconnect();
			window.removeEventListener('scrollend', onScrollEnd);
		};
	});

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
			<a class="nav__link" class:nav__link--active={activeSection === 'benefits'} href="#benefits" onclick={() => setActive('benefits')}>Benefits</a>
			<a class="nav__link" class:nav__link--active={activeSection === 'services'} href="#services" onclick={() => setActive('services')}>Services</a>
			<a class="nav__link" class:nav__link--active={activeSection === 'for-plumbers'} href="#for-plumbers" onclick={() => setActive('for-plumbers')}>For Plumbers</a>
			
			<a class="nav__link" class:nav__link--active={activeSection === 'faq'} href="#faq" onclick={() => setActive('faq')}>FAQ</a>
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
		<div id="nav-mobile-panel" class="nav__mobile md-hide-panel">
			<nav class="nav__mobile-nav">
				{#each [['benefits','Benefits'],['services','Services'],['for-plumbers','For Plumbers'],['faq','FAQ']] as [id, label]}
					<a
						class="nav__mobile-link"
						class:nav__mobile-link--active={activeSection === id}
						href="#{id}"
						onclick={() => { setActive(id); closeMenu(); }}
					>
						{label}
						{#if activeSection === id}
							<span class="nav__mobile-dot" aria-hidden="true"></span>
						{/if}
					</a>
				{/each}
			</nav>

			<hr class="nav__mobile-rule" />

			{#if isAuthenticated}
				<p class="nav__mobile-user">{session.user?.email ?? ''}</p>
				<button type="button" class="nav__mobile-link nav__mobile-link--btn" disabled={logoutBusy} onclick={onLogout}>
					Log out
				</button>
				{#if canLogoutEverywhere}
					<button type="button" class="nav__mobile-link nav__mobile-link--btn nav__mobile-link--muted" disabled={logoutBusy} onclick={onLogoutEverywhere}>
						Log out everywhere
					</button>
				{/if}
			{:else}
				<a class="nav__mobile-link" href={`${base}/login`} onclick={closeMenu}>Log in</a>
			{/if}

			<div class="nav__mobile-actions">
				<a class="nav__mobile-plumber" href={`${base}/register/plumber`} onclick={closeMenu}>Join as Plumber</a>
				<a class="nav__mobile-cta lp-btn lp-btn--primary" href={`${base}/register`} onclick={closeMenu}>Book Now</a>
			</div>
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
		display: none;
		align-items: center;
		gap: var(--space-4);
	}

	@media (min-width: 768px) {
		.nav__actions {
			display: flex;
		}
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
		padding: var(--space-6);
		background: var(--color-surface-elevated);
		border-top: 1px solid var(--color-outline-variant);
		box-shadow: 0 8px 32px rgba(0,0,0,0.08);
	}

	.md-hide-panel {
		display: flex;
	}

	@media (min-width: 768px) {
		.md-hide-panel {
			display: none;
		}
	}

	.nav__mobile-nav {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.nav__mobile-link {
		display: flex;
		align-items: center;
		justify-content: space-between;
		color: var(--color-text);
		font-weight: var(--font-weight-medium);
		font-size: var(--text-base);
		text-decoration: none;
		padding: var(--space-4);
		border-radius: var(--radius-lg);
		transition: background 0.15s ease, color 0.15s ease;
	}

	.nav__mobile-link:hover {
		background: var(--color-surface-container-low);
		color: var(--color-primary);
	}

	.nav__mobile-link--active {
		background: color-mix(in srgb, var(--color-primary) 8%, transparent);
		color: var(--color-primary);
		font-weight: var(--font-weight-bold);
	}

	.nav__mobile-dot {
		width: 0.4rem;
		height: 0.4rem;
		border-radius: 50%;
		background: var(--color-primary);
	}

	.nav__mobile-rule {
		border: none;
		border-top: 1px solid var(--color-outline-variant);
		margin: 0;
	}

	.nav__mobile-user {
		margin: 0;
		padding: var(--space-2) var(--space-4);
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
	}

	.nav__mobile-actions {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		padding-top: var(--space-2);
	}

	.nav__mobile-plumber {
		display: block;
		text-align: center;
		padding: var(--space-3) var(--space-4);
		border-radius: var(--radius-lg);
		border: 1.5px solid var(--color-outline-variant);
		color: var(--color-text);
		font-weight: var(--font-weight-semibold);
		text-decoration: none;
		transition: border-color 0.15s, color 0.15s;
	}

	.nav__mobile-plumber:hover {
		border-color: var(--color-primary);
		color: var(--color-primary);
	}

	.nav__mobile-cta {
		text-align: center;
	}
</style>
