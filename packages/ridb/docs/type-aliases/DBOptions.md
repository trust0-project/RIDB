[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / DBOptions

# Type Alias: DBOptions\<T\>

> **DBOptions**\<`T`\> = `object` & `MigrationsParameter`\<`T`\>

Defined in: [types.ts:29](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/types.ts#L29)

Options for the RIDB constructor.

## Type declaration

### ~~dbName?~~

> `optional` **dbName**: `string`

#### Deprecated

Use the dbName option in the start method instead.

### plugins?

> `optional` **plugins**: *typeof* `BasePlugin`[]

### schemas

> **schemas**: `T`

### worker?

> `optional` **worker**: `boolean`

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord` = `SchemaTypeRecord`
