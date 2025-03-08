[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Operation

# Type Alias: Operation\<T\>

> **Operation**\<`T`\>: `object`

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:479

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

### opType

> **opType**: [`OpType`](../enumerations/OpType.md)

The type of operation to be performed (e.g., CREATE, UPDATE, DELETE).

### primaryKey?

> `optional` **primaryKey**: `string`

### primaryKeyField?

> `optional` **primaryKeyField**: `string`
