declare namespace NodeJS {
	interface ProcessEnv {
		EXPO_PUBLIC_API_URL?: string;
		/** When `true`, sets Android `usesCleartextTraffic` (dev only; use HTTPS in production). */
		EXPO_PUBLIC_ANDROID_CLEARTEXT?: string;
	}
}
