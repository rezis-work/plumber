import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { StyleSheet, Text, View } from 'react-native';
import { profileHrefForRole, useAuth } from '../../src/auth';
import { PrimaryButton, Screen } from '../../src/components/ui';
import { colors, space } from '../../src/theme';

export default function ForbiddenScreen() {
	const router = useRouter();
	const { user } = useAuth();

	return (
		<Screen scroll>
			<StatusBar style="dark" />
			<Text style={styles.title}>Access denied</Text>
			<Text style={styles.body}>
				You do not have permission to open this screen with your current account.
			</Text>
			<View style={styles.actions}>
				{user ? (
					<PrimaryButton
						label="Go to my profile"
						onPress={() => router.replace(profileHrefForRole(user.role))}
					/>
				) : (
					<PrimaryButton label="Back to home" onPress={() => router.replace('/')} />
				)}
			</View>
		</Screen>
	);
}

const styles = StyleSheet.create({
	title: {
		fontSize: 22,
		fontWeight: '700',
		color: colors.text,
		marginBottom: space[3]
	},
	body: {
		fontSize: 16,
		lineHeight: 24,
		color: colors.textMuted,
		marginBottom: space[8]
	},
	actions: { gap: space[3] }
});
