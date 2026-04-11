export {
	loginErrorMessage,
	registerFormErrorMessage,
	verifyEmailErrorMessage
} from './authErrors';
export {
	applyNativeLoginResponse,
	authLogin,
	authLogout,
	authLogoutAll,
	authRegisterClient,
	authRegisterPlumber,
	authVerifyEmail
} from './authApi';
export type { MobileAuthBridge } from './authBridge';
export { getMobileAuthBridge, registerMobileAuthBridge } from './authBridge';
export { apiRequestAuthenticated } from './authenticatedRequest';
export {
	ApiError,
	apiRequest,
	NATIVE_CLIENT_HEADER,
	NATIVE_CLIENT_VALUE,
	nativeClientHeaders,
	type ApiRequestOptions
} from './client';
export { authMe, refreshWithStoredToken } from './refreshNative';
export type {
	LoginRequest,
	LoginResponse,
	MeResponse,
	NativeRefreshBody,
	Role
} from './types';
