<section class="why lp-section" id="benefits">
	<div class="lp-wrap">
		<div class="why__head">
			<div>
				<h2 class="lp-heading-lg">Why Choose Fixavon</h2>
				<p class="why__sub lp-text-muted">
					We're redefining plumbing services in Tbilisi with a focus on trust and transparency.
				</p>
			</div>
			<div class="why__pill">
				<div class="why__dots" aria-hidden="true">
					<span class="why__dot why__dot--p"></span>
					<span class="why__dot why__dot--s"></span>
					<span class="why__dot why__dot--t"></span>
				</div>
				<div>
					<p class="why__pill-title">Trusted Experts</p>
					<p class="why__pill-meta lp-text-muted">Over 150+ Verified Plumbers</p>
				</div>
			</div>
		</div>
		<div class="why__grid">
			<div class="why__card">
				<span class="material-symbols-outlined why__card-icon">verified_user</span>
				<h3 class="why__card-title">Verified plumbers</h3>
				<p class="why__card-text lp-text-muted">
					Every professional undergoes a strict background check and skill verification process.
				</p>
			</div>
			<div class="why__card">
				<span class="material-symbols-outlined why__card-icon">bolt</span>
				<h3 class="why__card-title">Fast response</h3>
				<p class="why__card-text lp-text-muted">
					Emergency plumbing needs can't wait. We average a 30-minute response time in central Tbilisi.
				</p>
			</div>
			<div class="why__card">
				<span class="material-symbols-outlined why__card-icon">payments</span>
				<h3 class="why__card-title">Transparent pricing</h3>
				<p class="why__card-text lp-text-muted">
					No hidden fees or "estimates" that double at the end. See fixed prices before you book.
				</p>
			</div>
			<div class="why__card">
				<span class="material-symbols-outlined why__card-icon">touch_app</span>
				<h3 class="why__card-title">Easy booking</h3>
				<p class="why__card-text lp-text-muted">
					Our platform is designed for speed. Book your expert in under 60 seconds.
				</p>
			</div>
			<div class="why__card">
				<span class="material-symbols-outlined why__card-icon">handyman</span>
				<h3 class="why__card-title">Reliable service</h3>
				<p class="why__card-text lp-text-muted">
					Quality is our priority. We guarantee all work performed through our platform.
				</p>
			</div>
			<div class="why__card">
				<span class="material-symbols-outlined why__card-icon">map</span>
				<h3 class="why__card-title">Local coverage</h3>
				<p class="why__card-text lp-text-muted">
					Available in all major districts of Tbilisi, 24 hours a day, 7 days a week.
				</p>
			</div>
		</div>
	</div>
</section>

<style>
	.why__head {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
		margin-bottom: var(--space-16);
		align-items: flex-start;
	}

	@media (min-width: 768px) {
		.why__head {
			flex-direction: row;
			align-items: flex-end;
			justify-content: space-between;
		}
	}

	.why__sub {
		max-width: 36rem;
		margin-top: var(--space-4);
		font-size: var(--text-base);
	}

	.why__pill {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		background: var(--color-surface-container-high);
		padding: var(--space-4) var(--space-6);
		border-radius: var(--radius-xl);
	}

	.why__dots {
		display: flex;
		margin-left: -0.5rem;
	}

	.why__dot {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 50%;
		border: 2px solid var(--color-surface-elevated);
		margin-left: -0.75rem;
	}

	.why__dot--p {
		background: var(--color-primary);
	}
	.why__dot--s {
		background: var(--color-secondary);
	}
	.why__dot--t {
		background: var(--color-tertiary);
	}

	.why__pill-title {
		margin: 0;
		font-size: var(--text-sm);
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.why__pill-meta {
		margin: 0;
		font-size: 0.75rem;
	}

	.why__grid {
		display: grid;
		gap: var(--space-8);
	}

	@media (min-width: 768px) {
		.why__grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.why__card {
		padding: var(--space-8);
		border-radius: var(--radius-xl);
		background: var(--color-surface);
		border: 1px solid transparent;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.why__card:hover {
		border-color: color-mix(in srgb, var(--color-primary) 10%, transparent);
		box-shadow: 0 12px 40px color-mix(in srgb, var(--color-primary) 5%, transparent);
	}

	.why__card-icon {
		font-size: 2rem;
		color: var(--color-secondary);
		margin-bottom: var(--space-6);
	}

	.why__card-title {
		margin: 0 0 var(--space-3);
		font-size: var(--text-lg);
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.why__card-text {
		margin: 0;
		line-height: 1.6;
		font-size: var(--text-sm);
	}
</style>
