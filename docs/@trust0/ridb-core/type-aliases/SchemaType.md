[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / SchemaType

# Type Alias: SchemaType

> **SchemaType** = `object`

Defined in: ridb\_core.d.ts:655

Represents the type definition for a schema.

## Properties

### encrypted?

> `optional` **encrypted**: `string`[]

Defined in: ridb\_core.d.ts:671

***

### indexes?

> `optional` **indexes**: `string`[]

Defined in: ridb\_core.d.ts:670

***

### primaryKey

> **primaryKey**: `string`

Defined in: ridb\_core.d.ts:664

The primary key of the schema.

***

### properties

> **properties**: `object`

Defined in: ridb\_core.d.ts:675

The properties defined in the schema.

#### Index Signature

\[`name`: `string`\]: [`Property`](../classes/Property.md)

***

### type

> **type**: `SchemaFieldType`

Defined in: ridb\_core.d.ts:669

The type of the schema.

***

### version

> **version**: `number`

Defined in: ridb\_core.d.ts:659

The version of the schema.
