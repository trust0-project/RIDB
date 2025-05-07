
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
import pathWorkaround from 'path';
import {fileURLToPath} from 'url';
const require = createRequire(import.meta.url);
global.__filename = fileURLToPath(import.meta.url);
global.__dirname = pathWorkaround.dirname(__filename);
if (typeof Buffer === 'undefined') {
    global.Buffer = require('buffer').Buffer;
}
`}
}).catch((err) => {
    console.log(err)
    process.exit(1)
});

