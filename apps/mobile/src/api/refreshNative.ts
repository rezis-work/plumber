import { getRefreshToken, setRefreshToken } from '../auth/secureRefreshToken';
import { ApiError, apiRequest } from './client';
import type { LoginResponse, MeResponse } from './types';

/**
 * POST /auth/refresh with JSON body (ADR 002). Requires API support.
 * Persists rotated refresh_token when present in the response.
 */
export async function refreshWithStoredToken(): Promise<LoginResponse> {
	const refresh_token = await getRefreshToken();
	if (!refresh_token) {
		throw new ApiError(401, 'no_refresh', 'No refresh token stored');
	}

	const login = await apiRequest<LoginResponse>('/auth/refresh', {
		method: 'POST',
		jsonBody: { refresh_token },
		nativeClient: true
	});

	if (login.refresh_token) {
		await setRefreshToken(login.refresh_token);
	}

	return login;
}

export function authMe(accessToken: string): Promise<MeResponse> {
	return apiRequest<MeResponse>('/auth/me', {
		method: 'GET',
		accessToken
	});
}
