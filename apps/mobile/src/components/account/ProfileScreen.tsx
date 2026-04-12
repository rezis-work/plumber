import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { StyleSheet, Text, View } from 'react-native';
import { PrimaryButton, Screen } from '../ui';
import { useAuth } from '../../auth';
import { useAuthMe, useLogoutAllMutation, useLogoutMutation } from '../../query';
import { colors, space } from '../../theme';

export function ProfileScreen() {
	const router = useRouter();
	const { user } = useAuth();
	const { isFetching } = useAuthMe();
	const logout = useLogoutMutation();
	const logoutAll = useLogoutAllMutation();

	const onLogout = async () => {
		await logout.mutateAsync();
		router.replace('/');
	};

	const onLogoutAll = async () => {
		await logoutAll.mutateAsync();
		router.replace('/');
	};

	return (
		<Screen scroll>
			<StatusBar style="dark" />
			<Text style={styles.title}>Profile</Text>
			<Text style={styles.subtitle}>
				{user?.email ?? 'Loading profile…'}
				{isFetching ? ' (updating…)' : ''}
			</Text>
			{user ? (
				<Text style={styles.meta}>
					Role: {user.role}
					{' · '}
					{user.is_email_verified ? 'Email verified' : 'Email not verified'}
				</Text>
			) : null}
			<View style={styles.actions}>
				<PrimaryButton
					label={logout.isPending ? 'Signing out…' : 'Log out'}
					onPress={() => void onLogout()}
					disabled={logout.isPending || logoutAll.isPending}
				/>
				<PrimaryButton
					variant="outline"
					label={logoutAll.isPending ? 'Signing out everywhere…' : 'Log out all devices'}
					onPress={() => void onLogoutAll()}
					disabled={logout.isPending || logoutAll.isPending}
				/>
			</View>
		</Screen>
	);
}

const styles = StyleSheet.create({
	title: {
		fontSize: 24,
		fontWeight: '700',
		color: colors.text,
		marginBottom: space[2]
	},
	subtitle: {
		fontSize: 16,
		color: colors.textMuted,
		marginBottom: space[4]
	},
	meta: { fontSize: 14, color: colors.textMuted, marginBottom: space[6] },
	actions: { gap: space[3] }
});
