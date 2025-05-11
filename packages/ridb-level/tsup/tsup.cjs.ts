import createConfig from '@trust0/ridb-build';

export default createConfig({
  format:[ 'cjs'],
  entry: ['src/index.ts'],
  platform: 'node'
});
