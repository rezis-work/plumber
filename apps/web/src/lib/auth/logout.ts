import { browser } from '$app/environment';
import { goto } from '$app/navigation';
/* eslint-disable svelte/no-navigation-without-resolve -- goto uses pathWithLangFromWindow for ?lang= */
import { base } from '$app/paths';
import { authLogout, authLogoutAll } from '$lib/api/client';
import { clearSession, session } from '$lib/auth/session.svelte';
import { pathWithLangFromWindow } from '$lib/i18n/url';

/** C5: revoke current refresh session, clear client state, go to public home. */
export async function logoutFromApp(): Promise<void> {
	if (!browser) {
		throw new Error('logoutFromApp is browser-only');
	}
	try {
		await authLogout();
	} catch {
		/* still clear client state */
	}
	clearSession();
	await goto(`${base}${pathWithLangFromWindow('/')}`);
}

/** C6: revoke all refresh sessions (Bearer + cookie), clear client state, go to login. No-op if no access token. */
export async function logoutEverywhere(): Promise<void> {
	if (!browser) {
		throw new Error('logoutEverywhere is browser-only');
	}
	const token = session.accessToken;
	if (!token) {
		return;
	}
	try {
		await authLogoutAll(token);
	} catch {
		/* still clear client state */
	}
	clearSession();
	await goto(`${base}${pathWithLangFromWindow('/login')}`);
}
