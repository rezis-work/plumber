import tokens from '../design/tokens.json';

const n = tokens.theme.namedColors;

/**
 * Semantic colors aligned with apps/web/src/app.css (--color-*).
 * Values come from Stitch namedColors via src/design/tokens.json.
 */
export const colors = {
	primary: n.primary,
	primaryContainer: n.primary_container,
	onPrimary: n.on_primary,
	secondary: n.secondary,
	secondaryContainer: n.secondary_container,
	onSecondaryContainer: n.on_secondary_container,
	tertiary: n.tertiary,
	surface: n.surface,
	surfaceContainerLow: n.surface_container_low,
	surfaceElevated: n.surface_container_lowest,
	surfaceContainerHigh: n.surface_container_high,
	text: n.on_surface,
	textMuted: n.on_surface_variant,
	outline: n.outline,
	outlineVariant: n.outline_variant,
	/** Alias for web `--color-border` (outline_variant). */
	border: n.outline_variant,
	/** Alias for web `--color-primary-bright` (primary_container). */
	primaryBright: n.primary_container,
	error: n.error,
	onError: n.on_error,
	success: n.tertiary,
	inverseSurface: n.inverse_surface,
	inverseOnSurface: n.inverse_on_surface
} as const;

export type ColorName = keyof typeof colors;
