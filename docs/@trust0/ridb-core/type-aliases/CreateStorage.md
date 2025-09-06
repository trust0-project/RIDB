[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / CreateStorage

# Type Alias: CreateStorage()

> **CreateStorage** = \<`T`\>(`records`) => `Promise`\<[`BaseStorage`](../classes/BaseStorage.md)\<`T`\>\>

Defined in: ridb\_core.d.ts:359

Represents a function type for creating storage with the provided schema type records.

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](SchemaTypeRecord.md)

The schema type record.

## Parameters

### records

`T`

The schema type records.

## Returns

`Promise`\<[`BaseStorage`](../classes/BaseStorage.md)\<`T`\>\>

A promise that resolves to the created internals record.
