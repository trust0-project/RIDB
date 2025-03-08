[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / MigrationPathsForSchema

# Type Alias: MigrationPathsForSchema\<T\>

> **MigrationPathsForSchema**\<`T`\>: `T`\[`"version"`\] *extends* `0` ? `object` : `{ [K in EnumerateFrom1To<T["version"]>]: MigrationFunction<T> }`

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:524

## Type Parameters

• **T** *extends* [`SchemaType`](SchemaType.md)
