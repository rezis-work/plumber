import { StatusBar } from 'expo-status-bar';
import { StyleSheet, Text, View } from 'react-native';
import { colors, space } from './src/theme';

export default function App() {
	return (
		<View style={styles.container}>
			<Text style={styles.title}>Fixavon</Text>
			<Text style={styles.subtitle}>Phase MS — shared design tokens with web</Text>
			<StatusBar style="dark" />
		</View>
	);
}

const styles = StyleSheet.create({
	container: {
		flex: 1,
		backgroundColor: colors.surface,
		alignItems: 'center',
		justifyContent: 'center',
		padding: space[6]
	},
	title: {
		color: colors.primary,
		fontSize: 22,
		fontWeight: '700',
		marginBottom: space[2]
	},
	subtitle: {
		color: colors.textMuted,
		fontSize: 14,
		textAlign: 'center'
	}
});
