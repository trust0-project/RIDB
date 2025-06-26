[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / DBOptions

# Type Alias: DBOptions\<T\>

> **DBOptions**\<`T`\> = `object` & `MigrationsParameter`\<`T`\>

Defined in: [types.ts:71](https://github.com/trust0-project/RIDB/blob/132707f2a67423a7d9b542db2e8d36b2c6b917ad/packages/ridb/src/types.ts#L71)

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

`T` *extends* `SchemaTypeRecord` = `SchemaTypeRecord`

The schema type record defining the database structure
