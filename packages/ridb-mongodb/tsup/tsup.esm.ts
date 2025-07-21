import createConfig from '@trust0/ridb-build';

export default createConfig({
  format:[ 'esm'],
  entry: ['src/index.ts'],
  platform: 'node'
});
