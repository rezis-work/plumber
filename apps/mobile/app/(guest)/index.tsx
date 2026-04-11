import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { Image, StyleSheet, Text, View } from 'react-native';
import { Screen, PrimaryButton } from '../../src/components/ui';
import { colors, radius, space } from '../../src/theme';

const heroImage = require('../../assets/stitch/landing-mobile/hero.png');

export default function LandingScreen() {
	const router = useRouter();

	return (
		<Screen scroll>
			<StatusBar style="dark" />
			<Image
				source={heroImage}
				style={styles.hero}
				resizeMode="cover"
				accessibilityLabel="Fixavon marketing visual"
			/>
			<Text style={styles.brand}>Fixavon</Text>
			<Text style={styles.headline}>Trusted plumbing, on your schedule</Text>
			<Text style={styles.body}>
				Find vetted pros or grow your business—all in one place.
			</Text>
			<View style={styles.ctaColumn}>
				<PrimaryButton label="Sign up as a client" onPress={() => router.push('/register')} />
				<View style={styles.gap} />
				<PrimaryButton
					label="Become a plumber"
					variant="secondary"
					onPress={() => router.push('/register/plumber')}
				/>
				<View style={styles.gap} />
				<PrimaryButton label="Log in" variant="outline" onPress={() => router.push('/login')} />
			</View>
		</Screen>
	);
}

const styles = StyleSheet.create({
	hero: {
		width: '100%',
		aspectRatio: 16 / 9,
		borderRadius: radius.lg,
		marginBottom: space[6],
		backgroundColor: colors.surfaceContainerLow
	},
	brand: {
		fontSize: 28,
		fontWeight: '800',
		color: colors.primary,
		marginBottom: space[2]
	},
	headline: {
		fontSize: 22,
		fontWeight: '700',
		color: colors.text,
		marginBottom: space[3]
	},
	body: {
		fontSize: 16,
		lineHeight: 24,
		color: colors.textMuted,
		marginBottom: space[8]
	},
	ctaColumn: { gap: 0 },
	gap: { height: space[3] }
});
