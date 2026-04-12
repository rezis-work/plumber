import { Redirect, Stack } from 'expo-router';
import { useAuth } from '../../../src/auth';

export default function PlumberGroupLayout() {
	const { user } = useAuth();
	if (user && user.role !== 'plumber') {
		return <Redirect href="/forbidden" />;
	}
	return <Stack screenOptions={{ headerShown: false }} />;
}
