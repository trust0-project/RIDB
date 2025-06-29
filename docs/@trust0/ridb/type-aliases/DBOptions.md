[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb](../README.md) / DBOptions

# Type Alias: DBOptions\<T\>

> **DBOptions**\<`T`\> = `object` & `MigrationsParameter`\<`T`\>

Defined in: [types.ts:71](https://github.com/trust0-project/RIDB/blob/3305fbba832297c7c9cd86d5d666516d921be44e/packages/ridb/src/types.ts#L71)

Options for initializing the RIDB database.

## Type declaration

### ~~dbName?~~

> `optional` **dbName**: `string`

Database name

#### Deprecated

Use the dbName option in the start method instead.

### plugins?

> `optional` **plugins**: *typeof* `BasePlugin`[]

Optional plugins to extend database functionality

### schemas

> **schemas**: `T`

Schema definitions for all collections in the database

### worker?

> `optional` **worker**: `boolean`

Whether to use a SharedWorker for database operations

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/docs/@trust0/ridb-core/type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/docs/@trust0/ridb-core/type-aliases/SchemaTypeRecord.md)

The schema type record defining the database structure
