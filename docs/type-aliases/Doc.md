[**@trust0/ridb**](../README.md) • **Docs**

***

[@trust0/ridb](../README.md) / Doc

# Type Alias: Doc\<T\>

> **Doc**\<`T`\>: `{ [name in keyof T["properties"]]: ExtractType<T["properties"][name]["type"]> }` & `object`

Doc is a utility type that transforms a schema type into a document type where each property is mapped to its extracted type.

## Type declaration

### \_\_version?

> `optional` **\_\_version**: `number`

## Type Parameters

• **T** *extends* [`SchemaType`](SchemaType.md)

A schema type with a 'properties' field where each property's type is represented as a string.

type Document = Doc<Schema>; // Document is { name: string; age: number; }

## Defined in

pkg/ridb\_rust.d.ts:91
