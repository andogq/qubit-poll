import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

import presetEnv from 'postcss-preset-env';

export default defineConfig({
	plugins: [sveltekit()],
	css: {
		transformer: 'postcss',
		postcss: {
			plugins: [presetEnv()]
		}
	},
	server: {
		proxy: {
			'/_': {
				target: 'ws://localhost:3030',
				rewrite: (path) => path.slice(2),
				ws: true
			}
		}
	}
});
