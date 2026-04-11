import { type ReactNode } from 'react';
import { ScrollView, StyleSheet, View, type ViewStyle } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { colors, space } from '../../theme';

type Props = {
	children: ReactNode;
	scroll?: boolean;
	contentStyle?: ViewStyle;
};

export function Screen({ children, scroll, contentStyle }: Props) {
	const inner = scroll ? (
		<ScrollView
			contentContainerStyle={[styles.scrollContent, contentStyle]}
			keyboardShouldPersistTaps="handled"
			showsVerticalScrollIndicator={false}
		>
			{children}
		</ScrollView>
	) : (
		<View style={[styles.fill, contentStyle]}>{children}</View>
	);

	return (
		<SafeAreaView style={styles.safe} edges={['top', 'left', 'right']}>
			{inner}
		</SafeAreaView>
	);
}

const styles = StyleSheet.create({
	safe: { flex: 1, backgroundColor: colors.surface },
	fill: { flex: 1, paddingHorizontal: space[4], paddingBottom: space[6] },
	scrollContent: {
		flexGrow: 1,
		paddingHorizontal: space[4],
		paddingBottom: space[8]
	}
});
