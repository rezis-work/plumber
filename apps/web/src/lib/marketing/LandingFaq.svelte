<script lang="ts">
	import { page } from '$app/state';
	import { translate } from '$lib/i18n/translate';

	const loc = $derived(page.data.locale);
	const FAQ_COUNT = 3;
</script>

<section class="faq lp-section" id="faq">
	<div class="lp-wrap faq__inner">
		<h2 class="lp-heading-lg faq__title">{translate(loc, 'marketing.faq.title')}</h2>
		<p class="faq__subtitle lp-text-muted">
			{translate(loc, 'marketing.faq.subtitle', { count: FAQ_COUNT })}
		</p>
		<div class="faq__list">
			<details class="faq__item" open>
				<summary class="faq__summary">
					<span class="faq__q">{translate(loc, 'marketing.faq.q1')}</span>
					<span class="material-symbols-outlined faq__icon">expand_more</span>
				</summary>
				<div class="faq__a lp-text-muted">
					{translate(loc, 'marketing.faq.a1')}
				</div>
			</details>
			<details class="faq__item">
				<summary class="faq__summary">
					<span class="faq__q">{translate(loc, 'marketing.faq.q2')}</span>
					<span class="material-symbols-outlined faq__icon">expand_more</span>
				</summary>
				<div class="faq__a lp-text-muted">
					{translate(loc, 'marketing.faq.a2')}
				</div>
			</details>
			<details class="faq__item">
				<summary class="faq__summary">
					<span class="faq__q">{translate(loc, 'marketing.faq.q3')}</span>
					<span class="material-symbols-outlined faq__icon">expand_more</span>
				</summary>
				<div class="faq__a lp-text-muted">
					{translate(loc, 'marketing.faq.a3')}
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
		margin-bottom: var(--space-4);
	}

	.faq__subtitle {
		text-align: center;
		margin: 0 auto var(--space-12);
		max-width: 40rem;
		font-size: var(--text-sm);
		line-height: 1.5;
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
