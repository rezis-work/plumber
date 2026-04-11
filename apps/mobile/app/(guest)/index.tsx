import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { Image, Pressable, StyleSheet, Text, View } from 'react-native';
import { PrimaryButton, Screen } from '../../src/components/ui';
import { colors, radius, space } from '../../src/theme';

const coverageMap = require('../../assets/stitch/landing-mobile/coverage-map.png');

const categories = [
	{ title: 'Emergency Repair', wide: true },
	{ title: 'Leak Fix', wide: false },
	{ title: 'Bathroom', wide: false },
	{ title: 'Kitchen', wide: false },
	{ title: 'Heaters', wide: false },
	{ title: 'Piping', wide: false }
] as const;

const howItWorks = [
	{
		step: '1',
		filled: true,
		title: 'Request Service',
		body: 'Describe your issue in 60 seconds through our simple form.'
	},
	{
		step: '2',
		filled: false,
		title: 'Get Matched',
		body: 'We find the nearest verified plumber in your Tbilisi district.'
	},
	{
		step: '3',
		filled: false,
		title: 'Professional Fix',
		body: 'The expert arrives with all tools to resolve the issue permanently.'
	},
	{
		step: '4',
		filled: false,
		title: 'Secure Payment',
		body: 'Pay digitally or via cash only after the job is successfully done.'
	}
] as const;

const plumberBullets = ['Flexible scheduling', 'Direct payments', 'Premium tools access'] as const;

export default function LandingScreen() {
	const router = useRouter();

	return (
		<Screen scroll>
			<StatusBar style="dark" />
			<View style={styles.topBar}>
				<Text style={styles.brandMark}>Fixavon</Text>
				<Pressable
					onPress={() => router.push('/login')}
					style={styles.logInHit}
					accessibilityRole="button"
					accessibilityLabel="Log in"
				>
					<Text style={styles.logInLabel}>Log in</Text>
				</Pressable>
			</View>

			<View style={styles.hero}>
				<View style={styles.badge}>
					<View style={styles.badgeDot} />
					<Text style={styles.badgeText}>Local Tbilisi Experts</Text>
				</View>
				<Text style={styles.headline}>
					Fast, Trusted Plumbers in <Text style={styles.headlineAccent}>Tbilisi</Text>
				</Text>
				<Text style={styles.lead}>
					Professional plumbing solutions delivered with architectural precision. Available 24/7 for
					emergencies.
				</Text>
				<View style={styles.ctaColumn}>
					<PrimaryButton label="Book a Plumber" onPress={() => router.push('/register')} />
					<View style={styles.gap} />
					<PrimaryButton
						variant="secondary"
						label="Become a Partner"
						onPress={() => router.push('/register/plumber')}
					/>
					<View style={styles.gap} />
					<PrimaryButton label="Log in" variant="outline" onPress={() => router.push('/login')} />
				</View>
			</View>

			<View style={styles.sectionMuted}>
				<Text style={styles.sectionTitle}>Service Categories</Text>
				<View style={styles.catGrid}>
					{categories.map((c) => (
						<View
							key={c.title}
							style={[styles.catCard, c.wide ? styles.catCardWide : styles.catCardHalf]}
						>
							<Text style={styles.catTitle}>{c.title}</Text>
						</View>
					))}
					<View style={[styles.catCard, styles.catCardHalf, styles.catMore]}>
						<Text style={styles.catMoreText}>More</Text>
					</View>
				</View>
			</View>

			<View style={styles.section}>
				<Text style={styles.sectionTitleLarge}>How it works</Text>
				{howItWorks.map((item) => (
					<View key={item.step} style={styles.stepRow}>
						<View
							style={[
								styles.stepNum,
								item.filled ? styles.stepNumFilled : styles.stepNumSoft
							]}
						>
							<Text
								style={[
									styles.stepNumText,
									item.filled ? styles.stepNumTextOnPrimary : styles.stepNumTextPrimary
								]}
							>
								{item.step}
							</Text>
						</View>
						<View style={styles.stepBody}>
							<Text style={styles.stepTitle}>{item.title}</Text>
							<Text style={styles.stepDesc}>{item.body}</Text>
						</View>
					</View>
				))}
			</View>

			<View style={styles.section}>
				<View style={styles.plumberCard}>
					<Text style={styles.plumberHeadline}>
						Master Plumbers:{'\n'}Scale your income
					</Text>
					<Text style={styles.plumberLead}>
						Join the elite network of Tbilisi's top plumbing professionals. We handle the marketing,
						you handle the craft.
					</Text>
					{plumberBullets.map((line) => (
						<Text key={line} style={styles.plumberBullet}>
							• {line}
						</Text>
					))}
					<View style={styles.gapLg} />
					<PrimaryButton
						variant="secondary"
						label="Join the Network"
						onPress={() => router.push('/register/plumber')}
					/>
				</View>
			</View>

			<View style={styles.section}>
				<Text style={styles.sectionTitle}>Tbilisi Coverage</Text>
				<Text style={styles.coverageLead}>
					From Vake to Varketili, we cover 100% of the capital.
				</Text>
				<View style={styles.mapWrap}>
					<Image
						source={coverageMap}
						style={styles.mapImage}
						resizeMode="cover"
						accessibilityLabel="Stylized map of Tbilisi coverage areas"
					/>
					<View style={styles.mapPill} pointerEvents="none">
						<View style={styles.mapPillInner}>
							<View style={styles.mapPillDot} />
							<Text style={styles.mapPillText}>12 Active Plumbers Near You</Text>
						</View>
					</View>
				</View>
			</View>

			<View style={styles.finalCta}>
				<Text style={[styles.sectionTitleLarge, styles.finalTitle]}>Ready to fix it?</Text>
				<Text style={styles.finalLead}>
					Experience the most professional plumbing service in Georgia.
				</Text>
				<PrimaryButton label="Book Now" onPress={() => router.push('/register')} />
				<Text style={styles.trustLine}>Fully Insured & Guaranteed</Text>
			</View>

			<View style={styles.footer}>
				<Text style={styles.footerBrand}>Fixavon</Text>
				<Text style={styles.footerLinks}>Services · Emergency · Terms · Privacy</Text>
				<Text style={styles.footerCopy}>© 2024 Fixavon Tbilisi. Professional Plumbing.</Text>
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
	brandMark: {
		fontSize: 20,
		fontWeight: '800',
		color: colors.primary,
		letterSpacing: -0.5
	},
	logInHit: { paddingVertical: space[2], paddingHorizontal: space[2] },
	logInLabel: { fontSize: 15, fontWeight: '600', color: colors.textMuted },
	hero: { marginBottom: space[8] },
	badge: {
		flexDirection: 'row',
		alignItems: 'center',
		alignSelf: 'flex-start',
		gap: space[2],
		paddingVertical: space[2],
		paddingHorizontal: space[3],
		borderRadius: 9999,
		backgroundColor: colors.secondaryContainer,
		marginBottom: space[6]
	},
	badgeDot: {
		width: 8,
		height: 8,
		borderRadius: 4,
		backgroundColor: colors.tertiary
	},
	badgeText: {
		fontSize: 12,
		fontWeight: '700',
		color: colors.onSecondaryContainer,
		letterSpacing: 0.5,
		textTransform: 'uppercase'
	},
	headline: {
		fontSize: 36,
		lineHeight: 42,
		fontWeight: '800',
		color: colors.text,
		letterSpacing: -0.5,
		marginBottom: space[6]
	},
	headlineAccent: { color: colors.primary },
	lead: {
		fontSize: 18,
		lineHeight: 28,
		color: colors.textMuted,
		marginBottom: space[8],
		paddingRight: space[4]
	},
	ctaColumn: { gap: 0 },
	gap: { height: space[4] },
	gapLg: { height: space[6] },
	section: { marginBottom: space[12] },
	sectionMuted: {
		marginHorizontal: -space[4],
		paddingHorizontal: space[4],
		paddingVertical: space[8],
		marginBottom: space[8],
		backgroundColor: colors.surfaceContainerLow
	},
	sectionTitle: {
		fontSize: 22,
		fontWeight: '700',
		color: colors.text,
		marginBottom: space[6]
	},
	sectionTitleLarge: {
		fontSize: 28,
		fontWeight: '800',
		color: colors.text,
		letterSpacing: -0.5,
		marginBottom: space[8]
	},
	catGrid: { flexDirection: 'row', flexWrap: 'wrap', gap: space[4] },
	catCard: {
		backgroundColor: colors.surfaceElevated,
		borderRadius: radius.lg,
		padding: space[6],
		justifyContent: 'center'
	},
	catCardWide: { width: '100%' },
	catCardHalf: { flexGrow: 1, flexBasis: '44%', minWidth: '44%', maxWidth: '48%' },
	catTitle: { fontSize: 16, fontWeight: '700', color: colors.text },
	catMore: {
		borderWidth: 2,
		borderStyle: 'dashed',
		borderColor: colors.outlineVariant,
		alignItems: 'center',
		minHeight: 72
	},
	catMoreText: { fontSize: 15, fontWeight: '700', color: colors.textMuted },
	stepRow: { flexDirection: 'row', gap: space[6], marginBottom: space[6] },
	stepNum: {
		width: 48,
		height: 48,
		borderRadius: radius.lg,
		alignItems: 'center',
		justifyContent: 'center'
	},
	stepNumFilled: { backgroundColor: colors.primary },
	stepNumSoft: { backgroundColor: colors.surfaceContainerHigh },
	stepNumText: { fontSize: 18, fontWeight: '800' },
	stepNumTextOnPrimary: { color: colors.onPrimary },
	stepNumTextPrimary: { color: colors.primary },
	stepBody: { flex: 1, paddingTop: 4 },
	stepTitle: { fontSize: 20, fontWeight: '700', color: colors.text, marginBottom: space[2] },
	stepDesc: { fontSize: 16, lineHeight: 24, color: colors.textMuted },
	plumberCard: {
		backgroundColor: colors.inverseSurface,
		borderRadius: radius.xl,
		padding: space[8],
		overflow: 'hidden'
	},
	plumberHeadline: {
		fontSize: 28,
		fontWeight: '800',
		color: colors.inverseOnSurface,
		lineHeight: 34,
		marginBottom: space[6]
	},
	plumberLead: {
		fontSize: 16,
		lineHeight: 24,
		color: colors.inverseOnSurface,
		opacity: 0.85,
		marginBottom: space[6]
	},
	plumberBullet: {
		fontSize: 16,
		color: colors.inverseOnSurface,
		marginBottom: space[3],
		paddingLeft: space[2]
	},
	coverageLead: {
		fontSize: 16,
		lineHeight: 24,
		color: colors.textMuted,
		marginBottom: space[6]
	},
	mapWrap: {
		height: 256,
		borderRadius: radius.lg,
		overflow: 'hidden',
		backgroundColor: colors.surfaceContainerHigh
	},
	mapImage: { width: '100%', height: '100%', opacity: 0.55 },
	mapPill: {
		...StyleSheet.absoluteFillObject,
		alignItems: 'center',
		justifyContent: 'center'
	},
	mapPillInner: {
		flexDirection: 'row',
		alignItems: 'center',
		gap: space[2],
		paddingVertical: space[2],
		paddingHorizontal: space[4],
		borderRadius: 9999,
		backgroundColor: colors.surfaceElevated,
		borderWidth: 1,
		borderColor: colors.outlineVariant
	},
	mapPillDot: {
		width: 8,
		height: 8,
		borderRadius: 4,
		backgroundColor: colors.primary
	},
	mapPillText: { fontSize: 14, fontWeight: '700', color: colors.text },
	finalCta: { alignItems: 'stretch', marginBottom: space[12] },
	finalTitle: { textAlign: 'center', alignSelf: 'stretch' },
	finalLead: {
		fontSize: 16,
		lineHeight: 24,
		color: colors.textMuted,
		marginBottom: space[8],
		textAlign: 'center'
	},
	trustLine: {
		marginTop: space[4],
		fontSize: 14,
		color: colors.textMuted,
		textAlign: 'center'
	},
	footer: {
		alignItems: 'center',
		paddingVertical: space[8],
		paddingHorizontal: space[4],
		backgroundColor: colors.surfaceContainerLow,
		marginHorizontal: -space[4],
		marginBottom: -space[8],
		borderTopLeftRadius: radius.xl,
		borderTopRightRadius: radius.xl,
		gap: space[3]
	},
	footerBrand: { fontSize: 18, fontWeight: '700', color: colors.primary },
	footerLinks: { fontSize: 14, color: colors.textMuted, textAlign: 'center' },
	footerCopy: {
		fontSize: 14,
		color: colors.textMuted,
		textAlign: 'center',
		lineHeight: 22
	}
});
