[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / DBOptions

# Type Alias: DBOptions\<T\>

> **DBOptions**\<`T`\> = `object` & `MigrationsParameter`\<`T`\>

Defined in: [index.ts:141](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L141)

Options for the RIDB constructor.

## Type declaration

### dbName

> **dbName**: `string`

### plugins?

> `optional` **plugins**: *typeof* `BasePlugin`[]

### schemas

> **schemas**: `T`

### worker?

> `optional` **worker**: `boolean`

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord` = `SchemaTypeRecord`
