<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve -- hrefs use pathWithLang for ?lang= */
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { pathWithLang } from '$lib/i18n/url';
	import { translate } from '$lib/i18n/translate';

	const sp = $derived(page.url.searchParams);
	const loc = $derived(page.data.locale);
	const hrefRegister = $derived(`${base}${pathWithLang('/register', sp, loc)}`);
</script>

<section class="exp lp-section" id="services">
	<div class="lp-wrap">
		<div class="exp__intro">
			<h2 class="exp__title lp-heading-lg">{translate(loc, 'marketing.expertise.title')}</h2>
			<p class="exp__sub">{translate(loc, 'marketing.expertise.subtitle')}</p>
		</div>
		<div class="exp__grid">
			<button type="button" class="exp__tile">
				<span class="material-symbols-outlined exp__icon">water_damage</span>
				<span class="exp__label">{translate(loc, 'marketing.expertise.pipeLeaks')}</span>
			</button>
			<button type="button" class="exp__tile">
				<span class="material-symbols-outlined exp__icon">format_color_reset</span>
				<span class="exp__label">{translate(loc, 'marketing.expertise.cloggedDrains')}</span>
			</button>
			<button type="button" class="exp__tile">
				<span class="material-symbols-outlined exp__icon">potted_plant</span>
				<span class="exp__label">{translate(loc, 'marketing.expertise.toiletRepair')}</span>
			</button>
			<button type="button" class="exp__tile">
				<span class="material-symbols-outlined exp__icon">countertops</span>
				<span class="exp__label">{translate(loc, 'marketing.expertise.sinkRepair')}</span>
			</button>
			<button type="button" class="exp__tile">
				<span class="material-symbols-outlined exp__icon">hot_tub</span>
				<span class="exp__label">{translate(loc, 'marketing.expertise.waterHeater')}</span>
			</button>
			<button type="button" class="exp__tile">
				<span class="material-symbols-outlined exp__icon">plumbing</span>
				<span class="exp__label">{translate(loc, 'marketing.expertise.installation')}</span>
			</button>
			<a class="exp__tile exp__tile--cta" href={hrefRegister}>
				<span class="material-symbols-outlined exp__icon exp__icon--on-cta">e911_emergency</span>
				<span class="exp__label exp__label--bold">{translate(loc, 'marketing.expertise.emergency')}</span>
			</a>
		</div>
	</div>
</section>

<style>
	.exp {
		background: var(--color-inverse-surface);
		color: var(--color-inverse-on-surface);
	}

	.exp__intro {
		margin-bottom: var(--space-16);
	}

	.exp__title {
		color: var(--color-inverse-on-surface);
		margin-bottom: var(--space-4);
	}

	.exp__sub {
		margin: 0;
		color: color-mix(in srgb, var(--color-inverse-on-surface) 60%, transparent);
		font-size: var(--text-base);
	}

	.exp__grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-4);
	}

	@media (min-width: 768px) {
		.exp__grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	@media (min-width: 1024px) {
		.exp__grid {
			grid-template-columns: repeat(7, 1fr);
		}
	}

	.exp__tile {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-4);
		padding: var(--space-6);
		border-radius: var(--radius-xl);
		background: color-mix(in srgb, var(--color-inverse-on-surface) 5%, transparent);
		border: none;
		cursor: pointer;
		color: inherit;
		font-family: inherit;
		transition: background 0.2s ease;
		text-decoration: none;
		text-align: center;
	}

	.exp__tile:hover {
		background: color-mix(in srgb, var(--color-inverse-on-surface) 10%, transparent);
	}

	.exp__tile--cta {
		grid-column: span 2;
		background: var(--color-primary-container);
		color: var(--color-on-primary);
	}

	@media (min-width: 768px) {
		.exp__tile--cta {
			grid-column: span 1;
		}
	}

	.exp__tile--cta:hover {
		filter: brightness(1.05);
	}

	.exp__icon {
		font-size: 2rem;
		color: var(--color-secondary-container);
		transition: transform 0.2s ease;
	}

	.exp__tile:hover .exp__icon {
		transform: scale(1.1);
	}

	.exp__icon--on-cta {
		color: var(--color-on-primary);
	}

	.exp__label {
		font-size: var(--text-sm);
		font-weight: var(--font-weight-semibold);
	}

	.exp__label--bold {
		font-weight: var(--font-weight-bold);
	}
</style>
