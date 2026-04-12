import { ApiError } from '$lib/api/client';
import type { AppLocale } from '$lib/i18n/config';
import { translate } from '$lib/i18n/translate';

/** Map known `ApiError` shapes to catalog keys; safe `validation_error` message passthrough via ICU. */
export function translateAuthApiError(locale: AppLocale, err: ApiError): string {
	if (err.status === 401) {
		return translate(locale, 'auth.api.invalidCredentials');
	}
	if (err.status === 403 && err.code === 'account_inactive') {
		return translate(locale, 'auth.api.accountInactive');
	}
	if (err.status === 409 && err.code === 'conflict') {
		return translate(locale, 'auth.api.emailExists');
	}
	if (err.status === 400 && err.code === 'validation_error') {
		const msg = err.message?.trim();
		if (msg) {
			return translate(locale, 'auth.api.validationMessage', { message: msg });
		}
		return translate(locale, 'auth.api.validationGeneric');
	}
	if (err.status === 401 && err.code === 'invalid_token') {
		return translate(locale, 'auth.api.verifyInvalidToken');
	}
	if (err.status === 410 && err.code === 'token_expired') {
		return translate(locale, 'auth.api.verifyTokenExpired');
	}
	return translate(locale, 'auth.api.generic');
}
