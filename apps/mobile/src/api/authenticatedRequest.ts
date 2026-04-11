import { getMobileAuthBridge } from './authBridge';
import { ApiError, apiRequest, type ApiRequestOptions } from './client';
import { authMe, refreshWithStoredToken } from './refreshNative';

/**
 * JSON fetch with Bearer from the auth bridge unless `options.accessToken` is set.
 * On first 401: one native refresh + `/auth/me`, update bridge, retry once.
 * If refresh fails: clearSession and rethrow the original 401.
 *
 * Parallel 401s may each attempt refresh (no mutex in v1).
 */
export async function apiRequestAuthenticated<T>(
	path: string,
	options: ApiRequestOptions = {}
): Promise<T> {
	const { accessToken: explicitToken, ...restOptions } = options;
	const bridge = getMobileAuthBridge();

	const run = (): Promise<T> => {
		const token =
			explicitToken !== undefined ? (explicitToken ?? null) : bridge.getAccessToken();
		return apiRequest<T>(path, {
			...restOptions,
			accessToken: token ?? undefined
		});
	};

	try {
		return await run();
	} catch (e) {
		if (!(e instanceof ApiError) || e.status !== 401) {
			throw e;
		}

		try {
			const login = await refreshWithStoredToken();
			bridge.setAccessSession(login.access_token, login.expires_in);
			bridge.setHasRefreshTokenStored(true);
			const user = await authMe(login.access_token);
			bridge.setUser(user);
			// Retry with fresh token — do not read bridge yet; React state may not have flushed.
			return await apiRequest<T>(path, {
				...restOptions,
				accessToken: login.access_token
			});
		} catch {
			await bridge.clearSession();
			throw e;
		}
	}
}
