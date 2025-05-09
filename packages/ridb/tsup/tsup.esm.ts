import createConfig from '@trust0/ridb-build';

export default createConfig({
  format:[ 'esm'],
  entry: ['src/index.ts', 'src/worker.ts', 'src/testing/index.ts'],
});
