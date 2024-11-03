[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / Doc

# Type Alias: Doc\<T\>

> **Doc**\<`T`\>: `{ [name in keyof T["properties"]]: ExtractType<T["properties"][name]["type"]> }`

Doc is a utility type that transforms a schema type into a document type where each property is mapped to its extracted type.

## Type Parameters

• **T** *extends* [`SchemaType`](SchemaType.md)

A schema type with a 'properties' field where each property's type is represented as a string.

type Document = Doc<Schema>; // Document is { name: string; age: number; }

## Defined in

pkg/ridb\_rust.d.ts:334
