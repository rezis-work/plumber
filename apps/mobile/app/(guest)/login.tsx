import { loginErrorMessage } from '../../src/api';
import { validateEmailInput, validatePasswordInput } from '../../src/auth';
import { useLocalSearchParams, useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useState } from 'react';
import { Image, StyleSheet, Text, View } from 'react-native';
import { LabeledField, PrimaryButton, Screen, TextLink } from '../../src/components/ui';
import { useLoginMutation } from '../../src/query';
import { colors, radius, space } from '../../src/theme';

const illustration = require('../../assets/stitch/login-mobile/illustration.png');

export default function LoginScreen() {
	const router = useRouter();
	const { verified } = useLocalSearchParams<{ verified?: string | string[] }>();
	const verifiedRaw = Array.isArray(verified) ? verified[0] : verified;
	const verifiedBanner = verifiedRaw === '1' || verifiedRaw === 'true';
	const login = useLoginMutation();
	const [email, setEmail] = useState('');
	const [password, setPassword] = useState('');
	const [clientError, setClientError] = useState<string | null>(null);

	const pending = login.isPending;

	const onSubmit = () => {
		setClientError(null);

		const emailResult = validateEmailInput(email);
		if (!emailResult.ok) {
			setClientError(emailResult.message);
			return;
		}

		const pwErr = validatePasswordInput(password);
		if (pwErr) {
			setClientError(pwErr);
			return;
		}

		login.mutate(
			{ email: emailResult.email, password },
			{
				onSuccess: () => {
					router.replace('/home');
				},
				onError: (err) => {
					setClientError(loginErrorMessage(err));
				}
			}
		);
	};

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
			<Text style={styles.title}>Welcome Back</Text>
			<Text style={styles.subtitle}>
				Securely access your Tbilisi plumbing services dashboard.
			</Text>
			{verifiedBanner ? (
				<Text
					style={styles.verifiedBanner}
					accessibilityRole="text"
					accessibilityLiveRegion="polite"
				>
					Email verified — you can log in.
				</Text>
			) : null}
			{clientError ? (
				<Text style={styles.error} accessibilityRole="alert">
					{clientError}
				</Text>
			) : null}
			<View style={styles.card}>
				<LabeledField
					label="Email Address"
					value={email}
					onChangeText={setEmail}
					keyboardType="email-address"
					placeholder="name@example.com"
					editable={!pending}
				/>
				<LabeledField
					label="Password"
					labelRight={
						<TextLink label="Forgot Password?" onPress={() => router.push('/verify-email')} />
					}
					value={password}
					onChangeText={setPassword}
					secureTextEntry
					placeholder="••••••••"
					editable={!pending}
				/>
				<PrimaryButton
					label={pending ? 'Signing in…' : 'Log In to Account'}
					onPress={onSubmit}
					disabled={pending}
				/>
			</View>
			<Text style={styles.newUserHeading}>New to Fixavon?</Text>
			<View style={styles.registerLinks}>
				<TextLink
					label="Register as Client — I need a plumbing expert"
					onPress={() => router.push('/register')}
				/>
				<TextLink
					label="Register as Plumber — I want to provide services"
					onPress={() => router.push('/register/plumber')}
				/>
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
		borderRadius: radius.lg
	},
	title: {
		fontSize: 34,
		fontWeight: '800',
		letterSpacing: -0.5,
		color: colors.text,
		marginBottom: space[2],
		textAlign: 'center'
	},
	subtitle: {
		fontSize: 16,
		lineHeight: 24,
		color: colors.textMuted,
		marginBottom: space[6],
		textAlign: 'center'
	},
	verifiedBanner: {
		fontSize: 15,
		fontWeight: '600',
		color: colors.tertiary,
		backgroundColor: colors.surfaceContainerLow,
		paddingVertical: space[3],
		paddingHorizontal: space[4],
		borderRadius: radius.md,
		marginBottom: space[4],
		overflow: 'hidden'
	},
	error: {
		color: colors.error,
		fontSize: 15,
		marginBottom: space[4]
	},
	card: {
		backgroundColor: colors.surfaceElevated,
		borderRadius: radius.lg,
		padding: space[8],
		shadowColor: colors.text,
		shadowOffset: { width: 0, height: 8 },
		shadowOpacity: 0.06,
		shadowRadius: 20,
		elevation: 2,
		gap: 0
	},
	newUserHeading: {
		marginTop: space[8],
		marginBottom: space[4],
		fontSize: 13,
		fontWeight: '700',
		color: colors.textMuted,
		textAlign: 'center',
		letterSpacing: 1.2,
		textTransform: 'uppercase'
	},
	registerLinks: { gap: space[3], alignItems: 'center' }
});
