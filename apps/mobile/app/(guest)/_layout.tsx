import { Redirect, Stack } from 'expo-router';
import { ActivityIndicator, StyleSheet, View } from 'react-native';
import { profileHrefForRole, useAuth } from '../../src/auth';
import { useAuthMe } from '../../src/query';

export default function GuestLayout() {
	const { accessToken, user } = useAuth();
	const { isPending, isFetching, data: meData } = useAuthMe();

	if (!accessToken) {
		return <Stack screenOptions={{ headerShown: false }} />;
	}

	const me = user ?? meData ?? null;
	if (!me && (isPending || isFetching)) {
		return (
			<View style={styles.gate} accessibilityLabel="Redirecting to your profile">
				<ActivityIndicator size="large" />
			</View>
		);
	}

	if (me) {
		return <Redirect href={profileHrefForRole(me.role)} />;
	}

	return <Stack screenOptions={{ headerShown: false }} />;
}

const styles = StyleSheet.create({
	gate: {
		flex: 1,
		alignItems: 'center',
		justifyContent: 'center'
	}
});
