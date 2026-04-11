import { useMutation, useQueryClient } from '@tanstack/react-query';
import {
	applyNativeLoginResponse,
	authLogin,
	authLogout,
	authLogoutAll,
	authRegisterClient,
	authRegisterPlumber,
	authVerifyEmail,
	getMobileAuthBridge
} from '../api';
import type {
	LoginRequest,
	RegisterClientRequest,
	RegisterPlumberRequest,
	VerifyEmailRequest
} from '../api/types';
import { useAuth } from '../auth';
import { authMeQueryKey } from './authKeys';

export function useRegisterClientMutation() {
	return useMutation({
		mutationFn: (body: RegisterClientRequest) => authRegisterClient(body)
	});
}

export function useRegisterPlumberMutation() {
	return useMutation({
		mutationFn: (body: RegisterPlumberRequest) => authRegisterPlumber(body)
	});
}

export function useVerifyEmailMutation() {
	return useMutation({
		mutationFn: (body: VerifyEmailRequest) => authVerifyEmail(body)
	});
}

export function useLoginMutation() {
	const queryClient = useQueryClient();
	return useMutation({
		mutationFn: (body: LoginRequest) => authLogin(body),
		onSuccess: async (data) => {
			await applyNativeLoginResponse(data);
			await queryClient.invalidateQueries({ queryKey: authMeQueryKey });
		}
	});
}

export function useLogoutMutation() {
	const { clearSession } = useAuth();
	return useMutation({
		mutationFn: () => authLogout(),
		onSuccess: async () => {
			await clearSession();
		}
	});
}

export function useLogoutAllMutation() {
	const { clearSession } = useAuth();
	return useMutation({
		mutationFn: async () => {
			const t = getMobileAuthBridge().getAccessToken();
			if (!t) {
				throw new Error('Not authenticated');
			}
			return authLogoutAll(t);
		},
		onSuccess: async () => {
			await clearSession();
		}
	});
}
