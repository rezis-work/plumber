import type { ExpoConfig } from 'expo/config';

import appJson from './app.json';

const base = appJson.expo as ExpoConfig;

const config: ExpoConfig = {
	...base,
	extra: {
		...(typeof base.extra === 'object' && base.extra !== null ? base.extra : {}),
		apiUrl: process.env.EXPO_PUBLIC_API_URL ?? ''
	}
};

export default config;
