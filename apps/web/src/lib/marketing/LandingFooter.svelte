<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve -- hrefs use pathWithLang for ?lang= */
	import { base } from '$app/paths';
	import { page } from '$app/state';
	import { pathWithLang } from '$lib/i18n/url';
	import { translate } from '$lib/i18n/translate';

	const sp = $derived(page.url.searchParams);
	const loc = $derived(page.data.locale);
	const hrefRoot = $derived(`${base}${pathWithLang('/', sp, loc)}`);
	const hrefRegisterPlumber = $derived(`${base}${pathWithLang('/register/plumber', sp, loc)}`);
	const hrefLogin = $derived(`${base}${pathWithLang('/login', sp, loc)}`);
</script>

<footer class="ft">
	<div class="lp-wrap ft__grid">
		<div class="ft__brand">
			<div class="ft__logo">{translate(loc, 'auth.shared.brand')}</div>
			<p class="ft__tagline lp-text-muted">
				{translate(loc, 'marketing.footer.tagline')}
			</p>
		</div>
		<div>
			<h4 class="ft__heading">{translate(loc, 'marketing.footer.headingServices')}</h4>
			<ul class="ft__links">
				<li><a href="#services">{translate(loc, 'marketing.footer.svcEmergency')}</a></li>
				<li><a href="#services">{translate(loc, 'marketing.footer.svcResidential')}</a></li>
				<li><a href="#services">{translate(loc, 'marketing.footer.svcCommercial')}</a></li>
			</ul>
		</div>
		<div>
			<h4 class="ft__heading">{translate(loc, 'marketing.footer.headingPartners')}</h4>
			<ul class="ft__links">
				<li><a href={hrefRegisterPlumber}>{translate(loc, 'marketing.footer.partnerProgram')}</a></li>
				<li><a href="#for-plumbers">{translate(loc, 'marketing.footer.training')}</a></li>
				<li><a href={hrefLogin}>{translate(loc, 'marketing.footer.plumberApp')}</a></li>
			</ul>
		</div>
		<div>
			<h4 class="ft__heading">{translate(loc, 'marketing.footer.headingCompany')}</h4>
			<ul class="ft__links">
				<li><a href={hrefRoot}>{translate(loc, 'auth.shared.privacyPolicy')}</a></li>
				<li><a href={hrefRoot}>{translate(loc, 'auth.shared.termsOfService')}</a></li>
				<li><a href={hrefRoot}>{translate(loc, 'marketing.footer.contact')}</a></li>
			</ul>
		</div>
	</div>
	<div class="lp-wrap ft__bottom">
		<p class="ft__copy lp-text-muted">{translate(loc, 'marketing.footer.copyright')}</p>
		<div class="ft__social">
			<a href={hrefRoot} aria-label={translate(loc, 'marketing.footer.socialAria')} class="ft__soc"><span class="material-symbols-outlined">public</span></a>
		</div>
	</div>
</footer>

<style>
	.ft {
		padding-block: var(--space-16);
		background: var(--color-surface-container-low);
	}

	.ft__grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-8);
		margin-bottom: var(--space-12);
	}

	@media (min-width: 768px) {
		.ft__grid {
			grid-template-columns: 2fr 1fr 1fr 1fr;
		}
	}

	.ft__brand {
		grid-column: span 2;
	}

	@media (min-width: 768px) {
		.ft__brand {
			grid-column: span 1;
		}
	}

	.ft__logo {
		font-size: 1.25rem;
		font-weight: var(--font-weight-bold);
		color: var(--color-text);
		margin-bottom: var(--space-6);
	}

	.ft__tagline {
		font-size: var(--text-sm);
		line-height: 1.6;
		max-width: 20rem;
	}

	.ft__heading {
		margin: 0 0 var(--space-6);
		font-size: var(--text-sm);
		font-weight: var(--font-weight-bold);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text);
	}

	.ft__links {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		font-size: var(--text-sm);
	}

	.ft__links a {
		color: var(--color-text-muted);
		text-decoration: none;
	}

	.ft__links a:hover {
		color: var(--color-primary);
		text-decoration: underline;
		text-underline-offset: 4px;
	}

	.ft__bottom {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		align-items: center;
		padding-top: var(--space-8);
		border-top: 1px solid color-mix(in srgb, var(--color-outline-variant) 20%, transparent);
	}

	@media (min-width: 768px) {
		.ft__bottom {
			flex-direction: row;
			justify-content: space-between;
		}
	}

	.ft__copy {
		margin: 0;
		font-size: 0.75rem;
	}

	.ft__social {
		display: flex;
		gap: var(--space-6);
	}

	.ft__soc {
		color: var(--color-text-muted);
	}

	.ft__soc:hover {
		color: var(--color-primary);
	}
</style>
