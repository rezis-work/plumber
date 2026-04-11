import { Pressable, StyleSheet, Text } from 'react-native';
import { colors, space } from '../../theme';

type Props = {
	label: string;
	onPress: () => void;
};

export function TextLink({ label, onPress }: Props) {
	return (
		<Pressable onPress={onPress} style={styles.hit}>
			<Text style={styles.text}>{label}</Text>
		</Pressable>
	);
}

const styles = StyleSheet.create({
	hit: { paddingVertical: space[2], alignSelf: 'flex-start' },
	text: { fontSize: 15, fontWeight: '600', color: colors.primary }
});
