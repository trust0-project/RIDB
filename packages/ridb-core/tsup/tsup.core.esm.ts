

import createConfig from '@trust0/ridb-build';
export default createConfig({
  format: ['esm'],
  entry: ['pkg/ridb_core.js'],
  banner:{
    js: `export const SchemaFieldType = {
/**
 * String type for text data
 */
string: 'string',

/**
 * Number type for numeric data (integers and floats)
 */
number: 'number',

/**
 * Boolean type for true/false values
 */
boolean: 'boolean',

/**
 * Array type for ordered collections of items
 */
array: 'array',

/**
 * Object type for nested document structures
 */
object: 'object',
};`
  }
});
