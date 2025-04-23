
import esbuild from 'esbuild';
import { generic } from '../../esbuild.base.js';

// Build ES module
esbuild.build({
    ...generic,
    platform: 'node',
    entryPoints: ['src/index.ts'],
    external: ['buffer', '@trust0/ridb', '@trust0/ridb-core'],
    banner: {
        js: `import { createRequire } from 'module';
const require = createRequire(import.meta.url);
if (typeof Buffer === 'undefined') {
    global.Buffer = require('buffer').Buffer;
}
`}
}).catch((err) => {
    console.log(err)
    process.exit(1)
});

