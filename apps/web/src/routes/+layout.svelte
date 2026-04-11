<script lang="ts">
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { hydrateFromRefresh, session } from '$lib/auth/session.svelte';

	let { children } = $props();

	onMount(() => {
		if (browser && session.accessToken === null) {
			void hydrateFromRefresh();
		}
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children()}
