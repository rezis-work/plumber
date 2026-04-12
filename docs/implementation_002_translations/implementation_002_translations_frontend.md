# Implementation 002 — Translations frontend (SvelteKit)

**Purpose:** Step-by-step instructions to add **English (`en`)**, **Georgian (`ka`)**, and **Russian (`ru`)** to the web app (`apps/web`), with **SEO-first** URL structure and metadata. Instructions only—no embedded code.

**Assumptions:**

- You adopt **[ADR 003 — Locale routing and SEO](./adr_003_locale_routing_and_seo.md)** (**query parameter** `?lang=en|ka|ru`) unless product approves a documented alternative.
- SvelteKit **SSR** is enabled for public pages that must be indexed.
- You will pick one **i18n library** (or a thin custom layer) that supports **server-side** message resolution for SEO-critical pages.

**Recommended libraries (choose one and document in the repo):**

| Approach | Notes |
|----------|--------|
| **inlang Paraglide** + SvelteKit adapter | Strong TypeScript ergonomics, compile-time message validation; fits SvelteKit layouts. |
| **typesafe-i18n** | Explicitly typesafe keys; works with SvelteKit with a small setup. |
| **sveltekit-i18n** | Simpler runtime JSON; ensure SSR path loads the correct locale per request. |

The phases below are **library-agnostic**; map each step to your chosen stack.

**Order:** **T0 (conventions)** → **T1–T2 (routing + integration)** → **T3–T4 (messages + SSR)** → **T5–T8 (SEO)** → **T9 (verification)**.

---

## Phase T0 — Conventions and scope

### Step T0.1 — Locale allowlist

1. Fix the allowlist: **`en`**, **`ka`**, **`ru`** only (BCP 47).
2. Choose **default locale** for redirects when `lang` is missing/invalid and for **`x-default` hreflang** (business decision: e.g. `ka` for Georgia-first, or `en` for broader default).
3. Choose **fallback chain** when a translation key is missing (e.g. requested `ru` → `en` → display key).

### Step T0.2 — Key naming

1. Use **dot-separated** namespaces: e.g. `auth.login.title`, `marketing.hero.headline`.
2. **No** user-facing English copy in component files as the long-term state—only keys (allow temporary literals during migration).
3. Align key names with the **mobile** guide so the same logical string can share a name across clients later.

### Step T0.3 — Acceptable verification

- Written table of locales + default + fallback in repo (README or ADR appendix).
- Key naming doc (short paragraph) linked from `apps/web` README.

---

## Phase T1 — URL routing and layout

### Step T1.1 — Keep path structure; resolve locale from query

1. **Do not** add a leading **locale path segment** (e.g. `/en/...`, `/ka/...`); keep your existing **pathname** structure (e.g. `/login`, `/`).
2. Resolve locale from **`event.url.searchParams.get('lang')`** in **root `+layout.server.ts`**, a **handle hook**, or equivalent — validate against **`en` | `ka` | `ru`**; if missing or invalid, use **default locale** for SSR (see ADR for optional **redirect** to `?lang=<default>` on indexable routes).
3. Align with **ADR 003** on whether bare URLs **redirect** to `?lang=<default>` or serve default without redirect; stay consistent for **canonical** and duplicate-content signals.

### Step T1.2 — Invalid `lang` and internal links

1. If `lang` is not in the allowlist, **strip** it and treat as default, **404**, or **redirect** to `?lang=<default>` (pick one policy and apply consistently).
2. Ensure **deep links**, **navigation**, and **form actions** **preserve or set `lang`** (merge with existing search params safely; avoid dropping unrelated query keys).

### Step T1.3 — Language switcher

1. On switch, keep the **same pathname** and update only the **`lang`** query (or replace invalid values).

### Step T1.4 — Acceptance

- Hitting `/` (and other indexable routes) behaves per ADR: either **redirects** to `?lang=<default>` or serves default with a documented canonical policy.
- Changing **`lang`** in the query shows the same logical page in another language (once messages exist).

---

## Phase T2 — i18n runtime wiring (SvelteKit)

### Step T2.1 — Server load

1. In **`+layout.server.ts`** (or root layout), read **`event.url.searchParams.get('lang')`**, validate it against the allowlist, and **load messages** for the resolved locale (import JSON, fetch from CMS, or use compiled Paraglide output).
2. Expose **locale + messages** (or message loader) to the layout tree via **`data`** or context.

### Step T2.2 — Client hydration

1. Ensure the **same locale** is used on first client render as on the server (no flash of wrong language).
2. If you offer a **language switcher**, navigate to the **same pathname** with an updated **`lang`** query (merge with existing search params; keep order stable per ADR).

### Step T2.3 — Acceptance

- View page source (SSR HTML) shows **translated** title/headline for at least one page per locale.
- No hydration mismatch warnings for locale-dependent text on smoke-tested routes.

---

## Phase T3 — Message catalogs

### Step T3.1 — File layout

1. Store translations per locale, e.g. `messages/en.json`, `messages/ka.json`, `messages/ru.json` (or per-namespace splits).
2. **Georgian and Russian** must be **native-quality** for production; machine translation is acceptable only for scaffolding with a tracked review ticket.

### Step T3.2 — Interpolation and plurals

1. Use **ICU MessageFormat** (or library equivalent) for **variables** and **plural** rules.
2. Document how **numbers and dates** in sentences are formatted (prefer **`Intl`** in script or pre-formatted values passed into messages).

### Step T3.3 — Acceptance

- At least **marketing**, **auth**, and **error** namespaces have all three locales filled for shipped routes.
- Missing key behavior matches **T0.1** fallback (visible in dev, logged).

---

## Phase T4 — SSR-visible content for SEO

### Step T4.1 — Critical copy in HTML

1. **H1** and primary paragraph text on marketing pages must be rendered in **SSR HTML**, not inserted only after client JS.
2. **Navigation** labels and **footer** links text should be SSR for crawl budget on key pages.

### Step T4.2 — Acceptance

- Disable JavaScript in the browser; default-locale landing still shows correct language for above-the-fold text.

---

## Phase T5 — Per-locale `<title>` and meta description

### Step T5.1 — `svelte:head` or `+layout.ts` metadata

1. Each major route supplies a **translated `<title>`** unique per page and locale (avoid duplicate titles across languages for the same URL path pattern).
2. Add **meta name="description"** with translated, **concise** copy (roughly 150–160 characters per market guidance; adjust with SEO owner).

### Step T5.2 — Open Graph and Twitter

1. **`og:title`**, **`og:description`**, **`og:locale`** — use tags aligned with [Facebook / Open Graph locale](https://ogp.me/) (e.g. `ka_GE`, `en_US`, `ru_RU` — confirm mapping table for your site).
2. **`og:locale:alternate`** for the other locales.
3. **`twitter:card`** and matching title/description if you use Twitter metadata.

### Step T5.3 — Acceptance

- **Rich sharing debugger** (or equivalent) shows correct language for at least one URL per locale.

---

## Phase T6 — `hreflang` and canonical URLs

### Step T6.1 — Alternate links

1. For every **indexable** localized page, emit in `<head>` (path and other query keys as appropriate; **`lang` must be present** in each alternate per ADR):

   - `<link rel="alternate" hreflang="en" href="https://yourdomain/...?lang=en">`
   - Same for `ka` and `ru`: `...?lang=ka`, `...?lang=ru`.
   - `<link rel="alternate" hreflang="x-default" href="https://yourdomain/...?lang={default}">`

2. Use **absolute HTTPS** URLs in production. If other query parameters exist, use a **stable ordering** (document whether `lang` is first).

### Step T6.2 — Canonical

1. `<link rel="canonical" href="https://yourdomain/...?lang={current}">` **self-referential** for that language variant (include full query string as agreed for that page).
2. Do **not** point all languages to a single canonical unless SEO owner explicitly requests it (usually **not** for multilingual sites).

### Step T6.3 — `html lang`

1. Set `<html lang="{lang}">` (use full regional tags only if you standardize them, e.g. `ka` vs `ka-GE`; be consistent with `hreflang`).

### Step T6.4 — Acceptance

- Automated or manual checklist: every template in the **indexable** set has alternates + canonical + `lang`.

---

## Phase T7 — Sitemap and robots

### Step T7.1 — XML sitemap

1. Generate **`sitemap.xml`** (or split sitemaps) listing **full URLs** for every **indexable pathname × each locale**, e.g. `https://yourdomain/login?lang=en`, `...?lang=ka`, `...?lang=ru` (omit bare URLs for language variants if ADR mandates **normalize** to `?lang=`).
2. Optionally one **sitemap index** pointing to split files (by section or locale).

### Step T7.2 — `robots.txt`

1. Reference the sitemap URL(s).
2. Do not block paths or query patterns you want indexed.

### Step T7.3 — Acceptance

- Submit sitemap in **Google Search Console** (and Yandex Webmaster if targeting Russian search) after deploy.

---

## Phase T8 — Structured data (optional, SEO boost)

### Step T8.1 — JSON-LD

1. For **organization** / **local business** (Fixavon, Tbilisi), add **JSON-LD** with **`@language`** or localized fields where schema allows.
2. Keep **one graph per page** or a consistent include strategy to avoid duplicates.

### Step T8.2 — Acceptance

- **Rich results test** passes without errors for a sample page.

---

## Phase T9 — Verification checklist

- [ ] All three locales render for **marketing** and **auth** flows without broken links.
- [ ] **SSR** shows translated **title**, **H1**, and **meta description** per locale.
- [ ] **`hreflang`** + **canonical** + **`html lang`** present on indexable pages.
- [ ] **Sitemap** lists full **`?lang=`** URLs for each indexable variant; **robots.txt** points to sitemap.
- [ ] Language switcher preserves **pathname** and non-`lang` **query** while updating **`lang`**.
- [ ] **No** mixed-language sentences in production catalogs (review `ka` and `ru`).

---

## Long-term notes

- **CMS:** If marketing copy moves to a headless CMS, replace static JSON loading with API fetch in `load` and cache; keep **`lang` query**, `hreflang`, and canonical rules unchanged.
- **Legal pages** (terms, privacy) may require **jurisdiction-specific** text, not just translation—track as separate content items.
