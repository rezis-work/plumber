/** Mirrors `apps/api` auth DTO JSON (snake_case), aligned with `apps/web/src/lib/api/types.ts`. */

export type Role = 'client' | 'plumber' | 'admin';

export interface LoginRequest {
	email: string;
	password: string;
}

/** Body for native refresh / logout when cookie absent (ADR 002). */
export interface NativeRefreshBody {
	refresh_token: string;
}

export interface LoginResponse {
	access_token: string;
	token_type: string;
	expires_in: number;
	/** Present once API implements ADR 002 native login body. */
	refresh_token?: string;
}

export interface PlumberProfileResponse {
	full_name: string;
	phone: string;
	years_of_experience: number;
}

export interface MeResponse {
	id: string;
	email: string;
	role: Role;
	is_active: boolean;
	is_email_verified: boolean;
	created_at: string;
	updated_at: string;
	profile: PlumberProfileResponse | null;
}

export interface LogoutAllResponse {
	sessions_revoked: number;
}

export interface UserResponse {
	id: string;
	email: string;
	role: Role;
	is_active: boolean;
	is_email_verified: boolean;
	created_at: string;
	updated_at: string;
}

export interface RegisterClientRequest {
	email: string;
	password: string;
}

export interface RegisterPlumberRequest {
	email: string;
	password: string;
	full_name: string;
	phone: string;
	years_of_experience: number;
}

export interface RegisterPlumberResponse extends UserResponse {
	profile: PlumberProfileResponse;
}

export interface RegisterClientResponse extends UserResponse {
	email_verification_token: string;
	email_verification_expires_at: string;
}

export interface VerifyEmailRequest {
	token: string;
}

export interface VerifyEmailResponse {
	verified: boolean;
	already_verified: boolean;
}
