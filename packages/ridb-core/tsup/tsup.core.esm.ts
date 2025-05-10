

import createConfig from '@trust0/ridb-build';
import banner from './banner'
export default createConfig({
  format: ['esm'],
  entry: ['pkg/ridb_core.js'],
    banner:banner
});
