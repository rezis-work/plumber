import { apiUrl } from './publicOrigin';
import type { LoginRequest, LoginResponse, LogoutAllResponse, MeResponse } from './types';

export class ApiError extends Error {
	constructor(
		readonly status: number,
		readonly code?: string,
		message?: string
	) {
		super(message ?? `HTTP ${status}`);
		this.name = 'ApiError';
	}
}

export type ApiRequestOptions = Omit<RequestInit, 'body'> & {
	jsonBody?: unknown;
	accessToken?: string | null;
	credentials?: RequestCredentials;
};

async function parseJsonError(res: Response): Promise<{ code?: string; message?: string }> {
	const ct = res.headers.get('content-type') ?? '';
	if (!ct.includes('application/json')) {
		return {};
	}
	try {
		const j: unknown = await res.json();
		if (j && typeof j === 'object' && 'error' in j) {
			const o = j as { error?: string; message?: string };
			return { code: o.error, message: o.message };
		}
	} catch {
		/* ignore */
	}
	return {};
}

/**
 * Low-level JSON fetch. Does not log tokens or passwords.
 * Default `credentials: 'include'` for refresh cookies (override with `'omit'` when appropriate).
 */
export async function apiRequest<T>(path: string, options: ApiRequestOptions = {}): Promise<T> {
	const {
		jsonBody,
		accessToken,
		credentials = 'include',
		method = 'GET',
		headers: initHeaders,
		...rest
	} = options;

	const headers = new Headers(initHeaders);
	if (jsonBody !== undefined) {
		headers.set('Content-Type', 'application/json');
	}
	if (accessToken) {
		headers.set('Authorization', `Bearer ${accessToken}`);
	}

	const url = apiUrl(path);
	const body = jsonBody !== undefined ? JSON.stringify(jsonBody) : undefined;

	const res = await fetch(url, {
		...rest,
		method,
		headers,
		credentials,
		body
	});

	if (!res.ok) {
		const { code, message } = await parseJsonError(res);
		throw new ApiError(res.status, code, message);
	}

	if (res.status === 204) {
		return undefined as T;
	}

	const ct = res.headers.get('content-type') ?? '';
	if (!ct.includes('application/json')) {
		return undefined as T;
	}

	return res.json() as Promise<T>;
}

export function authRefresh(): Promise<LoginResponse> {
	return apiRequest<LoginResponse>('/auth/refresh', { method: 'POST' });
}

export function authLogin(body: LoginRequest): Promise<LoginResponse> {
	return apiRequest<LoginResponse>('/auth/login', {
		method: 'POST',
		jsonBody: body
	});
}

export function authLogout(): Promise<void> {
	return apiRequest<void>('/auth/logout', { method: 'POST' });
}

export function authLogoutAll(accessToken: string): Promise<LogoutAllResponse> {
	return apiRequest<LogoutAllResponse>('/auth/logout-all', {
		method: 'POST',
		accessToken
	});
}

export function authMe(accessToken: string): Promise<MeResponse> {
	return apiRequest<MeResponse>('/auth/me', {
		method: 'GET',
		accessToken
	});
}
