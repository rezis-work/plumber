import { StyleSheet, Text, TextInput, View } from 'react-native';
import { colors, radius, space } from '../../theme';

type Props = {
	label: string;
	value: string;
	onChangeText: (text: string) => void;
	secureTextEntry?: boolean;
	keyboardType?: 'default' | 'email-address' | 'numeric' | 'phone-pad';
	autoCapitalize?: 'none' | 'sentences' | 'words' | 'characters';
	placeholder?: string;
};

export function LabeledField({
	label,
	value,
	onChangeText,
	secureTextEntry,
	keyboardType = 'default',
	autoCapitalize = 'none',
	placeholder
}: Props) {
	return (
		<View style={styles.wrap}>
			<Text style={styles.label}>{label}</Text>
			<TextInput
				value={value}
				onChangeText={onChangeText}
				secureTextEntry={secureTextEntry}
				keyboardType={keyboardType}
				autoCapitalize={autoCapitalize}
				placeholder={placeholder}
				placeholderTextColor={colors.textMuted}
				style={styles.input}
			/>
		</View>
	);
}

const styles = StyleSheet.create({
	wrap: { marginBottom: space[4] },
	label: {
		fontSize: 14,
		fontWeight: '600',
		color: colors.text,
		marginBottom: space[2]
	},
	input: {
		borderWidth: 1,
		borderColor: colors.border,
		borderRadius: radius.md,
		paddingVertical: space[3],
		paddingHorizontal: space[4],
		fontSize: 16,
		color: colors.text,
		backgroundColor: colors.surfaceElevated
	}
});
