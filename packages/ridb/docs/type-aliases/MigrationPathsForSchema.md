[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / MigrationPathsForSchema

# Type Alias: MigrationPathsForSchema\<T\>

> **MigrationPathsForSchema**\<`T`\>: `T`\[`"version"`\] *extends* `0` ? `object` : `{ [K in EnumerateFrom1To<T["version"]>]: MigrationFunction<T> }`

## Type Parameters

• **T** *extends* [`SchemaType`](SchemaType.md)

## Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:123
