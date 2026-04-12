import type { AppLocale } from '$lib/i18n/config';
import { translate } from '$lib/i18n/translate';
import type { EmailValidationResult } from './validation';
import { PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH, YEARS_EXPERIENCE_MAX } from './validation';

export function translateEmailValidation(locale: AppLocale, result: EmailValidationResult): EmailValidationResult {
	if (result.ok) return result;
	if (result.message === 'Email is required.') {
		return { ok: false, message: translate(locale, 'auth.validation.emailRequired') };
	}
	if (result.message === 'Enter a valid email address.') {
		return { ok: false, message: translate(locale, 'auth.validation.emailInvalid') };
	}
	return result;
}

export function translatePasswordValidation(locale: AppLocale, err: string | null): string | null {
	if (!err) return null;
	if (err === 'Password is required.') return translate(locale, 'auth.validation.passwordRequired');
	if (err === `Password must be at least ${PASSWORD_MIN_LENGTH} characters.`) {
		return translate(locale, 'auth.validation.passwordMin', { min: PASSWORD_MIN_LENGTH });
	}
	if (err === `Password must be at most ${PASSWORD_MAX_LENGTH} characters.`) {
		return translate(locale, 'auth.validation.passwordMax', { max: PASSWORD_MAX_LENGTH });
	}
	return err;
}

export function translateFullNameError(locale: AppLocale, err: string | null): string | null {
	if (!err) return null;
	if (err === 'Full name is required.') return translate(locale, 'auth.validation.fullNameRequired');
	if (err === 'Full name is too long.') return translate(locale, 'auth.validation.fullNameTooLong');
	return err;
}

export function translatePhoneError(locale: AppLocale, err: string | null): string | null {
	if (!err) return null;
	if (err === 'Enter a valid phone number (at least 8 digits).') {
		return translate(locale, 'auth.validation.phoneInvalid');
	}
	return err;
}

export function translateYearsError(locale: AppLocale, err: string | null): string | null {
	if (!err) return null;
	if (err === 'Years of experience must be a whole number.') {
		return translate(locale, 'auth.validation.yearsWhole');
	}
	if (err === 'Years of experience cannot be negative.') {
		return translate(locale, 'auth.validation.yearsNegative');
	}
	if (err === `Years of experience must be at most ${YEARS_EXPERIENCE_MAX}.`) {
		return translate(locale, 'auth.validation.yearsMax', { max: YEARS_EXPERIENCE_MAX });
	}
	return err;
}
