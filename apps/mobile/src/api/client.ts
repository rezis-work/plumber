/**
 * Low-level JSON fetch for native clients (ADR 002). No cookies — use SecureStore refresh + JSON body on auth routes.
 * Requires API support for native refresh/logout bodies per docs/implementation_001_auth/adr_002_mobile_refresh_transport.md
 */

import { apiBaseUrl, apiUrl } from '../config/apiBaseUrl';

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

export const NATIVE_CLIENT_HEADER = 'X-Auth-Client';
export const NATIVE_CLIENT_VALUE = 'native';

export function nativeClientHeaders(): Record<string, string> {
	return { [NATIVE_CLIENT_HEADER]: NATIVE_CLIENT_VALUE };
}

export type ApiRequestOptions = Omit<RequestInit, 'body'> & {
	jsonBody?: unknown;
	accessToken?: string | null;
	/** Set on login / refresh / logout for ADR 002 client identification. */
	nativeClient?: boolean;
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
 * JSON fetch. Does not log tokens or passwords.
 * Uses `credentials: 'omit'` (no cookie jar).
 */
export async function apiRequest<T>(path: string, options: ApiRequestOptions = {}): Promise<T> {
	if (!apiBaseUrl) {
		throw new ApiError(0, 'config', 'EXPO_PUBLIC_API_URL is not set');
	}

	const {
		jsonBody,
		accessToken,
		nativeClient = false,
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
	if (nativeClient) {
		const nh = nativeClientHeaders();
		for (const [k, v] of Object.entries(nh)) {
			headers.set(k, v);
		}
	}

	const url = apiUrl(path);
	const body = jsonBody !== undefined ? JSON.stringify(jsonBody) : undefined;

	const res = await fetch(url, {
		...rest,
		method,
		headers,
		credentials: 'omit',
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
