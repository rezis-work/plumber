<script lang="ts">
	import '$lib/marketing/landing.css';
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
</script>

<svelte:head>
	<title>Fixavon | Fast, Trusted Plumbers in Tbilisi</title>
	<meta
		name="description"
		content="Book verified plumbers in Tbilisi for repairs, installations, and emergencies."
	/>
	<link
		rel="stylesheet"
		href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0&display=swap"
	/>
</svelte:head>

<div class="marketing">
	<LandingNav />
	<main>
		<LandingHero />
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
