import { Redirect, Stack } from 'expo-router';
import { useAuth } from '../../src/auth';

export default function AppGroupLayout() {
	const { accessToken } = useAuth();
	if (!accessToken) {
		return <Redirect href="/" />;
	}
	return <Stack screenOptions={{ headerShown: false }} />;
}
