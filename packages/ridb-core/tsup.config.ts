import { defineConfig } from 'tsup';
import { generic, plugins } from '../../esbuild.base';
import { wasmPlugin } from '../../esbuild.base.js';

export default defineConfig(({ watch }) => ({
  entry: ['pkg/ridb_core.js', 'pkg/ridb_core_bg.wasm'],
  format: 'esm',
  outDir: 'build',
  target: 'esnext',
  minify: false,
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
  external: ['buffer', 'next','vitest', 'react-server-dom-webpack', 'tsup', 'react-server-dom-webpack/client.edge'],
  esbuildOptions(options, context) {
    options = {
        ...options,
        ...generic,
        format:'cjs'
    } as any
  },
  outExtension({ format }) {
    return {
      js: `.js`,
    };
  },
}));