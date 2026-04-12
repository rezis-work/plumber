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
	/** Stitch MS.3: small caps label above the field. */
	labelTone?: 'default' | 'overline';
	/** Filled surface, no border (Stitch card inputs). */
	inputVariant?: 'default' | 'filled';
	/** Renders at the end of the input row (e.g. password visibility). */
	trailingAccessory?: ReactNode;
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
	labelRight,
	labelTone = 'default',
	inputVariant = 'default',
	trailingAccessory
}: Props) {
	const labelDefaultStyle = labelTone === 'overline' ? styles.labelOverline : styles.label;
	const labelRowStyle =
		labelTone === 'overline' ? styles.labelOverlineInRow : styles.labelInRow;

	return (
		<View style={styles.wrap}>
			{labelRight ? (
				<View style={[styles.labelRow, labelTone === 'overline' && styles.labelRowOverline]}>
					<Text style={labelRowStyle}>{label}</Text>
					{labelRight}
				</View>
			) : (
				<Text style={labelDefaultStyle}>{label}</Text>
			)}
			<View style={styles.inputOuter}>
				<TextInput
					editable={editable}
					value={value}
					onChangeText={onChangeText}
					secureTextEntry={secureTextEntry}
					keyboardType={keyboardType}
					autoCapitalize={autoCapitalize}
					placeholder={placeholder}
					placeholderTextColor={colors.textMuted}
					style={[
						styles.input,
						inputVariant === 'filled' ? styles.inputFilled : undefined,
						trailingAccessory ? styles.inputWithAccessory : undefined
					]}
				/>
				{trailingAccessory ? (
					<View style={styles.trailingSlot}>{trailingAccessory}</View>
				) : null}
			</View>
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
	labelRowOverline: { marginBottom: space[2] },
	labelOverline: {
		fontSize: 12,
		fontWeight: '600',
		color: colors.textMuted,
		textTransform: 'uppercase',
		letterSpacing: 1.2,
		marginBottom: space[2]
	},
	labelOverlineInRow: {
		fontSize: 12,
		fontWeight: '600',
		color: colors.textMuted,
		textTransform: 'uppercase',
		letterSpacing: 1.2
	},
	inputOuter: {
		position: 'relative',
		flexDirection: 'row',
		alignItems: 'center'
	},
	input: {
		flex: 1,
		borderWidth: 1,
		borderColor: colors.border,
		borderRadius: radius.md,
		paddingVertical: space[3],
		paddingHorizontal: space[4],
		fontSize: 16,
		color: colors.text,
		backgroundColor: colors.surfaceElevated
	},
	inputFilled: {
		borderWidth: 0,
		borderRadius: radius.lg,
		backgroundColor: colors.surfaceContainerLow,
		paddingVertical: space[4]
	},
	inputWithAccessory: {
		paddingRight: 56
	},
	trailingSlot: {
		position: 'absolute',
		right: space[3],
		top: 0,
		bottom: 0,
		justifyContent: 'center'
	}
});
