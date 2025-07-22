[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / SchemaType

# Type Alias: SchemaType

> **SchemaType** = `object`

Defined in: ridb\_core.d.ts:143

Represents the type definition for a schema.

## Properties

### encrypted?

> `optional` **encrypted**: `string`[]

Defined in: ridb\_core.d.ts:159

***

### indexes?

> `optional` **indexes**: `string`[]

Defined in: ridb\_core.d.ts:158

***

### primaryKey

> **primaryKey**: `string`

Defined in: ridb\_core.d.ts:152

The primary key of the schema.

***

### properties

> **properties**: `object`

Defined in: ridb\_core.d.ts:163

The properties defined in the schema.

#### Index Signature

\[`name`: `string`\]: [`Property`](../classes/Property.md)

***

### type

> **type**: `SchemaFieldType`

Defined in: ridb\_core.d.ts:157

The type of the schema.

***

### version

> **version**: `number`

Defined in: ridb\_core.d.ts:147

The version of the schema.
