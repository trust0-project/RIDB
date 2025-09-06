[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb](../README.md) / StorageClass

# Type Alias: StorageClass\<T\>

> **StorageClass**\<`T`\> = `object`

Defined in: [types.ts:9](https://github.com/trust0-project/RIDB/blob/9786676f4132a55aaec34d1edb0da16200ab0eba/packages/ridb/src/types.ts#L9)

Represents a factory class for creating storage instances.

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/SchemaTypeRecord.md)

The schema type record defining the database structure

## Properties

### create()

> **create**: (`name`, `schemas`, `options`) => `Promise`\<[`BaseStorage`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/classes/BaseStorage.md)\<`T`\>\>

Defined in: [types.ts:18](https://github.com/trust0-project/RIDB/blob/9786676f4132a55aaec34d1edb0da16200ab0eba/packages/ridb/src/types.ts#L18)

Creates a storage instance with the specified parameters.

#### Parameters

##### name

`string`

The name of the database

##### schemas

`T`

The schema definitions for the database collections

##### options

`any`

Additional options for the storage implementation

#### Returns

`Promise`\<[`BaseStorage`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/classes/BaseStorage.md)\<`T`\>\>

A Promise resolving to the created storage instance
