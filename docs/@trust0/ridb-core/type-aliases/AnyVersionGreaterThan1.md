[**Documentation**](../../../README.md)

***

[Documentation](../../../packages.md) / [@trust0/ridb-core](../README.md) / AnyVersionGreaterThan1

# Type Alias: AnyVersionGreaterThan1\<T\>

> **AnyVersionGreaterThan1**\<`T`\> = `true` *extends* `{ [K in keyof T]: IsVersionGreaterThan0<T[K]["version"]> }`\[keyof `T`\] ? `true` : `false`

Defined in: ridb\_core.d.ts:535

## Type Parameters

### T

`T` *extends* `Record`\<`string`, [`SchemaType`](SchemaType.md)\>
