<script lang="ts">
	import { browser } from '$app/environment';
	import { base } from '$app/paths';
	import { onMount } from 'svelte';
	import {
		PENDING_EMAIL_VERIFICATION_KEY,
		type PendingEmailVerification
	} from '$lib/auth/pendingVerification';

	let devPayload = $state<PendingEmailVerification | null>(null);

	onMount(() => {
		if (!browser) {
			return;
		}
		const raw = sessionStorage.getItem(PENDING_EMAIL_VERIFICATION_KEY);
		if (import.meta.env.DEV && raw) {
			try {
				devPayload = JSON.parse(raw) as PendingEmailVerification;
			} catch {
				devPayload = null;
			}
		}
		if (raw) {
			sessionStorage.removeItem(PENDING_EMAIL_VERIFICATION_KEY);
		}
	});
</script>

<svelte:head>
	<title>Verify your email | Fixavon</title>
</svelte:head>

<main class="wrap">
	<div class="card">
		<h1 class="title">Check your email</h1>
		<p class="lead">
			We sent a verification link to your address. Open it to activate your account. If you do not see
			the message, check your spam folder.
		</p>
		{#if import.meta.env.DEV && devPayload}
			<div class="dev">
				<p class="dev__label">Development only</p>
				<p class="dev__meta">Expires: {devPayload.expires_at}</p>
				<pre class="dev__token">{devPayload.token}</pre>
			</div>
		{/if}
		<div class="actions">
			<a class="link" href="{base}/login">Go to log in</a>
			<a class="link-muted" href="{base}/">Back to home</a>
		</div>
	</div>
</main>

<style>
	.wrap {
		min-height: 60vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-24) var(--space-4);
	}

	.card {
		max-width: 28rem;
		width: 100%;
		background: var(--color-surface-elevated);
		border-radius: var(--radius-xl);
		padding: var(--space-8);
		box-shadow: var(--shadow-ambient);
	}

	.title {
		margin: 0 0 var(--space-4);
		font-size: var(--text-lg);
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.lead {
		margin: 0 0 var(--space-8);
		color: var(--color-text-muted);
		line-height: 1.6;
		font-size: var(--text-base);
	}

	.dev {
		margin-bottom: var(--space-8);
		padding: var(--space-4);
		border-radius: var(--radius-md);
		background: var(--color-surface-container-low);
		border: 1px dashed var(--color-outline-variant);
	}

	.dev__label {
		margin: 0 0 var(--space-2);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-semibold);
		color: var(--color-primary);
	}

	.dev__meta {
		margin: 0 0 var(--space-2);
		font-size: var(--text-sm);
		color: var(--color-text-muted);
	}

	.dev__token {
		margin: 0;
		font-size: 0.7rem;
		word-break: break-all;
		white-space: pre-wrap;
		color: var(--color-text);
		background: var(--color-surface);
		padding: var(--space-3);
		border-radius: var(--radius-sm);
		overflow-x: auto;
	}

	.actions {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.link {
		color: var(--color-primary);
		font-weight: var(--font-weight-semibold);
		text-decoration: none;
	}

	.link:hover {
		text-decoration: underline;
	}

	.link-muted {
		font-size: var(--text-sm);
		color: var(--color-text-muted);
		text-decoration: none;
	}

	.link-muted:hover {
		color: var(--color-primary);
	}
</style>
