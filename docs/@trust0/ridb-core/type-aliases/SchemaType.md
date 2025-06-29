[**Documentation**](../../../README.md)

***

[Documentation](../../../packages.md) / [@trust0/ridb-core](../README.md) / SchemaType

# Type Alias: SchemaType

> **SchemaType** = `object`

Defined in: ridb\_core.d.ts:415

Represents the type definition for a schema.

## Properties

### encrypted?

> `optional` **encrypted**: `string`[]

Defined in: ridb\_core.d.ts:431

***

### indexes?

> `optional` **indexes**: `string`[]

Defined in: ridb\_core.d.ts:430

***

### primaryKey

> **primaryKey**: `string`

Defined in: ridb\_core.d.ts:424

The primary key of the schema.

***

### properties

> **properties**: `object`

Defined in: ridb\_core.d.ts:435

The properties defined in the schema.

#### Index Signature

\[`name`: `string`\]: [`Property`](../classes/Property.md)

***

### type

> **type**: `SchemaFieldType`

Defined in: ridb\_core.d.ts:429

The type of the schema.

***

### version

> **version**: `number`

Defined in: ridb\_core.d.ts:419

The version of the schema.
