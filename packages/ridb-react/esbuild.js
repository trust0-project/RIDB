import esbuild from 'esbuild';
import { globPlugin } from 'esbuild-plugin-glob';
import { nodeExternalsPlugin } from 'esbuild-node-externals';

esbuild.build({
  entryPoints: ['src/index.tsx'],
  outdir: 'build',
  bundle: true,
  sourcemap: true,
  format: 'esm',
  target: ['esnext'],
  jsxFactory: 'React.createElement',
  jsxFragment: 'React.Fragment',
  plugins: [
    globPlugin(),
    nodeExternalsPlugin()
  ],
  external: [
    'react',
    'react-dom'
  ]
})
.then(() => {
  console.log('Build complete');
})
.catch(() => process.exit(1));