import { QueryClientProvider } from '@tanstack/react-query';
import { Stack } from 'expo-router';
import { useCallback, useState } from 'react';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { AuthProvider, SessionGate } from '../src/auth';
import { createQueryClient } from '../src/query/queryClient';

export default function RootLayout() {
	const [queryClient] = useState(() => createQueryClient());
	const onSessionCleared = useCallback(() => {
		queryClient.removeQueries({ queryKey: ['auth'] });
	}, [queryClient]);

	return (
		<QueryClientProvider client={queryClient}>
			{/* Phase M5: gate stacks on accessToken + useAuthMe; keep QueryClient outside Auth so onSessionCleared can clear ['auth'] queries. */}
			<AuthProvider onSessionCleared={onSessionCleared}>
				<SafeAreaProvider>
					<SessionGate>
						<Stack screenOptions={{ headerShown: false }} />
					</SessionGate>
				</SafeAreaProvider>
			</AuthProvider>
		</QueryClientProvider>
	);
}
