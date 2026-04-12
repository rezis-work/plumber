<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve -- hrefs use pathWithLang for ?lang= */
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { pathWithLang } from '$lib/i18n/url';
	import { translate } from '$lib/i18n/translate';

	const sp = $derived(page.url.searchParams);
	const loc = $derived(page.data.locale);
	const hrefRegisterPlumber = $derived(`${base}${pathWithLang('/register/plumber', sp, loc)}`);
</script>

<section class="plumber lp-section" id="for-plumbers">
	<div class="lp-wrap">
		<div class="plumber__panel">
			<div class="plumber__copy">
				<h2 class="plumber__title">{translate(loc, 'marketing.plumberSection.title')}</h2>
				<ul class="plumber__list">
					<li class="plumber__li">
						<span class="material-symbols-outlined plumber__check">check</span>
						<div>
							<h4 class="plumber__li-title">{translate(loc, 'marketing.plumberSection.li1Title')}</h4>
							<p class="plumber__li-text lp-text-muted">
								{translate(loc, 'marketing.plumberSection.li1Text')}
							</p>
						</div>
					</li>
					<li class="plumber__li">
						<span class="material-symbols-outlined plumber__check">check</span>
						<div>
							<h4 class="plumber__li-title">{translate(loc, 'marketing.plumberSection.li2Title')}</h4>
							<p class="plumber__li-text lp-text-muted">
								{translate(loc, 'marketing.plumberSection.li2Text')}
							</p>
						</div>
					</li>
					<li class="plumber__li">
						<span class="material-symbols-outlined plumber__check">check</span>
						<div>
							<h4 class="plumber__li-title">{translate(loc, 'marketing.plumberSection.li3Title')}</h4>
							<p class="plumber__li-text lp-text-muted">
								{translate(loc, 'marketing.plumberSection.li3Text')}
							</p>
						</div>
					</li>
				</ul>
				<a class="lp-btn lp-btn--primary" href={hrefRegisterPlumber} style="margin-top: var(--space-4);"
					>{translate(loc, 'marketing.plumberSection.cta')}</a
				>
			</div>
			<div class="plumber__media">
				<div class="plumber__img-wrap">
					<img
						class="plumber__img"
						src="/marketing/plumber-hero.png"
						alt={translate(loc, 'marketing.plumberSection.imageAlt')}
						width="560"
						height="560"
					/>
				</div>
				<div class="plumber__stat">
					<div class="plumber__stat-row">
						<div class="plumber__stat-icon">
							<span class="material-symbols-outlined">trending_up</span>
						</div>
						<p class="plumber__stat-num">+45%</p>
					</div>
					<p class="plumber__stat-note lp-text-muted">
						{translate(loc, 'marketing.plumberSection.statNote')}
					</p>
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.plumber {
		overflow: hidden;
	}

	.plumber__panel {
		display: flex;
		flex-direction: column;
		gap: var(--space-16);
		align-items: center;
		background: var(--color-surface-container-high);
		border-radius: 2.5rem;
		padding: var(--space-8);
	}

	@media (min-width: 1024px) {
		.plumber__panel {
			flex-direction: row;
			padding: var(--space-16);
		}
	}

	.plumber__copy {
		flex: 1;
	}

	.plumber__title {
		font-size: clamp(1.75rem, 4vw, 2.5rem);
		font-weight: 900;
		line-height: 1.15;
		margin: 0 0 var(--space-8);
		color: var(--color-text);
	}

	.plumber__list {
		list-style: none;
		margin: 0 0 var(--space-10);
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
	}

	.plumber__li {
		display: flex;
		gap: var(--space-4);
		align-items: flex-start;
	}

	.plumber__check {
		flex-shrink: 0;
		color: var(--color-primary);
		background: var(--color-surface-elevated);
		padding: 0.25rem;
		border-radius: 50%;
		font-size: 1.125rem;
		box-shadow: 0 2px 8px color-mix(in srgb, var(--color-text) 8%, transparent);
	}

	.plumber__li-title {
		margin: 0 0 var(--space-1);
		font-size: var(--text-base);
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.plumber__li-text {
		margin: 0;
		font-size: var(--text-sm);
	}

	.plumber__media {
		flex: 1;
		position: relative;
		width: 100%;
		max-width: 32rem;
	}

	.plumber__img-wrap {
		aspect-ratio: 1;
		border-radius: 2rem;
		overflow: hidden;
		box-shadow: 0 25px 50px color-mix(in srgb, var(--color-text) 15%, transparent);
	}

	.plumber__img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.plumber__stat {
		position: absolute;
		bottom: -2rem;
		left: -2rem;
		background: var(--color-surface-elevated);
		padding: var(--space-6);
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-ambient);
		max-width: 15rem;
	}

	@media (max-width: 639px) {
		.plumber__stat {
			position: relative;
			bottom: auto;
			left: auto;
			margin-top: var(--space-4);
			max-width: none;
		}
	}

	.plumber__stat-row {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		margin-bottom: var(--space-4);
	}

	.plumber__stat-icon {
		width: 3rem;
		height: 3rem;
		border-radius: 50%;
		background: color-mix(in srgb, var(--color-tertiary) 10%, transparent);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-tertiary);
	}

	.plumber__stat-num {
		margin: 0;
		font-size: 1.5rem;
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.plumber__stat-note {
		margin: 0;
		font-size: 0.75rem;
		line-height: 1.4;
	}
</style>
