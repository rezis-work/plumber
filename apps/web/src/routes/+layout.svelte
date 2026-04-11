<script lang="ts">
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { hydrateFromRefresh } from '$lib/auth/session.svelte';

	let { children } = $props();

	onMount(() => {
		if (browser) {
			void hydrateFromRefresh();
		}
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children()}
