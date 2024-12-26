[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Doc

# Type Alias: Doc\<T\>

> **Doc**\<`T`\>: \{ \[K in keyof T\["properties"\] as T\["properties"\]\[K\]\["required"\] extends false \| (T\["properties"\]\[K\]\["default"\] extends undefined ? true : false) ? K : never\]?: ExtractType\<T\["properties"\]\[K\]\["type"\]\> \} & `{ [K in keyof T["properties"] as T["properties"][K]["required"] extends false ? never : K]: ExtractType<T["properties"][K]["type"]> }` & `object`

Doc is a utility type that transforms a schema type into a document type where each property is mapped to its extracted type.

## Type declaration

### \_\_version?

> `optional` **\_\_version**: `number`

## Type Parameters

• **T** *extends* [`SchemaType`](SchemaType.md)

A schema type with a 'properties' field where each property's type is represented as a string.

type Document = Doc<Schema>; // Document is { name: string; age: number; }

## Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:473