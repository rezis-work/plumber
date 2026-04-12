import { Redirect, Stack } from 'expo-router';
import { useAuth } from '../../../src/auth';

export default function ClientGroupLayout() {
	const { user } = useAuth();
	if (user && user.role !== 'client') {
		return <Redirect href="/forbidden" />;
	}
	return <Stack screenOptions={{ headerShown: false }} />;
}
