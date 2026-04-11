import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useState } from 'react';
import { Image, StyleSheet, Text, View } from 'react-native';
import { LabeledField, PrimaryButton, Screen, TextLink } from '../../src/components/ui';
import { colors, radius, space } from '../../src/theme';

const illustration = require('../../assets/stitch/login-mobile/illustration.png');

export default function LoginScreen() {
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
				accessibilityLabel="Login illustration"
			/>
			<Text style={styles.title}>Welcome back</Text>
			<Text style={styles.subtitle}>Log in to manage bookings and messages.</Text>
			<LabeledField
				label="Email"
				value={email}
				onChangeText={setEmail}
				keyboardType="email-address"
			/>
			<LabeledField label="Password" value={password} onChangeText={setPassword} secureTextEntry />
			<PrimaryButton
				label="Log in"
				onPress={() => {
					/* Phase M4: POST /auth/login */
				}}
			/>
			<View style={styles.links}>
				<TextLink label="Create a client account" onPress={() => router.push('/register')} />
				<TextLink label="Forgot password?" onPress={() => router.push('/verify-email')} />
			</View>
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
	},
	links: { marginTop: space[4], gap: space[2] }
});
