<footer class="ft">
	<div class="lp-wrap ft__grid">
		<div class="ft__brand">
			<div class="ft__logo">Fixavon</div>
			<p class="ft__tagline lp-text-muted">
				The modern solution for plumbing services in Tbilisi. Professional, fast, and fully verified.
			</p>
		</div>
		<div>
			<h4 class="ft__heading">Services</h4>
			<ul class="ft__links">
				<li><a href="#services">Emergency Service</a></li>
				<li><a href="#services">Residential</a></li>
				<li><a href="#services">Commercial</a></li>
			</ul>
		</div>
		<div>
			<h4 class="ft__heading">Partners</h4>
			<ul class="ft__links">
				<li><a href="/register/plumber">Partner Program</a></li>
				<li><a href="#for-plumbers">Training</a></li>
				<li><a href="/login">Plumber App</a></li>
			</ul>
		</div>
		<div>
			<h4 class="ft__heading">Company</h4>
			<ul class="ft__links">
				<li><a href="/">Privacy Policy</a></li>
				<li><a href="/">Terms of Service</a></li>
				<li><a href="/">Contact</a></li>
			</ul>
		</div>
	</div>
	<div class="lp-wrap ft__bottom">
		<p class="ft__copy lp-text-muted">© 2026 Fixavon Tbilisi. Professional Plumbing Infrastructure.</p>
		<div class="ft__social">
			<a href="/" aria-label="Social" class="ft__soc"><span class="material-symbols-outlined">public</span></a>
		</div>
	</div>
</footer>

<style>
	.ft {
		padding-block: var(--space-16);
		background: var(--color-surface-container-low);
	}

	.ft__grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-8);
		margin-bottom: var(--space-12);
	}

	@media (min-width: 768px) {
		.ft__grid {
			grid-template-columns: 2fr 1fr 1fr 1fr;
		}
	}

	.ft__brand {
		grid-column: span 2;
	}

	@media (min-width: 768px) {
		.ft__brand {
			grid-column: span 1;
		}
	}

	.ft__logo {
		font-size: 1.25rem;
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
		margin-bottom: var(--space-6);
	}

	.ft__tagline {
		font-size: var(--text-sm);
		line-height: 1.6;
		max-width: 20rem;
	}

	.ft__heading {
		margin: 0 0 var(--space-6);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-bold);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text);
	}

	.ft__links {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		font-size: var(--text-sm);
	}

	.ft__links a {
		color: var(--color-text-muted);
		text-decoration: none;
	}

	.ft__links a:hover {
		color: var(--color-primary);
		text-decoration: underline;
		text-underline-offset: 4px;
	}

	.ft__bottom {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		align-items: center;
		padding-top: var(--space-8);
		border-top: 1px solid color-mix(in srgb, var(--color-outline-variant) 20%, transparent);
	}

	@media (min-width: 768px) {
		.ft__bottom {
			flex-direction: row;
			justify-content: space-between;
		}
	}

	.ft__copy {
		margin: 0;
		font-size: 0.75rem;
	}

	.ft__social {
		display: flex;
		gap: var(--space-6);
	}

	.ft__soc {
		color: var(--color-text-muted);
	}

	.ft__soc:hover {
		color: var(--color-primary);
	}
</style>
