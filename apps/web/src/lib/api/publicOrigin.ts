import { env } from '$env/dynamic/public';

/**
 * Trailing-slash-stripped API origin, or empty string when unset.
 * Empty means use relative paths (`/auth/...`) with the Vite dev proxy or a same-host deployment.
 */
export function publicApiOrigin(): string {
	const raw = env.PUBLIC_API_URL?.trim() ?? '';
	if (!raw) return '';
	return raw.replace(/\/+$/, '');
}

/**
 * Absolute URL when `PUBLIC_API_URL` is set; otherwise same-origin `path` (must start with `/`).
 */
export function apiUrl(path: string): string {
	const origin = publicApiOrigin();
	const p = path.startsWith('/') ? path : `/${path}`;
	return origin ? `${origin}${p}` : p;
}
