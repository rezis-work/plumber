import type { ExpoConfig } from 'expo/config';

import appJson from './app.json';

const base = appJson.expo as ExpoConfig;

const apiUrl = process.env.EXPO_PUBLIC_API_URL ?? '';
/** Dev-only: allow plain HTTP on Android (blocked by default on modern API levels). Never enable for production HTTPS URLs. */
const androidCleartextExplicit = process.env.EXPO_PUBLIC_ANDROID_CLEARTEXT === 'true';
const androidCleartextFromUrl = apiUrl.trimStart().toLowerCase().startsWith('http://');
const androidUsesCleartextTraffic = androidCleartextExplicit || androidCleartextFromUrl;

const config: ExpoConfig = {
	...base,
	android: {
		...(typeof base.android === 'object' && base.android !== null ? base.android : {}),
		...(androidUsesCleartextTraffic ? { usesCleartextTraffic: true } : {})
	},
	extra: {
		...(typeof base.extra === 'object' && base.extra !== null ? base.extra : {}),
		apiUrl,
		androidCleartextTraffic: androidUsesCleartextTraffic
	}
};

export default config;
