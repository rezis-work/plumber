import type { MeResponse } from './types';

/** Wired from AuthProvider — avoids circular imports with AuthContext. */
export type MobileAuthBridge = {
	getAccessToken: () => string | null;
	setAccessSession: (accessToken: string, expiresInSecs: number) => void;
	setUser: (user: MeResponse | null) => void;
	clearSession: () => Promise<void>;
	setHasRefreshTokenStored: (value: boolean) => void;
};

let bridge: MobileAuthBridge | null = null;

export function registerMobileAuthBridge(next: MobileAuthBridge): void {
	bridge = next;
}

export function getMobileAuthBridge(): MobileAuthBridge {
	if (!bridge) {
		throw new Error('Mobile auth bridge not registered (AuthProvider missing?)');
	}
	return bridge;
}
