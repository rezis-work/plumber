import { useQuery } from '@tanstack/react-query';
import { useEffect } from 'react';
import { apiRequestAuthenticated } from '../api';
import type { MeResponse } from '../api/types';
import { useSessionBootstrap } from '../auth/SessionGate';
import { useAuth } from '../auth';
import { authMeQueryKey } from './authKeys';

export { authMeQueryKey } from './authKeys';

/**
 * `GET /auth/me` with 401 refresh via `apiRequestAuthenticated`.
 * Syncs `AuthContext.user` when data updates (aligns with M2 + M3 refresh path).
 */
export function useAuthMe() {
	const { accessToken, setUser, isHydrated } = useAuth();
	const { isBootstrapped } = useSessionBootstrap();

	const query = useQuery({
		queryKey: authMeQueryKey,
		enabled: isHydrated && isBootstrapped && !!accessToken,
		staleTime: 45_000,
		queryFn: () => apiRequestAuthenticated<MeResponse>('/auth/me', { method: 'GET' })
	});

	useEffect(() => {
		if (query.data) {
			setUser(query.data);
		}
	}, [query.data, setUser]);

	return query;
}
