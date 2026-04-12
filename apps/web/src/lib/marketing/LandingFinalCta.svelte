<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve -- hrefs use pathWithLang for ?lang= */
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { pathWithLang } from '$lib/i18n/url';
	import { translate } from '$lib/i18n/translate';

	const sp = $derived(page.url.searchParams);
	const loc = $derived(page.data.locale);
	const hrefRegister = $derived(`${base}${pathWithLang('/register', sp, loc)}`);
	const hrefRegisterPlumber = $derived(`${base}${pathWithLang('/register/plumber', sp, loc)}`);
</script>

<section class="final lp-section">
	<div class="lp-wrap final__grid">
		<div class="final__card final__card--primary">
			<div class="final__glow final__glow--tr" aria-hidden="true"></div>
			<div class="final__content">
				<h2 class="final__title">{translate(loc, 'marketing.finalCta.needHelpTitle')}</h2>
				<p class="final__text">
					{translate(loc, 'marketing.finalCta.needHelpText')}
				</p>
				<a class="lp-btn lp-btn--inverse" href={hrefRegister}
					>{translate(loc, 'marketing.finalCta.needHelpCta')}</a
				>
			</div>
		</div>
		<div class="final__card final__card--mint">
			<div class="final__glow final__glow--bl" aria-hidden="true"></div>
			<div class="final__content">
				<h2 class="final__title final__title--dark">{translate(loc, 'marketing.finalCta.plumberTitle')}</h2>
				<p class="final__text final__text--dark">
					{translate(loc, 'marketing.finalCta.plumberText')}
				</p>
				<a class="lp-btn lp-btn--on-dark" href={hrefRegisterPlumber}
					>{translate(loc, 'marketing.finalCta.plumberCta')}</a
				>
			</div>
		</div>
	</div>
</section>

<style>
	.final__grid {
		display: grid;
		gap: var(--space-6);
	}

	@media (min-width: 768px) {
		.final__grid {
			grid-template-columns: 1fr 1fr;
		}
	}

	.final__card {
		position: relative;
		padding: var(--space-12);
		border-radius: 2.5rem;
		overflow: hidden;
	}

	.final__card--primary {
		background: var(--color-primary);
		color: var(--color-on-primary);
	}

	.final__card--mint {
		background: var(--color-secondary-container);
		color: var(--color-on-secondary-container);
	}

	.final__glow {
		position: absolute;
		width: 16rem;
		height: 16rem;
		border-radius: 50%;
		background: color-mix(in srgb, var(--color-on-primary) 10%, transparent);
		filter: blur(48px);
		pointer-events: none;
		transition: transform 0.5s ease;
	}

	.final__glow--tr {
		top: -6rem;
		right: -6rem;
	}

	.final__glow--bl {
		bottom: -6rem;
		left: -6rem;
		background: color-mix(in srgb, var(--color-primary) 10%, transparent);
	}

	.final__card:hover .final__glow {
		transform: scale(1.5);
	}

	.final__content {
		position: relative;
		z-index: 1;
	}

	.final__title {
		font-size: clamp(1.5rem, 3vw, 1.875rem);
		font-weight: 900;
		margin: 0 0 var(--space-4);
	}

	.final__title--dark {
		color: var(--color-on-secondary-container);
	}

	.final__text {
		margin: 0 0 var(--space-8);
		max-width: 28ch;
		color: color-mix(in srgb, var(--color-on-primary) 70%, transparent);
		line-height: 1.5;
		font-size: var(--text-base);
	}

	.final__text--dark {
		color: color-mix(in srgb, var(--color-on-secondary-container) 70%, transparent);
	}
</style>
