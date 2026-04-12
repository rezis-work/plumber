# Implementation 002 — Translations mobile (Expo / React Native)

**Purpose:** Step-by-step instructions to add **English (`en`)**, **Georgian (`ka`)**, and **Russian (`ru`)** to the mobile app (`apps/mobile`). **SEO does not apply** to in-app screens; focus on **UX**, **locale detection**, and **consistency** with web **message keys** where possible. Instructions only—no embedded code.

**Assumptions:**

- **Expo SDK** in use with **`expo-localization`** (or equivalent) to read device preferences.
- You will use a mainstream i18n library (**`i18next` + `react-i18next`**) or a smaller wrapper; phases below are **library-agnostic** where possible.
- **Web URLs** and **`hreflang`** are defined in [implementation_002_translations_frontend.md](./implementation_002_translations_frontend.md) and [ADR 003](./adr_003_locale_routing_and_seo.md); mobile does **not** duplicate SEO mechanics.

**Order:** **TM0–TM1** (alignment + detection) → **TM2–TM3** (catalogs + provider) → **TM4** (formatting) → **TM5** (persistence & switcher) → **TM6** (store metadata) → **TM7** (verification).

---

## Phase TM0 — Alignment with web

### Step TM0.1 — Same locale tags

1. Use **`en`**, **`ka`**, **`ru`** only—same strings as the web allowlist.
2. Map **Expo locale tags** (e.g. `ka-GE`) to your **canonical** app locale (`ka`) with an explicit mapping table.

### Step TM0.2 — Message keys

1. Prefer **the same key paths** as the web app (`auth.login.title`, etc.) to allow a future **shared package** of JSON.
2. Until shared, **duplicate JSON files** in `apps/mobile` are fine; avoid divergent key names for the same concept.

### Step TM0.3 — Acceptance

- Short mapping document: device tag → app locale (`en` | `ka` | `ru`).

---

## Phase TM1 — Device locale detection

### Step TM1.1 — Read OS preferences

1. On launch, read **`expo-localization`** (`getLocales()` / `locale`) to infer the user’s **preferred language**.
2. If the device language is **not** in the allowlist, fall back to **default locale** (match web default from **ADR 003** / frontend guide).

### Step TM1.2 — Acceptance

- Simulator/device set to **Georgian** → app starts in **`ka`** (once messages exist).
- Unsupported language (e.g. `de`) → **default** locale.

---

## Phase TM2 — Message catalogs

### Step TM2.1 — File layout

1. Add per-locale JSON (or TS modules) under e.g. `apps/mobile/src/i18n/locales/{en,ka,ru}.json`.
2. Bootstrap **Georgian** and **Russian** with the same quality bar as web (native review for production).

### Step TM2.2 — Namespaces (optional)

1. Split large apps by namespace (`auth`, `marketing`) mirroring web structure.
2. **Lazy-load** heavy namespaces if startup time becomes an issue.

### Step TM2.3 — Acceptance

- All **user-visible** strings on **shipped** screens use keys (no stray literals for those screens).

---

## Phase TM3 — i18n provider and hooks

### Step TM3.1 — Root provider

1. Wrap the app root (inside **`SessionGate`** / **`QueryClient`** as needed) with your i18n **Provider** so **Expo Router** screens can call **`useTranslation()`** (or equivalent).

### Step TM3.2 — React Navigation / Expo Router

1. **Screen titles** (`options.title`) and **header** strings must go through i18n.
2. **Deep links** do not carry locale in the path for native (unlike web); locale remains **app state**.

### Step TM3.3 — Acceptance

- Navigating between stacks does not reset language unintentionally.

---

## Phase TM4 — Dates, numbers, and relative time

### Step TM4.1 — `Intl` API

1. Use **`Intl.DateTimeFormat`** and **`Intl.NumberFormat`** with the **active app locale** (not hard-coded `en-US`).
2. For **relative time** (“2 hours ago”), use a small helper or library compatible with RN.

### Step TM4.2 — Acceptance

- A sample date renders with **Georgian** month/day conventions when locale is `ka` (verify visually with a native speaker).

---

## Phase TM5 — User override and persistence

### Step TM5.1 — In-app language switcher

1. Provide **Settings** (or profile) control: **English / ქართული / Русский** (labels can be autonyms).
2. On change, update i18n **language** and **persist** choice in **`expo-secure-store`** or **AsyncStorage** (SecureStore not required for non-secret preference—AsyncStorage is typical).

### Step TM5.2 — Precedence

1. Define order: **saved preference** overrides **device locale** on next launch.
2. Optional: “**Use device language**” toggle clears override.

### Step TM5.3 — Acceptance

- Change language in settings → **restart app** → choice persists.
- Clear app data → behavior returns to **device** inference.

---

## Phase TM6 — App Store / Play Store (metadata only)

### Step TM6.1 — Store listings

1. **Localized listing** text (title, subtitle, description, keywords) is configured in **App Store Connect** and **Google Play Console**—outside this repo.
2. Keep **screenshots** aligned with at least **one** primary locale; add localized screenshots when marketing is ready.

### Step TM6.2 — Acceptance

- Checklist item for release: listing exists for **en** (minimum) and ideally **ka** / **ru** as product requires.

---

## Phase TM7 — Verification checklist

- [ ] **Three** in-app languages work on **iOS** and **Android** for main flows.
- [ ] **Fallback** when translation missing matches dev/prod policy.
- [ ] **No RTL** bugs (N/A for en/ka/ru, but confirm no accidental RTL flags).
- [ ] **Dates/numbers** respect active locale.
- [ ] **Persisted** language override survives process kill.
- [ ] **Accessibility:** screen reader reads translated strings (system language may still affect TTS voice—document limitation if any).

---

## Long-term notes

- **Push notifications** in multiple languages require **server-side** templates or per-user locale; out of scope for this guide.
- **OTA updates** (EAS Update) ship message JSON changes without store review—useful for typo fixes in `ka`/`ru`.
