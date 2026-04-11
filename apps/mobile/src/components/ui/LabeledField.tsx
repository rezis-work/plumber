import type { ReactNode } from 'react';
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
	editable?: boolean;
	/** Renders beside the label (e.g. forgot-password link). */
	labelRight?: ReactNode;
};

export function LabeledField({
	label,
	value,
	onChangeText,
	secureTextEntry,
	keyboardType = 'default',
	autoCapitalize = 'none',
	placeholder,
	editable = true,
	labelRight
}: Props) {
	return (
		<View style={styles.wrap}>
			{labelRight ? (
				<View style={styles.labelRow}>
					<Text style={styles.labelInRow}>{label}</Text>
					{labelRight}
				</View>
			) : (
				<Text style={styles.label}>{label}</Text>
			)}
			<TextInput
				editable={editable}
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
	labelRow: {
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'center',
		marginBottom: space[2]
	},
	label: {
		fontSize: 14,
		fontWeight: '600',
		color: colors.text,
		marginBottom: space[2]
	},
	labelInRow: {
		fontSize: 14,
		fontWeight: '600',
		color: colors.text
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
