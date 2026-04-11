<section class="hero lp-section">
	<div class="lp-wrap hero__grid">
		<div class="hero__copy">
			<div class="hero__badge">
				<span class="material-symbols-outlined hero__badge-icon">verified</span>
				Trusted by 2,000+ Tbilisi Households
			</div>
			<h1 class="lp-heading-xl">
				Fast, <span class="hero__accent">Trusted Plumbers</span> in Tbilisi
			</h1>
			<p class="hero__lead lp-text-muted">
				Book verified plumbers for urgent repairs, installations, and home plumbing services — all in one
				place. Professional care for your home is just a few clicks away.
			</p>
			<div class="hero__ctas">
				<a class="lp-btn lp-btn--primary" href="/register">
					Book a Plumber
					<span class="material-symbols-outlined">arrow_forward</span>
				</a>
				<a class="lp-btn lp-btn--outline" href="/register/plumber">Become a Partner</a>
			</div>
		</div>
		<div class="hero__visual">
			<div class="hero__glow" aria-hidden="true"></div>
			<div class="hero__cards">
				<div class="hero__col">
					<div class="hero__card">
						<div class="hero__card-head">
							<span class="material-symbols-outlined hero__icon">location_on</span>
							<span class="hero__tag">Live Map</span>
						</div>
						<div class="hero__map-wrap">
							<img
								class="hero__map-img"
								src="/marketing/hero-map.png"
								alt="Map of Tbilisi showing plumbing service areas"
								width="400"
								height="200"
							/>
						</div>
					</div>
					<div class="hero__card">
						<h4 class="hero__card-title">Recent Booking</h4>
						<div class="hero__booking">
							<div class="hero__avatar-mini">
								<span class="material-symbols-outlined">plumbing</span>
							</div>
							<div>
								<p class="hero__booking-title">Pipe Leak Repair</p>
								<p class="hero__booking-meta">Saburtalo · 14 mins ago</p>
							</div>
						</div>
					</div>
				</div>
				<div class="hero__col hero__col--offset">
					<div class="hero__card hero__card--primary">
						<h4 class="hero__flow-title">Service Flow</h4>
						<ol class="hero__flow">
							<li class="hero__flow-step hero__flow-step--muted">
								<span class="hero__step-num">1</span> Select Service
							</li>
							<li class="hero__flow-step hero__flow-step--active">
								<span class="hero__step-num hero__step-num--solid">2</span> Pick Plumber
							</li>
							<li class="hero__flow-step hero__flow-step--muted">
								<span class="hero__step-num">3</span> Job Start
							</li>
						</ol>
					</div>
					<img
						class="hero__app-shot"
						src="/marketing/hero-app-ui.png"
						alt="App interface listing verified plumbers with ratings"
						width="400"
						height="280"
					/>
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.hero {
		padding-top: 8rem;
		overflow: hidden;
	}

	.hero__grid {
		display: grid;
		gap: var(--space-16);
		align-items: center;
	}

	@media (min-width: 1024px) {
		.hero__grid {
			grid-template-columns: 1fr 1fr;
		}
	}

	.hero__copy {
		display: flex;
		flex-direction: column;
		gap: var(--space-8);
	}

	.hero__badge {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		width: fit-content;
		padding: 0.375rem var(--space-4);
		border-radius: 9999px;
		background: var(--color-secondary-container);
		color: var(--color-on-secondary-container);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-semibold);
	}

	.hero__badge-icon {
		font-size: var(--text-sm);
		font-variation-settings: 'FILL' 1, 'wght' 400, 'GRAD' 0, 'opsz' 24;
	}

	.hero__accent {
		color: var(--color-primary);
	}

	.hero__lead {
		font-size: var(--text-lg);
		line-height: 1.6;
		max-width: 36rem;
	}

	.hero__ctas {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-4);
	}

	.hero__visual {
		position: relative;
	}

	.hero__glow {
		position: absolute;
		inset: 50% auto auto 50%;
		transform: translate(-50%, -50%);
		width: 120%;
		height: 120%;
		background: radial-gradient(
			circle at center,
			color-mix(in srgb, var(--color-primary) 10%, transparent) 0%,
			transparent 70%
		);
		border-radius: 50%;
		filter: blur(48px);
		z-index: -1;
	}

	.hero__cards {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: var(--space-4);
	}

	.hero__col {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.hero__col--offset {
		padding-top: var(--space-12);
	}

	@media (max-width: 639px) {
		.hero__col--offset {
			padding-top: 0;
		}

		.hero__cards {
			grid-template-columns: 1fr;
		}
	}

	.hero__card {
		background: var(--color-surface-elevated);
		padding: var(--space-6);
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-ambient);
		border: 1px solid color-mix(in srgb, var(--color-outline-variant) 10%, transparent);
	}

	.hero__card-head {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: var(--space-4);
	}

	.hero__icon {
		color: var(--color-primary);
		background: color-mix(in srgb, var(--color-primary) 10%, transparent);
		padding: var(--space-2);
		border-radius: var(--radius-lg);
		font-size: 1.25rem;
	}

	.hero__tag {
		font-size: 0.75rem;
		font-weight: var(--font-weight-bold);
		color: var(--color-tertiary);
	}

	.hero__map-wrap {
		height: 8rem;
		border-radius: var(--radius-lg);
		overflow: hidden;
		background: var(--color-surface-container-low);
	}

	.hero__map-img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		filter: grayscale(0.3);
		opacity: 0.85;
	}

	.hero__card-title {
		margin: 0 0 var(--space-3);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.hero__booking {
		display: flex;
		gap: var(--space-3);
		align-items: center;
	}

	.hero__avatar-mini {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 50%;
		background: var(--color-secondary-container);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-on-secondary-container);
	}

	.hero__booking-title {
		margin: 0;
		font-size: 0.75rem;
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.hero__booking-meta {
		margin: 0;
		font-size: 10px;
		color: var(--color-text-muted);
	}

	.hero__card--primary {
		background: var(--color-primary);
		color: var(--color-on-primary);
		border: none;
	}

	.hero__flow-title {
		margin: 0 0 var(--space-4);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-bold);
	}

	.hero__flow {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.hero__flow-step {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		font-size: 0.75rem;
	}

	.hero__flow-step--muted {
		opacity: 0.6;
	}

	.hero__flow-step--active {
		font-weight: var(--font-weight-bold);
	}

	.hero__step-num {
		width: 1.5rem;
		height: 1.5rem;
		border-radius: 50%;
		border: 1px solid color-mix(in srgb, var(--color-on-primary) 30%, transparent);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 10px;
	}

	.hero__step-num--solid {
		background: var(--color-on-primary);
		color: var(--color-primary);
		font-weight: var(--font-weight-bold);
		border: none;
	}

	.hero__app-shot {
		width: 100%;
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-ambient);
		border: 1px solid color-mix(in srgb, var(--color-outline-variant) 10%, transparent);
		object-fit: cover;
	}
</style>
