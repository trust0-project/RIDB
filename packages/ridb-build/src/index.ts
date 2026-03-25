/** biome-ignore-all lint/suspicious/noExplicitAny: Not needed here */

import fs from "node:fs";
import path from "node:path";
import { NodeResolvePlugin } from "@esbuild-plugins/node-resolve";
import { defineConfig, type Format } from "tsup";

const packagesDir = path.resolve(process.cwd(), "../../", "packages/ridb-core");
export const wasmPlugin = {
  name: "wasm",
  setup(build: any) {
    build.onResolve({ filter: /\.wasm$/ }, (args: any) => {
      if (fs.existsSync(path.resolve(packagesDir, args.path))) {
        return { path: path.resolve(packagesDir, args.path), namespace: "wasm" };
      }
      return { path: path.resolve("../../node_modules", args.path), namespace: "wasm" };
    });
    build.onLoad({ filter: /.*/, namespace: "wasm" }, async (args: any) => {
      const buffer = await fs.promises.readFile(args.path);
      const base64 = buffer.toString("base64");
      return {
        contents: `export default Buffer.from("${base64}", "base64")`,
        loader: "js",
      };
    });
  },
};

/**
 * Strips `new URL("*_bg.wasm", import.meta.url)` from wasm-bindgen JS output.
 * esbuild can't tree-shake the async default export that contains this reference.
 */
export const wasmUrlStripPlugin = {
  name: "strip-wasm-url",
  setup(build: any) {
    build.onLoad({ filter: /\.js$/ }, async (args: any) => {
      if (!args.path.includes("pkg") && !args.path.includes("generated")) return undefined;
      const source = await fs.promises.readFile(args.path, "utf-8");
      if (!source.includes("_bg.wasm")) return undefined;
      const stripped = source.replace(
        /new URL\(["'][^"']*_bg\.wasm["'],\s*import\.meta\.url\)/g,
        "undefined"
      );
      return { contents: stripped, loader: "js" };
    });
  },
};

export const plugins = [
  NodeResolvePlugin({
    extensions: [".ts", ".js", ".wasm"],
    onResolved: (resolved) => {
      if (resolved.includes("node_modules")) {
        return {
          external: true,
        };
      }
      return resolved;
    },
  }),
];

export default function createConfig({
  format,
  entry,
  banner,
  platform,
}: {
  format: Format | Format[] | undefined;
  entry: string[] | undefined;
  banner?: { js: string };
  platform?: "neutral" | "node" | "browser";
}) {
  return defineConfig(({ watch: _watch }) => ({
    entry,
    format,
    outDir: "build",
    target: "esnext",
    minify: false,
    clean: false,
    esbuildPlugins: [wasmPlugin, wasmUrlStripPlugin, ...plugins],
    banner,
    esbuildOptions: (options, _context) => {
      options.platform = platform || "neutral";
    },
    external: ["buffer", "next", "vitest", "react-server-dom-webpack", "tsup", "react-server-dom-webpack/client.edge", "@trust0/ridb/worker"],
  }));
}
