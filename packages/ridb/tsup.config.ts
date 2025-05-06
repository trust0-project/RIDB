import { defineConfig } from 'tsup';
import { generic, plugins } from '../../esbuild.base';
import { wasmPlugin } from '../../esbuild.base.js';

export default defineConfig(({ watch }) => ({
  entry: ['src/index.ts','src/worker.ts','src/testing/index.ts'],
  format: 'esm',
  outDir: 'build',
  target: 'esnext',
  minify: true,
  clean: true,
  esbuildPlugins: [
    wasmPlugin,
    ...plugins
  ],
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
`},
  external: ['buffer', 'next','vitest', 'react-server-dom-webpack', 'tsup', 'react-server-dom-webpack/client.edge'],
  esbuildOptions(options, context) {
    options = {
        ...options,
        ...generic,
        format:'esm' as const,
    } as any
  },
  outExtension({ format }) {
    return {
      js: `.js`,
    };
  },
}));