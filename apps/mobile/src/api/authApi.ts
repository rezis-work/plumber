import { getRefreshToken, setRefreshToken } from '../auth/secureRefreshToken';
import { getMobileAuthBridge } from './authBridge';
import { apiRequest } from './client';
import type {
	LoginRequest,
	LoginResponse,
	LogoutAllResponse,
	RegisterClientRequest,
	RegisterClientResponse,
	RegisterPlumberRequest,
	RegisterPlumberResponse,
	VerifyEmailRequest,
	VerifyEmailResponse
} from './types';

export function authLogin(body: LoginRequest): Promise<LoginResponse> {
	return apiRequest<LoginResponse>('/auth/login', {
		method: 'POST',
		jsonBody: body,
		nativeClient: true
	});
}

/** Persist refresh + session flags after native login (ADR 002). Call after `authLogin`. */
export async function applyNativeLoginResponse(login: LoginResponse): Promise<void> {
	const bridge = getMobileAuthBridge();
	if (login.refresh_token) {
		await setRefreshToken(login.refresh_token);
		bridge.setHasRefreshTokenStored(true);
	}
	bridge.setAccessSession(login.access_token, login.expires_in);
}

export async function authLogout(): Promise<void> {
	const refresh_token = await getRefreshToken();
	await apiRequest<void>('/auth/logout', {
		method: 'POST',
		jsonBody: refresh_token ? { refresh_token } : undefined,
		nativeClient: true
	});
}

export function authLogoutAll(accessToken: string): Promise<LogoutAllResponse> {
	return apiRequest<LogoutAllResponse>('/auth/logout-all', {
		method: 'POST',
		accessToken,
		nativeClient: true
	});
}

export function authRegisterClient(body: RegisterClientRequest): Promise<RegisterClientResponse> {
	return apiRequest<RegisterClientResponse>('/auth/register/client', {
		method: 'POST',
		jsonBody: body
	});
}

export function authRegisterPlumber(body: RegisterPlumberRequest): Promise<RegisterPlumberResponse> {
	return apiRequest<RegisterPlumberResponse>('/auth/register/plumber', {
		method: 'POST',
		jsonBody: body
	});
}

export function authVerifyEmail(body: VerifyEmailRequest): Promise<VerifyEmailResponse> {
	return apiRequest<VerifyEmailResponse>('/auth/verify-email', {
		method: 'POST',
		jsonBody: body
	});
}
