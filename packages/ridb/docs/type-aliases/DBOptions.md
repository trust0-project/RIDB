[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / DBOptions

# Type Alias: DBOptions\<T\>

> **DBOptions**\<`T`\> = `object` & `MigrationsParameter`\<`T`\>

Defined in: [index.ts:140](https://github.com/trust0-project/RIDB/blob/0889f61b208d392afc348597eadd8727a108be5a/packages/ridb/src/index.ts#L140)

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
