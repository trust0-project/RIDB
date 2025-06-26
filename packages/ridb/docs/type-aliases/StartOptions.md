[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / StartOptions

# Type Alias: StartOptions\<T\>

> **StartOptions**\<`T`\> = `object`

Defined in: [types.ts:44](https://github.com/trust0-project/RIDB/blob/132707f2a67423a7d9b542db2e8d36b2c6b917ad/packages/ridb/src/types.ts#L44)

Options for starting a database instance.

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord`

The schema type record defining the database structure

## Indexable

\[`name`: `string`\]: `any`

Additional custom options

## Properties

### dbName?

> `optional` **dbName**: `string`

Defined in: [types.ts:58](https://github.com/trust0-project/RIDB/blob/132707f2a67423a7d9b542db2e8d36b2c6b917ad/packages/ridb/src/types.ts#L58)

Database name to use (overrides the name provided during initialization)

***

### password?

> `optional` **password**: `string`

Defined in: [types.ts:53](https://github.com/trust0-project/RIDB/blob/132707f2a67423a7d9b542db2e8d36b2c6b917ad/packages/ridb/src/types.ts#L53)

Optional password for encrypting the database

***

### storageType?

> `optional` **storageType**: [`StorageClass`](StorageClass.md)\<`T`\> \| [`StorageType`](../enumerations/StorageType.md)

Defined in: [types.ts:48](https://github.com/trust0-project/RIDB/blob/132707f2a67423a7d9b542db2e8d36b2c6b917ad/packages/ridb/src/types.ts#L48)

The storage type or custom storage class implementation to use
