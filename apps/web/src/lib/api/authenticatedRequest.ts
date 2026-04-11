import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { base } from '$app/paths';
import { clearSession, session } from '$lib/auth/session.svelte';
import { ApiError, apiRequest, authMe, authRefresh } from './client';
import type { ApiRequestOptions } from './client';

/**
 * Browser-only JSON fetch for protected resources: sends `Authorization: Bearer` from
 * `session.accessToken` unless `options.accessToken` is set explicitly.
 * On first 401: one `POST /auth/refresh`, update session + `/auth/me`, retry once with the new token.
 * If refresh (or post-refresh `/me`) fails: `clearSession()`, `goto` login, rethrow original 401.
 */
export async function apiRequestAuthenticated<T>(
	path: string,
	options: ApiRequestOptions = {}
): Promise<T> {
	if (!browser) {
		throw new Error('apiRequestAuthenticated is browser-only');
	}

	const { accessToken: explicitToken, ...restOptions } = options;

	const run = (useSessionOnly: boolean): Promise<T> => {
		let token: string | null | undefined;
		if (useSessionOnly) {
			token = session.accessToken;
		} else if (explicitToken !== undefined) {
			token = explicitToken ?? null;
		} else {
			token = session.accessToken;
		}
		return apiRequest<T>(path, {
			...restOptions,
			accessToken: token ?? undefined
		});
	};

	try {
		return await run(false);
	} catch (e) {
		if (!(e instanceof ApiError) || e.status !== 401) {
			throw e;
		}

		try {
			const login = await authRefresh();
			session.accessToken = login.access_token;
			session.user = await authMe(login.access_token);
			session.lastError = null;
		} catch {
			clearSession();
			void goto(`${base}/login`);
			throw e;
		}

		return await run(true);
	}
}
