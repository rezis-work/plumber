<script lang="ts">
	import { page } from '$app/state';
	import { translate } from '$lib/i18n/translate';

	const loc = $derived(page.data.locale);
</script>

<section class="how lp-section">
	<div class="lp-wrap">
		<div class="how__intro">
			<h2 class="lp-heading-lg">{translate(loc, 'marketing.howItWorks.title')}</h2>
			<p class="how__sub lp-text-muted">
				{translate(loc, 'marketing.howItWorks.subtitle')}
			</p>
		</div>
		<div class="how__grid">
			<div class="how__item">
				<div class="how__card">
					<div class="how__icon-wrap">
						<span class="material-symbols-outlined how__icon">edit_note</span>
					</div>
					<h3 class="how__title">{translate(loc, 'marketing.howItWorks.step1Title')}</h3>
					<p class="how__desc lp-text-muted">
						{translate(loc, 'marketing.howItWorks.step1Desc')}
					</p>
				</div>
				<span class="how__chevron material-symbols-outlined md-only" aria-hidden="true">chevron_right</span>
			</div>
			<div class="how__item">
				<div class="how__card">
					<div class="how__icon-wrap">
						<span class="material-symbols-outlined how__icon">person_search</span>
					</div>
					<h3 class="how__title">{translate(loc, 'marketing.howItWorks.step2Title')}</h3>
					<p class="how__desc lp-text-muted">
						{translate(loc, 'marketing.howItWorks.step2Desc')}
					</p>
				</div>
				<span class="how__chevron material-symbols-outlined md-only" aria-hidden="true">chevron_right</span>
			</div>
			<div class="how__item">
				<div class="how__card">
					<div class="how__icon-wrap">
						<span class="material-symbols-outlined how__icon">event_available</span>
					</div>
					<h3 class="how__title">{translate(loc, 'marketing.howItWorks.step3Title')}</h3>
					<p class="how__desc lp-text-muted">
						{translate(loc, 'marketing.howItWorks.step3Desc')}
					</p>
				</div>
				<span class="how__chevron material-symbols-outlined md-only" aria-hidden="true">chevron_right</span>
			</div>
			<div class="how__item">
				<div class="how__card">
					<div class="how__icon-wrap how__icon-wrap--tertiary">
						<span class="material-symbols-outlined how__icon how__icon--fill">task_alt</span>
					</div>
					<h3 class="how__title">{translate(loc, 'marketing.howItWorks.step4Title')}</h3>
					<p class="how__desc lp-text-muted">
						{translate(loc, 'marketing.howItWorks.step4Desc')}
					</p>
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.how {
		background: var(--color-surface-container-low);
	}

	.how__intro {
		text-align: center;
		margin-bottom: var(--space-16);
	}

	.how__sub {
		max-width: 42rem;
		margin-inline: auto;
		margin-top: var(--space-4);
		font-size: var(--text-base);
	}

	.how__grid {
		display: grid;
		gap: var(--space-8);
	}

	@media (min-width: 768px) {
		.how__grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	.how__item {
		position: relative;
	}

	.how__card {
		background: var(--color-surface-elevated);
		padding: var(--space-8);
		border-radius: var(--radius-xl);
		height: 100%;
		transition: transform 0.3s ease;
	}

	.how__card:hover {
		transform: translateY(-0.5rem);
	}

	.how__icon-wrap {
		width: 3rem;
		height: 3rem;
		border-radius: var(--radius-lg);
		background: color-mix(in srgb, var(--color-primary) 10%, transparent);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: var(--space-6);
	}

	.how__icon-wrap--tertiary {
		background: color-mix(in srgb, var(--color-tertiary) 10%, transparent);
	}

	.how__icon {
		color: var(--color-primary);
		font-size: 1.5rem;
	}

	.how__icon-wrap--tertiary .how__icon {
		color: var(--color-tertiary);
	}

	.how__icon--fill {
		font-variation-settings: 'FILL' 1, 'wght' 400, 'GRAD' 0, 'opsz' 24;
	}

	.how__title {
		margin: 0 0 var(--space-2);
		font-size: var(--text-base);
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.how__desc {
		font-size: var(--text-sm);
		line-height: 1.5;
	}

	.how__chevron {
		display: none;
		position: absolute;
		top: 50%;
		right: -1rem;
		transform: translate(50%, -50%);
		color: color-mix(in srgb, var(--color-primary) 20%, transparent);
		font-size: 2.25rem;
	}

	.md-only {
		display: none;
	}

	@media (min-width: 768px) {
		.md-only {
			display: block;
		}
	}
</style>
