import { ApiError } from './client';

/** Matches web `(guest)/login/+page.svelte` user-facing messages. */
export function loginErrorMessage(err: unknown): string {
	if (err instanceof ApiError) {
		if (err.status === 401) {
			return 'Invalid email or password.';
		}
		if (err.status === 400 && err.code === 'validation_error') {
			return err.message ?? 'Please check your input and try again.';
		}
		if (err.status === 403 && err.code === 'account_inactive') {
			return 'This account is disabled. Contact support if you need help.';
		}
		return 'Something went wrong. Please try again.';
	}
	return 'Network error. Check your connection and try again.';
}

export function verifyEmailErrorMessage(err: unknown): string {
	if (err instanceof ApiError) {
		if (err.status === 400 && err.code === 'validation_error') {
			return err.message ?? 'That code does not look valid. Check and try again.';
		}
		if (err.status === 401 && err.code === 'invalid_token') {
			return 'This verification code is not valid. Request a new one when available.';
		}
		if (err.status === 410 && err.code === 'token_expired') {
			return 'This link has expired. Sign up again or request a new email when available.';
		}
		return 'Something went wrong. Please try again.';
	}
	return 'Network error. Check your connection and try again.';
}

export function registerFormErrorMessage(err: unknown): string {
	if (err instanceof ApiError) {
		if (err.status === 409 && err.code === 'conflict') {
			return 'An account with this email already exists.';
		}
		if (err.status === 400 && err.code === 'validation_error') {
			return err.message ?? 'Please check your input and try again.';
		}
		return 'Something went wrong. Please try again.';
	}
	return 'Network error. Check your connection and try again.';
}
