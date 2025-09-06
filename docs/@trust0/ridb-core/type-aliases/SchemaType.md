[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / SchemaType

# Type Alias: SchemaType

> **SchemaType** = `object`

Defined in: ridb\_core.d.ts:79

Represents the type definition for a schema.

## Properties

### encrypted?

> `optional` **encrypted**: `string`[]

Defined in: ridb\_core.d.ts:95

***

### indexes?

> `optional` **indexes**: `string`[]

Defined in: ridb\_core.d.ts:94

***

### primaryKey

> **primaryKey**: `string`

Defined in: ridb\_core.d.ts:88

The primary key of the schema.

***

### properties

> **properties**: `object`

Defined in: ridb\_core.d.ts:99

The properties defined in the schema.

#### Index Signature

\[`name`: `string`\]: [`Property`](../classes/Property.md)

***

### type

> **type**: `SchemaFieldType`

Defined in: ridb\_core.d.ts:93

The type of the schema.

***

### version

> **version**: `number`

Defined in: ridb\_core.d.ts:83

The version of the schema.
