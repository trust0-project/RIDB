[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb](../README.md) / DBOptions

# Type Alias: DBOptions\<T\>

> **DBOptions**\<`T`\> = `object` & [`MigrationsParameter`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/MigrationsParameter.md)\<`T`\>

Defined in: [types.ts:68](https://github.com/trust0-project/RIDB/blob/adcea5a8e0ca2a9f4529ab773589d792e4b69ed9/packages/ridb/src/types.ts#L68)

Options for initializing the RIDB database.

## Type Declaration

### ~~dbName?~~

> `optional` **dbName?**: `string`

Database name

#### Deprecated

Use the dbName option in the start method instead.

### plugins?

> `optional` **plugins?**: *typeof* `BasePlugin`[]

Optional plugins to extend database functionality

### schemas

> **schemas**: `T`

Schema definitions for all collections in the database

### worker?

> `optional` **worker?**: `boolean`

Whether to use a SharedWorker for database operations

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/SchemaTypeRecord.md)

The schema type record defining the database structure
