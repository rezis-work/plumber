<script lang="ts">
	import type { PageProps } from './$types';
	import { page } from '$app/state';
	import '$lib/marketing/landing.css';
	import SeoHead from '$lib/seo/SeoHead.svelte';
	import { translate } from '$lib/i18n/translate';
	import LandingNav from '$lib/marketing/LandingNav.svelte';
	import LandingHero from '$lib/marketing/LandingHero.svelte';
	import LandingHowItWorks from '$lib/marketing/LandingHowItWorks.svelte';
	import LandingWhy from '$lib/marketing/LandingWhy.svelte';
	import LandingExpertise from '$lib/marketing/LandingExpertise.svelte';
	import LandingPlumber from '$lib/marketing/LandingPlumber.svelte';
	import LandingTestimonials from '$lib/marketing/LandingTestimonials.svelte';
	import LandingCoverage from '$lib/marketing/LandingCoverage.svelte';
	import LandingFaq from '$lib/marketing/LandingFaq.svelte';
	import LandingFinalCta from '$lib/marketing/LandingFinalCta.svelte';
	import LandingFooter from '$lib/marketing/LandingFooter.svelte';
	import LandingTrustFloat from '$lib/marketing/LandingTrustFloat.svelte';

	let { data }: PageProps = $props();

	const pageTitle = $derived(translate(data.locale, 'marketing.landing.title'));
	const pageDescription = $derived(translate(data.locale, 'marketing.landing.description'));
	const heroHeadline = $derived(translate(data.locale, 'marketing.hero.headline'));
</script>

<SeoHead
	title={pageTitle}
	description={pageDescription}
	locale={data.locale}
	url={page.url}
	siteOrigin={data.siteOrigin}
/>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0&display=swap"
	/>
</svelte:head>

<div class="marketing">
	<LandingNav />
	<main>
		<LandingHero headline={heroHeadline} />
		<LandingHowItWorks />
		<LandingWhy />
		<LandingExpertise />
		<LandingPlumber />
		<LandingTestimonials />
		<LandingCoverage />
		<LandingFaq />
		<LandingFinalCta />
	</main>
	<LandingFooter />
	<LandingTrustFloat />
</div>
