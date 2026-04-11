import { Pressable, StyleSheet, Text } from 'react-native';
import { colors, radius, space } from '../../theme';

type Props = {
	label: string;
	onPress: () => void;
	variant?: 'primary' | 'secondary' | 'outline';
};

export function PrimaryButton({ label, onPress, variant = 'primary' }: Props) {
	return (
		<Pressable
			onPress={onPress}
			style={({ pressed }) => [
				styles.base,
				variant === 'primary' && styles.primary,
				variant === 'secondary' && styles.secondary,
				variant === 'outline' && styles.outline,
				pressed && styles.pressed
			]}
		>
			<Text
				style={[
					styles.label,
					variant === 'primary' && styles.labelOnPrimary,
					variant === 'secondary' && styles.labelSecondary,
					variant === 'outline' && styles.labelOutline
				]}
			>
				{label}
			</Text>
		</Pressable>
	);
}

const styles = StyleSheet.create({
	base: {
		paddingVertical: space[3],
		paddingHorizontal: space[6],
		borderRadius: radius.md,
		alignItems: 'center',
		justifyContent: 'center',
		minHeight: 48
	},
	primary: {
		backgroundColor: colors.primaryBright
	},
	secondary: {
		backgroundColor: colors.secondaryContainer
	},
	outline: {
		backgroundColor: 'transparent',
		borderWidth: 1,
		borderColor: colors.outline
	},
	pressed: { opacity: 0.88 },
	label: { fontSize: 16, fontWeight: '600' },
	labelOnPrimary: { color: colors.onPrimary },
	labelSecondary: { color: colors.onSecondaryContainer },
	labelOutline: { color: colors.primary }
});
