[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / RequiredFieldNames

# Type Alias: RequiredFieldNames\<T\>

> **RequiredFieldNames**\<`T`\> = `T` *extends* `object` ? `R` *extends* readonly `string`[] ? `R`\[`number`\] : `never` : `never`

Defined in: ridb\_core.d.ts:641

The union of property names marked required at the schema level (JSON Schema
`required` array). Resolves to `never` when no `required` array is present.

## Type Parameters

### T

`T` *extends* [`SchemaType`](SchemaType.md)
