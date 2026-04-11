/**
 * Public API base URL from `EXPO_PUBLIC_API_URL` (see `.env.example`).
 * Restart Expo after changing `.env`. Phase M3 will use this for `fetch`.
 */

function normalizeBase(url: string): string {
	return url.trim().replace(/\/+$/, '');
}

const raw = process.env.EXPO_PUBLIC_API_URL?.trim() ?? '';

export const apiBaseUrl = normalizeBase(raw);

/** Absolute URL for an API path (must start with `/` or it will be prefixed). */
export function apiUrl(path: string): string {
	const suffix = path.startsWith('/') ? path : `/${path}`;
	if (!apiBaseUrl) {
		return suffix;
	}
	return `${apiBaseUrl}${suffix}`;
}

if (__DEV__ && !apiBaseUrl) {
	console.warn(
		'[config] EXPO_PUBLIC_API_URL is not set. Copy .env.example to .env and set your HTTPS API origin (e.g. ngrok).'
	);
}
