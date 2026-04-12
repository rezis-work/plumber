<script lang="ts">
	import { page } from '$app/state';
	import { translate } from '$lib/i18n/translate';

	const loc = $derived(page.data.locale);
</script>

<section class="t lp-section">
	<div class="lp-wrap">
		<h2 class="t__title lp-heading-lg">{translate(loc, 'marketing.testimonials.title')}</h2>
		<div class="t__grid">
			<article class="t__card">
				<div class="t__stars" aria-label={translate(loc, 'marketing.testimonials.starsLabel')}>
					{#each [1, 2, 3, 4, 5] as _}
						<span class="material-symbols-outlined t__star">star</span>
					{/each}
				</div>
				<p class="t__quote lp-text-muted">
					{translate(loc, 'marketing.testimonials.q1')}
				</p>
				<div class="t__person">
					<img
						class="t__avatar"
						src="/marketing/testimonial-1.png"
						alt=""
						width="48"
						height="48"
					/>
					<div>
						<p class="t__name">{translate(loc, 'marketing.testimonials.n1')}</p>
						<p class="t__role lp-text-muted">{translate(loc, 'marketing.testimonials.r1')}</p>
					</div>
				</div>
			</article>
			<article class="t__card">
				<div class="t__stars" aria-label={translate(loc, 'marketing.testimonials.starsLabel')}>
					{#each [1, 2, 3, 4, 5] as _}
						<span class="material-symbols-outlined t__star">star</span>
					{/each}
				</div>
				<p class="t__quote lp-text-muted">
					{translate(loc, 'marketing.testimonials.q2')}
				</p>
				<div class="t__person">
					<img
						class="t__avatar"
						src="/marketing/testimonial-2.png"
						alt=""
						width="48"
						height="48"
					/>
					<div>
						<p class="t__name">{translate(loc, 'marketing.testimonials.n2')}</p>
						<p class="t__role lp-text-muted">{translate(loc, 'marketing.testimonials.r2')}</p>
					</div>
				</div>
			</article>
			<article class="t__card">
				<div class="t__stars" aria-label={translate(loc, 'marketing.testimonials.starsLabel')}>
					{#each [1, 2, 3, 4, 5] as _}
						<span class="material-symbols-outlined t__star">star</span>
					{/each}
				</div>
				<p class="t__quote lp-text-muted">
					{translate(loc, 'marketing.testimonials.q3')}
				</p>
				<div class="t__person">
					<img
						class="t__avatar"
						src="/marketing/testimonial-3.png"
						alt=""
						width="48"
						height="48"
					/>
					<div>
						<p class="t__name">{translate(loc, 'marketing.testimonials.n3')}</p>
						<p class="t__role lp-text-muted">{translate(loc, 'marketing.testimonials.r3')}</p>
					</div>
				</div>
			</article>
		</div>
	</div>
</section>

<style>
	.t {
		background: var(--color-surface-container-low);
	}

	.t__title {
		text-align: center;
		margin-bottom: var(--space-16);
	}

	.t__grid {
		display: grid;
		gap: var(--space-8);
	}

	@media (min-width: 768px) {
		.t__grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.t__card {
		background: var(--color-surface-elevated);
		padding: var(--space-8);
		border-radius: var(--radius-xl);
		box-shadow: 0 2px 12px color-mix(in srgb, var(--color-text) 6%, transparent);
	}

	.t__stars {
		display: flex;
		gap: 0.125rem;
		margin-bottom: var(--space-6);
		color: var(--color-secondary);
	}

	.t__star {
		font-size: 1.25rem;
		font-variation-settings: 'FILL' 1, 'wght' 400, 'GRAD' 0, 'opsz' 24;
	}

	.t__quote {
		margin: 0 0 var(--space-8);
		font-style: italic;
		line-height: 1.6;
		font-size: var(--text-sm);
	}

	.t__person {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}

	.t__avatar {
		width: 3rem;
		height: 3rem;
		border-radius: 50%;
		object-fit: cover;
	}

	.t__name {
		margin: 0;
		font-size: var(--text-sm);
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.t__role {
		margin: 0;
		font-size: 0.75rem;
	}
</style>
