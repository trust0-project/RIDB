


import { defineConfig, Format } from 'tsup';
import { NodeResolvePlugin } from '@esbuild-plugins/node-resolve';
import fs from 'fs';
import path from 'path';

const packagesDir = path.resolve(process.cwd(), "../../", "packages/ridb-core");
export const wasmPlugin = {
    name: 'wasm',
    setup(build:any) {
        build.onResolve({ filter: /\.wasm$/ }, (args:any) => {
            if (fs.existsSync(path.resolve(packagesDir, args.path))) {
                return { path: path.resolve(packagesDir, args.path), namespace: 'wasm' };
            }
            return { path: path.resolve('../../node_modules', args.path), namespace: 'wasm' };
        });
        build.onLoad({ filter: /.*/, namespace: 'wasm' }, async (args:any) => {
            const buffer = await fs.promises.readFile(args.path);
            const base64 = buffer.toString('base64');
            return {
                contents: `export default Buffer.from("${base64}", "base64")`,
                loader: 'js',
            };
        });
    },
};

export const plugins = [
    NodeResolvePlugin({
        extensions: ['.ts', '.js', '.wasm'],
        onResolved: (resolved) => {
            if (resolved.includes('node_modules')) {
                return {
                    external: true,
                }
            }
            return resolved
        },
    }),
]

export default function createConfig({format, entry, banner, platform}:{
  format: Format | Format[] | undefined,
  entry: string[] | undefined,
  banner?: {js:string},
  platform?: 'neutral' | 'node' | 'browser'
}) {
  return defineConfig(({ watch }) => ({
    entry,
    format,
    outDir: 'build',
    target: 'esnext',
    minify: false,
    clean: false,
    esbuildPlugins: [
      wasmPlugin,
      ...plugins
    ],
    banner,
    esbuildOptions:(options, context) => {
      options.platform = platform || 'neutral'
    },
    external: [
      'buffer',
      'next',
      'vitest', 
      'react-server-dom-webpack', 
      'tsup', 
      'react-server-dom-webpack/client.edge',
      '@trust0/ridb/worker'
    ],
  }));
}