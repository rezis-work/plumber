import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useState } from 'react';
import { Image, StyleSheet, Text } from 'react-native';
import { LabeledField, PrimaryButton, Screen, TextLink } from '../../../src/components/ui';
import { colors, radius, space } from '../../../src/theme';

const illustration = require('../../../assets/stitch/register-client-mobile/illustration.png');

export default function RegisterClientScreen() {
	const router = useRouter();
	const [email, setEmail] = useState('');
	const [password, setPassword] = useState('');

	return (
		<Screen scroll>
			<StatusBar style="dark" />
			<TextLink label="Back" onPress={() => router.back()} />
			<Image
				source={illustration}
				style={styles.illustration}
				resizeMode="contain"
				accessibilityLabel="Registration illustration"
			/>
			<Text style={styles.title}>Create your account</Text>
			<Text style={styles.subtitle}>Book jobs and message plumbers with confidence.</Text>
			<LabeledField
				label="Email"
				value={email}
				onChangeText={setEmail}
				keyboardType="email-address"
				placeholder="you@example.com"
			/>
			<LabeledField
				label="Password"
				value={password}
				onChangeText={setPassword}
				secureTextEntry
				placeholder="At least 8 characters"
			/>
			<PrimaryButton
				label="Create account"
				onPress={() => router.push('/verify-email')}
			/>
			<TextLink label="Already have an account? Log in" onPress={() => router.push('/login')} />
		</Screen>
	);
}

const styles = StyleSheet.create({
	illustration: {
		width: '100%',
		height: 140,
		marginVertical: space[4],
		backgroundColor: colors.surfaceContainerLow,
		borderRadius: radius.md
	},
	title: {
		fontSize: 24,
		fontWeight: '700',
		color: colors.text,
		marginBottom: space[2]
	},
	subtitle: {
		fontSize: 15,
		color: colors.textMuted,
		marginBottom: space[6]
	}
});
