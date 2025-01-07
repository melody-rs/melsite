import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import { rust_crate } from './plugins/vite_wasm_pack';

export default defineConfig({
  plugins: [sveltekit(), wasm(), rust_crate("rust/tetris-3d")]
});
