import { verifyEmailErrorMessage } from '../../src/api';
import { useLocalSearchParams, useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useEffect, useState } from 'react';
import { Image, StyleSheet, Text, View } from 'react-native';
import { LabeledField, PrimaryButton, Screen, TextLink } from '../../src/components/ui';
import { useVerifyEmailMutation } from '../../src/query';
import { colors, radius, space } from '../../src/theme';

const URL_TOKEN_MAX_LEN = 128;

const illustration = require('../../assets/stitch/verify-email-mobile/illustration.png');

export default function VerifyEmailScreen() {
	const router = useRouter();
	const { token: tokenParam, email: emailParam } = useLocalSearchParams<{
		token?: string;
		email?: string;
	}>();
	const verify = useVerifyEmailMutation();
	const [code, setCode] = useState('');
	const [clientError, setClientError] = useState<string | null>(null);

	const pending = verify.isPending;

	const registeredEmail =
		typeof emailParam === 'string' && emailParam.trim().length > 0 ? emailParam.trim() : null;

	useEffect(() => {
		const raw = typeof tokenParam === 'string' ? tokenParam.trim() : '';
		if (raw.length > 0 && raw.length <= URL_TOKEN_MAX_LEN) {
			setCode(raw);
		}
	}, [tokenParam]);

	const onSubmit = () => {
		setClientError(null);
		const t = code.trim();
		if (!t) {
			setClientError('Enter the verification code from your email.');
			return;
		}
		verify.mutate(
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

	return (
		<Screen scroll>
			<StatusBar style="dark" />
			<TextLink label="Back" onPress={() => router.back()} />
			<View style={styles.heroIconWrap}>
				<Image
					source={illustration}
					style={styles.illustration}
					resizeMode="contain"
					accessibilityLabel="Email verification illustration"
				/>
			</View>
			<Text style={styles.title}>Verify Your Email</Text>
			<Text style={styles.subtitle}>
				{
					"We\u2019ve sent a verification code to your inbox. Enter it below to confirm your account."
				}
			</Text>
			{registeredEmail ? (
				<View style={styles.emailCard}>
					<View style={styles.emailIconBox}>
						<Text style={styles.emailIconGlyph} accessibilityLabel="Email">
							@
						</Text>
					</View>
					<View style={styles.emailCardText}>
						<Text style={styles.emailCardLabel}>Registered Email</Text>
						<Text style={styles.emailCardValue}>{registeredEmail}</Text>
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
				value={code}
				onChangeText={setCode}
				placeholder="Paste your verification code"
				editable={!pending}
			/>
			<Text style={styles.fieldHint}>
				When email delivery is enabled, use the code from your message. Check your spam folder if needed.
			</Text>
			<PrimaryButton
				label={pending ? 'Verifying…' : 'Verify email'}
				onPress={onSubmit}
				disabled={pending}
			/>
			<Text style={styles.resendHint}>Resend verification email (coming soon)</Text>
			<TextLink label="Back to log in" onPress={() => router.replace('/login')} />
			<View style={styles.trustPill}>
				<View style={styles.trustDot} />
				<Text style={styles.trustPillText}>3 plumbers active in Tbilisi right now</Text>
			</View>
		</Screen>
	);
}

const styles = StyleSheet.create({
	heroIconWrap: {
		alignItems: 'center',
		marginBottom: space[6]
	},
	illustration: {
		width: '100%',
		height: 120,
		backgroundColor: colors.surfaceContainerLow,
		borderRadius: radius.lg
	},
	title: {
		fontSize: 28,
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
		marginBottom: space[6],
		textAlign: 'center'
	},
	emailCard: {
		flexDirection: 'row',
		alignItems: 'center',
		gap: space[4],
		backgroundColor: colors.surfaceContainerLow,
		borderRadius: radius.lg,
		padding: 20,
		marginBottom: space[6]
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
		marginBottom: space[4]
	},
	fieldHint: {
		fontSize: 13,
		lineHeight: 20,
		color: colors.textMuted,
		marginTop: -space[2],
		marginBottom: space[4]
	},
	resendHint: {
		fontSize: 14,
		color: colors.textMuted,
		textAlign: 'center',
		marginTop: space[4],
		marginBottom: space[2]
	},
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
	}
});
