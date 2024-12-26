
import esbuild from 'esbuild';
import fs from 'fs';
import path from 'path';
import { NodeResolvePlugin } from '@esbuild-plugins/node-resolve';


const plugins = [
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

const generic = {
    chunkNames: "[name]",
    assetNames: "[name]",
    entryPoints: ['src/index.ts'],
    sourcemap: false,
    bundle: false,
    platform: 'neutral',
    splitting: false,
    resolveExtensions: ['.ts', '.js', '.wasm'],
    inject: [],
    mainFields: ['module', 'main'],
}

// Build ES module
esbuild.build({
    ...generic,
    outdir:"build/esm",
    outExtension: { ".js": ".mjs" },
    target: ['esnext'],
    format: 'esm',
    plugins: [
        ...plugins
    ],
}).then(() => {
    esbuild.build({
        ...generic,
        outdir:"build/cjs",
        outExtension: { ".js": ".cjs" },
        target: ['es6'],
        format: 'cjs',
        plugins: [
            ...plugins
        ],
    }).catch((err) => {
        console.log(err)
        process.exit(1)
    });
})

    .catch((err) => {
    console.log(err)
    process.exit(1)
});

