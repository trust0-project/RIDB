
import esbuild from 'esbuild';
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
    outdir:"build",
    outExtension: { ".js": ".js" },
    target: ['esnext'],
    format: 'esm',
    plugins: [
        ...plugins
    ],
}).catch((err) => {
    console.log(err)
    process.exit(1)
});

