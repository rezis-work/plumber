import { useQueryClient } from '@tanstack/react-query';
import {
	createContext,
	useContext,
	useEffect,
	useMemo,
	useRef,
	useState,
	type ReactNode
} from 'react';
import { ActivityIndicator, StyleSheet, View } from 'react-native';
import { apiRequest } from '../api/client';
import { refreshWithStoredToken } from '../api/refreshNative';
import type { MeResponse } from '../api/types';
import { authMeQueryKey } from '../query/authKeys';
import { useAuth } from './AuthContext';

type SessionBootstrapValue = {
	isBootstrapped: boolean;
};

const SessionBootstrapContext = createContext<SessionBootstrapValue>({ isBootstrapped: false });

export function useSessionBootstrap(): SessionBootstrapValue {
	return useContext(SessionBootstrapContext);
}

/**
 * After SecureStore hydration, attempts silent refresh when a refresh token exists and no access token.
 * Blocks the router until bootstrap finishes so guest/app redirects see a consistent session.
 */
export function SessionGate({ children }: { children: ReactNode }) {
	const queryClient = useQueryClient();
	const {
		isHydrated,
		hasRefreshTokenStored,
		accessToken,
		setAccessSession,
		setHasRefreshTokenStored,
		clearSession
	} = useAuth();
	const [isBootstrapped, setIsBootstrapped] = useState(false);
	const accessRef = useRef(accessToken);
	const refreshStoredRef = useRef(hasRefreshTokenStored);
	accessRef.current = accessToken;
	refreshStoredRef.current = hasRefreshTokenStored;

	useEffect(() => {
		if (!isHydrated) return;
		let cancelled = false;
		(async () => {
			try {
				if (!accessRef.current && refreshStoredRef.current) {
					const login = await refreshWithStoredToken();
					setAccessSession(login.access_token, login.expires_in);
					setHasRefreshTokenStored(true);
					try {
						await queryClient.prefetchQuery({
							queryKey: authMeQueryKey,
							queryFn: () =>
								apiRequest<MeResponse>('/auth/me', {
									method: 'GET',
									accessToken: login.access_token
								}),
							staleTime: 45_000
						});
					} catch {
						await queryClient.invalidateQueries({ queryKey: authMeQueryKey });
					}
				}
			} catch {
				await clearSession();
			} finally {
				if (!cancelled) setIsBootstrapped(true);
			}
		})();
		return () => {
			cancelled = true;
		};
	}, [isHydrated, clearSession, queryClient, setAccessSession, setHasRefreshTokenStored]);

	const ctx = useMemo(() => ({ isBootstrapped }), [isBootstrapped]);

	if (!isHydrated || !isBootstrapped) {
		return (
			<View style={styles.bootWrap} accessibilityLabel="Loading session">
				<ActivityIndicator size="large" />
			</View>
		);
	}

	return (
		<SessionBootstrapContext.Provider value={ctx}>{children}</SessionBootstrapContext.Provider>
	);
}

const styles = StyleSheet.create({
	bootWrap: {
		flex: 1,
		alignItems: 'center',
		justifyContent: 'center'
	}
});
