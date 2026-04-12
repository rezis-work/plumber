import { registerFormErrorMessage } from '../../../src/api';
import type { RegisterClientResponse } from '../../../src/api/types';
import { validateEmailInput, validatePasswordInput } from '../../../src/auth';
import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useState } from 'react';
import { Alert, Pressable, StyleSheet, Text, View } from 'react-native';
import { LabeledField, PrimaryButton, Screen } from '../../../src/components/ui';
import { useRegisterClientMutation } from '../../../src/query';
import { colors, radius, space } from '../../../src/theme';

const URL_TOKEN_MAX_LEN = 128;

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
			<View style={styles.topBar}>
				<View style={styles.brandRow} accessibilityRole="header">
					<Text style={styles.brandDrop}>💧</Text>
					<Text style={styles.brandName}>Fixavon</Text>
				</View>
				<Pressable
					onPress={() => router.replace('/')}
					style={styles.closeHit}
					accessibilityRole="button"
					accessibilityLabel="Close"
				>
					<Text style={styles.closeGlyph}>✕</Text>
				</Pressable>
			</View>

			<Text style={styles.title}>Create Client Account</Text>
			<Text style={styles.subtitle}>
				{'Join Tbilisi\u2019s premier plumbing network. Get expert help in minutes.'}
			</Text>
			{clientError ? (
				<Text style={styles.error} accessibilityRole="alert">
					{clientError}
				</Text>
			) : null}

			<View style={styles.formBlock}>
				<LabeledField
					label="Email Address"
					labelTone="overline"
					inputVariant="filled"
					value={email}
					onChangeText={setEmail}
					keyboardType="email-address"
					placeholder="name@example.com"
					editable={!pending}
				/>
				<LabeledField
					label="Password"
					labelTone="overline"
					inputVariant="filled"
					value={password}
					onChangeText={setPassword}
					secureTextEntry
					placeholder="Min. 8 characters"
					editable={!pending}
				/>
				<LabeledField
					label="Confirm Password"
					labelTone="overline"
					inputVariant="filled"
					value={confirmPassword}
					onChangeText={setConfirmPassword}
					secureTextEntry
					placeholder="Repeat your password"
					editable={!pending}
				/>
				<View style={styles.submitBlock}>
					<PrimaryButton
						label={pending ? 'Creating account…' : 'Start Service →'}
						onPress={onSubmit}
						disabled={pending}
					/>
				</View>
				<Text style={styles.terms}>
					By creating an account, you agree to our{' '}
					<Text style={styles.termsLink}>Terms of Service</Text>
					{' and '}
					<Text style={styles.termsLink}>Privacy Policy</Text>.
				</Text>
			</View>

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
				<Pressable onPress={() => router.push('/login')} accessibilityRole="link">
					<Text style={styles.signInLink}>Sign In</Text>
				</Pressable>
			</View>

			<View style={styles.footer}>
				<Text style={styles.footerBrand}>Fixavon Tbilisi</Text>
				<View style={styles.footerLinks}>
					<Text style={styles.footerLink}>Services</Text>
					<Text style={styles.footerLink}>Emergency</Text>
					<Text style={styles.footerLink}>Terms</Text>
					<Text style={styles.footerLink}>Privacy</Text>
				</View>
				<Text style={styles.footerCopy}>
					© 2024 FIXAVON TBILISI. PROFESSIONAL PLUMBING.
				</Text>
			</View>
		</Screen>
	);
}

const styles = StyleSheet.create({
	topBar: {
		flexDirection: 'row',
		alignItems: 'center',
		justifyContent: 'space-between',
		marginBottom: space[6],
		paddingTop: space[2]
	},
	brandRow: { flexDirection: 'row', alignItems: 'center', gap: space[2] },
	brandDrop: { fontSize: 22 },
	brandName: {
		fontSize: 20,
		fontWeight: '800',
		color: colors.primary,
		letterSpacing: -0.5
	},
	closeHit: { padding: space[2] },
	closeGlyph: {
		fontSize: 18,
		fontWeight: '600',
		color: colors.primary
	},
	title: {
		fontSize: 40,
		fontWeight: '800',
		lineHeight: 44,
		letterSpacing: -0.5,
		color: colors.text,
		marginBottom: space[3]
	},
	subtitle: {
		fontSize: 16,
		lineHeight: 24,
		color: colors.textMuted,
		marginBottom: space[8]
	},
	error: {
		color: colors.error,
		fontSize: 15,
		marginBottom: space[4]
	},
	formBlock: { gap: 0 },
	submitBlock: { paddingTop: space[4] },
	terms: {
		fontSize: 14,
		lineHeight: 22,
		color: colors.textMuted,
		textAlign: 'center',
		marginTop: space[4],
		paddingHorizontal: space[2]
	},
	termsLink: { color: colors.primary, fontWeight: '600' },
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
	signInLink: { fontSize: 14, fontWeight: '700', color: colors.primary },
	footer: {
		alignItems: 'center',
		marginTop: space[8],
		paddingVertical: space[8],
		paddingHorizontal: space[4],
		backgroundColor: colors.surfaceContainerLow,
		borderTopLeftRadius: radius.xl,
		borderTopRightRadius: radius.xl,
		gap: space[4]
	},
	footerBrand: { fontSize: 18, fontWeight: '700', color: colors.primary },
	footerLinks: { flexDirection: 'row', flexWrap: 'wrap', justifyContent: 'center', gap: space[6] },
	footerLink: { fontSize: 14, color: colors.textMuted },
	footerCopy: {
		fontSize: 11,
		fontWeight: '600',
		color: colors.textMuted,
		textAlign: 'center',
		letterSpacing: 1.2
	}
});
