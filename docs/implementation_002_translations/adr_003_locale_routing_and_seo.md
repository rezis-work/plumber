# ADR 003 — Locale routing and SEO (SvelteKit web)

## Status

**Accepted** — **query-parameter** locale (`?lang=`) chosen by product for **Implementation 002**.

## Context

- The product must serve **English (`en`)**, **Georgian (`ka`)**, and **Russian (`ru`)** with good **search visibility** in Georgia and for Russian-speaking users.
- **Cookie-only** or **Accept-Language-only** selection without **stable, linkable URLs per language** makes it hard for crawlers to index each variant.
- **Path prefixes** (`/ka/...`) give very clear URLs but require **restructuring** every route; **subdomains** add operational surface.
- **Query parameters** can still yield **distinct URLs per language** (e.g. `/login?lang=ka` vs `/login?lang=en`) as long as those URLs are **linked** (`hreflang`), **canonicalized**, and listed in **sitemaps** — not switched only in client JS without URL updates.

## Decision

1. **Query-parameter locale** on the web: use a single parameter **`lang`** with values **`en` | `ka` | `ru`** (BCP 47 tags). Example: `https://example.com/?lang=ka`, `https://example.com/login?lang=ru`.
2. **Resolution:** Read **`url.searchParams.get('lang')`** on the server (SvelteKit: `load`, `event.url`, or hooks). Validate against the allowlist; if **missing** or **invalid**, resolve to the **default locale** for SSR output.
3. **Canonical URL policy (pick one and apply site-wide):**
   - **Recommended:** **Normalize** indexable requests missing `lang` with a **redirect (301/302)** to the same pathname plus **`?lang=<default>`** so the indexed URL always explicitly includes `lang`, avoiding duplicate signals between bare path and default language.
   - **Alternative:** Serve default language on the bare URL without redirect; then set **canonical** on the bare URL for the default only and use **`?lang=`** URLs for the other languages — document carefully to avoid duplicate-content ambiguity.
4. **Internal links:** Navigation, `goto`, and form actions must **preserve or set `lang`** (helper that merges `lang` with existing search params; keep a **stable parameter order**, e.g. `lang` first when present).
5. **`hreflang`:** For each indexable page, emit **`<link rel="alternate" hreflang="...">`** with **absolute HTTPS** URLs that **include** the query string, e.g. `https://example.com/login?lang=en`, `...?lang=ka`, `...?lang=ru`, plus **`hreflang="x-default"`** pointing at the **default** locale URL (typically `...?lang=<default>` if using normalization).
6. **Canonical:** **Self-referential** link for the **current** language variant, **including** the `lang` query (and any other required query keys in a **fixed order**). One canonical per variant; do not collapse all languages to a single URL unless SEO owner explicitly requires it.
7. **`<html lang="...">`:** Set from the **resolved** active locale on every SSR response (consistent with `hreflang` tags).

## Consequences

- **Pros:** No migration to a **locale-prefixed** route tree; **distinct crawlable URLs** per language remain achievable with strict `hreflang`, canonicals, and sitemaps.
- **Cons / risks:** Search engines may treat query strings with more nuance than path segments; **mitigate** with: consistent `lang` name and values, **sitemap** entries for every `?lang=` variant you want indexed, and a **clear** policy for bare URL vs `?lang=default` (see **Decision 3**).
- **Mobile:** This ADR applies to **web only**; store listings supply metadata; in-app language is independent (see [implementation_002_translations_mobile.md](./implementation_002_translations_mobile.md)).

## Alternatives considered

- **Path-based** locale prefixes (e.g. `/ka/...`, `/en/...`) — strong default for SEO ergonomics; not chosen (product preference).
- **Separate subdomains** — valid; revisit if marketing or CDN rules require it.
- **Single URL + JS-only language switch** without URL updates — poor for indexing; not chosen.

## References

- [Google: localized versions / hreflang](https://developers.google.com/search/docs/specialty/international/localized-versions) (verify current URL in your implementation week).
- Implementation steps: [implementation_002_translations_frontend.md](./implementation_002_translations_frontend.md).
