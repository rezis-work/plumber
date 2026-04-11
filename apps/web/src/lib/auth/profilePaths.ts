import type { Role } from '$lib/api/types';

/** Path segment only; prepend `base` for `goto` / `href`. */
export function profilePathForRole(role: Role): string {
	return `/${role}/profile`;
}
