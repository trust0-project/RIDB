import { defineConfig } from 'tsup';

export default defineConfig(({ watch }) => ({
  entry: ['src/index.ts'],
  format: 'esm',
  outDir: 'build',
  target: 'esnext',
  bundle: false,
  minify: true,
  clean: true,
  external: [
    'esbuild'
  ]
}));