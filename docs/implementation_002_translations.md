# Implementation 002 — Translations (index)

This is the **entry document** for **Feature 002: internationalization (i18n)** across the **web** and **mobile** apps.

**Supported locales (v1):**

| Tag | Language |
|-----|----------|
| `en` | English |
| `ka` | Georgian |
| `ru` | Russian |

All three are **left-to-right**; no RTL layout work is required for this set.

**Detailed guides (step-by-step; instructions for planning and implementation):**

| Guide | File |
|--------|------|
| **Frontend (SvelteKit)** — UI strings, routing, **SEO** | [implementation_002_translations/implementation_002_translations_frontend.md](./implementation_002_translations/implementation_002_translations_frontend.md) |
| **Mobile (Expo)** — UI strings, device locale, persistence | [implementation_002_translations/implementation_002_translations_mobile.md](./implementation_002_translations/implementation_002_translations_mobile.md) |

**ADR:** [Locale routing and SEO (web)](./implementation_002_translations/adr_003_locale_routing_and_seo.md) — **`lang` query parameter** (`?lang=en|ka|ru`), `hreflang`, canonical policy.

**Order:**

1. Read the **ADR** and agree **URL shape** and **default locale** (product + SEO).
2. Define **message key conventions** and **namespace** layout (both apps should align on key names if you later share bundles).
3. Implement **frontend** (catalogs, layout, meta, sitemap).
4. Implement **mobile** (runtime locale, storage override, catalogs).

**API / CMS:** This feature set is **client-only** in these guides. If the API returns user-visible strings, add a separate task to version and translate API messages or to keep APIs key-based and render text only on clients.

**Folder overview:** [implementation_002_translations/README.md](./implementation_002_translations/README.md)

---

## Goals (summary)

- **Three languages:** `en`, `ka`, `ru` with a clear **fallback** (typically `en` or your primary market locale).
- **Web:** Discoverable localized URLs (including **`?lang=`** variants), correct **`html lang`**, **`hreflang`**, and **per-locale titles/descriptions** for SEO.
- **Mobile:** Respect **device locale**, allow optional **in-app override**, format dates/numbers with **`Intl`**.
- **Single source of truth for keys:** Prefer one naming scheme across web and mobile; optionally extract shared JSON later into `packages/` if the monorepo grows.

Use the linked guides when prompting an AI or assigning tasks: each is split into **ordered phases** with verification-style acceptance notes.
