import { registerFormErrorMessage } from '../../../src/api';
import type { RegisterClientResponse } from '../../../src/api/types';
import { validateEmailInput, validatePasswordInput } from '../../../src/auth';
import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useState } from 'react';
import { Alert, Image, StyleSheet, Text, View } from 'react-native';
import { LabeledField, PrimaryButton, Screen, TextLink } from '../../../src/components/ui';
import { useRegisterClientMutation } from '../../../src/query';
import { colors, radius, space } from '../../../src/theme';

const URL_TOKEN_MAX_LEN = 128;

const illustration = require('../../../assets/stitch/register-client-mobile/illustration.png');

function navigateToVerify(
	router: ReturnType<typeof useRouter>,
	data: RegisterClientResponse,
	registeredEmail: string
) {
	const t = data.email_verification_token?.trim() ?? '';
	const params = new URLSearchParams();
	if (t.length > 0 && t.length <= URL_TOKEN_MAX_LEN) {
		params.set('token', t);
	}
	if (registeredEmail) {
		params.set('email', registeredEmail);
	}
	const q = params.toString();
	router.push(q ? `/verify-email?${q}` : '/verify-email');
}

export default function RegisterClientScreen() {
	const router = useRouter();
	const register = useRegisterClientMutation();
	const [email, setEmail] = useState('');
	const [password, setPassword] = useState('');
	const [confirmPassword, setConfirmPassword] = useState('');
	const [clientError, setClientError] = useState<string | null>(null);

	const pending = register.isPending;

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

		if (password !== confirmPassword) {
			setClientError('Passwords do not match.');
			return;
		}

		register.mutate(
			{ email: emailResult.email, password },
			{
				onSuccess: (data) => {
					if (__DEV__) {
						const token = data.email_verification_token?.trim() ?? '';
						const exp = data.email_verification_expires_at;
						const body = token
							? `Verification token (dev only):\n${token}${exp ? `\n\nExpires: ${exp}` : ''}`
							: 'No verification token in API response.';
						Alert.alert('Development build', body, [
							{
								text: 'Continue',
								onPress: () => navigateToVerify(router, data, emailResult.email)
							}
						]);
						return;
					}
					navigateToVerify(router, data, emailResult.email);
				},
				onError: (err) => {
					setClientError(registerFormErrorMessage(err));
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
				accessibilityLabel="Registration illustration"
			/>
			<Text style={styles.title}>Create Client Account</Text>
			<Text style={styles.subtitle}>
				{'Join Tbilisi\u2019s premier plumbing network. Get expert help in minutes.'}
			</Text>
			{clientError ? (
				<Text style={styles.error} accessibilityRole="alert">
					{clientError}
				</Text>
			) : null}
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
				value={password}
				onChangeText={setPassword}
				secureTextEntry
				placeholder="Min. 8 characters"
				editable={!pending}
			/>
			<Text style={styles.fieldHint}>Must be at least 8 characters long.</Text>
			<LabeledField
				label="Confirm Password"
				value={confirmPassword}
				onChangeText={setConfirmPassword}
				secureTextEntry
				placeholder="Repeat your password"
				editable={!pending}
			/>
			<PrimaryButton
				label={pending ? 'Creating account…' : 'Start Service'}
				onPress={onSubmit}
				disabled={pending}
			/>
			<Text style={styles.terms}>
				By creating an account, you agree to our Terms of Service and Privacy Policy.
			</Text>
			<View style={styles.trustCard}>
				<View style={styles.trustIconWrap}>
					<Text style={styles.trustIconGlyph} accessibilityLabel="Verified">
						✓
					</Text>
				</View>
				<View style={styles.trustTextWrap}>
					<Text style={styles.trustTitle}>Verified Expertise</Text>
					<Text style={styles.trustSubtitle}>250+ Master Plumbers active in Tbilisi.</Text>
				</View>
			</View>
			<View style={styles.signInRow}>
				<Text style={styles.signInMuted}>Already have an account?</Text>
				<View style={styles.signInLinkWrap}>
					<TextLink label="Sign In" onPress={() => router.push('/login')} />
				</View>
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
		fontSize: 32,
		fontWeight: '800',
		lineHeight: 38,
		color: colors.text,
		marginBottom: space[3]
	},
	subtitle: {
		fontSize: 16,
		lineHeight: 24,
		color: colors.textMuted,
		marginBottom: space[6]
	},
	error: {
		color: colors.error,
		fontSize: 15,
		marginBottom: space[4]
	},
	fieldHint: {
		fontSize: 13,
		color: colors.textMuted,
		marginTop: -space[2],
		marginBottom: space[4]
	},
	terms: {
		fontSize: 14,
		lineHeight: 22,
		color: colors.textMuted,
		textAlign: 'center',
		marginTop: space[4],
		paddingHorizontal: space[2]
	},
	trustCard: {
		flexDirection: 'row',
		alignItems: 'center',
		gap: space[4],
		marginTop: space[8],
		padding: space[6],
		backgroundColor: colors.surfaceContainerLow,
		borderRadius: radius.lg,
		overflow: 'hidden'
	},
	trustIconWrap: {
		width: 48,
		height: 48,
		borderRadius: 24,
		backgroundColor: colors.secondaryContainer,
		alignItems: 'center',
		justifyContent: 'center'
	},
	trustIconGlyph: {
		fontSize: 22,
		fontWeight: '700',
		color: colors.onSecondaryContainer
	},
	trustTextWrap: { flex: 1 },
	trustTitle: { fontSize: 16, fontWeight: '700', color: colors.text, marginBottom: 4 },
	trustSubtitle: { fontSize: 12, color: colors.textMuted },
	signInRow: {
		flexDirection: 'row',
		flexWrap: 'wrap',
		alignItems: 'center',
		justifyContent: 'center',
		marginTop: space[8],
		gap: 4
	},
	signInMuted: { fontSize: 14, color: colors.textMuted },
	signInLinkWrap: { alignSelf: 'center' }
});
