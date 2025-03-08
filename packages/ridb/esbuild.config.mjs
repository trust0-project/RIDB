
import esbuild from 'esbuild';
import { generic, wasmPlugin, plugins } from '../../esbuild.base.mjs';

// Build ES module
esbuild.build({
    ...generic,
    entryPoints: ['src/index.ts', 'src/worker.ts', 'src/testing/index.ts'],
    plugins: [
        wasmPlugin,
        ...plugins
    ],
}).catch((err) => {
    console.log(err)
    process.exit(1)
});

