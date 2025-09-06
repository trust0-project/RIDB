[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb](../README.md) / StartOptions

# Type Alias: StartOptions\<T\>

> **StartOptions**\<`T`\> = `object`

Defined in: [types.ts:41](https://github.com/trust0-project/RIDB/blob/1178ca486da4caadbba0b876f695393e5ef3243c/packages/ridb/src/types.ts#L41)

Options for starting a database instance.

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/SchemaTypeRecord.md)

The schema type record defining the database structure

## Indexable

\[`name`: `string`\]: `any`

Additional custom options

## Properties

### dbName?

> `optional` **dbName**: `string`

Defined in: [types.ts:55](https://github.com/trust0-project/RIDB/blob/1178ca486da4caadbba0b876f695393e5ef3243c/packages/ridb/src/types.ts#L55)

Database name to use (overrides the name provided during initialization)

***

### password?

> `optional` **password**: `string`

Defined in: [types.ts:50](https://github.com/trust0-project/RIDB/blob/1178ca486da4caadbba0b876f695393e5ef3243c/packages/ridb/src/types.ts#L50)

Optional password for encrypting the database

***

### storageType?

> `optional` **storageType**: [`StorageClass`](StorageClass.md)\<`T`\> \| [`StorageType`](../enumerations/StorageType.md)

Defined in: [types.ts:45](https://github.com/trust0-project/RIDB/blob/1178ca486da4caadbba0b876f695393e5ef3243c/packages/ridb/src/types.ts#L45)

The storage type or custom storage class implementation to use
