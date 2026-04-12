import type { Role } from '../api/types';

export type ProfileHref = '/client/profile' | '/plumber/profile' | '/admin/profile';

/** Expo Router href; mirrors web `profilePathForRole` (`apps/web/src/lib/auth/profilePaths.ts`). */
export function profileHrefForRole(role: Role): ProfileHref {
	return `/${role}/profile` as ProfileHref;
}
