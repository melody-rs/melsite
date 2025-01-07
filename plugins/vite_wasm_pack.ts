import type { Plugin } from "vite"
import { minimatch } from "minimatch"
import path from "node:path"
import fs from "fs-extra";

export interface WasmPackOptions {
  out_dir?: string,
  out_name?: string,
  scope?: string,

  dev?: boolean,
  release?: boolean,
  profiling?: boolean,

  no_opt?: boolean,
  no_pack?: boolean,
  no_typescript?: boolean,

  weak_refs?: boolean,
  ref_types?: boolean,

  target?: "bundler" | "nodejs" | "web" | "no-modules" | "deno",
  mode?: "no-install" | "normal" | "force",

  cargo_options?: string[],
}
async function run_wasm_pack(crate_path: string, options: WasmPackOptions = {}) {
  let args = [
    "wasm-pack",
    "build",
    crate_path,
  ]

  if (options.out_dir) {
    args.push("--out-dir");
    args.push(options.out_dir)
  }
  if (options.out_name) {
    args.push("--out-name");
    args.push(options.out_name)
  }
  if (options.scope) {
    args.push("--scope");
    args.push(options.scope)
  }

  if (options.dev) args.push("--dev")
  if (options.release) args.push("--release")
  if (options.profiling) args.push("--profiling")

  if (options.no_opt) args.push("--no-opt")
  if (options.no_pack) args.push("--no-pack")
  if (options.no_typescript) args.push("--no-typescript")

  if (options.weak_refs) args.push("--weak-refs")
  if (options.ref_types) args.push("--reference-types")

  if (options.target) {
    args.push("--target");
    args.push(options.target)
  }
  if (options.mode) {
    args.push("--mode");
    args.push(options.mode)
  }

  if (options.cargo_options) {
    args = args.concat(options.cargo_options);
  }

  const process = Bun.spawn(args);
  await process.exited;
}

export function rust_crate(crate_path: string, pack_options: WasmPackOptions = {}): Plugin {
  let crate_name = pack_options.out_name ?? path.basename(crate_path);
  let pkg_folder = pack_options.out_dir ?? "pkg";
  let pkg_path = path.join(crate_path, pkg_folder);

  let wasm_filename = crate_name.replace(/\-/g, "_") + "_bg.wasm";
  let wasm_path = path.join(pkg_path, wasm_filename);

  if (process.env.NODE_ENV == "development" && pack_options.dev === undefined) {
    pack_options.dev = true;
  }
  if (process.env.NODE_ENV == "production" && pack_options.dev === undefined) {
    pack_options.release = true;
  }
  if (pack_options.mode === undefined) {
    pack_options.mode = "no-install";
  }

  let watch = false;
  let is_building = false;
  const build = async () => {
    if (is_building) return;

    is_building = true;
    await run_wasm_pack(crate_path, pack_options);
    is_building = false;
  }

  return {
    name: "wasm-pack",
    enforce: "pre",

    apply(config) {
      return !config.build?.ssr;
    },

    config(config, env) {
      if (!config.server) config.server = {};
      if (!config.server.fs) config.server.fs = {}
      if (!config.server.fs.allow) config.server.fs.allow = [];
      config.server.fs?.allow.push(pkg_path);
    },

    async buildStart(options) {
      watch = this.meta.watchMode;

      if (!watch) return;
      await build();
    },

    async handleHotUpdate(ctx) {
      if (!watch) return;

      const pattern = path.resolve(ctx.server.config.root, crate_path, "/**/*.rs");
      const should_run = minimatch(ctx.file, pattern);

      if (!should_run) return;

      await build();
    },
  }
}
