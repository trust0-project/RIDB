[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / CreateDoc

# Type Alias: CreateDoc\<T\>

> **CreateDoc**\<`T`\> = `{ [K in keyof T["properties"] as IsCreateOptional<T, K> extends true ? K : never]?: ExtractProperty<T["properties"][K]> }` & `{ [K in keyof T["properties"] as IsCreateOptional<T, K> extends true ? never : K]: ExtractProperty<T["properties"][K]> }` & `object`

Defined in: ridb\_core.d.ts:693

CreateDoc is a utility type for document creation that properly handles required vs optional fields
during the creation process. Fields with default values, or fields not listed in the schema-level
`required` array, become optional.

## Type Declaration

### \_\_version?

> `optional` **\_\_version?**: `number`

### createdAt?

> `optional` **createdAt?**: `number`

### updatedAt?

> `optional` **updatedAt?**: `number`

## Type Parameters

### T

`T` *extends* [`SchemaType`](SchemaType.md)

A schema type with a 'properties' field where each property's type is represented as a string.
