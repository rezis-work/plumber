/** Mirrors `apps/api` auth DTO JSON (snake_case), aligned with `apps/web/src/lib/api/types.ts`. */

export type Role = 'client' | 'plumber' | 'admin';

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
