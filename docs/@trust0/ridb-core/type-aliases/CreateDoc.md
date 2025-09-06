[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / CreateDoc

# Type Alias: CreateDoc\<T\>

> **CreateDoc**\<`T`\> = `{ [K in keyof T["properties"] as IsOptional<T["properties"][K]> extends true ? K : never]?: ExtractType<T["properties"][K]["type"]> }` & `{ [K in keyof T["properties"] as IsOptional<T["properties"][K]> extends true ? never : K]: ExtractType<T["properties"][K]["type"]> }` & `object`

Defined in: ridb\_core.d.ts:331

CreateDoc is a utility type for document creation that properly handles required vs optional fields
during the creation process. Fields with default values or required: false become optional.

## Type declaration

### \_\_version?

> `optional` **\_\_version**: `number`

### createdAt?

> `optional` **createdAt**: `number`

### updatedAt?

> `optional` **updatedAt**: `number`

## Type Parameters

### T

`T` *extends* [`SchemaType`](SchemaType.md)

A schema type with a 'properties' field where each property's type is represented as a string.
