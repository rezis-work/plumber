import { ApiError, authMe, authRefresh } from '$lib/api/client';
import type { LoginResponse, MeResponse } from '$lib/api/types';

/**
 * Auth state: mutate fields only; do not reassign this object.
 *
 * In `.svelte` files, derive flags locally so they stay reactive, e.g.:
 * `const isAuthenticated = $derived(session.user !== null && session.accessToken !== null);`
 * `const role = $derived(session.user?.role ?? null);`
 * `const isEmailVerified = $derived(session.user?.is_email_verified ?? false);`
 */
export const session = $state({
	accessToken: null as string | null,
	user: null as MeResponse | null,
	hydrating: false,
	lastError: null as string | null
});

export function clearSession(): void {
	session.accessToken = null;
	session.user = null;
	session.lastError = null;
}

let hydrateInFlight: Promise<void> | null = null;

/**
 * Silent refresh: cookie session → access token → `/auth/me`.
 * On failure (including 401), clears session. No retry loop.
 * Skips if `accessToken` is already set. Overlapping callers await the same in-flight attempt.
 */
export async function hydrateFromRefresh(): Promise<void> {
	if (session.accessToken !== null) {
		return;
	}
	if (hydrateInFlight) {
		return hydrateInFlight;
	}

	const run = async (): Promise<void> => {
		session.hydrating = true;
		session.lastError = null;
		try {
			const login = await authRefresh();
			session.accessToken = login.access_token;
			session.user = await authMe(login.access_token);
		} catch (e) {
			clearSession();
			if (e instanceof ApiError && e.status !== 401) {
				session.lastError = e.message;
			}
		} finally {
			session.hydrating = false;
		}
	};

	const p = run();
	hydrateInFlight = p;
	try {
		await p;
	} finally {
		hydrateInFlight = null;
	}
}

/** After `POST /auth/login` returns (cookie + body); loads user from `/auth/me`. */
export async function setSessionFromLogin(response: LoginResponse): Promise<void> {
	session.lastError = null;
	session.accessToken = response.access_token;
	try {
		session.user = await authMe(response.access_token);
	} catch (e) {
		clearSession();
		if (e instanceof Error) {
			session.lastError = e.message;
		}
		throw e;
	}
}
