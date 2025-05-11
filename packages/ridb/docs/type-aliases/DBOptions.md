[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / DBOptions

# Type Alias: DBOptions\<T\>

> **DBOptions**\<`T`\> = `object` & `MigrationsParameter`\<`T`\>

Defined in: [index.ts:141](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L141)

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
