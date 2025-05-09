import createConfig from '@trust0/ridb-build';

export default createConfig({
  format:[ 'cjs'],
  entry: ['src/index.ts', 'src/worker.ts', 'src/testing/index.ts'],
});
