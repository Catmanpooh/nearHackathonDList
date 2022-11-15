import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import { NodeGlobalsPolyfillPlugin } from '@esbuild-plugins/node-globals-polyfill';

const config: UserConfig = {
	plugins: [sveltekit()], 
	optimizeDeps: {
		include: ['near-api-js'],
		esbuildOptions: {
			target: 'esnext',
			plugins: [NodeGlobalsPolyfillPlugin({ buffer: true, process: true })]
		}
	},
	define: {
		global: 'globalThis',
		'process.env.BROWSER': true,
		'process.env.NODE_DEBUG': JSON.stringify('')
	}
};

export default config;
