[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / MigrationPathsForSchema

# Type Alias: MigrationPathsForSchema\<T\>

> **MigrationPathsForSchema**\<`T`\> = `T`\[`"version"`\] *extends* `0` ? `object` : `{ [K in EnumerateFrom1To<T["version"]>]: MigrationFunction<T> }`

Defined in: ridb\_core.d.ts:543

## Type Parameters

### T

`T` *extends* [`SchemaType`](SchemaType.md)
