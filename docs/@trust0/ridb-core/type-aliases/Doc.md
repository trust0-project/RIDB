[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / Doc

# Type Alias: Doc\<T\>

> **Doc**\<`T`\> = `{ [K in keyof T["properties"]]: ExtractType<T["properties"][K]["type"]> }` & `object`

Defined in: ridb\_core.d.ts:468

Doc is a utility type that transforms a schema type into a document type where each property is mapped to its extracted type.

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

type Document = Doc<Schema>; // Document is { name: string; age: number; }
