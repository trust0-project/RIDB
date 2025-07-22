[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / SchemaType

# Type Alias: SchemaType

> **SchemaType** = `object`

Defined in: ridb\_core.d.ts:97

Represents the type definition for a schema.

## Properties

### encrypted?

> `optional` **encrypted**: `string`[]

Defined in: ridb\_core.d.ts:113

***

### indexes?

> `optional` **indexes**: `string`[]

Defined in: ridb\_core.d.ts:112

***

### primaryKey

> **primaryKey**: `string`

Defined in: ridb\_core.d.ts:106

The primary key of the schema.

***

### properties

> **properties**: `object`

Defined in: ridb\_core.d.ts:117

The properties defined in the schema.

#### Index Signature

\[`name`: `string`\]: [`Property`](../classes/Property.md)

***

### type

> **type**: `SchemaFieldType`

Defined in: ridb\_core.d.ts:111

The type of the schema.

***

### version

> **version**: `number`

Defined in: ridb\_core.d.ts:101

The version of the schema.
