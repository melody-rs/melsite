import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import { rust_crate } from './plugins/vite_wasm_pack';

export default defineConfig({
  build: {
    target: "esnext",
  },
  plugins: [
    sveltekit(),
    wasm(),
    rust_crate("rust/tetris-3d", { no_opt: true }) // temporarily
  ],
  server: {
    fs: {
      allow: [
        "rust/tetris-3d/pkg",
      ]
    }
  }
});
