import adapter from '@sveltejs/adapter-node';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	compilerOptions: {
		// Force runes mode for the project, except for libraries. Can be removed in svelte 6.
		runes: ({ filename }) => (filename.split(/[/\\]/).includes('node_modules') ? undefined : true)
	},
	kit: {
		// Node adapter: SSR HTML for crawlers and `pnpm start` (Phase T4). Deploy behind your process manager / reverse proxy.
		adapter: adapter()
	}
};

export default config;
