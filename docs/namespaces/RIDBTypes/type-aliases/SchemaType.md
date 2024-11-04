[**@trust0/ridb**](../../../README.md) • **Docs**

***

[@trust0/ridb](../../../README.md) / [RIDBTypes](../README.md) / SchemaType

# Type Alias: SchemaType

> **SchemaType**: `object`

Represents the type definition for a schema.

## Type declaration

### encrypted?

> `readonly` `optional` **encrypted**: `string`[]

### indexes?

> `readonly` `optional` **indexes**: `string`[]

An optional array of indexes.

### primaryKey

> `readonly` **primaryKey**: `string`

The primary key of the schema.

### properties

> `readonly` **properties**: `object`

The properties defined in the schema.

#### Index Signature

 \[`name`: `string`\]: [`Property`](../classes/Property.md)

### required?

> `readonly` `optional` **required**: `string`[]

An optional array of required fields.

### type

> `readonly` **type**: `string`

The type of the schema.

### version

> `readonly` **version**: `number`

The version of the schema.

## Defined in

pkg/ridb\_rust.d.ts:397
