import * as SecureStore from 'expo-secure-store';

/** Namespaced key — never log values read or written under this key. */
export const REFRESH_TOKEN_SECURE_KEY = 'plumber.auth.refresh_token';

export async function getRefreshToken(): Promise<string | null> {
	try {
		const value = await SecureStore.getItemAsync(REFRESH_TOKEN_SECURE_KEY);
		return value ?? null;
	} catch {
		return null;
	}
}

export async function setRefreshToken(value: string): Promise<void> {
	await SecureStore.setItemAsync(REFRESH_TOKEN_SECURE_KEY, value);
}

export async function deleteRefreshToken(): Promise<void> {
	try {
		await SecureStore.deleteItemAsync(REFRESH_TOKEN_SECURE_KEY);
	} catch {
		/* idempotent */
	}
}
