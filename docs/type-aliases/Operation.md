[**@trust0/ridb**](../README.md) • **Docs**

***

[@trust0/ridb](../README.md) / Operation

# Type Alias: Operation\<T\>

> **Operation**\<`T`\>: `object`

Represents an operation to be performed on a collection.

## Type Parameters

• **T** *extends* [`SchemaType`](SchemaType.md)

The schema type of the collection.

## Type declaration

### collection

> **collection**: `string`

The name of the collection on which the operation will be performed.

### data

> **data**: [`Doc`](Doc.md)\<`T`\>

The data involved in the operation, conforming to the schema type.

### indexes

> **indexes**: `string`[]

An array of indexes related to the operation.

### opType

> **opType**: [`OpType`](../enumerations/OpType.md)

The type of operation to be performed (e.g., CREATE, UPDATE, DELETE).

## Defined in

pkg/ridb\_rust.d.ts:115
