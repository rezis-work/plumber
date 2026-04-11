<script lang="ts">
	import { onMount } from 'svelte';
	import ProfileMePanel from '$lib/account/ProfileMePanel.svelte';
	import { apiRequestAuthenticated } from '$lib/api/authenticatedRequest';
	import type { MeResponse } from '$lib/api/types';
	import { session } from '$lib/auth/session.svelte';

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

<svelte:head>
	<title>Profile | Fixavon</title>
</svelte:head>

<ProfileMePanel user={session.user} {loading} error={loadError} />
