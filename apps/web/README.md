# Fixavon web (SvelteKit)

## API and auth (Phase A)

1. Copy **`.env.example`** to **`.env`** (optional; only needed if you set vars).
2. Start the API on port **3001** from `apps/api` (`cargo run`).
3. **`pnpm dev`** — Vite proxies **`/auth` → `http://127.0.0.1:3001/auth`**. Use **relative** URLs like `/auth/login` with `credentials: 'include'` so the refresh cookie path matches.

**Cross-origin mode:** set `PUBLIC_API_URL=http://localhost:3001` (or your API origin) in `.env`. The browser calls the API directly; the API must allow your web origin via `CORS_ALLOWED_ORIGINS` and use `SameSite=None; Secure` cookies over HTTPS. See `docs/implementation_001_auth/adr_001_cors_and_cookies.md`.

Auth route list: [`src/lib/api/routes.md`](src/lib/api/routes.md).

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
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://svelte.dev/docs/kit/adapters) for your target environment.
