[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / StorageClass

# Type Alias: StorageClass\<T\>

> **StorageClass**\<`T`\> = `object`

Defined in: [types.ts:8](https://github.com/trust0-project/RIDB/blob/104aa2879acd25a4cc9a5ad43a4aff29b2b5117a/packages/ridb/src/types.ts#L8)

Represents a factory class for creating storage instances.

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord`

The schema type record defining the database structure

## Properties

### create()

> **create**: (`name`, `schemas`, `options`) => `Promise`\<`BaseStorage`\<`T`\>\>

Defined in: [types.ts:17](https://github.com/trust0-project/RIDB/blob/104aa2879acd25a4cc9a5ad43a4aff29b2b5117a/packages/ridb/src/types.ts#L17)

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

`Promise`\<`BaseStorage`\<`T`\>\>

A Promise resolving to the created storage instance
