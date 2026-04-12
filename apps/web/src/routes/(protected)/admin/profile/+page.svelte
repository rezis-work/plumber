<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import ProfileMePanel from '$lib/account/ProfileMePanel.svelte';
	import { apiRequestAuthenticated } from '$lib/api/authenticatedRequest';
	import type { MeResponse } from '$lib/api/types';
	import { session } from '$lib/auth/session.svelte';
	import SeoHead from '$lib/seo/SeoHead.svelte';
	import { translate } from '$lib/i18n/translate';

	const loc = $derived(page.data.locale);
	const pageTitle = $derived(translate(loc, 'auth.account.profilePageTitle'));
	const pageDescription = $derived(translate(loc, 'auth.account.profileMetaDescription'));

	let loading = $state(true);
	let loadError = $state<string | null>(null);

	onMount(async () => {
		try {
			const me = await apiRequestAuthenticated<MeResponse>('/auth/me', { method: 'GET' });
			session.user = me;
		} catch {
			loadError = 'Could not load your profile. Try again.';
		} finally {
			loading = false;
		}
	});
</script>

<SeoHead
	title={pageTitle}
	description={pageDescription}
	locale={loc}
	url={page.url}
	siteOrigin={page.data.siteOrigin}
/>

<ProfileMePanel user={session.user} {loading} error={loadError} />
