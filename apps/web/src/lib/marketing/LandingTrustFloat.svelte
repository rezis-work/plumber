<div class="float" role="status">
	<span class="float__dot" aria-hidden="true"></span>
	<span class="float__text">3 plumbers active in Tbilisi right now</span>
</div>

<style>
	.float {
		display: none;
		position: fixed;
		bottom: var(--space-6);
		left: var(--space-6);
		z-index: 40;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-3) var(--space-4);
		border-radius: 9999px;
		background: color-mix(in srgb, var(--color-surface-elevated) 70%, transparent);
		backdrop-filter: blur(24px);
		-webkit-backdrop-filter: blur(24px);
		box-shadow: var(--shadow-ambient);
		border: 1px solid color-mix(in srgb, var(--color-outline-variant) 10%, transparent);
		max-width: calc(100vw - var(--space-12));
	}

	@media (min-width: 640px) {
		.float {
			display: flex;
		}
	}

	.float__dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 50%;
		background: var(--color-success);
		flex-shrink: 0;
	}

	.float__text {
		font-size: 0.75rem;
		font-weight: var(--font-weight-semibold);
		color: var(--color-text);
	}
</style>
