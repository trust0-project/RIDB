[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / DBOptions

# Type Alias: DBOptions\<T\>

> **DBOptions**\<`T`\> = `object` & `MigrationsParameter`\<`T`\>

Defined in: [types.ts:29](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/types.ts#L29)

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
