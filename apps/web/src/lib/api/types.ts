/** Mirrors `apps/api` auth DTO JSON (snake_case). */

export type Role = 'client' | 'plumber' | 'admin';

export interface LoginRequest {
	email: string;
	password: string;
}

export interface LoginResponse {
	access_token: string;
	token_type: string;
	expires_in: number;
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
