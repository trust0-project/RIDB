[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Doc

# Type Alias: Doc\<T\>

> **Doc**\<`T`\>: `{ [K in keyof T["properties"] as IsOptional<T["properties"][K]> extends true ? K : never]?: ExtractType<T["properties"][K]["type"]> }` & `{ [K in keyof T["properties"] as IsOptional<T["properties"][K]> extends false ? K : never]: ExtractType<T["properties"][K]["type"]> }` & `object`

Defined in: ridb-core/pkg/ridb\_core.d.ts:498

Doc is a utility type that transforms a schema type into a document type where each property is mapped to its extracted type.

## Type declaration

### \_\_version?

> `optional` **\_\_version**: `number`

## Type Parameters

• **T** *extends* [`SchemaType`](SchemaType.md)

A schema type with a 'properties' field where each property's type is represented as a string.

type Document = Doc<Schema>; // Document is { name: string; age: number; }
