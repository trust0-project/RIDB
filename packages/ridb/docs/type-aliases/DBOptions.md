[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / DBOptions

# Type Alias: DBOptions\<T\>

> **DBOptions**\<`T`\> = `object` & `MigrationsParameter`\<`T`\>

Defined in: [index.ts:140](https://github.com/trust0-project/RIDB/blob/72c82bce7bddd4d481cf38c4e8b9492b9832268f/packages/ridb/src/index.ts#L140)

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
