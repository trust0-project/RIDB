[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / Doc

# Type Alias: Doc\<T\>

> **Doc**\<`T`\> = `{ [K in keyof T["properties"] as IsCreateOptional<T, K> extends true ? never : K]: ExtractProperty<T["properties"][K]> }` & `{ [K in keyof T["properties"] as IsCreateOptional<T, K> extends true ? K : never]?: ExtractProperty<T["properties"][K]> }` & `object`

Defined in: ridb\_core.d.ts:674

Doc is a utility type that transforms a schema type into a stored-document type. A
property is mandatory only when the validator guarantees its presence; properties that
are optional at creation (not listed in `required`, flagged `required: false`, or
carrying a `default`) may be absent on a stored document, so they are optional here
too. This keeps `find`/`findById`/`create` return types from claiming keys that may not
exist. Optionality uses the same [IsCreateOptional](IsCreateOptional.md) rules as [CreateDoc](CreateDoc.md).

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

type Document = Doc<Schema>; // Document is { name: string; age: number; }
