/**
 * Client-side checks aligned with `apps/api` (mirror of `apps/web/src/lib/auth/validation.ts`).
 */

export const PASSWORD_MIN_LENGTH = 8;
export const PASSWORD_MAX_LENGTH = 256;

/** Mirrors `email_format_ok` in `apps/api` (post-trim, lowercase). */
export function emailFormatOk(s: string): boolean {
	if (s.length > 254) {
		return false;
	}
	const at = s.split('@');
	if (at.length !== 2) {
		return false;
	}
	const [local, domain] = at;
	if (!local.length || !domain.length) {
		return false;
	}
	if (local.includes('@') || domain.includes('@')) {
		return false;
	}
	if (domain.startsWith('.') || domain.endsWith('.') || domain.includes('..')) {
		return false;
	}
	return true;
}

export type EmailValidationResult = { ok: true; email: string } | { ok: false; message: string };

export function validateEmailInput(raw: string): EmailValidationResult {
	const trimmed = raw.trim();
	if (!trimmed) {
		return { ok: false, message: 'Email is required.' };
	}
	const normalized = trimmed.toLowerCase();
	if (!emailFormatOk(normalized)) {
		return { ok: false, message: 'Enter a valid email address.' };
	}
	return { ok: true, email: normalized };
}

export function validatePasswordInput(plain: string): string | null {
	if (!plain.trim()) {
		return 'Password is required.';
	}
	if (plain.length < PASSWORD_MIN_LENGTH) {
		return `Password must be at least ${PASSWORD_MIN_LENGTH} characters.`;
	}
	if (plain.length > PASSWORD_MAX_LENGTH) {
		return `Password must be at most ${PASSWORD_MAX_LENGTH} characters.`;
	}
	return null;
}

/** Mirrors `registration.rs` (`FULL_NAME_MAX`). */
export const FULL_NAME_MAX_LENGTH = 200;

/** Mirrors `registration.rs` phone length and digit checks. */
export const PHONE_COLLAPSED_MIN_LENGTH = 8;
export const PHONE_COLLAPSED_MAX_LENGTH = 32;

/** Mirrors `registration.rs` `YEARS_MAX`. */
export const YEARS_EXPERIENCE_MAX = 80;

export function validateFullNameInput(raw: string): string | null {
	const s = raw.trim();
	if (!s) {
		return 'Full name is required.';
	}
	if (s.length > FULL_NAME_MAX_LENGTH) {
		return 'Full name is too long.';
	}
	return null;
}

/** Collapse whitespace like API `normalize_and_validate_phone` (non-whitespace chars only). */
export function collapsePhoneWhitespace(raw: string): string {
	return [...raw].filter((c) => !/\s/u.test(c)).join('');
}

export function validatePhoneInput(raw: string): string | null {
	const collapsed = collapsePhoneWhitespace(raw);
	const len = collapsed.length;
	if (len < PHONE_COLLAPSED_MIN_LENGTH || len > PHONE_COLLAPSED_MAX_LENGTH) {
		return 'Enter a valid phone number (at least 8 digits).';
	}
	const digitCount = [...collapsed].filter((c) => c >= '0' && c <= '9').length;
	if (digitCount < 8) {
		return 'Enter a valid phone number (at least 8 digits).';
	}
	return null;
}

export function validateYearsOfExperienceInput(years: number): string | null {
	if (!Number.isFinite(years) || !Number.isInteger(years)) {
		return 'Years of experience must be a whole number.';
	}
	if (years < 0) {
		return 'Years of experience cannot be negative.';
	}
	if (years > YEARS_EXPERIENCE_MAX) {
		return `Years of experience must be at most ${YEARS_EXPERIENCE_MAX}.`;
	}
	return null;
}
