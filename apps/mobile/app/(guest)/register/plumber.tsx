import { registerFormErrorMessage } from '../../../src/api';
import {
	collapsePhoneWhitespace,
	validateEmailInput,
	validateFullNameInput,
	validatePasswordInput,
	validatePhoneInput,
	validateYearsOfExperienceInput
} from '../../../src/auth';
import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useState } from 'react';
import { Pressable, StyleSheet, Text, TextInput, View } from 'react-native';
import { LabeledField, PrimaryButton, Screen, TextLink } from '../../../src/components/ui';
import { useRegisterPlumberMutation } from '../../../src/query';
import { colors, radius, space } from '../../../src/theme';

const GE_PHONE_PREFIX = '+995';

type YearsBucket = '1-3' | '3-7' | '7-15' | '15+';

const YEARS_BUCKETS: { id: YearsBucket; label: string; value: number }[] = [
	{ id: '1-3', label: '1–3 years', value: 2 },
	{ id: '3-7', label: '3–7 years', value: 5 },
	{ id: '7-15', label: '7–15 years', value: 10 },
	{ id: '15+', label: '15+ years', value: 20 }
];

function yearsForBucket(id: YearsBucket | null): number | null {
	if (!id) return null;
	const row = YEARS_BUCKETS.find((b) => b.id === id);
	return row?.value ?? null;
}

export default function RegisterPlumberScreen() {
	const router = useRouter();
	const register = useRegisterPlumberMutation();
	const [fullName, setFullName] = useState('');
	const [email, setEmail] = useState('');
	const [phoneLocal, setPhoneLocal] = useState('');
	const [yearsBucket, setYearsBucket] = useState<YearsBucket | null>(null);
	const [password, setPassword] = useState('');
	const [confirmPassword, setConfirmPassword] = useState('');
	const [clientError, setClientError] = useState<string | null>(null);
	const [succeeded, setSucceeded] = useState(false);

	const pending = register.isPending;

	const onSubmit = () => {
		setClientError(null);

		const nameErr = validateFullNameInput(fullName);
		if (nameErr) {
			setClientError(nameErr);
			return;
		}

		const emailResult = validateEmailInput(email);
		if (!emailResult.ok) {
			setClientError(emailResult.message);
			return;
		}

		const phoneForApi = `${GE_PHONE_PREFIX}${collapsePhoneWhitespace(phoneLocal)}`;
		const phoneErr = validatePhoneInput(phoneForApi);
		if (phoneErr) {
			setClientError(phoneErr);
			return;
		}

		const yearsParsed = yearsForBucket(yearsBucket);
		if (yearsParsed === null) {
			setClientError('Select your years of experience.');
			return;
		}
		const yearsErr = validateYearsOfExperienceInput(yearsParsed);
		if (yearsErr) {
			setClientError(yearsErr);
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
			{
				email: emailResult.email,
				password,
				full_name: fullName.trim(),
				phone: phoneForApi,
				years_of_experience: yearsParsed
			},
			{
				onSuccess: () => {
					setSucceeded(true);
				},
				onError: (err) => {
					setClientError(registerFormErrorMessage(err));
				}
			}
		);
	};

	if (succeeded) {
		return (
			<Screen scroll>
				<StatusBar style="dark" />
				<View style={styles.successCard}>
					<View style={styles.successIconWrap} accessibilityLabel="Success">
						<Text style={styles.successIconGlyph}>✓</Text>
					</View>
					<Text style={styles.successTitle}>{'You\u2019re registered'}</Text>
					<Text style={styles.successText}>
						Your plumber account is ready. Log in to start receiving jobs on Fixavon.
					</Text>
					<View style={styles.successActions}>
						<PrimaryButton label="Log in" onPress={() => router.push('/login')} />
						<View style={styles.gap} />
						<PrimaryButton
							variant="outline"
							label="Back to home"
							onPress={() => router.replace('/')}
						/>
					</View>
					<View style={styles.successFooterLinks}>
						<Text style={styles.successHint}>Looking for a household account?</Text>
						<TextLink label="Client sign up" onPress={() => router.push('/register')} />
					</View>
				</View>
			</Screen>
		);
	}

	return (
		<Screen scroll>
			<StatusBar style="dark" />
			<TextLink label="Back" onPress={() => router.back()} />
			<Text style={styles.title}>Join as a Plumber</Text>
			<Text style={styles.subtitle}>
				{'Empower your plumbing business with Tbilisi\u2019s most advanced digital platform.'}
			</Text>
			{clientError ? (
				<Text style={styles.error} accessibilityRole="alert">
					{clientError}
				</Text>
			) : null}

			<View style={styles.bento}>
				<View style={styles.bentoWide}>
					<View style={styles.bentoIconSecondary}>
						<Text style={styles.bentoIconGlyph}>₾</Text>
					</View>
					<View style={styles.bentoTextCol}>
						<Text style={styles.bentoTitle}>Steady Income</Text>
						<Text style={styles.bentoSubtitle}>Daily high-value leads across Tbilisi</Text>
					</View>
				</View>
				<View style={styles.bentoRow}>
					<View style={styles.bentoHalf}>
						<View style={styles.bentoIconPrimaryTint}>
							<Text style={[styles.bentoIconGlyphSm, styles.bentoIconColorPrimary]}>◇</Text>
						</View>
						<Text style={styles.bentoTitleSm}>Smart Management</Text>
					</View>
					<View style={styles.bentoHalf}>
						<View style={styles.bentoIconTertiaryTint}>
							<Text style={[styles.bentoIconGlyphSm, styles.bentoIconColorTertiary]}>✓</Text>
						</View>
						<Text style={styles.bentoTitleSm}>Verified Trust</Text>
					</View>
				</View>
			</View>

			<View style={styles.card}>
				<LabeledField
					label="Full Name"
					value={fullName}
					onChangeText={setFullName}
					autoCapitalize="words"
					placeholder="Gia Beridze"
					editable={!pending}
				/>
				<LabeledField
					label="Email Address"
					value={email}
					onChangeText={setEmail}
					keyboardType="email-address"
					placeholder="gia@fixavon.ge"
					editable={!pending}
				/>
				<View style={styles.phoneBlock}>
					<Text style={styles.phoneLabel}>Phone Number</Text>
					<View style={styles.phoneRow}>
						<View style={styles.phonePrefix}>
							<Text style={styles.phonePrefixText}>{GE_PHONE_PREFIX}</Text>
						</View>
						<TextInput
							value={phoneLocal}
							onChangeText={setPhoneLocal}
							keyboardType="phone-pad"
							placeholder="5XX XX XX XX"
							placeholderTextColor={colors.textMuted}
							editable={!pending}
							style={styles.phoneInput}
						/>
					</View>
				</View>
				<View style={styles.yearsBlock}>
					<Text style={styles.phoneLabel}>Years of Experience</Text>
					<View style={styles.yearsRow}>
						{YEARS_BUCKETS.map((b) => {
							const selected = yearsBucket === b.id;
							return (
								<Pressable
									key={b.id}
									onPress={() => setYearsBucket(b.id)}
									disabled={pending}
									style={({ pressed }) => [
										styles.yearChip,
										selected && styles.yearChipSelected,
										pressed && !pending && styles.yearChipPressed
									]}
									accessibilityRole="button"
									accessibilityState={{ selected }}
								>
									<Text
										style={[styles.yearChipLabel, selected && styles.yearChipLabelSelected]}
									>
										{b.label}
									</Text>
								</Pressable>
							);
						})}
					</View>
				</View>
				<LabeledField
					label="Password"
					value={password}
					onChangeText={setPassword}
					secureTextEntry
					placeholder="••••••••"
					editable={!pending}
				/>
				<LabeledField
					label="Confirm Password"
					value={confirmPassword}
					onChangeText={setConfirmPassword}
					secureTextEntry
					placeholder="••••••••"
					editable={!pending}
				/>
				<View style={styles.submitWrap}>
					<PrimaryButton
						label={pending ? 'Submitting…' : 'Create Pro Account →'}
						onPress={onSubmit}
						disabled={pending}
					/>
				</View>
			</View>

			<View style={styles.trustPill}>
				<View style={styles.trustDot} />
				<Text style={styles.trustPillText}>12 plumbers joined Fixavon today</Text>
			</View>
			<TextLink label="Sign up as a client instead" onPress={() => router.replace('/register')} />
		</Screen>
	);
}

const styles = StyleSheet.create({
	title: {
		fontSize: 40,
		fontWeight: '800',
		lineHeight: 44,
		letterSpacing: -0.5,
		color: colors.text,
		marginBottom: space[4],
		marginTop: space[2]
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
	bento: { marginBottom: space[12], gap: space[4] },
	bentoWide: {
		flexDirection: 'row',
		alignItems: 'center',
		gap: space[4],
		backgroundColor: colors.surfaceContainerLow,
		borderRadius: radius.lg,
		padding: 20
	},
	bentoRow: { flexDirection: 'row', gap: space[4] },
	bentoHalf: {
		flex: 1,
		backgroundColor: colors.surfaceContainerLow,
		borderRadius: radius.lg,
		padding: 20,
		gap: space[3]
	},
	bentoIconSecondary: {
		backgroundColor: colors.secondaryContainer,
		borderRadius: radius.lg,
		padding: space[3],
		alignItems: 'center',
		justifyContent: 'center'
	},
	bentoIconPrimaryTint: {
		backgroundColor: colors.primaryContainer,
		opacity: 0.35,
		alignSelf: 'flex-start',
		borderRadius: radius.md,
		paddingVertical: space[2],
		paddingHorizontal: space[3]
	},
	bentoIconTertiaryTint: {
		backgroundColor: colors.surfaceContainerHigh,
		alignSelf: 'flex-start',
		borderRadius: radius.md,
		paddingVertical: space[2],
		paddingHorizontal: space[3]
	},
	bentoIconGlyph: { fontSize: 22, fontWeight: '700', color: colors.onSecondaryContainer },
	bentoIconGlyphSm: { fontSize: 18, fontWeight: '700' },
	bentoIconColorPrimary: { color: colors.primary },
	bentoIconColorTertiary: { color: colors.tertiary },
	bentoTextCol: { flex: 1, gap: 4 },
	bentoTitle: { fontSize: 16, fontWeight: '700', color: colors.text },
	bentoSubtitle: { fontSize: 14, color: colors.textMuted, lineHeight: 20 },
	bentoTitleSm: { fontSize: 14, fontWeight: '700', color: colors.text },
	card: {
		backgroundColor: colors.surfaceElevated,
		borderRadius: radius.xl,
		padding: space[6],
		shadowColor: colors.text,
		shadowOffset: { width: 0, height: 8 },
		shadowOpacity: 0.06,
		shadowRadius: 20,
		elevation: 2
	},
	phoneBlock: { marginBottom: space[4] },
	phoneLabel: {
		fontSize: 14,
		fontWeight: '600',
		color: colors.text,
		marginBottom: space[2]
	},
	phoneRow: { flexDirection: 'row', alignItems: 'center', gap: space[2] },
	phonePrefix: {
		height: 48,
		paddingHorizontal: space[3],
		borderRadius: radius.lg,
		backgroundColor: colors.surfaceContainerLow,
		alignItems: 'center',
		justifyContent: 'center'
	},
	phonePrefixText: { fontSize: 14, fontWeight: '600', color: colors.textMuted },
	phoneInput: {
		flex: 1,
		minHeight: 48,
		paddingVertical: space[3],
		paddingHorizontal: space[4],
		borderRadius: radius.lg,
		borderWidth: 1,
		borderColor: colors.border,
		fontSize: 16,
		color: colors.text,
		backgroundColor: colors.surfaceContainerLow
	},
	yearsBlock: { marginBottom: space[4] },
	yearsRow: { flexDirection: 'row', flexWrap: 'wrap', gap: space[2] },
	yearChip: {
		paddingVertical: space[2],
		paddingHorizontal: space[3],
		borderRadius: radius.lg,
		backgroundColor: colors.surfaceContainerLow,
		borderWidth: 1,
		borderColor: colors.outlineVariant
	},
	yearChipSelected: {
		backgroundColor: colors.primaryContainer,
		borderColor: colors.primary
	},
	yearChipPressed: { opacity: 0.9 },
	yearChipLabel: { fontSize: 13, fontWeight: '600', color: colors.textMuted },
	yearChipLabelSelected: { color: colors.primary },
	submitWrap: { marginTop: space[4] },
	trustPill: {
		flexDirection: 'row',
		alignItems: 'center',
		alignSelf: 'center',
		gap: space[2],
		marginTop: space[8],
		marginBottom: space[4],
		paddingVertical: space[2],
		paddingHorizontal: space[4],
		borderRadius: 9999,
		backgroundColor: colors.surfaceContainerHigh,
		borderWidth: 1,
		borderColor: colors.outlineVariant
	},
	trustDot: {
		width: 8,
		height: 8,
		borderRadius: 4,
		backgroundColor: colors.tertiary
	},
	trustPillText: { fontSize: 12, fontWeight: '600', color: colors.textMuted },
	successCard: {
		alignItems: 'center',
		paddingVertical: space[8],
		paddingHorizontal: space[4]
	},
	successIconWrap: {
		width: 56,
		height: 56,
		borderRadius: 28,
		backgroundColor: colors.secondaryContainer,
		alignItems: 'center',
		justifyContent: 'center',
		marginBottom: space[6]
	},
	successIconGlyph: {
		fontSize: 28,
		fontWeight: '700',
		color: colors.onSecondaryContainer
	},
	successTitle: {
		fontSize: 24,
		fontWeight: '700',
		color: colors.text,
		marginBottom: space[3],
		textAlign: 'center'
	},
	successText: {
		fontSize: 16,
		lineHeight: 24,
		color: colors.textMuted,
		textAlign: 'center',
		marginBottom: space[8]
	},
	successActions: { alignSelf: 'stretch', width: '100%' },
	gap: { height: space[3] },
	successFooterLinks: { alignItems: 'center', marginTop: space[8] },
	successHint: {
		marginBottom: space[2],
		fontSize: 15,
		color: colors.textMuted,
		textAlign: 'center'
	}
});
