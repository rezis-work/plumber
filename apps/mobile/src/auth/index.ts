export { AuthProvider, useAuth } from './AuthContext';
export {
	collapsePhoneWhitespace,
	emailFormatOk,
	FULL_NAME_MAX_LENGTH,
	PASSWORD_MAX_LENGTH,
	PASSWORD_MIN_LENGTH,
	PHONE_COLLAPSED_MAX_LENGTH,
	PHONE_COLLAPSED_MIN_LENGTH,
	validateEmailInput,
	validateFullNameInput,
	validatePasswordInput,
	validatePhoneInput,
	validateYearsOfExperienceInput,
	YEARS_EXPERIENCE_MAX,
	type EmailValidationResult
} from './validation';
export { profileHrefForRole } from './profilePaths';
export { SessionGate, useSessionBootstrap } from './SessionGate';
export {
	deleteRefreshToken,
	getRefreshToken,
	REFRESH_TOKEN_SECURE_KEY,
	setRefreshToken
} from './secureRefreshToken';
