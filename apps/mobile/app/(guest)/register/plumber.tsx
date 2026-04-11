import { useRouter } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { useState } from 'react';
import { Image, StyleSheet, Text } from 'react-native';
import { LabeledField, PrimaryButton, Screen, TextLink } from '../../../src/components/ui';
import { colors, radius, space } from '../../../src/theme';

const illustration = require('../../../assets/stitch/register-plumber-mobile/illustration.png');

export default function RegisterPlumberScreen() {
	const router = useRouter();
	const [fullName, setFullName] = useState('');
	const [phone, setPhone] = useState('');
	const [years, setYears] = useState('');
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
				accessibilityLabel="Plumber registration illustration"
			/>
			<Text style={styles.title}>Join as a plumber</Text>
			<Text style={styles.subtitle}>
				Share your experience—we will follow up with next steps.
			</Text>
			<LabeledField label="Full name" value={fullName} onChangeText={setFullName} autoCapitalize="words" />
			<LabeledField
				label="Phone"
				value={phone}
				onChangeText={setPhone}
				keyboardType="phone-pad"
				placeholder="+1 …"
			/>
			<LabeledField
				label="Years of experience"
				value={years}
				onChangeText={setYears}
				keyboardType="numeric"
				placeholder="0"
			/>
			<LabeledField
				label="Email"
				value={email}
				onChangeText={setEmail}
				keyboardType="email-address"
			/>
			<LabeledField label="Password" value={password} onChangeText={setPassword} secureTextEntry />
			<PrimaryButton label="Submit application" onPress={() => router.replace('/login')} />
			<TextLink label="Sign up as a client instead" onPress={() => router.replace('/register')} />
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
	}
});
