<script lang="ts">
	import { page } from '$app/state';
	import { translate } from '$lib/i18n/translate';

	const loc = $derived(page.data.locale);
	const COV_ACTIVE = 3;
	const districtKeys = [
		'marketing.coverage.vake',
		'marketing.coverage.saburtalo',
		'marketing.coverage.dighomi',
		'marketing.coverage.gldani',
		'marketing.coverage.isani',
		'marketing.coverage.samgori'
	] as const;
</script>

<section class="cov lp-section">
	<div class="lp-wrap cov__grid">
		<div class="cov__map-block">
			<div class="cov__map-wrap">
				<img
					class="cov__map"
					src="/marketing/tbilisi-coverage.jpg"
					alt={translate(loc, 'marketing.coverage.mapAlt')}
					width="800"
					height="400"
				/>
				<div class="cov__map-grad" aria-hidden="true"></div>
				<div class="cov__badge">
					<span class="cov__dot lp-pulse" aria-hidden="true"></span>
					<span class="cov__badge-text"
						>{translate(loc, 'marketing.coverage.badge', { count: COV_ACTIVE })}</span
					>
				</div>
			</div>
		</div>
		<div class="cov__copy">
			<h2 class="lp-heading-lg">{translate(loc, 'marketing.coverage.title')}</h2>
			<p class="cov__lead lp-text-muted">
				{translate(loc, 'marketing.coverage.lead')}
			</p>
			<div class="cov__districts">
				{#each districtKeys as key}
					<div class="cov__district">
						<span class="material-symbols-outlined cov__pin">location_on</span>
						<span class="cov__district-name">{translate(loc, key)}</span>
					</div>
				{/each}
			</div>
		</div>
	</div>
</section>

<style>
	.cov__grid {
		display: grid;
		gap: var(--space-16);
		align-items: center;
	}

	@media (min-width: 1024px) {
		.cov__grid {
			grid-template-columns: 1fr 1fr;
		}
	}

	.cov__map-wrap {
		position: relative;
		height: 25rem;
		border-radius: 2rem;
		overflow: hidden;
		box-shadow: 0 25px 50px color-mix(in srgb, var(--color-text) 12%, transparent);
	}

	.cov__map {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.cov__map-grad {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			to top,
			color-mix(in srgb, var(--color-primary) 40%, transparent),
			transparent
		);
		pointer-events: none;
	}

	.cov__badge {
		position: absolute;
		bottom: var(--space-8);
		left: var(--space-8);
		display: flex;
		align-items: center;
		gap: var(--space-3);
		background: color-mix(in srgb, var(--color-surface-elevated) 90%, transparent);
		backdrop-filter: blur(12px);
		padding: var(--space-4);
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-ambient);
	}

	.cov__dot {
		width: 0.75rem;
		height: 0.75rem;
		border-radius: 50%;
		background: var(--color-tertiary);
	}

	.cov__badge-text {
		font-size: var(--text-sm);
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.cov__lead {
		margin: var(--space-8) 0;
		line-height: 1.6;
		font-size: var(--text-base);
	}

	.cov__districts {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-4);
	}

	.cov__district {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-4);
		border-radius: var(--radius-xl);
		background: var(--color-surface-container-low);
	}

	.cov__pin {
		color: var(--color-primary);
		font-size: 1.25rem;
	}

	.cov__district-name {
		font-weight: var(--font-weight-semibold);
		color: var(--color-text);
	}
</style>
