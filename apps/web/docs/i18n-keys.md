# Message key naming (web)

**Implementation 002 — Phase T0.** Aligns with [implementation_002_translations_frontend.md](../../../docs/implementation_002_translations/implementation_002_translations_frontend.md) §T0.2.

## Conventions

1. Use **dot-separated** namespaces for keys, e.g. `auth.login.title`, `marketing.hero.headline`.
2. **Shipped** UI should use **message keys** (via your chosen i18n layer), not user-facing English literals in Svelte files. Temporary literals are fine during migration.
3. Use the **same key paths** as the mobile app where the string is the same concept, so JSON can be shared later. See mobile **Phase TM0.2** in [implementation_002_translations_mobile.md](../../../docs/implementation_002_translations/implementation_002_translations_mobile.md).

## Locale defaults

Supported locales, default URL locale, and missing-key fallback are defined in [ADR 003 appendix](../../../docs/implementation_002_translations/adr_003_locale_routing_and_seo.md#appendix--web-locale-and-message-defaults-implementation-002-t0) and mirrored in [`src/lib/i18n/config.ts`](../src/lib/i18n/config.ts).

## URL locale (Phase T1)

The app **normalizes** GET/HEAD requests per [ADR 003 §3 (recommended)](../../../docs/implementation_002_translations/adr_003_locale_routing_and_seo.md): missing or invalid `lang` triggers a **307** to the same path with `?lang=<default>` first, then other query keys. [`src/hooks.server.ts`](../src/hooks.server.ts) performs the redirect; [`src/lib/i18n/url.ts`](../src/lib/i18n/url.ts) builds `lang`-first query strings for internal links and `goto`.

## Message files and `translate` (Phase T2)

- Catalogs live in [`src/lib/i18n/messages/`](../src/lib/i18n/messages/) as **`en.json`**, **`ka.json`**, **`ru.json`** with the **same nested shape** (add new keys in all three).
- Root layout [`+layout.server.ts`](../src/routes/+layout.server.ts) returns **`messages`** for the active locale plus **`locale`**.
- Resolve copy with [`translate.ts`](../src/lib/i18n/translate.ts): `translate(data.locale, 'marketing.landing.title')` (dot keys, fallback `ka`/`ru` → `en` → raw key per ADR).

## ICU and values (Phase T3)

- Use **`translate(locale, dotKey, values?)`** when a string has ICU placeholders or needs plural / number formatting. The runtime uses [`intl-messageformat`](https://formatjs.github.io/docs/intl-messageformat/) on the resolved template when `values` is passed **or** the template contains `{` (ICU).
- **Dev-only logging** (`import.meta.env.DEV`): missing keys after the full fallback chain log a **`console.warn`**; resolving from a **fallback locale** (e.g. `ka` → `en`) logs **`console.debug`** so incomplete catalogs are visible while authoring.
- **`formatIcuMessage(template, locale, values?)`** is exported for unit tests and pure formatting without catalog lookup.

### Numbers and dates

- Prefer **`Intl.NumberFormat`**, **`Intl.DateTimeFormat`**, or **`Intl.RelativeTimeFormat`** in `<script>` with **`locale`** from `data.locale` (same as the active UI).
- Either pass **already formatted strings** into ICU as `values` (e.g. `{ price: formattedPrice }`), or pass **numeric** `values` and use ICU **`number`** / **`date`** placeholders in the message template where appropriate.

**Example (badge-style stat):** format a count with `Intl.NumberFormat` for display, or pass `{ count: 2000 }` into a template like `Trusted by {count, number}+ Tbilisi households` so grouping follows the active locale (see [`LandingHero.svelte`](../src/lib/marketing/LandingHero.svelte)).
