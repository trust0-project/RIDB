
import esbuild from 'esbuild';
import { generic } from '../../esbuild.base.js';

// Build ES module
esbuild.build({
    ...generic,
    platform: 'node',
    entryPoints: ['src/index.ts'],
    external: ['buffer', '@trust0/ridb','@trust0/ridb-core'],
}).catch((err) => {
    console.log(err)
    process.exit(1)
});

