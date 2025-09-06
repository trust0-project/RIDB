[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / Operation

# Type Alias: Operation\<T\>

> **Operation**\<`T`\> = `object`

Defined in: ridb\_core.d.ts:655

Represents an operation to be performed on a collection.

## Type Parameters

### T

`T` *extends* [`SchemaType`](SchemaType.md) = [`SchemaType`](SchemaType.md)

The schema type of the collection.

## Properties

### collection

> **collection**: `string`

Defined in: ridb\_core.d.ts:659

The name of the collection on which the operation will be performed.

***

### data

> **data**: [`Doc`](Doc.md)\<`T`\>

Defined in: ridb\_core.d.ts:669

The data involved in the operation, conforming to the schema type.

***

### opType

> **opType**: [`OpType`](../enumerations/OpType.md)

Defined in: ridb\_core.d.ts:664

The type of operation to be performed (e.g., CREATE, UPDATE, DELETE).

***

### primaryKey?

> `optional` **primaryKey**: `string`

Defined in: ridb\_core.d.ts:672

***

### primaryKeyField?

> `optional` **primaryKeyField**: `string`

Defined in: ridb\_core.d.ts:671
