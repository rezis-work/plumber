<script lang="ts">
	import type { AppLocale } from '$lib/i18n/config';
	import { localeHeadLinks } from './localeHeadLinks';
	import { ogLocaleAlternates, ogLocaleFor } from './ogLocale';

	let {
		title,
		description,
		locale,
		url,
		siteOrigin
	}: {
		title: string;
		description: string;
		locale: AppLocale;
		url: URL;
		siteOrigin: string;
	} = $props();

	const ogLocale = $derived(ogLocaleFor(locale));
	const ogAlts = $derived(ogLocaleAlternates(locale));
	const headLinks = $derived(localeHeadLinks(url, siteOrigin));
	const canonicalHref = $derived(headLinks.canonicalHref);
	const hreflangAlternates = $derived(headLinks.alternates);
</script>

<svelte:head>
	<title>{title}</title>
	<link rel="canonical" href={canonicalHref} />
	{#each hreflangAlternates as alt}
		<link rel="alternate" hreflang={alt.hreflang} href={alt.href} />
	{/each}
	<meta name="description" content={description} />
	<meta property="og:title" content={title} />
	<meta property="og:description" content={description} />
	<meta property="og:url" content={canonicalHref} />
	<meta property="og:type" content="website" />
	<meta property="og:locale" content={ogLocale} />
	{#each ogAlts as alt}
		<meta property="og:locale:alternate" content={alt} />
	{/each}
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content={title} />
	<meta name="twitter:description" content={description} />
</svelte:head>
