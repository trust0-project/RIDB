import { defineConfig } from 'tsup';
import { generic, plugins } from '../../esbuild.base';
import { wasmPlugin } from '../../esbuild.base.js';

export default defineConfig(({ watch }) => ({
  entry: ['src/index.ts', 'src/worker.ts', 'src/testing/index.ts'],
  format: 'cjs',
  outDir: 'build',
  target: 'esnext',
  minify: true,
  clean: true,
  esbuildPlugins: [
    wasmPlugin,
    ...plugins
  ],
  banner: {
    js: `if (typeof Buffer === 'undefined') {
global.Buffer = require('buffer').Buffer;
}
`},
  external: [
    'buffer',
    'next',
    'vitest',
    'react-server-dom-webpack',
    'tsup',
    'react-server-dom-webpack/client.edge',
    '@trust0/ridb-core',
    '@trust0/ridb/worker'
  ],
  esbuildOptions(options, context) {
    options = {
      ...options,
      ...generic,
      format: 'cjs' as const,
    } as any
  },
}));