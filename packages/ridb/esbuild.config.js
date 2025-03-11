
import esbuild from 'esbuild';
import { generic, wasmPlugin, plugins } from '../../esbuild.base.js';

await esbuild.build({
    ...generic,
    entryPoints: ['src/index.ts','src/worker.ts','src/testing/index.ts'],
    plugins: [
        wasmPlugin,
        ...plugins
    ],
    outExtension: { ".js": ".js" },
})
