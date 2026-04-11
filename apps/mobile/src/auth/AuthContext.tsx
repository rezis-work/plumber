import {
	createContext,
	type ReactNode,
	useCallback,
	useContext,
	useEffect,
	useLayoutEffect,
	useMemo,
	useState
} from 'react';
import { registerMobileAuthBridge } from '../api/authBridge';
import type { MeResponse } from '../api/types';
import { deleteRefreshToken, getRefreshToken } from './secureRefreshToken';

type AuthContextValue = {
	accessToken: string | null;
	/** Epoch ms when access token should be treated as expired (from `expires_in`). */
	accessExpiresAtMs: number | null;
	user: MeResponse | null;
	/** True after first SecureStore read for refresh token (cold start). */
	isHydrated: boolean;
	/** Whether a refresh token is stored (no token value exposed here). */
	hasRefreshTokenStored: boolean;
	setAccessSession: (accessToken: string, expiresInSecs: number) => void;
	setUser: (user: MeResponse | null) => void;
	/** Sync flag after login / refresh (M3 / M4). */
	setHasRefreshTokenStored: (value: boolean) => void;
	clearSession: () => Promise<void>;
};

const AuthContext = createContext<AuthContextValue | null>(null);

type AuthProviderProps = {
	children: ReactNode;
	/**
	 * Phase MQ: register e.g. `() => queryClient.removeQueries({ queryKey: ['auth'] })`
	 * so `/auth/me` cannot flash stale UI after logout.
	 */
	onSessionCleared?: () => void;
};

export function AuthProvider({ children, onSessionCleared }: AuthProviderProps) {
	const [accessToken, setAccessToken] = useState<string | null>(null);
	const [accessExpiresAtMs, setAccessExpiresAtMs] = useState<number | null>(null);
	const [user, setUserState] = useState<MeResponse | null>(null);
	const [isHydrated, setIsHydrated] = useState(false);
	const [hasRefreshTokenStored, setHasRefreshTokenStoredState] = useState(false);

	const setHasRefreshTokenStored = useCallback((value: boolean) => {
		setHasRefreshTokenStoredState(value);
	}, []);

	useEffect(() => {
		let cancelled = false;
		(async () => {
			try {
				const t = await getRefreshToken();
				if (!cancelled) {
					setHasRefreshTokenStoredState(!!t);
				}
			} finally {
				if (!cancelled) {
					setIsHydrated(true);
				}
			}
		})();
		return () => {
			cancelled = true;
		};
	}, []);

	const setAccessSession = useCallback((token: string, expiresInSecs: number) => {
		setAccessToken(token);
		setAccessExpiresAtMs(Date.now() + Math.max(0, expiresInSecs) * 1000);
	}, []);

	const setUser = useCallback((next: MeResponse | null) => {
		setUserState(next);
	}, []);

	const clearSession = useCallback(async () => {
		setAccessToken(null);
		setAccessExpiresAtMs(null);
		setUserState(null);
		await deleteRefreshToken();
		setHasRefreshTokenStored(false);
		// Phase MQ: also `queryClient.removeQueries({ queryKey: ['auth'] })` via onSessionCleared.
		onSessionCleared?.();
	}, [onSessionCleared, setHasRefreshTokenStored]);

	useLayoutEffect(() => {
		registerMobileAuthBridge({
			getAccessToken: () => accessToken,
			setAccessSession,
			setUser,
			clearSession,
			setHasRefreshTokenStored
		});
	}, [accessToken, setAccessSession, setUser, clearSession, setHasRefreshTokenStored]);

	const value = useMemo(
		() =>
			({
				accessToken,
				accessExpiresAtMs,
				user,
				isHydrated,
				hasRefreshTokenStored,
				setAccessSession,
				setUser,
				setHasRefreshTokenStored,
				clearSession
			}) satisfies AuthContextValue,
		[
			accessToken,
			accessExpiresAtMs,
			user,
			isHydrated,
			hasRefreshTokenStored,
			setAccessSession,
			setUser,
			setHasRefreshTokenStored,
			clearSession
		]
	);

	return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextValue {
	const ctx = useContext(AuthContext);
	if (!ctx) {
		throw new Error('useAuth must be used within AuthProvider');
	}
	return ctx;
}
