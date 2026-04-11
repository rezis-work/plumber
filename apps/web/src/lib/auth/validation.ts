/**
 * Client-side checks aligned with `apps/api` `normalize_email` / `email_format_ok` / password policy defaults.
 */

export const PASSWORD_MIN_LENGTH = 8;
export const PASSWORD_MAX_LENGTH = 256;

/** Mirrors `email_format_ok` in `apps/api/src/modules/auth/passwords.rs` (post-trim, lowercase). */
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
