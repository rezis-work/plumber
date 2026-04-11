import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useState } from 'react';
import { Image, StyleSheet, Text } from 'react-native';
import { LabeledField, PrimaryButton, Screen, TextLink } from '../../src/components/ui';
import { colors, radius, space } from '../../src/theme';

const illustration = require('../../assets/stitch/verify-email-mobile/illustration.png');

export default function VerifyEmailScreen() {
	const router = useRouter();
	const [code, setCode] = useState('');

	return (
		<Screen scroll>
			<StatusBar style="dark" />
			<TextLink label="Back" onPress={() => router.back()} />
			<Image
				source={illustration}
				style={styles.illustration}
				resizeMode="contain"
				accessibilityLabel="Email verification illustration"
			/>
			<Text style={styles.title}>Verify your email</Text>
			<Text style={styles.subtitle}>
				Enter the code we sent to your inbox to continue.
			</Text>
			<LabeledField
				label="Verification code"
				value={code}
				onChangeText={setCode}
				keyboardType="numeric"
				placeholder="6-digit code"
			/>
			<PrimaryButton label="Verify and continue" onPress={() => router.replace('/login')} />
			<TextLink label="Wrong email? Go back" onPress={() => router.replace('/register')} />
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
