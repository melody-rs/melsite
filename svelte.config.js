// @ts-check
import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

import { escapeSvelte, mdsvex } from "mdsvex"
import { createHighlighter } from 'shiki';

// TODO better syntax highlighting. Maybe tree sitter
const theme = "github-dark";
const shiki_highlighter = await createHighlighter({
  themes: [theme],
  langs: ["rust", "svelte", "javascript", "typescript", "css"] // expected languages
});
const highlight = async (code, lang = "text") => {
  const base_html = shiki_highlighter.codeToHtml(code, { lang, theme })
  const html = escapeSvelte(base_html);
  return `{@html \`${html}\`}`
}

/** @type {import('@sveltejs/kit').Config} */
const config = {
  extensions: [".svelte", ".md"],
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: [
    vitePreprocess({ script: true }),
    // the type decls for this are shit
    mdsvex({
      // @ts-ignore
      extensions: [".md", ".svx"],
      // @ts-ignore
      highlight,
    })
  ],

  kit: {
    // Normally I'd use adapter-bun but bun's csrf support is... meh...
    adapter: adapter({
      precompress: false,
    }),
    // csrf: true,
  }
};

export default config;
