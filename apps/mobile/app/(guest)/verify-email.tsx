import { verifyEmailErrorMessage } from '../../src/api';
import { useLocalSearchParams, useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useEffect, useRef, useState } from 'react';
import { Alert, Image, Pressable, StyleSheet, Text, View } from 'react-native';
import { LabeledField, PrimaryButton, Screen } from '../../src/components/ui';
import { useVerifyEmailMutation } from '../../src/query';
import { colors, radius, space } from '../../src/theme';

const URL_TOKEN_MAX_LEN = 128;

const illustration = require('../../assets/stitch/verify-email-mobile/illustration.png');

/** Display-only mask aligned with Stitch reference (e.g. `re***@gmail.com`). */
function maskEmail(email: string): string {
	const trimmed = email.trim();
	const at = trimmed.indexOf('@');
	if (at <= 0 || at === trimmed.length - 1) {
		return trimmed;
	}
	const local = trimmed.slice(0, at);
	const domain = trimmed.slice(at + 1);
	if (local.length <= 1) {
		return `${local}***@${domain}`;
	}
	return `${local[0]}***@${domain}`;
}

export default function VerifyEmailScreen() {
	const router = useRouter();
	const { token: tokenParam, email: emailParam } = useLocalSearchParams<{
		token?: string;
		email?: string;
	}>();
	const { mutate: verifyMutate, isPending: verifyPending } = useVerifyEmailMutation();
	const [code, setCode] = useState('');
	const [clientError, setClientError] = useState<string | null>(null);
	const autoSubmittedRef = useRef(false);

	const pending = verifyPending;

	const registeredEmail =
		typeof emailParam === 'string' && emailParam.trim().length > 0 ? emailParam.trim() : null;

	useEffect(() => {
		const raw = typeof tokenParam === 'string' ? tokenParam.trim() : '';
		if (raw.length > 0 && raw.length <= URL_TOKEN_MAX_LEN) {
			setCode(raw);
		}
	}, [tokenParam]);

	useEffect(() => {
		const raw = typeof tokenParam === 'string' ? tokenParam.trim() : '';
		if (!raw || raw.length > URL_TOKEN_MAX_LEN || autoSubmittedRef.current) {
			return;
		}
		autoSubmittedRef.current = true;
		verifyMutate(
			{ token: raw },
			{
				onSuccess: () => {
					router.replace('/login?verified=1');
				},
				onError: (err) => {
					autoSubmittedRef.current = false;
					setClientError(verifyEmailErrorMessage(err));
				}
			}
		);
	}, [tokenParam, router, verifyMutate]);

	const onSubmit = () => {
		setClientError(null);
		const t = code.trim();
		if (!t) {
			setClientError('Enter the verification code from your email.');
			return;
		}
		verifyMutate(
			{ token: t },
			{
				onSuccess: () => {
					router.replace('/login?verified=1');
				},
				onError: (err) => {
					setClientError(verifyEmailErrorMessage(err));
				}
			}
		);
	};

	const onResend = () => {
		Alert.alert('Resend email', 'Resend verification is not available yet.');
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
					accessibilityLabel="Menu"
				>
					<Text style={styles.menuGlyph}>☰</Text>
				</Pressable>
			</View>

			<View style={styles.heroWrap}>
				<View style={styles.heroGlow} />
				<View style={styles.heroCircle}>
					<Text style={styles.heroIcon} accessibilityLabel="Email">
						✉️
					</Text>
				</View>
			</View>

			<Text style={styles.title}>Verify Your Email</Text>
			<Text style={styles.subtitle}>
				{
					"We\u2019ve sent a verification link to your inbox. Please click the link to confirm your account."
				}
			</Text>
			<Text style={styles.subtitleSecondary}>
				You can also paste the verification code from your email below.
			</Text>

			{registeredEmail ? (
				<View style={styles.emailCard}>
					<View style={styles.emailIconBox}>
						<Text style={styles.emailIconGlyph}>@</Text>
					</View>
					<View style={styles.emailCardText}>
						<Text style={styles.emailCardLabel}>Registered Email</Text>
						<Text style={styles.emailCardValue}>{maskEmail(registeredEmail)}</Text>
					</View>
				</View>
			) : null}

			{clientError ? (
				<Text style={styles.error} accessibilityRole="alert">
					{clientError}
				</Text>
			) : null}

			<LabeledField
				label="Verification code"
				labelTone="overline"
				inputVariant="filled"
				value={code}
				onChangeText={setCode}
				placeholder="Paste your verification code"
				editable={!pending}
			/>

			<View style={styles.actions}>
				<PrimaryButton
					label={pending ? 'Verifying…' : 'Verify email'}
					onPress={onSubmit}
					disabled={pending}
				/>
				<PrimaryButton
					variant="outline"
					label="Resend Verification Email →"
					onPress={onResend}
					disabled={pending}
				/>
				<PrimaryButton
					variant="outline"
					label="← Back to Login"
					onPress={() => router.replace('/login')}
					disabled={pending}
				/>
			</View>

			<View style={styles.trustPill}>
				<View style={styles.trustDot} />
				<Text style={styles.trustPillText}>3 plumbers active in Tbilisi right now</Text>
			</View>

			<View style={styles.promoCard}>
				<Image
					source={illustration}
					style={styles.promoImage}
					resizeMode="cover"
					accessibilityLabel="Professional plumbing"
				/>
				<View style={styles.promoOverlay}>
					<Text style={styles.promoTitle}>Need immediate help?</Text>
					<Text style={styles.promoSub}>
						Our emergency services operate 24/7 in Tbilisi.
					</Text>
				</View>
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
	menuGlyph: {
		fontSize: 20,
		fontWeight: '600',
		color: colors.textMuted
	},
	heroWrap: {
		alignItems: 'center',
		marginBottom: space[8],
		marginTop: space[2]
	},
	heroGlow: {
		position: 'absolute',
		width: 120,
		height: 120,
		borderRadius: 60,
		backgroundColor: colors.secondaryContainer,
		opacity: 0.25
	},
	heroCircle: {
		width: 96,
		height: 96,
		borderRadius: 48,
		backgroundColor: colors.surfaceElevated,
		alignItems: 'center',
		justifyContent: 'center',
		shadowColor: colors.text,
		shadowOffset: { width: 0, height: 8 },
		shadowOpacity: 0.06,
		shadowRadius: 20,
		elevation: 2
	},
	heroIcon: { fontSize: 40 },
	title: {
		fontSize: 30,
		fontWeight: '800',
		letterSpacing: -0.5,
		color: colors.text,
		marginBottom: space[3],
		textAlign: 'center'
	},
	subtitle: {
		fontSize: 16,
		lineHeight: 24,
		color: colors.textMuted,
		marginBottom: space[3],
		textAlign: 'center'
	},
	subtitleSecondary: {
		fontSize: 14,
		lineHeight: 22,
		color: colors.textMuted,
		marginBottom: space[6],
		textAlign: 'center',
		paddingHorizontal: space[2]
	},
	emailCard: {
		flexDirection: 'row',
		alignItems: 'center',
		gap: space[4],
		backgroundColor: colors.surfaceContainerLow,
		borderRadius: radius.lg,
		padding: 20,
		marginBottom: space[6],
		width: '100%'
	},
	emailIconBox: {
		backgroundColor: colors.surfaceContainerHigh,
		padding: space[3],
		borderRadius: radius.md
	},
	emailIconGlyph: {
		fontSize: 22,
		fontWeight: '700',
		color: colors.primary
	},
	emailCardText: { flex: 1 },
	emailCardLabel: {
		fontSize: 12,
		fontWeight: '600',
		color: colors.textMuted,
		marginBottom: 4
	},
	emailCardValue: { fontSize: 16, fontWeight: '600', color: colors.text },
	error: {
		color: colors.error,
		fontSize: 15,
		marginBottom: space[4],
		textAlign: 'center'
	},
	actions: { gap: space[4], marginTop: space[2] },
	trustPill: {
		flexDirection: 'row',
		alignItems: 'center',
		alignSelf: 'center',
		gap: space[2],
		marginTop: space[8],
		paddingVertical: space[2],
		paddingHorizontal: space[4],
		borderRadius: 9999,
		backgroundColor: colors.surfaceContainerLow
	},
	trustDot: {
		width: 8,
		height: 8,
		borderRadius: 4,
		backgroundColor: colors.tertiary
	},
	trustPillText: {
		fontSize: 14,
		fontWeight: '600',
		color: colors.tertiary
	},
	promoCard: {
		marginTop: space[8],
		height: 192,
		borderRadius: radius.lg,
		overflow: 'hidden',
		backgroundColor: colors.surfaceContainerHigh
	},
	promoImage: { ...StyleSheet.absoluteFillObject },
	promoOverlay: {
		...StyleSheet.absoluteFillObject,
		justifyContent: 'flex-end',
		padding: space[6],
		backgroundColor: 'rgba(11,28,48,0.55)'
	},
	promoTitle: {
		fontSize: 18,
		fontWeight: '700',
		color: '#ffffff',
		marginBottom: space[2]
	},
	promoSub: { fontSize: 14, color: 'rgba(255,255,255,0.85)' },
	footer: {
		alignItems: 'center',
		marginTop: space[8],
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
