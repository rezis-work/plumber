/** sessionStorage handoff from `/register` to `/verify-email` (avoid token in URL). */

export const PENDING_EMAIL_VERIFICATION_KEY = 'fixavon_pending_email_verification';

export type PendingEmailVerification = {
	token: string;
	expires_at: string;
};
