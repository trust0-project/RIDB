import { NodeResolvePlugin } from '@esbuild-plugins/node-resolve';

import fs from 'fs';
import path from 'path';


export const wasmPlugin = {
    name: 'wasm',
    setup(build) {
        build.onResolve({ filter: /\.wasm$/ }, args => {
            return { path: path.resolve('../../node_modules', args.path), namespace: 'wasm' };
        });
        build.onLoad({ filter: /.*/, namespace: 'wasm' }, async (args) => {
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

export const generic = {
    chunkNames: "[name][hash]",
    assetNames: "[name][hash]",
    sourcemap: false,
    bundle: true,
    platform: 'neutral',
    splitting: true,
    resolveExtensions: ['.ts', '.js', '.wasm'],
    inject: [],
    mainFields: ['module', 'main'],
    banner: {
        js:  "\nif (typeof Buffer === 'undefined') {\n  global.Buffer = require('buffer').Buffer;\n}",
    },
    define: {
        'global.Buffer': 'Buffer',
    },
    external: ['buffer'],
    outdir:"build",
    outExtension: { ".js": ".js" },
    target: ['esnext'],
    format: 'esm',
}