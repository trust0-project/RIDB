[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / SchemaType

# Type Alias: SchemaType

> **SchemaType**: `object`

Represents the type definition for a schema.

## Type declaration

### encrypted?

> `optional` **encrypted**: `string`[]

### indexes?

> `optional` **indexes**: `string`[]

### primaryKey

> **primaryKey**: `string`

The primary key of the schema.

### properties

> **properties**: `object`

The properties defined in the schema.

#### Index Signature

 \[`name`: `string`\]: [`Property`](../classes/Property.md)

### type

> **type**: `string`

The type of the schema.

### version

> **version**: `number`

The version of the schema.

## Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:556
