import { Redirect, Stack } from 'expo-router';
import { useAuth } from '../../../src/auth';

export default function AdminGroupLayout() {
	const { user } = useAuth();
	if (user && user.role !== 'admin') {
		return <Redirect href="/forbidden" />;
	}
	return <Stack screenOptions={{ headerShown: false }} />;
}
