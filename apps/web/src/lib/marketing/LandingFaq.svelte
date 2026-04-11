<section class="faq lp-section" id="faq">
	<div class="lp-wrap faq__inner">
		<h2 class="lp-heading-lg faq__title">Frequently Asked Questions</h2>
		<div class="faq__list">
			<details class="faq__item" open>
				<summary class="faq__summary">
					<span class="faq__q">How quickly can a plumber arrive?</span>
					<span class="material-symbols-outlined faq__icon">expand_more</span>
				</summary>
				<div class="faq__a lp-text-muted">
					In central districts like Vake and Saburtalo, our average arrival time is 30 minutes for emergency
					calls. In other districts, it typically takes 45-60 minutes.
				</div>
			</details>
			<details class="faq__item">
				<summary class="faq__summary">
					<span class="faq__q">Are the prices fixed?</span>
					<span class="material-symbols-outlined faq__icon">expand_more</span>
				</summary>
				<div class="faq__a lp-text-muted">
					Yes, for standard services like faucet replacement or minor leaks, we have flat rates. For complex
					projects, you'll receive a detailed quote before the work begins.
				</div>
			</details>
			<details class="faq__item">
				<summary class="faq__summary">
					<span class="faq__q">What kind of guarantee do you provide?</span>
					<span class="material-symbols-outlined faq__icon">expand_more</span>
				</summary>
				<div class="faq__a lp-text-muted">
					All work completed through Fixavon is covered by our 6-month satisfaction guarantee. If anything
					goes wrong with the repair, we'll fix it for free.
				</div>
			</details>
		</div>
	</div>
</section>

<style>
	.faq__inner {
		max-width: 50rem;
	}

	.faq__title {
		text-align: center;
		margin-bottom: var(--space-12);
	}

	.faq__list {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.faq__item {
		background: var(--color-surface-elevated);
		border-radius: var(--radius-xl);
		box-shadow: 0 2px 12px color-mix(in srgb, var(--color-text) 6%, transparent);
		overflow: hidden;
	}

	.faq__summary {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: var(--space-4);
		padding: var(--space-6);
		cursor: pointer;
		list-style: none;
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.faq__summary::-webkit-details-marker {
		display: none;
	}

	.faq__icon {
		transition: transform 0.2s ease;
		flex-shrink: 0;
	}

	.faq__item[open] .faq__icon {
		transform: rotate(180deg);
	}

	.faq__q {
		font-size: var(--text-base);
	}

	.faq__a {
		padding: 0 var(--space-6) var(--space-6);
		font-size: var(--text-sm);
		line-height: 1.5;
	}
</style>
