[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / SchemaType

# Type Alias: SchemaType

> **SchemaType** = `object`

Defined in: ridb\_core.d.ts:763

Represents the type definition for a schema.

## Properties

### encrypted?

> `optional` **encrypted?**: `string`[]

Defined in: ridb\_core.d.ts:779

***

### indexes?

> `optional` **indexes?**: `string`[]

Defined in: ridb\_core.d.ts:778

***

### primaryKey

> **primaryKey**: `string`

Defined in: ridb\_core.d.ts:772

The primary key of the schema.

***

### properties

> **properties**: `object`

Defined in: ridb\_core.d.ts:788

The properties defined in the schema.

#### Index Signature

\[`name`: `string`\]: [`Property`](../classes/Property.md)

***

### required?

> `optional` **required?**: `string`[]

Defined in: ridb\_core.d.ts:784

The names of the required top-level properties. Follows JSON Schema
semantics: only the listed properties are required.

***

### type

> **type**: `SchemaFieldType`

Defined in: ridb\_core.d.ts:777

The type of the schema.

***

### version

> **version**: `number`

Defined in: ridb\_core.d.ts:767

The version of the schema.
