import { defineConfig } from 'tsup';
import { generic, plugins } from '../../esbuild.base';
import { wasmPlugin } from '../../esbuild.base.js';

export default defineConfig(({ watch }) => ({
  entry: ['src/index.tsx'],
  format: 'cjs',
  outDir: 'build',
  target: 'esnext',
  minify: true,
  clean: true,
  esbuildPlugins: [
    wasmPlugin,
    ...plugins
  ],
  external: ['buffer', 'next', 'react-server-dom-webpack', 'tsup', 'react-server-dom-webpack/client.edge'],
  esbuildOptions(options, context) {
    options = {
        ...options,
        ...generic,
        format:'cjs',
    platform: 'node',
    entryPoints: ['src/index.ts'],
    external: ['buffer', '@trust0/ridb', '@trust0/ridb-core'],
    banner: {
        js: `if (typeof Buffer === 'undefined') {
    global.Buffer = require('buffer').Buffer;
}
`}
    }
  },
  outExtension({ format }) {
    return {
      js: `.js`,
    };
  },
}));