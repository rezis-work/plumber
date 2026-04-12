# Implementation 002 — Translations (i18n)

This folder contains **step-by-step implementation guides** (instructions only; no embedded code) for adding **English, Georgian, and Russian** UI translations to the **SvelteKit** web app and the **Expo** mobile app, with **strong SEO** on the web.

## Documents

| Document | Scope |
|----------|--------|
| [implementation_002_translations_frontend.md](./implementation_002_translations_frontend.md) | SvelteKit: **`?lang=`** routing, message catalogs, SSR-safe loading, **meta, hreflang, sitemap** |
| [implementation_002_translations_mobile.md](./implementation_002_translations_mobile.md) | Expo: device locale, optional user preference, catalogs, `Intl` formatting |

## ADR

| ADR | Topic |
|-----|--------|
| [adr_003_locale_routing_and_seo.md](./adr_003_locale_routing_and_seo.md) | Web: **`lang` query** locales, defaults, `hreflang`, canonical URLs |

## Locales (v1)

| Tag | Language |
|-----|----------|
| `en` | English |
| `ka` | Georgian |
| `ru` | Russian |

Use **BCP 47** tags consistently in the **`lang` query**, filenames, and HTML `lang` attributes.

## Suggested order

1. **ADR 003** — Lock **URL strategy** and **default locale** with stakeholders (SEO implications).
2. **Frontend guide** — Phases T0–T8: catalogs, routing, components, then **SEO** surfaces.
3. **Mobile guide** — Phases TM0–TM7: align keys with web where possible, ship device + override behavior, store listing checklist.

## Shared keys (optional follow-up)

If web and mobile should share identical JSON, introduce a small workspace package (e.g. `packages/i18n-messages`) **after** both apps have stable key lists; the guides assume separate files per app until then.
