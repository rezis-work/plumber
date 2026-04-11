<script lang="ts">
	import type { MeResponse } from '$lib/api/types';
	import { logoutEverywhere, logoutFromApp } from '$lib/auth/logout';

	type Props = {
		user: MeResponse | null;
		loading?: boolean;
		error?: string | null;
	};

	let { user, loading = false, error = null }: Props = $props();

	let logoutBusy = $state(false);

	function fmtDate(iso: string): string {
		const d = new Date(iso);
		if (Number.isNaN(d.getTime())) return iso;
		return d.toLocaleString();
	}

	async function onLogout() {
		if (logoutBusy) return;
		logoutBusy = true;
		try {
			await logoutFromApp();
		} finally {
			logoutBusy = false;
		}
	}

	async function onLogoutAll() {
		if (logoutBusy) return;
		logoutBusy = true;
		try {
			await logoutEverywhere();
		} finally {
			logoutBusy = false;
		}
	}
</script>

<div class="panel">
	{#if loading}
		<p class="muted">Loading profile…</p>
	{:else if error}
		<p class="err" role="alert">{error}</p>
	{:else if user}
		<h1 class="h1">Your profile</h1>
		<dl class="dl">
			<dt>Email</dt>
			<dd>{user.email}</dd>
			<dt>Role</dt>
			<dd>{user.role}</dd>
			<dt>Account status</dt>
			<dd>{user.is_active ? 'Active' : 'Inactive'}</dd>
			<dt>Email verified</dt>
			<dd>{user.is_email_verified ? 'Yes' : 'No'}</dd>
			<dt>User ID</dt>
			<dd class="mono">{user.id}</dd>
			<dt>Member since</dt>
			<dd>{fmtDate(user.created_at)}</dd>
			<dt>Last updated</dt>
			<dd>{fmtDate(user.updated_at)}</dd>
		</dl>
		{#if user.profile}
			<h2 class="h2">Plumber profile</h2>
			<dl class="dl">
				<dt>Full name</dt>
				<dd>{user.profile.full_name}</dd>
				<dt>Phone</dt>
				<dd>{user.profile.phone}</dd>
				<dt>Years of experience</dt>
				<dd>{user.profile.years_of_experience}</dd>
			</dl>
		{/if}
		<div class="actions">
			<button type="button" class="btn btn--secondary" disabled={logoutBusy} onclick={onLogout}>
				Log out
			</button>
			<button type="button" class="btn btn--muted" disabled={logoutBusy} onclick={onLogoutAll}>
				Log out everywhere
			</button>
		</div>
	{/if}
</div>

<style>
	.panel {
		max-width: 36rem;
		margin: 0 auto;
		padding: var(--space-8);
		background: var(--color-surface-elevated);
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-ambient);
		border: 1px solid color-mix(in srgb, var(--color-outline-variant) 35%, transparent);
	}

	.h1 {
		margin: 0 0 var(--space-6);
		font-size: 1.5rem;
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.h2 {
		margin: var(--space-8) 0 var(--space-4);
		font-size: 1.125rem;
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
	}

	.dl {
		margin: 0;
		display: grid;
		grid-template-columns: 10rem 1fr;
		gap: var(--space-3) var(--space-6);
		font-size: var(--text-sm);
	}

	.dl dt {
		margin: 0;
		color: var(--color-text-muted);
		font-weight: var(--font-weight-medium);
	}

	.dl dd {
		margin: 0;
		color: var(--color-text);
	}

	.mono {
		font-family: ui-monospace, monospace;
		font-size: 0.8125rem;
		word-break: break-all;
	}

	.muted {
		margin: 0;
		color: var(--color-text-muted);
		font-size: var(--text-sm);
	}

	.err {
		margin: 0;
		color: var(--color-error);
		font-size: var(--text-sm);
	}

	.actions {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-4);
		margin-top: var(--space-8);
		padding-top: var(--space-6);
		border-top: 1px solid color-mix(in srgb, var(--color-outline-variant) 35%, transparent);
	}

	.btn {
		padding: 0.625rem 1.25rem;
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-semibold);
		font-family: inherit;
		cursor: pointer;
		border: none;
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn--secondary {
		background: var(--color-primary);
		color: var(--color-on-primary);
	}

	.btn--secondary:hover:not(:disabled) {
		background: var(--color-primary-container);
	}

	.btn--muted {
		background: transparent;
		color: var(--color-text-muted);
		border: 1px solid var(--color-outline-variant);
	}

	.btn--muted:hover:not(:disabled) {
		background: var(--color-surface-container-low);
		color: var(--color-text);
	}

	@media (max-width: 520px) {
		.dl {
			grid-template-columns: 1fr;
		}
	}
</style>
