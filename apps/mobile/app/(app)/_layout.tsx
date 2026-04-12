import { Redirect, Stack } from 'expo-router';
import { useAuth } from '../../src/auth';
import { useAuthMe } from '../../src/query';

export default function AppGroupLayout() {
	const { accessToken } = useAuth();
	useAuthMe();

	if (!accessToken) {
		return <Redirect href="/" />;
	}
	return <Stack screenOptions={{ headerShown: false }} />;
}
