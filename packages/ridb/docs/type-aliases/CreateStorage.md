[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / CreateStorage

# Type Alias: CreateStorage()

> **CreateStorage**: \<`T`\>(`records`) => `Promise`\<[`BaseStorage`](../classes/BaseStorage.md)\<`T`\>\>

Represents a function type for creating storage with the provided schema type records.

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](SchemaTypeRecord.md)

The schema type record.

## Parameters

### records

`T`

The schema type records.

## Returns

`Promise`\<[`BaseStorage`](../classes/BaseStorage.md)\<`T`\>\>

A promise that resolves to the created internals record.

## Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:425
