import { useQueryClient } from '@tanstack/react-query';
import { loginErrorMessage } from '../../src/api';
import { apiRequest } from '../../src/api/client';
import type { MeResponse } from '../../src/api/types';
import { profileHrefForRole, useAuth, validateEmailInput, validatePasswordInput } from '../../src/auth';
import { useLocalSearchParams, useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useState } from 'react';
import { Pressable, StyleSheet, Text, View } from 'react-native';
import { LabeledField, PrimaryButton, Screen } from '../../src/components/ui';
import { authMeQueryKey } from '../../src/query/authKeys';
import { useLoginMutation } from '../../src/query';
import { colors, radius, space } from '../../src/theme';

export default function LoginScreen() {
	const router = useRouter();
	const queryClient = useQueryClient();
	const { setUser } = useAuth();
	const { verified } = useLocalSearchParams<{ verified?: string | string[] }>();
	const verifiedRaw = Array.isArray(verified) ? verified[0] : verified;
	const verifiedBanner = verifiedRaw === '1' || verifiedRaw === 'true';
	const login = useLoginMutation();
	const [email, setEmail] = useState('');
	const [password, setPassword] = useState('');
	const [showPassword, setShowPassword] = useState(false);
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
				onSuccess: async (data) => {
					const me = await apiRequest<MeResponse>('/auth/me', {
						method: 'GET',
						accessToken: data.access_token
					});
					queryClient.setQueryData(authMeQueryKey, me);
					setUser(me);
					router.replace(profileHrefForRole(me.role));
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
					labelRight={
						<Pressable
							onPress={() => router.push('/verify-email')}
							hitSlop={8}
							accessibilityRole="link"
							accessibilityLabel="Forgot password"
						>
							<Text style={styles.forgotLink}>Forgot Password?</Text>
						</Pressable>
					}
					value={password}
					onChangeText={setPassword}
					secureTextEntry={!showPassword}
					placeholder="••••••••"
					editable={!pending}
					trailingAccessory={
						<Pressable
							onPress={() => setShowPassword((v) => !v)}
							hitSlop={8}
							disabled={pending}
							accessibilityRole="button"
							accessibilityLabel={showPassword ? 'Hide password' : 'Show password'}
						>
							<Text style={styles.visibilityLink}>{showPassword ? 'Hide' : 'Show'}</Text>
						</Pressable>
					}
				/>
				<PrimaryButton
					label={pending ? 'Signing in…' : 'Log In to Account'}
					onPress={onSubmit}
					disabled={pending}
				/>
			</View>

			<Text style={styles.newUserHeading}>New to Fixavon?</Text>
			<View style={styles.registerBento}>
				<Pressable
					style={({ pressed }) => [styles.registerCard, pressed && styles.registerCardPressed]}
					onPress={() => router.push('/register')}
					disabled={pending}
					accessibilityRole="button"
				>
					<View style={styles.registerCardLeft}>
						<View style={styles.registerIconClient}>
							<Text style={styles.registerIconGlyphClient}>◎</Text>
						</View>
						<View>
							<Text style={styles.registerTitle}>Register as Client</Text>
							<Text style={styles.registerSubtitle}>I need a plumbing expert</Text>
						</View>
					</View>
					<Text style={styles.registerChevron}>›</Text>
				</Pressable>
				<Pressable
					style={({ pressed }) => [styles.registerCard, pressed && styles.registerCardPressed]}
					onPress={() => router.push('/register/plumber')}
					disabled={pending}
					accessibilityRole="button"
				>
					<View style={styles.registerCardLeft}>
						<View style={styles.registerIconPlumber}>
							<Text style={styles.registerIconGlyphPlumber}>🔧</Text>
						</View>
						<View>
							<Text style={styles.registerTitle}>Register as Plumber</Text>
							<Text style={styles.registerSubtitle}>I want to provide services</Text>
						</View>
					</View>
					<Text style={styles.registerChevron}>›</Text>
				</Pressable>
			</View>

			<View style={styles.trustPill}>
				<View style={styles.trustDot} />
				<Text style={styles.trustPillText}>Secure, encrypted login powered by Fixavon</Text>
			</View>

			<View style={styles.footer}>
				<Text style={styles.footerBrand}>Fixavon</Text>
				<View style={styles.footerLinks}>
					<Text style={styles.footerLink}>Services</Text>
					<Text style={styles.footerLink}>Emergency</Text>
					<Text style={styles.footerLink}>Terms</Text>
					<Text style={styles.footerLink}>Privacy</Text>
				</View>
				<Text style={styles.footerCopy}>
					© 2024 Fixavon Tbilisi. Professional Plumbing.
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
		fontSize: 36,
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
		marginBottom: space[8],
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
		overflow: 'hidden',
		textAlign: 'center'
	},
	error: {
		color: colors.error,
		fontSize: 15,
		marginBottom: space[4],
		textAlign: 'center'
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
	forgotLink: {
		fontSize: 12,
		fontWeight: '700',
		color: colors.primary
	},
	visibilityLink: {
		fontSize: 12,
		fontWeight: '600',
		color: colors.textMuted
	},
	newUserHeading: {
		marginTop: space[8],
		marginBottom: space[6],
		fontSize: 13,
		fontWeight: '700',
		color: colors.textMuted,
		textAlign: 'center',
		letterSpacing: 2,
		textTransform: 'uppercase'
	},
	registerBento: { gap: space[4] },
	registerCard: {
		flexDirection: 'row',
		alignItems: 'center',
		justifyContent: 'space-between',
		backgroundColor: colors.surfaceContainerLow,
		borderRadius: radius.lg,
		padding: space[6]
	},
	registerCardPressed: { backgroundColor: colors.surfaceContainerHigh },
	registerCardLeft: { flexDirection: 'row', alignItems: 'center', gap: space[4], flex: 1 },
	registerIconClient: {
		width: 48,
		height: 48,
		borderRadius: 24,
		backgroundColor: colors.secondaryContainer,
		alignItems: 'center',
		justifyContent: 'center'
	},
	registerIconPlumber: {
		width: 48,
		height: 48,
		borderRadius: 24,
		backgroundColor: colors.surfaceContainerHigh,
		alignItems: 'center',
		justifyContent: 'center'
	},
	registerIconGlyphClient: { fontSize: 22, color: colors.onSecondaryContainer },
	registerIconGlyphPlumber: { fontSize: 22, color: colors.primary },
	registerTitle: { fontSize: 16, fontWeight: '700', color: colors.text },
	registerSubtitle: { fontSize: 12, color: colors.textMuted, marginTop: 2 },
	registerChevron: {
		fontSize: 22,
		fontWeight: '300',
		color: colors.outlineVariant,
		paddingLeft: space[2]
	},
	trustPill: {
		flexDirection: 'row',
		alignItems: 'center',
		alignSelf: 'center',
		gap: space[2],
		marginTop: space[12],
		marginBottom: space[6],
		paddingVertical: space[2],
		paddingHorizontal: space[4],
		borderRadius: 9999,
		backgroundColor: colors.surfaceContainerLow,
		borderWidth: 1,
		borderColor: colors.outlineVariant
	},
	trustDot: {
		width: 8,
		height: 8,
		borderRadius: 4,
		backgroundColor: colors.tertiary
	},
	trustPillText: { fontSize: 12, fontWeight: '500', color: colors.textMuted },
	footer: {
		alignItems: 'center',
		marginTop: space[4],
		paddingVertical: space[8],
		paddingHorizontal: space[4],
		backgroundColor: colors.surfaceContainerLow,
		borderTopLeftRadius: radius.lg,
		borderTopRightRadius: radius.lg,
		gap: space[4]
	},
	footerBrand: { fontSize: 18, fontWeight: '700', color: colors.primary },
	footerLinks: { flexDirection: 'row', flexWrap: 'wrap', justifyContent: 'center', gap: space[6] },
	footerLink: { fontSize: 14, color: colors.textMuted },
	footerCopy: {
		fontSize: 14,
		color: colors.textMuted,
		textAlign: 'center',
		lineHeight: 22
	}
});
