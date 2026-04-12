# Fixavon web (SvelteKit)

## API and auth (Phase A)

1. Copy **`.env.example`** to **`.env`** (optional; only needed if you set vars).
2. Start the API on port **3001** from `apps/api` (`cargo run`).
3. **`pnpm dev`** — Vite proxies **`/auth` → `http://127.0.0.1:3001/auth`**. Use **relative** URLs like `/auth/login` with `credentials: 'include'` so the refresh cookie path matches.

**Cross-origin mode:** set `PUBLIC_API_URL=http://localhost:3001` (or your API origin) in `.env`. The browser calls the API directly; the API must allow your web origin via `CORS_ALLOWED_ORIGINS` and use `SameSite=None; Secure` cookies over HTTPS. See `docs/implementation_001_auth/adr_001_cors_and_cookies.md`.

Auth route list: [`src/lib/api/routes.md`](src/lib/api/routes.md).

---

## Internationalization (T0–T4)

- **Locales, default, and missing-key fallback** — [ADR 003 appendix](../../docs/implementation_002_translations/adr_003_locale_routing_and_seo.md#appendix--web-locale-and-message-defaults-implementation-002-t0).
- **Message key naming** (dot-separated keys, alignment with mobile) — [`docs/i18n-keys.md`](docs/i18n-keys.md) (includes **ICU / Intl** patterns for Phase T3).
- **Typed config** — [`src/lib/i18n/config.ts`](src/lib/i18n/config.ts).
- **URL `lang` query** — redirect normalization and helpers: [`src/hooks.server.ts`](src/hooks.server.ts), [`src/lib/i18n/url.ts`](src/lib/i18n/url.ts) (see ADR [§3 recommended](../../docs/implementation_002_translations/adr_003_locale_routing_and_seo.md)).
- **Runtime messages** — JSON per locale in [`src/lib/i18n/messages/`](src/lib/i18n/messages/) (`marketing`, `auth`, `error`, `common`); root [`+layout.server.ts`](src/routes/+layout.server.ts) exposes `data.messages`; use [`translate.ts`](src/lib/i18n/translate.ts) (`translate(locale, 'dot.key', values?)`) in components. Same module on server and client avoids hydration skew.
- **SSR / SEO (Phase T4)** — Guest marketing and auth routes use **server-side rendering** for translated HTML. Production: **`pnpm build`** then **`pnpm start`** ([`@sveltejs/adapter-node`](https://svelte.dev/docs/kit/adapter-node)). See [implementation doc § Phase T4](../../docs/implementation_002_translations/implementation_002_translations_frontend.md#phase-t4--ssr-visible-content-for-seo).
- **Canonical / hreflang (Phase T6)** — Set **`PUBLIC_SITE_URL`** (HTTPS, no trailing slash) in production so `<link rel="canonical">`, `hreflang` alternates, and `og:url` use your public host behind a reverse proxy. If unset, the request origin is used (fine for local dev).
- **Georgian and Russian copy** is intended to be **native-quality** for production. Any **machine-translated** placeholder strings must be tracked for human review (internal ticket: **`I18N-KA-RU-REVIEW`**).

---

## Creating a project (template boilerplate)

If you're seeing this, you've probably already done this step. Congrats!

```sh
# create a new project
npx sv create my-app
```

To recreate this project with the same configuration:

```sh
# recreate this project
npx sv@0.15.1 create --template minimal --types ts --add eslint vitest="usages:unit" --no-download-check --no-install apps/web
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```sh
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```sh
pnpm build
```

Run the **Node** server (SSR, including localized HTML for SEO):

```sh
pnpm start
```

(`vite preview` serves the client shell only; use `pnpm start` after `pnpm build` to verify production SSR locally.)
